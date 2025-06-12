use dioxus::prelude::*;
use dioxus_router::prelude::*;
use dioxus::{desktop::Config, desktop::WindowBuilder};
use std::path::PathBuf;
use std::fs;
use dioxus_desktop::wry::http::{Request, Response, StatusCode}; 
use std::borrow::Cow; 

mod homepage;
mod select_images;
mod report;
mod create_project;
mod image_processor;
mod manual_processor;
mod ui;
mod report_structures;
mod validation_screen;

use homepage::HomePage;
use select_images::SelectImages;
use report::ReportView;
use create_project::NewProject;
use ui::Home;
mod graph;
use graph::GraphView;
use validation_screen::ValidationScreen;

#[component]
fn Process() -> Element {
    rsx! {
        Home {}
    }
}

fn main() {
    // Obter o diretório base do CARGO_MANIFEST_DIR
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let projects_dir = base_dir.join("Projects");

    // Configurar o Dioxus Desktop
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_window(WindowBuilder::new().with_resizable(true))
                .with_custom_protocol("project-image", move |request: Request<Vec<u8>>| {
                    let path_str = request.uri().path();
                    let relative_path = PathBuf::from(path_str.trim_start_matches('/'));
                    let full_path = projects_dir.join(&relative_path);

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
        _ => "application/octet-stream", // Tipo genérico para o que não for reconhecido
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

    #[route("/validate")]
    ValidationScreen {},
}