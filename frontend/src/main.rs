use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Yew're welcome"}</h1>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
