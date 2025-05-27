use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use rfd::AsyncFileDialog;
use crate::image_processor::{process_folder, ProcessingStats};
use crate::Route;
use std::path::PathBuf;

#[component]
pub fn Home() -> Element {
    let mut folder_path = use_signal(|| None::<String>);
    let mut status = use_signal(|| String::new());
    let mut threshold = use_signal(|| 200.0_f64);
    let mut stats = use_signal(|| None::<ProcessingStats>);
    let mut is_processing = use_signal(|| false);
    let mut is_selecting_folder = use_signal(|| false);

    let mut processed_folder_signal = use_context::<Signal<Option<PathBuf>>>();

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        div { class: "min-h-screen bg-gray-100 text-gray-900 font-sans",
            document::Link {
                href: "https://fonts.googleapis.com/icon?family=Material+Icons",
                rel: "stylesheet"
            }

            div { class: "container mx-auto px-4 py-8 max-w-4xl",
                h1 { class: "text-3xl font-bold text-center mb-8 text-gray-800", 
                    "Organizador de Fotos por Localização" 
                }
                div { class: "bg-white rounded-lg shadow-md p-6 mb-6",
                    div { class: "flex gap-4 mb-6",
                        input {
                            class: "flex-1 px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "text",
                            value: folder_path().unwrap_or_default(),
                            readonly: true,
                            placeholder: "Selecione uma pasta..."
                        }
                        button {
                            class: "px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2",
                            disabled: is_selecting_folder(),
                            onclick: move |_| {
                                is_selecting_folder.set(true);
                                spawn(async move {
                                    if let Some(file_handle) = AsyncFileDialog::new().pick_folder().await {
                                        folder_path.set(Some(file_handle.path().display().to_string()));
                                        processed_folder_signal.set(None);
                                    }
                                    is_selecting_folder.set(false);
                                });
                            },
                            i { class: "material-icons", "folder" }
                            if is_selecting_folder() { "Selecionando..." } else { "Selecionar Pasta" }
                        }
                    }
                    div { class: "mb-6",
                        label { class: "block text-gray-700 mb-2", 
                            "Distância máxima entre imagens do mesmo prédio (metros):" 
                        }
                        input {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                            r#type: "number",
                            value: "{threshold()}",
                            min: "10",
                            step: "10",
                            onchange: move |e| {
                                if let Ok(val) = e.value().parse::<f64>() {
                                    threshold.set(val);
                                }
                            }
                        }
                    }
                    button {
                        class: "w-full px-6 py-3 bg-green-600 text-white rounded-md hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2",
                        disabled: is_processing() || folder_path().is_none(),
                        onclick: move |_| {
                            if let Some(path_str) = folder_path() {
                                is_processing.set(true);
                                status.set("Processando imagens...".to_string());
                                
                                let path_clone_for_processing = path_str.clone();
                                let threshold_value = threshold();
                                let path_clone_for_state = path_str.clone();
                                
                                spawn(async move {
                                    let result = process_folder(&path_clone_for_processing, threshold_value);
                                    
                                    match result {
                                        Ok(result_data) => {
                                            stats.set(Some(result_data.clone()));
                                            if result_data.images_with_gps > 0 {
                                                status.set(format!("Processamento concluído! {} imagens com GPS organizadas em {} prédios.", 
                                                                 result_data.images_with_gps, result_data.predio_groups));
                                                processed_folder_signal.set(Some(PathBuf::from(path_clone_for_state)));
                                            } else {
                                                status.set("Processamento concluído, mas nenhuma imagem com GPS foi encontrada.".to_string());
                                                processed_folder_signal.set(None);
                                            }
                                        }
                                        Err(e) => {
                                            status.set(format!("Erro: {}", e));
                                            processed_folder_signal.set(None);
                                        }
                                    }
                                    
                                    is_processing.set(false);
                                });
                            }
                        },
                        i { class: "material-icons", "sync" }
                        if is_processing() { "Processando..." } else { "Processar automaticamente (experimental)" }
                    }
                }

                if is_processing() {
                    div { class: "text-center py-4 text-gray-600",
                        "Carregando... Por favor, aguarde."
                    }
                }

                if let Some(stats_data) = stats.read().as_ref() {
                    div { class: "bg-white rounded-lg shadow-md p-6 mb-6",
                        h2 { class: "text-2xl font-semibold mb-4 text-gray-800", "Estatísticas" }
                        div { class: "grid grid-cols-2 gap-4",
                            p { class: "text-gray-700", "Total de imagens: {stats_data.total_images}" }
                            p { class: "text-gray-700", "Imagens com GPS: {stats_data.images_with_gps}" }
                            p { class: "text-gray-700", "Imagens sem GPS: {stats_data.images_without_gps}" }
                            p { class: "text-gray-700", "Imagens com direção: {stats_data.images_with_direction}" }
                            p { class: "text-gray-700", "Prédios identificados: {stats_data.predio_groups}" }
                        }
                        if !stats_data.errors.is_empty() {
                            div { class: "mt-4",
                                h3 { class: "text-xl font-semibold mb-2 text-red-600", "Erros:" }
                                ul { class: "list-disc list-inside text-red-500",
                                    {stats_data.errors.iter().map(|error| rsx! {
                                        li { "{error}" }
                                    })}
                                }
                            }
                        }
                    }

                    if !is_processing() && stats_data.images_with_gps > 0 {
                        div { class: "text-center",
                            Link {
                                to: Route::Folders {},
                                button { 
                                    class: "px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 flex items-center gap-2",
                                    i { class: "material-icons", "folder" }
                                    "Visualizar Pastas Organizadas"
                                }
                            }
                        }
                    }
                }

                if !status().is_empty() {
                    p { class: "mt-4 text-center text-gray-700", "{status()}" }
                }
            }
        }
    }
}
