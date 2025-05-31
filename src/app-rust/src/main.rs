use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus::{desktop::Config, desktop::WindowBuilder, prelude::*};
use std::path::PathBuf;


mod homepage;
use homepage::Homepage;
mod select_images;
use select_images::Select_images;
mod report;
use report::ReportView;
mod create_project;
use create_project::New_project;
mod manual_processor;
use manual_processor::ManualProcessor;
mod image_processor;


fn main() {
    dioxus::LaunchBuilder::desktop()
        .with_cfg(Config::new().with_window(WindowBuilder::new().with_resizable(true)))
        .launch(App);
}

#[component]
fn App() -> Element {
    let initial_folder_path: Signal<Option<PathBuf>> = Signal::new(None);
    use_context_provider(|| initial_folder_path);

    rsx! {
        Router::<Route> {}
    }
}

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[route("/")]
    Homepage {},
    #[route("/report")]
    ReportView {},
    #[route("/new-project")]
    New_project {},
    #[route("/select-images")]
    Select_images {}
}