use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use crate::image_processor::{process_folder, ProcessingStats};

// Componente principal da interface do usuário
pub fn app() -> Element {
    let mut folder_path = use_signal(|| None::<String>);
    let mut status = use_signal(|| String::new());
    let mut threshold = use_signal(|| 200.0_f64); // Alterado para metros em vez de graus
    let mut stats = use_signal(|| None::<ProcessingStats>);
    let mut is_processing = use_signal(|| false);
    let mut is_selecting_folder = use_signal(|| false);

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
                    if let Some(path) = folder_path() {
                        is_processing.set(true);
                        status.set("Processando imagens...".to_string());
                        
                        let path_clone = path.clone();
                        let threshold_value = threshold();
                        
                        spawn(async move {
                            let result = process_folder(&path_clone, threshold_value);
                            
                            match result {
                                Ok(result) => {
                                    stats.set(Some(result.clone()));
                                    if result.images_with_gps > 0 {
                                        status.set(format!("Processamento concluído! {} imagens com GPS organizadas em {} prédios.", 
                                                         result.images_with_gps, result.predio_groups));
                                    } else {
                                        status.set("Processamento concluído, mas nenhuma imagem com GPS foi encontrada.".to_string());
                                    }
                                }
                                Err(e) => {
                                    status.set(format!("Erro: {}", e));
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
            if let Some(stats) = stats.read().as_ref() {
                div { class: "stats",
                    h2 { "Estatísticas" }
                    p { "Total de imagens: {stats.total_images}" }
                    p { "Imagens com GPS: {stats.images_with_gps}" }
                    p { "Imagens sem GPS: {stats.images_without_gps}" }
                    p { "Imagens com direção: {stats.images_with_direction}" }
                    p { "Prédios identificados: {stats.predio_groups}" }
                    if !stats.errors.is_empty() {
                        div { class: "errors",
                            h3 { "Erros:" }
                            ul {
                                {stats.errors.iter().map(|error| rsx! {
                                    li { "{error}" }
                                })}
                            }
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
