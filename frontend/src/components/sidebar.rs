use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <div class="sidebar">
            <nav>
                <ul>
                    <li><a href="#">{"Accueil"}</a></li>
                    <li><a href="#">{"Historique"}</a></li>
                    <li><a href="#">{"Paramètres"}</a></li>
                </ul>
            </nav>
        </div>
    }
}
