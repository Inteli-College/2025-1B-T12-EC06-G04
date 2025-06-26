use dioxus::prelude::*;
use std::path::{Path, PathBuf};
use dioxus_router::prelude::{use_navigator, Link};
use crate::Route;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Write;
use std::default::Default;

pub static PROJECT_NAME: GlobalSignal<Option<String>> = Signal::global(|| None);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub enum ProjectStatus {
    #[default]
    Created,
    ProcessingComplete, 
    ValidationComplete, 
}


#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProjectMetadata {
    pub name: String,
    pub description: String,
    pub year: String,
    pub leader: String,
    pub structure_type: String,
    pub observations: String,
    #[serde(default)]
    pub status: ProjectStatus,
}

fn get_or_create_projects_dir() -> Option<PathBuf> {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let projects_dir = base_dir.join("Projects");

    if !projects_dir.exists() {
        if let Err(e) = std::fs::create_dir_all(&projects_dir) {
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

fn save_project_metadata(project_folder: &Path, metadata: &ProjectMetadata) -> Result<(), std::io::Error> {
    let metadata_path = project_folder.join("project_meta.json");
    let file = File::create(metadata_path)?;
    let mut writer = std::io::BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, metadata)?;
    writer.flush()?;
    Ok(())
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
    let navigator = use_navigator();

    let create_project = move |_| {
        let mut missing_fields = Vec::new();
        let mut year_error = None;

        if name().trim().is_empty() {
            missing_fields.push("Nome do Projeto");
        }
        if leader().trim().is_empty() {
            missing_fields.push("Líder responsável pelo projeto");
        }
        if structure_type().trim().is_empty() {
            missing_fields.push("Tipo de estrutura do edifício");
        }

        match year().trim().parse::<i32>() {
            Ok(y) if (2000..=2100).contains(&y) => {},
            Ok(_) => {
                year_error = Some("O Ano deve estar entre 2000 e 2100.");
            },
            Err(_) => {
                missing_fields.push("Ano");
            }
        }

        if !missing_fields.is_empty() {
            let error_msg = format!("Por favor, preencha os campos obrigatórios: {}.", missing_fields.join(", "));
            status.set(error_msg);
            return;
        }

        if let Some(err) = year_error {
            status.set(err.to_string());
            return;
        }

        is_creating.set(true);
        let project_name_raw = name().trim().to_string();
        let sanitized_project_name = sanitize_name(&project_name_raw);

        if sanitized_project_name.is_empty() {
            status.set("Nome do projeto inválido após sanitização. Use letras, números, '_' ou '-'.".to_string());
            is_creating.set(false);
            return;
        }

        *PROJECT_NAME.write() = Some(sanitized_project_name.clone());

        let metadata = ProjectMetadata {
            name: name().trim().to_string(),
            description: description().trim().to_string(),
            year: year().trim().to_string(),
            leader: leader().trim().to_string(),
            structure_type: structure_type().trim().to_string(),
            observations: observations().trim().to_string(),
            status: ProjectStatus::Created,
        };

        spawn(async move {
            if let Some(projects_dir) = get_or_create_projects_dir() {
                let new_folder = projects_dir.join(&sanitized_project_name);

                if new_folder.exists() {
                    status.set(format!("Erro: Projeto '{}' já existe.", sanitized_project_name));
                    is_creating.set(false);
                    return;
                }

                if let Err(e) = std::fs::create_dir_all(&new_folder) {
                    status.set(format!("Erro ao criar pasta: {}", e));
                } else {
                    match save_project_metadata(&new_folder, &metadata) {
                        Ok(_) => {
                            status.set(format!("Projeto criado em: {}", new_folder.display()));
                            project_path.set(Some(new_folder));
                        }
                        Err(e) => {
                             status.set(format!("Erro ao salvar metadados do projeto: {}", e));
                             _ = std::fs::remove_dir_all(&new_folder);
                        }
                    }
                }
            } else {
                status.set("Erro: Não foi possível criar ou acessar o diretório Projects".to_string());
            }

            is_creating.set(false);
        });
    };

    let handle_back = move || {
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
            button {
                class: "btn btn-secondary",
                style: "position: fixed; top: 1.5rem; left: 1.5rem; z-index: 10;",
                title: "Voltar para a página inicial",
                onclick: move |_| {
                    handle_back();
                    navigator.push(Route::HomePage {});
                },
                i { class: "material-icons", "arrow_back" }
                "Voltar ao Início"
            }

            div { 
                class: "container",
                style: "max-width: 700px;",

                div {
                    style:"display: flex; align-items: center; gap: 1rem; margin-bottom: 2rem; margin-top: 4rem;",
                    hr { class: "form-divider", style: "flex-grow: 1;" },
                    h1 {
                        style: "font-weight: bold; font-size: 1.5rem; text-align: center; white-space: nowrap;",
                        "Criar Novo Projeto"
                    },
                    hr { class: "form-divider", style: "flex-grow: 1;" },
                }
                
                div { 
                    class: "card",
                    div {
                        class: "form-group",
                        label { "Nome do Projeto" span { class: "required-indicator", "*" }}
                        input {
                            class: "form-input",
                            r#type: "text",
                            placeholder: "Ex: Edifício Residencial Centro",
                            value: "{name()}",
                            oninput: move |e| name.set(e.value())
                        }
                    }

                    div {
                        class: "form-group",
                        label { "Descrição" }
                        textarea {
                            class: "form-textarea",
                            placeholder: "Descreva o projeto, suas características principais e objetivos...",
                            value: "{description()}",
                            rows: "4",
                            oninput: move |e| description.set(e.value())
                        }
                    }

                    div {
                        class: "form-group",
                        label { "Líder responsável pelo projeto" span { class: "required-indicator", "*" }}
                        input {
                            class: "form-input",
                            r#type: "text",
                            placeholder: "Ex: João Silva",
                            value: "{leader()}",
                            oninput: move |e| leader.set(e.value())
                        }
                    }
                    div {
                        class: "form-group",
                        label { "Tipo de estrutura do edifício" span { class: "required-indicator", "*" }}
                        input {
                            class: "form-input",
                            r#type: "text",
                            placeholder: "Ex: Concreto armado, Alvenaria estrutural, Metálica",
                            value: "{structure_type()}",
                            oninput: move |e| structure_type.set(e.value())
                        }
                    }

                    div {
                        class: "form-group",
                        label { "Ano" span { class: "required-indicator", "*" }}
                        input {
                            class: "form-input",
                            r#type: "number",
                            placeholder: "2025",
                            value: "{year()}",
                            min: "2000",
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
                            placeholder: "Informações adicionais, considerações especiais...",
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
                        p { 
                            class: if status().starts_with("Projeto criado") { "status-message success" } else { "status-message error" }, 
                            "{status()}" 
                        }
                        
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