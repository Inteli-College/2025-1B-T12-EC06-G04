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
    let projects_root_dir = base_dir.join("Projects"); // Este será o diretório raiz para o protocolo

    // Configurar o Dioxus Desktop
    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::new()
                .with_window(WindowBuilder::new().with_resizable(true))
                // Configura um protocolo personalizado 'project-image'
                // para servir arquivos do diretório `projects_root_dir`.
                .with_custom_protocol("project-image", move |request: Request<Vec<u8>>| {
                    let path_str = request.uri().path();
                    // Remove a barra inicial para obter o caminho relativo correto.
                    let relative_to_projects = PathBuf::from(path_str.trim_start_matches('/'));
                    // Constrói o caminho completo do arquivo no sistema de arquivos.
                    let full_path = projects_root_dir.join(&relative_to_projects);

                    // Tenta ler o arquivo do sistema de arquivos.
                    match fs::read(&full_path) {
                        Ok(bytes) => {
                            // Se a leitura for bem-sucedida, retorna a imagem com o tipo MIME correto.
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
                            // Se houver um erro na leitura, imprime o erro e retorna um status NOT_FOUND.
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

// Função auxiliar para adivinhar o tipo MIME do arquivo com base na extensão.
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
    provide_root_context(initial_folder_path);

    rsx! {
        Router::<Route> {}
    }
}

// Define as rotas da aplicação.
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
