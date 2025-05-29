use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus::{desktop::Config, desktop::WindowBuilder};
use std::path::PathBuf;

mod ui;
use ui::Home;
mod image_processor;
mod folders;
use folders::Folders;
mod report;
use report::ReportView;

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
    #[route("/home")]
    Home {},
    #[route("/")]
    Folders {},
    #[route("/report/:project_name/:building_name")]
    ReportView { project_name: String, building_name: String }
}