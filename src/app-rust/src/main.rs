// Import de bibliotecas externas
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus::desktop::{Config, WindowBuilder};
use std::path::PathBuf;

mod pages {
    pub mod report;
    pub mod proccess;
    pub mod homepage;
    pub mod create_project;
    pub mod graph;
    pub mod select_images;
    pub mod manual_processor;
}
mod utils {
    pub mod file_manager;
    pub mod report_generator;
    pub mod image_processor;
}

// Import de arquivos locais
mod report_structures;


use pages::homepage::HomePage;
use pages::select_images::SelectImages;
use pages::report::ReportView;
use pages::create_project::NewProject;
use pages::proccess::Process;
use pages::graph::GraphView;
use pages::manual_processor::ManualProcessor;


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

    #[route("/processamento-manual")]
    ManualProcessor {},
}
