use dioxus::prelude::*;
use std::path::{Path, PathBuf};
use dioxus_router::prelude::Link;
use crate::Route;

// Add context provider for project name
pub static PROJECT_NAME: GlobalSignal<Option<String>> = Signal::global(|| None);

fn get_or_create_projects_dir() -> Option<PathBuf> {
    // Construct path relative to CARGO_MANIFEST_DIR
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let projects_dir = base_dir.join("Projects"); // This will be src/app-rust/Projects
    
    // Tenta criar o diretório se não existir
    if !projects_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&projects_dir) { // Use create_dir_all for robustness
            eprintln!("Erro ao criar diretório Projects em {}: {}", projects_dir.display(), e);
            return None;
        }
    }
    
    Some(projects_dir)
}

#[component]
pub fn NewProject() -> Element {
    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut year = use_signal(|| "".to_string());
    let mut leader = use_signal(|| String::new());
    let mut structure_type = use_signal(|| String::new());
    let mut observations = use_signal(|| String::new());
    let mut status = use_signal(|| String::new());
    let mut is_creating = use_signal(|| false);
    let mut project_path = use_signal(|| None::<PathBuf>);
    let mut images_path = use_signal(|| None::<PathBuf>);

    let create_project = move |_| {
        if name().trim().is_empty() || year().trim().is_empty() {
            status.set("Por favor, preencha nome e ano.".to_string());
            return;
        }

        is_creating.set(true);
        let project_name = name().trim().to_string();
        let project_year = year().trim().to_string();

        // Store project name in global signal
        *PROJECT_NAME.write() = Some(project_name.clone());
        
        spawn(async move {
            if let Some(projects_dir) = get_or_create_projects_dir() {
                let new_folder = projects_dir.join(format!("{} - {}", project_year, project_name));

                if let Err(e) = std::fs::create_dir_all(&new_folder) {
                    status.set(format!("Erro ao criar pasta: {}", e));
                } else {
                    status.set(format!("Projeto criado em: {}", new_folder.display()));
                    project_path.set(Some(new_folder));
                }
            } else {
                status.set("Erro: Não foi possível criar ou acessar o diretório Projects".to_string());
            }

            is_creating.set(false);
        });
    };

    let handle_back = move |_| {
        if let Some(path) = project_path() {
            if let Err(e) = std::fs::remove_dir_all(&path) {
                eprintln!("Erro ao remover pasta: {}", e);
            }
        }
    };

    let handle_image_upload = move |_| {
        if let Some(path) = project_path() {
            let images_dir = path.join("images");
            if let Err(e) = std::fs::create_dir_all(&images_dir) {
                status.set(format!("Erro ao criar pasta de imagens: {}", e));
            } else {
                status.set("Pasta de imagens criada com sucesso!".to_string());
                images_path.set(Some(images_dir));
            }
        } else {
            status.set("Erro: Projeto não foi criado ainda".to_string());
        }
    };

    rsx! {
        div { class: "min-h-screen bg-gray-100 text-gray-900 font-sans",
            div { class: "container mx-auto px-4 py-12 max-w-2xl",
                h1 { class: "text-3xl font-bold text-center mb-8", "Criar Novo Projeto" }

                div { class: "bg-white rounded-lg shadow-md p-6 space-y-6",

                    div {
                        label { class: "block text-gray-700 mb-1", "Nome do Projeto" }
                        input {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "text",
                            value: "{name()}",
                            oninput: move |e| name.set(e.value())
                        }
                    }

                    div {
                        label { class: "block text-gray-700 mb-1", "Descrição" }
                        textarea {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            value: "{description()}",
                            rows: "4",
                            oninput: move |e| description.set(e.value())
                        }
                    }

                    div {
                        label { class: "block text-gray-700 mb-1", "Líder responsável pelo projeto" }
                        input {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "text",
                            value: "{leader()}",
                            oninput: move |e| leader.set(e.value())
                        }
                    }
                    div {
                        label { class: "block text-gray-700 mb-1", "Tipo de estrutura do edifício" }
                        input {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "text",
                            value: "{structure_type()}",
                            oninput: move |e| structure_type.set(e.value())
                        }
                    }

                    div {
                        label { class: "block text-gray-700 mb-1", "Ano" }
                        input {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "number",
                            value: "{year()}",
                            min: "1800",
                            max: "2100",
                            oninput: move |e| year.set(e.value())
                        }
                    }

                    div {
                        label { class: "block text-gray-700 mb-1", "Observações gerais" }
                        input {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "text",
                            value: "{observations()}",
                            oninput: move |e| observations.set(e.value())
                        }
                    }

                    button {
                        class: "w-full px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors duration-200 disabled:opacity-50 disabled:cursor-not-allowed",
                        disabled: is_creating(),
                        onclick: create_project,
                        if is_creating() { "Criando projeto..." } else { "Criar Projeto" }
                    }

                    if !status().is_empty() {
                        p { class: "text-center text-gray-700", "{status()}" }
                        
                        div { class: "flex justify-between mt-4",
                            Link {
                                to: Route::HomePage {},
                                button {
                                    class: "px-4 py-2 bg-red-100 hover:bg-red-200 text-red-600 rounded-md shadow",
                                    onclick: handle_back,
                                    title: "Voltar para a página inicial",
                                    i { class: "material-icons", "arrow_back" }
                                }
                            }
                            
                            if let Some(_) = images_path() {
                                Link {
                                    to: Route::Process {},
                                    button {
                                        class: "px-4 py-2 bg-green-100 hover:bg-green-200 text-green-600 rounded-md shadow",
                                        title: "Ir para a página de processamento",
                                        i { class: "material-icons", "arrow_forward" }
                                    }
                                }
                            } else {
                                button {
                                    class: "px-4 py-2 bg-green-100 hover:bg-green-200 text-green-600 rounded-md shadow",
                                    onclick: handle_image_upload,
                                    title: "Adicionar imagens ao projeto",
                                    i { class: "material-icons", "add_photo_alternate" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
