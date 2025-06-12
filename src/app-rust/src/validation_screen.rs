use dioxus::prelude::*;
use dioxus_router::prelude::*;
use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::create_project::PROJECT_NAME;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct FissuraValidation {
    pub name: String,
    pub confidence: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ImageValidationData {
    pub path: String,
    pub fissura: Vec<FissuraValidation>,
}

#[derive(Clone, PartialEq)]
pub struct ImageValidationState {
    pub path: String,
    pub fissuras: Vec<FissuraValidation>,
    pub is_incorrect: bool,
    pub has_been_viewed: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValidationResults {
    pub total_images: usize,
    pub incorrect_images: Vec<String>,
    pub validation_date: String,
    pub project_name: String,
}

fn carregar_dados_deteccao(project_name: &str) -> Result<Vec<ImageValidationData>, String> {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let json_path = base_dir.join("Projects").join(project_name).join("detection_results.json");
    
    if !json_path.exists() {
        return Err(format!("Arquivo de resultados não encontrado: {:?}", json_path));
    }
    
    let json_content = fs::read_to_string(&json_path)
        .map_err(|e| format!("Erro ao ler arquivo JSON: {}", e))?;
    
    serde_json::from_str::<Vec<ImageValidationData>>(&json_content)
        .map_err(|e| format!("Erro ao parsear JSON: {}", e))
}

fn salvar_resultados_validacao(project_name: &str, results: &ValidationResults) -> Result<(), String> {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let validation_path = base_dir.join("Projects").join(project_name).join("validation_results.json");
    
    let json_content = serde_json::to_string_pretty(results)
        .map_err(|e| format!("Erro ao serializar resultados: {}", e))?;
    
    fs::write(&validation_path, json_content)
        .map_err(|e| format!("Erro ao salvar arquivo de validação: {}", e))?;
    
    Ok(())
}

#[component]
pub fn ValidationScreen() -> Element {
    let navigator = use_navigator();
    let mut current_image_index = use_signal(|| 0usize);
    let mut validation_data = use_signal(|| Vec::<ImageValidationState>::new());
    let mut loading = use_signal(|| true);
    let mut error_message = use_signal(|| String::new());
    let mut show_confirmation_dialog = use_signal(|| false);
    let mut status_message = use_signal(|| String::new());

    // Carregar dados na inicialização
    use_effect(move || {
        spawn(async move {
            match PROJECT_NAME.try_read() {
                Ok(guard) => {
                    if let Some(project_name) = &*guard {
                        match carregar_dados_deteccao(project_name) {
                            Ok(data) => {
                                let validation_states: Vec<ImageValidationState> = data
                                    .into_iter()
                                    .map(|img| ImageValidationState {
                                        path: img.path,
                                        fissuras: img.fissura,
                                        is_incorrect: false,
                                        has_been_viewed: false,
                                    })
                                    .collect();
                                validation_data.set(validation_states);
                                loading.set(false);
                            }
                            Err(e) => {
                                error_message.set(e);
                                loading.set(false);
                            }
                        }
                    } else {
                        error_message.set("Nome do projeto não encontrado".to_string());
                        loading.set(false);
                    }
                }
                Err(_) => {
                    error_message.set("Erro ao acessar nome do projeto".to_string());
                    loading.set(false);
                }
            }
        });
    });

    let total_images = validation_data.read().len();
    let current_idx = current_image_index();
    let has_images = total_images > 0;

    // Marcar imagem atual como visualizada
    use_effect(move || {
        if has_images && current_idx < total_images {
            let mut data = validation_data.write();
            data[current_idx].has_been_viewed = true;
        }
    });

    let next_image = move |_| {
        if current_idx < total_images - 1 {
            current_image_index.set(current_idx + 1);
        }
    };

    let previous_image = move |_| {
        if current_idx > 0 {
            current_image_index.set(current_idx - 1);
        }
    };

    let toggle_incorrect = move |_| {
        if has_images && current_idx < total_images {
            let mut data = validation_data.write();
            data[current_idx].is_incorrect = !data[current_idx].is_incorrect;
        }
    };

    let mut confirm_validation = move || {
        spawn(async move {
            match PROJECT_NAME.try_read() {
                Ok(guard) => {
                    if let Some(project_name) = &*guard {
                        let data = validation_data.read();
                        let incorrect_images: Vec<String> = data
                            .iter()
                            .filter(|img| img.is_incorrect)
                            .map(|img| img.path.clone())
                            .collect();

                        let results = ValidationResults {
                            total_images: data.len(),
                            incorrect_images,
                            validation_date: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                            project_name: project_name.clone(),
                        };

                        match salvar_resultados_validacao(project_name, &results) {
                            Ok(_) => {
                                status_message.set("Validação salva com sucesso!".to_string());
                                // Navegar de volta ou para próxima tela
                                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                                navigator.go_back();
                            }
                            Err(e) => {
                                status_message.set(format!("Erro ao salvar validação: {}", e));
                            }
                        }
                    }
                }
                Err(_) => {
                    status_message.set("Erro ao acessar projeto".to_string());
                }
            }
        });
        show_confirmation_dialog.set(false);
    };

    let attempt_confirm = move |_| {
        let data = validation_data.read();
        let all_viewed = data.iter().all(|img| img.has_been_viewed);
        
        if all_viewed {
            confirm_validation();
        } else {
            show_confirmation_dialog.set(true);
        }
    };

    let close_dialog = move |_| {
        show_confirmation_dialog.set(false);
    };

    if loading() {
        return rsx! {
            div { class: "min-h-screen bg-gray-100 flex items-center justify-center",
                div { class: "text-center",
                    div { class: "animate-spin rounded-full h-32 w-32 border-b-2 border-blue-600 mx-auto mb-4" }
                    p { class: "text-gray-600", "Carregando dados de validação..." }
                }
            }
        };
    }

    if !error_message().is_empty() {
        return rsx! {
            div { class: "min-h-screen bg-gray-100 flex items-center justify-center",
                div { class: "bg-white rounded-lg shadow-md p-8 max-w-md",
                    div { class: "text-center",
                        i { class: "material-icons text-red-500 text-6xl mb-4", "error" }
                        h2 { class: "text-xl font-bold text-gray-800 mb-4", "Erro ao Carregar Dados" }
                        p { class: "text-gray-600 mb-6", "{error_message()}" }
                        button {
                            class: "px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700",
                            onclick: move |_| navigator.go_back(),
                            "Voltar"
                        }
                    }
                }
            }
        };
    }

    if total_images == 0 {
        return rsx! {
            div { class: "min-h-screen bg-gray-100 flex items-center justify-center",
                div { class: "bg-white rounded-lg shadow-md p-8 max-w-md text-center",
                    i { class: "material-icons text-yellow-500 text-6xl mb-4", "warning" }
                    h2 { class: "text-xl font-bold text-gray-800 mb-4", "Nenhuma Imagem para Validar" }
                    p { class: "text-gray-600 mb-6", "Não foram encontradas imagens com detecções para validação." }
                    button {
                        class: "px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700",
                        onclick: move |_| navigator.go_back(),
                        "Voltar"
                    }
                }
            }
        };
    }

    let current_image = &validation_data.read()[current_idx];
    let viewed_count = validation_data.read().iter().filter(|img| img.has_been_viewed).count();
    let incorrect_count = validation_data.read().iter().filter(|img| img.is_incorrect).count();

    rsx! {
        div { class: "min-h-screen bg-gray-100",
            document::Stylesheet { href: asset!("/assets/tailwind.css") }
            document::Link {
                href: "https://fonts.googleapis.com/icon?family=Material+Icons",
                rel: "stylesheet"
            }

            // Header
            div { class: "bg-white shadow-sm border-b",
                div { class: "container mx-auto px-6 py-4",
                    div { class: "flex items-center justify-between",
                        div {
                            h1 { class: "text-2xl font-bold text-gray-800", "Validação de Fissuras" }
                            p { class: "text-gray-600", "Selecione as imagens com detecções incorretas" }
                        }
                        div { class: "text-right",
                            p { class: "text-sm text-gray-600", 
                                "Imagem {current_idx + 1} de {total_images}" 
                            }
                            p { class: "text-sm text-gray-600", 
                                "Visualizadas: {viewed_count}/{total_images}" 
                            }
                            p { class: "text-sm text-gray-600", 
                                "Marcadas como incorretas: {incorrect_count}" 
                            }
                        }
                    }
                }
            }

            // Progress Bar
            div { class: "bg-white border-b",
                div { class: "container mx-auto px-6 py-2",
                    div { class: "w-full bg-gray-200 rounded-full h-2",
                        div { 
                            class: "bg-blue-600 h-2 rounded-full transition-all duration-300",
                            style: "width: {(viewed_count as f64 / total_images as f64 * 100.0)}%"
                        }
                    }
                }
            }

            // Main Content
            div { class: "container mx-auto px-6 py-8",
                div { class: "grid grid-cols-1 lg:grid-cols-3 gap-8",
                    
                    // Image Display (Left Side - 2/3 width)
                    div { class: "lg:col-span-2",
                        div { class: "bg-white rounded-lg shadow-md overflow-hidden",
                            div { class: "p-6",
                                div { class: "aspect-w-16 aspect-h-12 bg-gray-100 rounded-lg overflow-hidden mb-4",
                                    img {
                                        src: "file://{current_image.path}",
                                        class: "w-full h-full object-contain",
                                        alt: "Imagem para validação"
                                    }
                                }
                                
                                div { class: "flex items-center justify-between",
                                    button {
                                        class: "flex items-center gap-2 px-4 py-2 bg-red-600 text-red rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed",
                                        disabled: current_idx == 0,
                                        onclick: previous_image,
                                        i { class: "material-icons", "arrow_back" }
                                        "Anterior"
                                    }
                                    
                                    button {
                                        class: format!("flex items-center gap-2 px-6 py-3 rounded-md text-black font-medium transition-colors {}",
                                            if current_image.is_incorrect { "bg-red-600 hover:bg-red-700" } else { "bg-gray-400 hover:bg-gray-500" }
                                        ),
                                        onclick: toggle_incorrect,
                                        i { class: "material-icons", 
                                            if current_image.is_incorrect { "close" } else { "check" }
                                        }
                                        if current_image.is_incorrect { "Marcada como Incorreta" } else { "Marcar como Incorreta" }
                                    }
                                    
                                    button {
                                        class: "flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed",
                                        disabled: current_idx >= total_images - 1,
                                        onclick: next_image,
                                        "Próxima"
                                        i { class: "material-icons", "arrow_forward" }
                                    }
                                }
                            }
                        }
                    }

                    // Information Panel (Right Side - 1/3 width)
                    div { class: "space-y-6",
                        
                        // Image Info
                        div { class: "bg-white rounded-lg shadow-md p-6",
                            h3 { class: "text-lg font-semibold text-gray-800 mb-4", "Informações da Imagem" }
                            div { class: "space-y-3",
                                div {
                                    span { class: "font-medium text-gray-700", "Caminho: " }
                                    span { class: "text-sm text-gray-600 break-all", "{current_image.path.split('/').last().unwrap_or(&current_image.path)}" }
                                }
                                div {
                                    span { class: "font-medium text-gray-700", "Status: " }
                                    span { 
                                        class: format!("text-sm px-2 py-1 rounded {}",
                                            if current_image.is_incorrect { "bg-red-100 text-red-800" } else { "bg-green-100 text-green-800" }
                                        ),
                                        if current_image.is_incorrect { "Incorreta" } else { "Correta" }
                                    }
                                }
                                div {
                                    span { class: "font-medium text-gray-700", "Visualizada: " }
                                    span { 
                                        class: format!("text-sm px-2 py-1 rounded {}",
                                            if current_image.has_been_viewed { "bg-blue-100 text-blue-800" } else { "bg-gray-100 text-gray-800" }
                                        ),
                                        if current_image.has_been_viewed { "Sim" } else { "Não" }
                                    }
                                }
                            }
                        }

                        // Fissures Detected
                        div { class: "bg-white rounded-lg shadow-md p-6",
                            h3 { class: "text-lg font-semibold text-gray-800 mb-4", "Fissuras Detectadas" }
                            if current_image.fissuras.is_empty() {
                                p { class: "text-gray-500 italic", "Nenhuma fissura detectada" }
                            } else {
                                div { class: "space-y-3",
                                    for (idx, fissura) in current_image.fissuras.iter().enumerate() {
                                        div { 
                                            key: "{idx}",
                                            class: "border border-gray-200 rounded-lg p-3 bg-gray-50",
                                            div { class: "flex justify-between items-center",
                                                span { class: "font-medium text-gray-800", "{fissura.name}" }
                                                span { 
                                                    class: "text-sm text-gray-600 bg-white px-2 py-1 rounded",
                                                    "{(fissura.confidence * 100.0):.1}%" 
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Action Buttons
                        div { class: "bg-white rounded-lg shadow-md p-6",
                            h3 { class: "text-lg font-semibold text-gray-800 mb-4", "Ações" }
                            div { class: "space-y-3",
                                button {
                                    class: "w-full px-4 py-3 bg-green-600 text-black rounded-md hover:bg-green-700 font-medium",
                                    onclick: attempt_confirm,
                                    "Confirmar Validação"
                                }
                                button {
                                    class: "w-full px-4 py-2 bg-gray-600 text-black rounded-md hover:bg-gray-700",
                                    onclick: move |_| navigator.go_back(),
                                    "Cancelar"
                                }
                            }
                            
                            if !status_message().is_empty() {
                                div { class: "mt-4 p-3 bg-blue-100 border border-blue-200 rounded-md",
                                    p { class: "text-blue-800 text-sm", "{status_message()}" }
                                }
                            }
                        }
                    }
                }
            }

            // Confirmation Dialog
            if show_confirmation_dialog() {
                div { class: "fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50",
                    div { class: "bg-white rounded-lg shadow-xl p-6 max-w-md mx-4",
                        div { class: "text-center",
                            i { class: "material-icons text-yellow-500 text-6xl mb-4", "warning" }
                            h3 { class: "text-lg font-semibold text-gray-800 mb-4", "Confirmação" }
                            p { class: "text-gray-600 mb-6", 
                                "Tem certeza que deseja confirmar sem checar se todas as imagens estão corretas? Você visualizou {viewed_count} de {total_images} imagens." 
                            }
                            div { class: "flex gap-4 justify-center",
                                button {
                                    class: "px-6 py-2 bg-gray-600 text-black rounded-md hover:bg-gray-700",
                                    onclick: close_dialog,
                                    "Cancelar"
                                }
                                button {
                                    class: "px-6 py-2 bg-yellow-600 text-black rounded-md hover:bg-yellow-700",
                                    onclick: move |_| confirm_validation(),
                                    "Confirmar Mesmo Assim"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}