use super::DownloadSection;
use super::UploadSection;
use web_sys::HtmlSelectElement;
use yew::events::Event;
use yew::prelude::*;

#[function_component(MainContent)]
pub fn main_content() -> Html {
    html! {
        <div class="main-content">
            <UploadSection />
            <TransformationOptions />
            <DownloadSection />
        </div>
    }
}

#[function_component(TransformationOptions)]
fn transformation_options() -> Html {
    let selected_option = use_state(|| String::from("GA"));

    let on_option_change = {
        let selected_option = selected_option.clone();
        Callback::from(move |e: Event| {
            let target = e.target_dyn_into::<HtmlSelectElement>().unwrap();
            selected_option.set(target.value());
        })
    };

    html! {
        <div class="transformation-options">
            <h3>{"Transformation Options"}</h3>
            <select onchange={on_option_change}>
                <option value="GA">{"GA"}</option>
                <option value="GB">{"GB"}</option>
            </select>
            <input type="text" placeholder="Paramètre 1" />
            <input type="text" placeholder="Paramètre 2" />
        </div>
    }
}
