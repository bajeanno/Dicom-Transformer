use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
    html! {
        <div class="sidebar">
            <nav>
                <ul>
                    <li><img src="/static/logo.jpg" href="#" width="70" height="50"/></li>
                    <li><a href="#">{"Accueil"}</a></li>
                    <li><a href="#">{"Historique"}</a></li>
                    <li><a href="#">{"Paramètres"}</a></li>
                </ul>
            </nav>
        </div>
    }
}
