mod download;
mod main_content;
mod sidebar;
mod upload;

use download::DownloadSection;
use main_content::MainContent;
use sidebar::Sidebar;
use upload::UploadSection;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app-container">
            <Sidebar />
            <MainContent />
        </div>
    }
}
