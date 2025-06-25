use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use crate::Route;
use dioxus_router::prelude::Link;
use rfd::AsyncFileDialog;
use futures_util::StreamExt;
use std::{
    path::PathBuf,
    fs, // MODIFICAÇÃO: Adicionado para manipulação de arquivos.
};
use crate::{
    utils::image_processor::{process_folder, ProcessingStats},
    Route as AppRoute,
    pages::create_project::PROJECT_NAME,
    manual_processor::{
        run_yolo_script_and_parse_results,
        ImageAnalysisResult, // MODIFICAÇÃO: Importado para usar no salvamento.
    }
};
use tokio::task;
use tokio::time::{sleep, Duration};
use dioxus::hooks::{use_coroutine, to_owned};
use crate::pages::create_project::ProjectStatus;
use crate::utils::file_manager::update_project_status;

#[derive(Clone)]
struct ProcessRequest {
    path: String,
    threshold: f64,
    project_name: String,
}

// MODIFICAÇÃO: Função auxiliar para salvar os resultados, semelhante à do manual_processor.
// Como esta lógica é necessária em ambos os locais, o ideal seria movê-la para um módulo de utilitários compartilhado.
// Por enquanto, replicamos aqui para simplicidade.
fn save_detection_results_for_auto(
    project_name: &str,
    results: &Vec<ImageAnalysisResult>
) -> Result<(), String> {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let results_path = base_dir.join("Projects").join(project_name).join("detection_results.json");

    let json_content = serde_json::to_string_pretty(results)
        .map_err(|e| format!("Erro ao serializar resultados da detecção: {}", e))?;

    fs::write(&results_path, json_content)
        .map_err(|e| format!("Erro ao salvar arquivo de resultados da detecção ({}): {}", results_path.display(), e))?;

    Ok(())
}

#[component]
pub fn Process() -> Element {
    let mut folder_path = use_signal(|| None::<String>);
    let mut status = use_signal(String::new);
    let mut threshold = use_signal(|| 200.0_f64);
    let mut stats = use_signal(|| None::<ProcessingStats>);
    let mut is_processing = use_signal(|| false);
    let mut is_selecting_folder = use_signal(|| false);
    let navigator = use_navigator();

    let processor_coroutine = use_coroutine(move |mut rx: UnboundedReceiver<ProcessRequest>| {
        to_owned![status, stats, is_processing, navigator];
        async move {
            while let Some(req) = rx.next().await {
                status.set("Organizando imagens... (Isso pode demorar um pouco)".to_string());

                let project_name_for_thread = req.project_name.clone();

                let blocking_result = task::spawn_blocking(move || {
                    process_folder(&req.path, req.threshold, &project_name_for_thread)
                }).await;

                match blocking_result {
                    Ok(Ok(result_data)) => {
                        stats.set(Some(result_data.clone()));
                        if result_data.images_with_gps > 0 {
                            status.set(format!("{} prédios encontrados. Iniciando análise de IA...", result_data.predio_groups));
                            let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                            
                            match run_yolo_script_and_parse_results(&req.project_name, status, &base_dir).await {
                                Ok(analysis_results) => {
                                    // MODIFICAÇÃO: Salva os resultados em um arquivo JSON.
                                    if let Err(e) = save_detection_results_for_auto(&req.project_name, &analysis_results) {
                                        status.set(format!("Erro Crítico: Falha ao salvar resultados da análise: {}", e));
                                        is_processing.set(false);
                                        return;
                                    }

                                    if let Err(e) = update_project_status(&req.project_name, ProjectStatus::ProcessingComplete) {
                                        status.set(format!("Análise concluída, mas falha ao atualizar status: {}", e));
                                    } else {
                                        status.set(format!("Análise concluída e salva! {} resultados.", analysis_results.len()));
                                    }

                                    sleep(Duration::from_secs(2)).await;
                                    status.set("Redirecionando...".to_string());
                                    sleep(Duration::from_secs(1)).await;
                                    navigator.push(AppRoute::ValidationPage {});
                                }
                                Err(e) => {
                                    status.set(format!("Erro na análise de IA: {}", e));
                                }
                            }
                        } else {
                            status.set("Concluído. Nenhuma imagem com GPS foi encontrada.".to_string());
                            sleep(Duration::from_secs(2)).await;
                            status.set("Redirecionando...".to_string());
                            sleep(Duration::from_secs(1)).await;
                            // MODIFICAÇÃO: Mesmo sem GPS, o usuário pode ter processado manualmente. Leva para a página de validação.
                            navigator.push(AppRoute::ValidationPage {});
                        }
                    },
                    Ok(Err(e)) => status.set(format!("Erro no processamento de pastas: {}", e)),
                    Err(e) => status.set(format!("Erro crítico na thread de processamento: {}", e)),
                };
                
                is_processing.set(false);
            }
        }
    });

    let project_name_available = use_memo(move || {
        PROJECT_NAME.try_read().map_or(false, |guard| guard.is_some())
    });
    
    // O CÓDIGO RSX ABAIXO NÃO PRECISA DE ALTERAÇÕES.
    // Ele já lida corretamente com a navegação e a exibição de status.

    rsx! {
        document::Stylesheet { href: asset!("/assets/styles.css") }
        document::Link {
            href: "https://fonts.googleapis.com/icon?family=Material+Icons",
            rel: "stylesheet"
        }

        if is_processing() {
            div {
                style: "position: fixed; top: 0; left: 0; width: 100%; height: 100%; background-color: rgba(0, 0, 0, 0.5); display: flex; justify-content: center; align-items: center; z-index: 9999; flex-direction: column; gap: 1.5rem;",
                div { class: "spinner" }
                p {
                    style: "color: white; font-size: 1.2rem; font-family: 'Poppins', sans-serif;",
                    "{status}"
                }
            }
        }

        div {
            div {
                class: "container",
                style: "max-width: 800px;",

                Link {
                    to: Route::HomePage {},
                    class: "btn btn-secondary",
                    style: "position: fixed; top: 1.5rem; left: 1.5rem; padding: 0.5rem; display: flex; align-items: center; gap: 0.5rem;",
                    title: "Voltar para a página inicial",
                    i { class: "material-icons", "arrow_back" }
                    "Voltar ao Início"
                }

                div {
                    style:"display: flex; justify-content: center; align-items: center; gap: 1rem; margin-bottom: 2rem; margin-top: 4rem;",
                    hr { class: "form-divider", style: "flex-grow: 1;" },
                    h1 {
                        style: "color: black; font-weight: bold; font-size: 1.5rem; text-align: center; white-space: nowrap;",
                        "Organizador de Fotos por Localização"
                    },
                    hr { class: "form-divider", style: "flex-grow: 1;" },
                }

                div { class: "card",

                    div { class: "input-group",
                        input {
                            class: "form-input",
                            r#type: "text",
                            value: folder_path().unwrap_or_default(),
                            readonly: true,
                            placeholder: "Caminho da pasta de imagens...",
                        }

                        button {
                            class: "btn btn-primary",
                            disabled: is_selecting_folder() || is_processing(),
                            onclick: move |_| {
                                is_selecting_folder.set(true);
                                spawn(async move {
                                    if let Some(file_handle) = AsyncFileDialog::new().pick_folder().await {
                                        folder_path.set(Some(file_handle.path().display().to_string()));
                                    }
                                    is_selecting_folder.set(false);
                                });
                            },
                            i { class: "material-icons", "folder" }
                            if is_selecting_folder() { "Selecionando..." } else { "Selecionar Pasta" }
                        }
                    }

                    hr { class: "form-divider" }

                    div {
                        class: "form-group",
                        label { "Distância máxima entre imagens do mesmo prédio (metros):" }
                        input {
                            class: "form-input",
                            r#type: "number",
                            value: "{threshold()}",
                            min: "10",
                            step: "10",
                            onchange: move |e| {
                                if let Ok(val) = e.value().parse::<f64>() {threshold.set(val);}
                            }
                        }
                    }

                    hr { class: "form-divider" }

                    if !project_name_available() {
                        p { class: "status-message error",
                            "Para habilitar o processamento, por favor, primeiro crie um projeto na tela 'Criar Novo Projeto'."
                        }
                    }

                    div {
                        class: "flex",
                        style: "gap: 1.5rem; display: flex; flex-direction: row; justify-content: space-between;",
                            button {
                                class:"btn btn-primary",
                                style: "flex: 1; font-size: 1rem;",
                                disabled: is_processing() || folder_path().is_none() || !project_name_available(),
                                onclick: move |_| {
                                    if let Some(path_str) = folder_path() {
                                        let project_name_guard = match PROJECT_NAME.try_read() {
                                            Ok(guard) => guard,
                                            Err(_) => {
                                                status.set("Erro: Não foi possível acessar o nome do projeto. Tente novamente.".to_string());
                                                return;
                                            }
                                        };

                                        let project_name = match project_name_guard.as_ref() {
                                            Some(name) => name.clone(),
                                            None => {
                                                status.set("Erro: Nome do projeto é inválido.".to_string());
                                                return;
                                            }
                                        };

                                        is_processing.set(true);
                                        status.set("Iniciando processamento...".to_string());

                                        processor_coroutine.send(ProcessRequest {
                                            path: path_str,
                                            threshold: threshold(),
                                            project_name: project_name
                                        });
                                    }
                                },
                                    i { class: "material-icons", "sync" }
                                    if is_processing() { "Processando..." } else { "Automático" }
                            }

                        div {
                            style:"display: flex; flex-direction: column; justify-content: center; align-items: center;",
                            div { style:"border-left: 1px solid #ccc; height: 24px;" }
                            p { style:"color: #888; margin: 4px 0;" }
                            div { style:"border-left: 1px solid #ccc; height: 24px;" }
                        }
                        Link {
                            // MODIFICAÇÃO: Corrigido para passar o nome do projeto corretamente.
                            to: Route::ManualProcessor {
                                project_name: PROJECT_NAME.with(|opt| opt.clone().unwrap_or_default())
                            },
                            button {
                                class:"btn btn-primary",
                                style: "flex: 1; font-size: 1rem;",
                                disabled: is_processing() || !project_name_available(),
                                i { class: "material-icons", "folder_open" }
                                "Processar Manualmente"
                            }
                        }
                    }

                    if !is_processing() && !status.read().is_empty() {
                        p { class: "status-message info", "{status}" }
                    }

                    if let Some(stats_data) = stats.read().as_ref() {
                        div { class: "card", style: "background: #f8f9fa;",
                            h2 { style: "font-size: 1.25rem; font-weight: 600; margin-bottom: 1rem;", "Estatísticas" }
                            div { class: "grid grid-cols-2 gap-4",
                                p { "Total de imagens: {stats_data.total_images}" }
                                p { "Imagens com GPS: {stats_data.images_with_gps}" }
                                p { "Imagens sem GPS: {stats_data.images_without_gps}" }
                                p { "Imagens com direção: {stats_data.images_with_direction}" }
                                p { "Prédios identificados: {stats_data.predio_groups}" }
                            }
                            if !stats_data.errors.is_empty() {
                                div { class: "mt-4",
                                    h3 { class: "text-xl font-semibold mb-2 text-red-600", "Erros:" }
                                    ul { class: "list-disc list-inside text-red-500",
                                        {stats_data.errors.iter().map(|error| rsx! { li { "{error}" } })}
                                    }
                                }
                            }
                        }

                        if !is_processing() && stats_data.images_with_gps > 0 {
                            div { class: "text-center",

                                {
                                    let project_name = PROJECT_NAME.try_read().ok().and_then(|guard| guard.clone());
                                    if let Some(name) = project_name {
                                        let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                                        let detection_file = base_dir.join("Projects").join(&name).join("detection_results.json");
                                        if detection_file.exists() {
                                            rsx! {
                                                Link {
                                                    to: AppRoute::ValidationPage {},
                                                    button {
                                                        class: "btn btn-primary",
                                                        i { class: "material-icons", "verified" }
                                                        "Validar Resultados da IA"
                                                    }
                                                }
                                            }
                                        } else {
                                            rsx! {
                                                p { class: "text-gray-600 text-sm italic",
                                                    "Processamento de IA ainda não concluído"
                                                }
                                            }
                                        }
                                    } else {
                                        rsx! { }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}