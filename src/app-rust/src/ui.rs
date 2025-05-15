use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use crate::image_processor::{process_folder, ProcessingStats};

// Componente principal da interface do usuário
pub fn app() -> Element {
    let mut folder_path = use_signal(|| None::<String>);
    let mut status = use_signal(|| String::new());
    let mut threshold = use_signal(|| 0.001_f64);
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
                        
                        // Usando AsyncFileDialog da rfd para evitar problemas de threads
                        spawn(async move {
                            // Uso do AsyncFileDialog que é mais compatível com frameworks como Dioxus
                            if let Some(file_handle) = AsyncFileDialog::new().pick_folder().await {
                                // A classe FileHandle tem o método path() que retorna o caminho
                                folder_path.set(Some(file_handle.path().display().to_string()));
                            }
                            is_selecting_folder.set(false);
                        });
                    },
                    if is_selecting_folder() { "Selecionando..." } else { "Selecionar Pasta" }
                }
            }
            div { class: "input-group",
                label { "Limiar de Proximidade (graus):" }
                input {
                    r#type: "number",
                    value: "{threshold()}",
                    min: "0.0001",
                    step: "0.0001",
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
                        // Indica que está processando
                        is_processing.set(true);
                        status.set("Processando imagens...".to_string());
                        
                        // Clone os valores para uso na task
                        let path_clone = path.clone();
                        let threshold_value = threshold();
                        
                        // Inicia uma tarefa em segundo plano
                        spawn(async move {
                            // Execute o processamento em segundo plano
                            let result = process_folder(&path_clone, threshold_value);
                            
                            // Atualize a interface com o resultado
                            match result {
                                Ok(result) => {
                                    stats.set(Some(result));
                                    status.set("Processamento concluído com sucesso!".to_string());
                                }
                                Err(e) => {
                                    status.set(format!("Erro: {}", e));
                                }
                            }
                            
                            // Finaliza o processamento
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
                    p { "Grupos de localização: {stats.location_groups}" }
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