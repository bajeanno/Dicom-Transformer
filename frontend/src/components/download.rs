use gloo_net::http::Request;
use js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys;
use yew::prelude::*;

#[function_component(DownloadSection)]
pub fn download_section() -> Html {
    let uuid = use_state(|| String::new());
    let on_download = {
        let uuid = uuid.clone();
        Callback::from(move |_| {
            let uuid_clone = (*uuid).clone();
            spawn_local(async move {
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
    // 1. Construis l'URL avec l'UUID
    let url = format!("http://localhost:3000/download/{}", uuid);

    // 2. Effectue la requête GET
    let response = Request::get(&url)
        .send()
        .await
        .map_err(|e| format!("Erreur lors de la requête : {:?}", e))?;

    // 3. Récupère les bytes du fichier
    let bytes = response
        .binary()
        .await
        .map_err(|e| format!("Erreur : {:?}", e))?;

    // 4. Crée un blob et un URL pour le téléchargement
    let options = web_sys::BlobPropertyBag::new();
    options.set_type("application/dicom");

    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
        &js_sys::Array::of1(&bytes.into()),
        &options,
    )
    .unwrap();

    let blob_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();

    // 5. Crée un lien invisible pour déclencher le téléchargement
    let document = web_sys::window().unwrap().document().unwrap();
    let a = document.create_element("a").unwrap();
    a.set_attribute("href", &blob_url).unwrap();
    a.set_attribute("download", &format!("transformed_{}.dcm", uuid))
        .unwrap();
    document.body().unwrap().append_child(&a).unwrap();
    a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
    document.body().unwrap().remove_child(&a).unwrap();

    web_sys::Url::revoke_object_url(&blob_url).unwrap();

    Ok(())
}
