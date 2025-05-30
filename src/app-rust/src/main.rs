use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus::{desktop::Config, desktop::WindowBuilder, prelude::*};
use std::path::PathBuf;

mod create_project;
use create_project::Create_project;
mod image_processor;
mod report;
mod manual_processor;
use report::ReportView;
mod home;
use home::Home;

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
    Home {},
    #[route("/create-project")]
    Create_project {},
    #[route("/report")]
    ReportView {}
}