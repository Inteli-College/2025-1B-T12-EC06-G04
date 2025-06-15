use dioxus::prelude::*;
use std::path::{Path, PathBuf};
use dioxus_router::prelude::Link;
use crate::Route;

pub static PROJECT_NAME: GlobalSignal<Option<String>> = Signal::global(|| None);

fn get_or_create_projects_dir() -> Option<PathBuf> {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let projects_dir = base_dir.join("Projects"); 
    
    // Tenta criar o diretório se não existir
    if !projects_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&projects_dir) { // Use create_dir_all for robustness
            eprintln!("Erro ao criar diretório Projects em {}: {}", projects_dir.display(), e);
            return None;
        }
    }
    
    Some(projects_dir)
}

fn sanitize_name(name: &str) -> String {
    name.replace(' ', "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect::<String>()
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
        let project_name_raw = name().trim().to_string();
        let project_year = year().trim().to_string();
        let sanitized_project_name = sanitize_name(&project_name_raw);

        if sanitized_project_name.is_empty() {
            status.set("Nome do projeto inválido após sanitização. Use letras, números, '_' ou '-'.".to_string());
            is_creating.set(false);
            return;
        }

        *PROJECT_NAME.write() = Some(sanitized_project_name.clone());
        
        let project_name_for_folder = sanitized_project_name;

        spawn(async move {
            if let Some(projects_dir) = get_or_create_projects_dir() {
                let new_folder = projects_dir.join(project_name_for_folder);

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
        document::Stylesheet { href: asset!("/assets/styles.css") }
        document::Link {
            href: "https://fonts.googleapis.com/icon?family=Material+Icons",
            rel: "stylesheet"
        }
        
        div {
            div { 
                class: "container",
                style: "max-width: 700px;",

                div {
                    style:"display: flex; align-items: center; gap: 1rem; margin-bottom: 2rem;",
                    hr { class: "form-divider", style: "flex-grow: 1;" },
                    h1 {
                        style: "font-weight: bold; font-size: 1.5rem; text-align: center; white-space: nowrap;",
                        "Criar Novo Projeto"
                    },
                    hr { class: "form-divider", style: "flex-grow: 1;" },
                }
                
                Link {
                    to: Route::HomePage {},
                    class: "btn btn-danger",
                    style: "position: fixed; top: 1.5rem; left: 1.5rem; padding: 0.5rem;",
                    onclick: handle_back,
                    title: "Voltar para a página inicial",
                    i { class: "material-icons", "arrow_back" }
                }

                div { 
                    class: "card",
                    div {
                        class: "form-group",
                        label { "Nome do Projeto" }
                        input {
                            class: "form-input",
                            r#type: "text",
                            value: "{name()}",
                            oninput: move |e| name.set(e.value())
                        }
                    }

                    div {
                        class: "form-group",
                        label { "Descrição" }
                        textarea {
                            class: "form-textarea",
                            value: "{description()}",
                            rows: "4",
                            oninput: move |e| description.set(e.value())
                        }
                    }

                    div {
                        class: "form-group",
                        label { "Líder responsável pelo projeto" }
                        input {
                            class: "form-input",
                            r#type: "text",
                            value: "{leader()}",
                            oninput: move |e| leader.set(e.value())
                        }
                    }
                    div {
                        class: "form-group",
                        label { "Tipo de estrutura do edifício" }
                        input {
                            class: "form-input",
                            r#type: "text",
                            value: "{structure_type()}",
                            oninput: move |e| structure_type.set(e.value())
                        }
                    }

                    div {
                        class: "form-group",
                        label { "Ano" }
                        input {
                            class: "form-input",
                            r#type: "number",
                            value: "{year()}",
                            min: "1800",
                            max: "2100",
                            oninput: move |e| year.set(e.value())
                        }
                    }

                    div {

                        class: "form-group",
                        label { "Observações gerais" }
                        input {
                            class: "form-input",
                            r#type: "text",
                            value: "{observations()}",
                            oninput: move |e| observations.set(e.value())
                        }
                    }

                    button {
                        class: "btn btn-primary",
                        disabled: is_creating(),
                        onclick: create_project,
                        if is_creating() { "Criando projeto..." } else { "Criar Projeto" }
                    }

                    if !status().is_empty() {
                        p { class: "status-message info", "{status()}" }
                        
                        div { class: "flex justify-between mt-4",
                            
                            if let Some(_) = images_path() {
                                Link {
                                    to: Route::Process {},
                                    class: "btn btn-secondary",
                                    title: "Ir para a página de processamento",
                                    i { class: "material-icons", "arrow_forward" }
                                }
                            } else {
                                button {
                                    class: "btn btn-secondary",
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