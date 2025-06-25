// Import de bibliotecas externas
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus::desktop::{Config, WindowBuilder};
use std::path::{Path, PathBuf};
use std::fs;
use dioxus_desktop::wry::http::{Request, Response, StatusCode};
use std::borrow::Cow;

mod pages {
    pub mod report;
    pub mod proccess;
    pub mod homepage;
    pub mod create_project;
    pub mod graph;
    pub mod select_images;
    pub mod validation_screen;
}
mod utils {
    pub mod file_manager;
    pub mod report_generator;
    pub mod image_processor;
}

// Import de arquivos locais
mod report_structures;
mod manual_processor;

// MODIFICAÇÃO: Importa o componente com o novo nome `ValidationPage`.
use pages::report::ReportView;
use pages::graph::GraphView;
use pages::select_images::SelectImages;
use pages::homepage::HomePage;
use pages::validation_screen::ValidationPage;
use manual_processor::ManualProcessor;
use pages::create_project::NewProject;
use pages::proccess::Process;


fn main() {
    // Obter o diretório base do CARGO_MANIFEST_DIR
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let projects_root_dir = base_dir.join("Projects");

    // Configurar o Dioxus Desktop
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_window(WindowBuilder::new().with_resizable(true))
                .with_custom_protocol("project-image", move |request: Request<Vec<u8>>| {
                    let path_str = request.uri().path();
                    let relative_to_projects = PathBuf::from(path_str.trim_start_matches('/'));
                    let full_path = projects_root_dir.join(&relative_to_projects);

                    match fs::read(&full_path) {
                        Ok(bytes) => {
                            Response::builder()
                                .status(StatusCode::OK)
                                .header("Content-Type", guess_mime_type(&full_path))
                                .body(Cow::from(bytes))
                                .unwrap_or_else(|_| {
                                    Response::builder()
                                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                                        .body(Cow::from(Vec::new()))
                                        .unwrap()
                                })
                        }
                        Err(e) => {
                            eprintln!("Erro ao ler arquivo {}: {:?}", full_path.display(), e);
                            Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(Cow::from(Vec::new())) 
                                .unwrap_or_else(|_| {
                                    Response::builder()
                                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                                        .body(Cow::from(Vec::new()))
                                        .unwrap()
                                })
                        }
                    }
                })
        )
        .launch(App);
}

fn guess_mime_type(path: &PathBuf) -> &'static str {
    match path.extension().and_then(|s| s.to_str()) {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("bmp") => "image/bmp",
        Some("webp") => "image/webp",
        Some("tiff") | Some("tif") => "image/tiff",
        _ => "application/octet-stream",
    }
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
    ManualProcessor {project_name: String},

    // MODIFICAÇÃO: A rota agora aponta para o componente renomeado `ValidationPage`.
    #[route("/validate")]
    ValidationPage {},
}