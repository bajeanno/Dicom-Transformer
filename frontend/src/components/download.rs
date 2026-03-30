use gloo_storage::SessionStorage;
use gloo_storage::Storage;
use js_sys;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Request, RequestInit, Response};
use yew::prelude::*;

#[function_component(DownloadSection)]
pub fn download_section() -> Html {
    let uuid = use_state(|| String::new());
    let on_download = {
        let uuid = uuid.clone();
        Callback::from(move |_| {
            let uuid_clone = (*uuid).clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Err(e) = download_transformed_file(uuid_clone).await {
                    log::error!("{}", e);
                }
            });
        })
    };

    html! {
        <div class="download-section">
            <button onclick={on_download} disabled={uuid.is_empty()}>
                {"Télécharger le fichier transformé"}
            </button>
        </div>
    }
}

async fn download_transformed_file(uuid: String) -> Result<(), String> {
    let url = format!("http://localhost:8080/download/{}", uuid);

    let window_obj = window().ok_or("No window object available")?;

    // 1. Fetch the file
    let bytes = fetch_file(&url).await?;

    // 2. Create blob with DICOM type
    let options = web_sys::BlobPropertyBag::new();
    options.set_type("application/dicom");

    let uint8array = js_sys::Uint8Array::from(bytes.as_slice());
    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
        &js_sys::Array::of1(&uint8array),
        &options,
    )
    .map_err(|_| "Error creating blob".to_string())?;

    // 3. Create download link
    let blob_url = web_sys::Url::create_object_url_with_blob(&blob)
        .map_err(|_| "Error creating object URL".to_string())?;

    // 4. Trigger download
    let document = window_obj.document().ok_or("No document available")?;

    let a = document
        .create_element("a")
        .map_err(|_| "Error creating element".to_string())?;

    a.set_attribute("href", &blob_url)
        .map_err(|_| "Error setting href".to_string())?;
    a.set_attribute("download", &format!("transformed_{}.dcm", uuid))
        .map_err(|_| "Error setting download attribute".to_string())?;

    let body = document.body().ok_or("No body element")?;

    body.append_child(&a)
        .map_err(|_| "Error appending element".to_string())?;

    a.dyn_ref::<web_sys::HtmlElement>()
        .ok_or("Error casting to HtmlElement")?
        .click();

    body.remove_child(&a)
        .map_err(|_| "Error removing element".to_string())?;

    web_sys::Url::revoke_object_url(&blob_url).map_err(|_| "Error revoking URL".to_string())?;

    Ok(())
}

async fn fetch_file(url: &str) -> Result<Vec<u8>, String> {
    let window_obj = window().ok_or("No window object")?;

    let init = RequestInit::new();
    init.set_method("GET");

    let request = Request::new_with_str_and_init(url, &init)
        .map_err(|_| "Error creating request".to_string())?;

    let fetch_promise = window_obj.fetch_with_request(&request);
    let response = JsFuture::from(fetch_promise)
        .await
        .map_err(|_| "Fetch failed".to_string())?;

    let response: Response = response
        .dyn_into()
        .map_err(|_| "Error casting response".to_string())?;

    if !response.ok() {
        return Err(format!("HTTP Error: {}", response.status()));
    }

    let array_buffer_promise = response
        .array_buffer()
        .map_err(|_| "Error getting array buffer".to_string())?;

    let array_buffer = JsFuture::from(array_buffer_promise)
        .await
        .map_err(|_| "Error reading array buffer".to_string())?;

    let array = js_sys::Uint8Array::new(&array_buffer);
    Ok(array.to_vec())
}
