use dioxus::prelude::*;
use dioxus_router::prelude::*;
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::pages::create_project::PROJECT_NAME; // Importa o GlobalSignal
use crate::Route;

// Estrutura para os dados de validação de fissuras
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct FissuraValidation {
    pub name: String,
    pub confidence: f64,
}

// Estrutura para os dados de validação de imagem
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ImageValidationData {
    pub path: String, 
    pub fissura: Vec<FissuraValidation>,
}

// Estado de validação da imagem para a UI
#[derive(Clone, PartialEq)]
pub struct ImageValidationState {
    pub path: String, 
    pub fissuras: Vec<FissuraValidation>,
    pub is_incorrect: bool,
    pub has_been_viewed: bool,
}

// Resultados da validação a serem salvos
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidationResults {
    pub total_images: usize,
    pub incorrect_images: Vec<String>,
    pub validation_date: String,
    pub project_name: String,
}

// Função para carregar os dados de detecção de fissuras a partir de um arquivo JSON.
fn carregar_dados_deteccao(project_name: &str) -> Result<Vec<ImageValidationData>, String> {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let json_path = base_dir.join("Projects").join(project_name).join("detection_results.json");
    
    if !json_path.exists() {
        return Err(format!("Arquivo de resultados não encontrado: {:?}\n\nPara usar a validação, primeiro você precisa:\n1. Adicionar imagens ao projeto usando o Processador Manual\n2. Executar o processamento de IA\n3. Aguardar a criação do arquivo detection_results.json", json_path));
    }
    
    let json_content = fs::read_to_string(&json_path)
        .map_err(|e| format!("Erro ao ler arquivo JSON: {}", e))?;
    
    serde_json::from_str::<Vec<ImageValidationData>>(&json_content)
        .map_err(|e| format!("Erro ao parsear JSON: {}", e))
}

// Função para salvar os resultados da validação em um arquivo JSON.
fn salvar_resultados_validacao(project_name: &str, results: &ValidationResults) -> Result<(), String> {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let validation_path = base_dir.join("Projects").join(project_name).join("validation_results.json");
    
    let json_content = serde_json::to_string_pretty(results)
        .map_err(|e| format!("Erro ao serializar resultados: {}", e))?;
    
    fs::write(&validation_path, json_content)
        .map_err(|e| format!("Erro ao salvar arquivo de validação: {}", e))?;
    
    Ok(())
}

/// Componente principal da tela de validação.
#[component]
pub fn ValidationScreen() -> Element {
    let navigator = use_navigator();
    let mut current_image_index = use_signal(|| 0usize);
    let mut validation_data = use_signal(|| Vec::<ImageValidationState>::new());
    let mut loading = use_signal(|| true);
    let mut error_message = use_signal(|| String::new());
    let mut show_confirmation_dialog = use_signal(|| false);
    let mut status_message = use_signal(|| String::new());
    let mut project_display_name: Signal<Option<String>> = use_signal(|| None);
    let mut project_folder_name: Signal<Option<String>> = use_signal(|| None);
    let projects_root_dir_signal = use_signal(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Projects")
    });

    use_effect(move || {
        let projects_root_dir_for_strip = projects_root_dir_signal.clone(); 
        spawn(async move {
            match PROJECT_NAME.try_read() {
                Ok(project_name_guard) => {
                    if let Some(absolute_project_path_str) = &*project_name_guard { 
                        let absolute_project_path_buf = PathBuf::from(absolute_project_path_str);
                        let p_name_only = absolute_project_path_buf.file_name()
                                                     .and_then(|os_str| os_str.to_str())
                                                     .map(|s| s.to_string());
                        project_display_name.set(p_name_only.clone()); 
                        project_folder_name.set(p_name_only.clone());

                        match carregar_dados_deteccao(p_name_only.unwrap_or_default().as_str()) {
                            Ok(data) => {
                                let validation_states: Vec<ImageValidationState> = data.into_iter().filter_map(|img| {
                                    let full_image_path_from_json = PathBuf::from(&img.path);
                                    let projects_root_dir_val = projects_root_dir_for_strip.read();
                                    let relative_image_path = if full_image_path_from_json.is_absolute() {
                                        let stripped = full_image_path_from_json.strip_prefix(&*projects_root_dir_val);
                                        stripped.ok().and_then(|p| p.to_str()).map(|s| s.to_string())
                                    } else {
                                        Some(img.path.clone()) 
                                    };
                                    relative_image_path.map(|rel_path| ImageValidationState {
                                        path: rel_path,
                                        fissuras: img.fissura,
                                        is_incorrect: false,
                                        has_been_viewed: false,
                                    })
                                }).collect();
                                validation_data.set(validation_states);
                                loading.set(false);
                            }
                            Err(e) => { error_message.set(e); loading.set(false); }
                        }
                    } else { error_message.set("Caminho do projeto não encontrado".to_string()); loading.set(false); }
                }
                Err(_) => { error_message.set("Erro ao acessar caminho do projeto (GlobalSignal)".to_string()); loading.set(false); }
            }
        });
    });

    let total_images = validation_data.read().len();
    let has_images = total_images > 0;

    use_effect(move || {
        if has_images {
            let idx = *current_image_index.read();
            let mut data = validation_data.write();
            if idx < data.len() { data[idx].has_been_viewed = true; }
        }
    });

    let next_image = move |_| { if current_image_index() < total_images - 1 { current_image_index.set(current_image_index() + 1); } };
    let previous_image = move |_| { if current_image_index() > 0 { current_image_index.set(current_image_index() - 1); } };
    let toggle_incorrect = move |_| { if has_images { let idx = *current_image_index.read(); let mut data = validation_data.write(); if idx < data.len() { data[idx].is_incorrect = !data[idx].is_incorrect; } } };

    let mut confirm_validation = move || {
        spawn(async move {
            let project_name_for_save = project_display_name.read().clone().unwrap_or_else(|| "unknown_project".to_string());
            let data = validation_data.read();
            let incorrect_images: Vec<String> = data.iter().filter(|img| img.is_incorrect).map(|img| img.path.clone()).collect();
            let results = ValidationResults {
                total_images: data.len(),
                incorrect_images,
                validation_date: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                project_name: project_name_for_save.clone(),
            };
            match salvar_resultados_validacao(&project_name_for_save, &results) {
                Ok(_) => {
                    status_message.set("Validação salva com sucesso!".to_string());
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    navigator.push(Route::HomePage {});
                }
                Err(e) => { status_message.set(format!("Erro ao salvar validação: {}", e)); }
            }
        });
        show_confirmation_dialog.set(false);
    };

    let attempt_confirm = move |_| { if validation_data.read().iter().all(|img| img.has_been_viewed) { confirm_validation(); } else { show_confirmation_dialog.set(true); } };
    let close_dialog = move |_| { show_confirmation_dialog.set(false); };

    if loading() {
        return rsx! {
            div { class: "status-screen-container",
                div { class: "spinner" }
                p { class: "text-gray-600", "Carregando dados de validação..." }
            }
        };
    }

    if !error_message().is_empty() {
        return rsx! {
            div { class: "status-screen-container",
                div { class: "status-card",
                    i { class: "material-icons status-card-icon text-red", "error_outline" }
                    h2 { class: "text-xl font-bold text-gray-800 mb-4", "Erro ao Carregar Dados" }
                    p { class: "text-gray-600 mb-6", "{error_message()}" }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| navigator.go_back(),
                        "Voltar"
                    }
                }
            }
        };
    }

    if total_images == 0 {
        return rsx! {
            div { class: "status-screen-container",
                div { class: "status-card",
                    i { class: "material-icons status-card-icon text-yellow", "warning" }
                    h2 { class: "text-xl font-bold text-gray-800 mb-4", "Nenhuma Imagem para Validar" }
                    p { class: "text-gray-600 mb-6", "Não foram encontradas imagens com detecções para validação." }
                    button {
                        class: "btn btn-primary",
                        onclick: move |_| navigator.go_back(),
                        "Voltar"
                    }
                }
            }
        };
    }

    let current_image = &validation_data.read()[current_image_index()];
    let viewed_count = validation_data.read().iter().filter(|img| img.has_been_viewed).count();
    let incorrect_count = validation_data.read().iter().filter(|img| img.is_incorrect).count();
    let fissura_classification = if !current_image.fissuras.is_empty() { current_image.fissuras[0].name.clone() } else { "Nenhuma Fissura Detectada".to_string() };
    let image_src_path: String = if let Some(project_folder_name_val) = project_folder_name.read().as_ref() { format!("{}/{}", project_folder_name_val, current_image.path) } else { current_image.path.clone() };

    rsx! {
        div { class: "validation-page",
            document::Link { href: "https://fonts.googleapis.com/icon?family=Material+Icons", rel: "stylesheet" }

            div { class: "page-subheader",
                div { class: "container",
                    div { class: "d-flex items-center justify-between py-4",
                        div {
                            h1 { class: "text-2xl font-bold text-gray-800",
                                "Validação para o Projeto ",
                                if let Some(name) = project_display_name.read().as_ref() {
                                    span { class: "text-primary", "{name}" }
                                }
                            }
                            p { class: "text-gray-600", "Selecione as imagens com detecções incorretas" }
                        }
                        div { class: "text-right text-sm text-gray-600",
                            p { "Imagem {current_image_index() + 1} de {total_images}" }
                            p { "Visualizadas: {viewed_count}/{total_images}" }
                            p { "Incorretas: {incorrect_count}" }
                        }
                    }
                }
            }

            div { class: "page-subheader",
                div { class: "container py-2",
                    div { class: "progress-bar-container",
                        div { class: "progress-bar-fill", style: "width: {(viewed_count as f64 / total_images as f64 * 100.0)}%" }
                    }
                }
            }

            div { class: "container py-8",
                div { class: "validation-grid",
                    div { class: "grid-col-span-2-lg",
                        div { class: "image-viewer-card p-6",
                            h2 { 
                                class: "text-3xl font-extrabold text-center uppercase mb-4",
                                class: if fissura_classification == "retracao" { "text-red" } else if fissura_classification == "termica" { "text-orange" } else { "text-gray-800" },
                                "{fissura_classification}"
                            }
                            div { class: "image-display-box",
                                img { src: "project-image://{image_src_path}", alt: "Imagem para validação" }
                            }
                            div { class: "d-flex items-center justify-between",
                                button {
                                    class: "btn btn-primary d-flex items-center gap-2",
                                    disabled: current_image_index() == 0,
                                    onclick: previous_image,
                                    i { class: "material-icons", "arrow_back" },
                                    "Anterior"
                                }
                                button {
                                    class: format!("btn {} d-flex items-center gap-2", if current_image.is_incorrect { "btn-danger" } else { "btn-secondary" }),
                                    onclick: toggle_incorrect,
                                    i { class: "material-icons", if current_image.is_incorrect { "close" } else { "check" } },
                                    if current_image.is_incorrect { "Incorreta" } else { "Marcar como Incorreta" }
                                }
                                button {
                                    class: "btn btn-primary d-flex items-center gap-2",
                                    disabled: current_image_index() >= total_images - 1,
                                    onclick: next_image,
                                    "Próxima",
                                    i { class: "material-icons", "arrow_forward" }
                                }
                            }
                        }
                    }

                    div { class: "space-y-6",
                        div { class: "info-card",
                            h3 { class: "info-card-title", "Informações da Imagem" }
                            div { class: "space-y-3",
                                div {
                                    span { class: "font-semibold text-gray-800", "Arquivo: " }
                                    span { class: "text-sm text-gray-600 break-all", "{current_image.path.split('/').last().unwrap_or(&current_image.path)}" }
                                }
                                div { class: "d-flex items-center gap-2",
                                    span { class: "font-semibold text-gray-800", "Status: " }
                                    span { class: format!("status-badge {}", if current_image.is_incorrect { "red" } else { "green" }),
                                        if current_image.is_incorrect { "Incorreta" } else { "Correta" }
                                    }
                                }
                                div { class: "d-flex items-center gap-2",
                                    span { class: "font-semibold text-gray-800", "Visualizada: " }
                                    span { class: format!("status-badge {}", if current_image.has_been_viewed { "blue" } else { "gray" }),
                                        if current_image.has_been_viewed { "Sim" } else { "Não" }
                                    }
                                }
                            }
                        }

                        div { class: "info-card",
                            h3 { class: "info-card-title", "Fissuras Detectadas" }
                            if current_image.fissuras.is_empty() {
                                p { class: "text-gray-500 italic", "Nenhuma fissura detectada" }
                            } else {
                                div { class: "space-y-3",
                                    for (idx, fissura) in current_image.fissuras.iter().enumerate() {
                                        div { key: "{idx}", class: "fissura-list-item",
                                            span { class: "name", "{fissura.name}" }
                                            span { class: "confidence", "{(fissura.confidence * 100.0):.1}%" }
                                        }
                                    }
                                }
                            }
                        }

                        div { class: "info-card",
                            h3 { class: "info-card-title", "Ações" }
                            div { class: "space-y-3",
                                button { class: "btn btn-primary w-full", onclick: attempt_confirm, "Confirmar Validação" }
                                button { class: "btn btn-secondary w-full", onclick: move |_| navigator.go_back(), "Cancelar e Voltar" }
                            }
                            if !status_message().is_empty() {
                                div { class: "status-box info",
                                    p { class: "status-box-text", "{status_message()}" }
                                }
                            }
                        }
                    }
                }
            }

            if show_confirmation_dialog() {
                div { class: "modal-overlay",
                    div { class: "modal-content",
                        i { class: "material-icons modal-icon", "warning" }
                        h3 { class: "text-lg font-semibold text-gray-800 mb-4", "Confirmação" }
                        p { class: "text-gray-600 mb-6", "Você ainda não visualizou todas as imagens. Deseja confirmar a validação mesmo assim?" }
                        div { class: "d-flex gap-4 justify-center",
                            button { class: "btn btn-secondary", onclick: close_dialog, "Cancelar" }
                            button { class: "btn btn-primary", onclick: move |_| confirm_validation(), "Confirmar" }
                        }
                    }
                }
            }
        }
    }
}