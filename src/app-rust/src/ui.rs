use dioxus::prelude::*;
use crate::image_processor;

pub fn app(cx: Scope) -> Element {
    let folder_path = use_state(&cx, || None::<String>);
    let status = use_state(&cx, || String::new());
    let threshold = use_state(&cx, || 0.001); // Limiar padrão para proximidade (em graus)

    cx.render(rsx! {
        div {
            class: "container",
            h1 { "Organizador de Imagens por Proximidade Geográfica" }
            
            div {
                class: "input-group",
                label { "Limiar de Proximidade (graus): " }
                input {
                    r#type: "number",
                    value: "{threshold}",
                    onchange: move |e| {
                        if let Ok(value) = e.value.parse::<f64>() {
                            threshold.set(value);
                        }
                    }
                }
            }

            button {
                onclick: move |_| {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        folder_path.set(Some(path.display().to_string()));
                        status.set("Processando imagens...".to_string());
                        
                        // Processa imagens em uma thread separada para não bloquear a UI
                        let path_str = path.display().to_string();
                        let threshold_value = threshold.get();
                        
                        std::thread::spawn(move || {
                            if let Err(e) = image_processor::process_images(&path_str, *threshold_value) {
                                status.set(format!("Erro: {}", e));
                            } else {
                                status.set("Processamento concluído!".to_string());
                            }
                        });
                    }
                },
                "Selecionar Pasta"
            }

            if let Some(path) = folder_path.get() {
                p { "Pasta selecionada: {path}" }
            }

            p { "{status}" }
        }
    })
} 