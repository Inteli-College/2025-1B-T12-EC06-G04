use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use rfd::AsyncFileDialog;
use crate::image_processor::{process_folder, ProcessingStats};
use crate::Route;
use std::path::PathBuf;

// Componente principal da interface do usuário

#[component]
pub fn Home() -> Element {
    let mut folder_path = use_signal(|| None::<String>);
    let mut status = use_signal(|| String::new());
    let mut threshold = use_signal(|| 200.0_f64); // Alterado para metros em vez de graus
    let mut stats = use_signal(|| None::<ProcessingStats>);
    let mut is_processing = use_signal(|| false);
    let mut is_selecting_folder = use_signal(|| false);

    let mut processed_folder_signal = use_context::<Signal<Option<PathBuf>>>();

    rsx! {
        div { class: "container",
            h1 { "Organizador de Fotos por Localização" }
            div { class: "input-group",
                input {
                    r#type: "text",
                    value: folder_path().unwrap_or_default(),
                    readonly: true,
                    placeholder: "Selecione uma pasta..."
                }
                button {
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
                    if is_selecting_folder() { "Selecionando..." } else { "Selecionar Pasta" }
                }
            }
            div { class: "input-group",
                label { "Distância máxima entre imagens do mesmo prédio (metros):" }
                input {
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
                if is_processing() { "Processando..." } else { "Processar Imagens" }
            }
            if is_processing() {
                div { class: "loading", "Carregando... Por favor, aguarde." }
            }
            if let Some(stats_data) = stats.read().as_ref() {
                div { class: "stats",
                    h2 { "Estatísticas" }
                    p { "Total de imagens: {stats_data.total_images}" }
                    p { "Imagens com GPS: {stats_data.images_with_gps}" }
                    p { "Imagens sem GPS: {stats_data.images_without_gps}" }
                    p { "Imagens com direção: {stats_data.images_with_direction}" }
                    p { "Prédios identificados: {stats_data.predio_groups}" }
                    if !stats_data.errors.is_empty() {
                        div { class: "errors",
                            h3 { "Erros:" }
                            ul {
                                {stats_data.errors.iter().map(|error| rsx! {
                                    li { "{error}" }
                                })}
                            }
                        }
                    }
                }
                if !is_processing() && stats_data.images_with_gps > 0 {
                    div { class: "input-group", style: "margin-top: 20px;",
                        Link {
                            to: Route::Folders {},
                            button { class: "action-button", "Visualizar Pastas Organizadas" }
                        }
                    }
                }
            }
            if !status().is_empty() {
                p { class: "status", "{status()}" }
            }
        }
    }
}
