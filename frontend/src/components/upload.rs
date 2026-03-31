use gloo_storage::{SessionStorage, Storage};
use log::info;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, FormData, HtmlInputElement, RequestInit};
use yew::events::Event;
use yew::prelude::*;

#[function_component(UploadSection)]
pub fn upload_section() -> Html {
    let file_ref = use_node_ref();

    let handle_file_upload = {
        let file_ref = file_ref.clone();
        Callback::from(move |_: Event| {
            let input: HtmlInputElement = file_ref.cast().unwrap();
            if let Some(file) = input.files().and_then(|f| f.get(0)) {
                info!("Fichier sélectionné : {:?}", file.name());
            }
        })
    };

    let on_upload = {
        let file_ref = file_ref.clone();
        Callback::from(move |_| {
            let file_ref = file_ref.clone();
            spawn_local(async move {
                handle_upload(file_ref).await;
            });
        })
    };

    html! {
        <div class="upload-section">
            <div>
                <h2>{"Upload DICOM RD v2"}</h2>
                <input
                    type="file"
                    ref={file_ref}
                    onchange={handle_file_upload}
                    accept=".*"
                    enctype="multipart/form-data"
                />
            </div>
            <div>
                <button class="button" onclick={on_upload}>{"Upload"}</button>
            </div>
        </div>
    }
}

async fn handle_upload(file_ref: NodeRef) {
    if let Some(input) = file_ref.cast::<HtmlInputElement>() {
        if let Some(file) = input.files().and_then(|f| f.get(0)) {
            if let Ok(form_data) = FormData::new() {
                if let Err(e) = form_data.append_with_blob_and_filename("file", &file, "file") {
                    log::error!("Erreur append_with_blob : {:?}", e);
                    return;
                }

                send_upload_request(form_data).await;
            }
        }
    }
}

async fn send_upload_request(form_data: FormData) {
    if let Some(window) = window() {
        let init = RequestInit::new();
        init.set_method("POST");

        let body_value: JsValue = form_data.into();
        init.set_body(&body_value);

        let promise = window.fetch_with_str_and_init("http://localhost:8080/transform", &init);

        match wasm_bindgen_futures::JsFuture::from(promise).await {
            Ok(response) => {
                let response: web_sys::Response = response.into();
                handle_upload_response(&response).await;
            }
            Err(e) => log::error!("Erreur envoi requête : {:?}", e),
        }
    }
}

async fn handle_upload_response(response: &web_sys::Response) {
    match response.text() {
        Ok(promise) => match wasm_bindgen_futures::JsFuture::from(promise).await {
            Ok(text) => {
                if let Some(text_str) = text.as_string() {
                    parse_uuid_from_response(&text_str);
                }
            }
            Err(e) => log::error!("Erreur lecture réponse : {:?}", e),
        },
        Err(e) => log::error!("Erreur création promesse : {:?}", e),
    }
}

fn parse_uuid_from_response(text: &str) {
    if let Some(start_pos) = text.find("\"uuid\":\"") {
        let start: usize = start_pos + 8;
        if let Some(end_pos) = text[start..].find('"') {
            let end: usize = start + end_pos;
            let uuid: &str = &text[start..end];
            SessionStorage::set("dicom_uuid", uuid).expect("storage error");
        }
    }
}
