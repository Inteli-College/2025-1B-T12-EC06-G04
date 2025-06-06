use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus::{desktop::Config, desktop::WindowBuilder};
use std::path::PathBuf;

mod homepage;
mod select_images;
mod report;
mod create_project;
mod image_processor;
mod manual_processor;
mod ui;
mod report_structures;

use homepage::HomePage;
use select_images::SelectImages;
use report::ReportView;
use create_project::NewProject;
use ui::Home;
mod graph;
use graph::GraphView;

#[component]
fn Process() -> Element {
    rsx! {
        Home {}
    }
}

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
    HomePage {},

    #[route("/new-project")]
    NewProject {},

    #[route("/select-images")]
    SelectImages {},

    #[route("/graph/:project_name")]
    GraphView { project_name: String },
    
    #[route("/report/:project_name/:building_name")]
    ReportView { project_name: String, building_name: String },

    #[route("/process")]
    Process {},
}
