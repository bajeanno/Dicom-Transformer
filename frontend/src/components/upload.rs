use log::info;
use web_sys::HtmlInputElement;
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
                // Ici, tu peux stocker le fichier dans un state ou le traiter.
            }
        })
    };

    html! {
        <div class="upload-section">
            <h2>{"Upload DICOM RD"}</h2>
            <input
                type="file"
                ref={file_ref}
                onchange={handle_file_upload}
                accept=".dcm"
            />
            <button>{"Upload"}</button>
        </div>
    }
}
