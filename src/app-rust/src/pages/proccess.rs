// proccess.rs

use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use crate::Route;
use dioxus_router::prelude::Link;
use rfd::AsyncFileDialog;
use futures_util::StreamExt;
use chrono::{DateTime, Local};
use std::{
    path::PathBuf,
    rc::Rc,
};
use crate::{
    utils::image_processor::{process_folder, ProcessingStats},
    Route as AppRoute,
    pages::create_project::PROJECT_NAME,
    utils::file_manager::{
        display_from_projects,
        Files,
        FileEntry,
    },
    manual_processor::{
        ManualProcessor,
        ManualProcessorProps,
        run_yolo_script_and_parse_results
    }
};
use tokio::task;
use tokio::time::{sleep, Duration};
use dioxus::hooks::{use_coroutine, to_owned};
use crate::pages::create_project::ProjectStatus;
use crate::utils::file_manager::update_project_status;


// Uma struct de mensagem para iniciar o processamento na coroutine
#[derive(Clone)]
struct ProcessRequest {
    path: String,
    threshold: f64,
    project_name: String,
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
                                    if let Err(e) = update_project_status(&req.project_name, ProjectStatus::ProcessingComplete) {
                                        status.set(format!("Análise concluída, mas falha ao atualizar status: {}", e));
                                    } else {
                                        status.set(format!("Análise concluída! {} resultados.", analysis_results.len()));
                                    }

                                    sleep(Duration::from_secs(2)).await;
                                    status.set("Redirecionando...".to_string());
                                    sleep(Duration::from_secs(1)).await;
                                    // MODIFICAÇÃO: Navega para a rota renomeada.
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
                            // MODIFICAÇÃO: Navega para a rota renomeada.
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

    let mut processed_folder_signal = use_context::<Signal<Option<PathBuf>>>();

    let project_name_available = use_memo(move || {
        PROJECT_NAME.try_read().map_or(false, |guard| guard.is_some())
    });

    let handle = use_coroutine(move |mut rx: UnboundedReceiver<Option<PathBuf>>| async move {
        use futures_util::StreamExt;
        while let Some(path) = rx.next().await {
            processed_folder_signal.set(path);
        }
    });

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

                // BOTÃO ADICIONADO AQUI
                div {
                    style: "margin-bottom: 2rem;",
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| {
                            navigator.push(Route::HomePage {});
                        },
                        i { class: "material-icons", "arrow_back" }
                        "Voltar ao Início"
                    }
                }

                div {
                    style:"display: flex; justify-content: center; align-items: center; gap: 1rem; margin-bottom: 2rem;",
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
                            to: Route::ManualProcessor {
                                project_name: project_name_available.to_string().clone()
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
                                                // MODIFICAÇÃO: Link para a rota renomeada.
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

// O componente folders_popup permanece o mesmo
fn folders_popup(send: Rc<dyn Fn(Option<PathBuf>)>) -> Element {
    let processed_folder_signal = use_context::<Signal<Option<PathBuf>>>();
    let initial_path_from_state = processed_folder_signal.read().clone();
    let mut files = use_signal(|| Files::new(initial_path_from_state));

    use_effect(move || {
        let new_path = processed_folder_signal.read().clone();
        files.write().update_base_path_if_different(new_path);
    });

    let mut new_folder_name = use_signal(|| String::new());
    let mut new_folder_description = use_signal(|| String::new());
    let mut show_new_folder_input = use_signal(|| false);

    let file_cards = files.read().path_names.iter().enumerate()
    .filter_map(|(dir_id, entry)| {
        let path = &entry.path;
        let path_end = path.file_name()?.to_string_lossy();
        let path_display = display_from_projects(path)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| path.display().to_string());
        let created = entry.created.clone().unwrap_or_default();

        Some(rsx!(
            div {
                class: "flex flex-col items-center text-center bg-white shadow rounded-lg p-4 cursor-pointer hover:shadow-lg hover:bg-blue-50 transition duration-300 ease-in-out",
                key: "{path_display}",
                onclick: move |_| files.write().enter_dir(dir_id),

                i { class: "material-icons text-6xl text-blue-500 mb-2", "folder" }
                h2 { class: "mt-2 font-semibold text-base text-gray-900 truncate max-w-full", "{path_end}" }
                p { class: "text-xs text-gray-400 mt-1", "{created}" }
            }
        ))
    })
    .filter_map(Result::ok)
    .collect::<Vec<_>>();

    rsx! {
        document::Stylesheet { href: asset!("/assets/styles.css") }

        div { class: "min-h-screen bg-gray-100 text-gray-900 font-sans",
            document::Link {
                href: "https://fonts.googleapis.com/icon?family=Material+Icons",
                rel: "stylesheet"
            }

            header { class: "flex items-center justify-between bg-blue-600 text-black p-4 shadow",
                div { class: "flex items-center gap-4",
                    i { class: "material-icons", "menu" }
                    h1 { class: "text-xl font-bold", "Files: {files.read().current()}" }
                }
                i {
                    class: "material-icons cursor-pointer hover:text-red-200",
                    onclick: move |_| files.write().go_up(),
                    "logout"
                }
            }

            main {
                class: "p-6 grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6 max-w-7xl mx-auto",
                { file_cards.into_iter() }
            }

            if let Some(err) = files.read().err.as_ref() {
                div { class: "bg-red-100 text-red-700 p-4 rounded shadow flex justify-between items-center col-span-full",
                    code { class: "text-sm", "{err}" }
                    button {
                        class: "text-red-500 hover:text-red-700",
                        onclick: move |_| files.write().clear_err(),
                        "x"
                    }
                }
            }

            if *show_new_folder_input.read() {
                div {
                    class: "fixed bottom-24 right-6 bg-white border shadow-lg rounded-lg p-4 flex flex-col gap-2 w-80 max-w-full z-50",

                    h2 { class: "text-lg font-semibold text-gray-800", "Novo Projeto" }

                    input {
                        class: "border rounded px-3 py-2 w-full",
                        r#type: "text",
                        placeholder: "Nome da nova pasta",
                        value: "{new_folder_name.read()}",
                        oninput: move |e| new_folder_name.set(e.value())
                    }

                    textarea {
                        class: "border rounded px-3 py-2 w-full resize-none",
                        rows: "4",
                        placeholder: "Descrição do projeto",
                        value: "{new_folder_description.read()}",
                        oninput: move |e| new_folder_description.set(e.value())
                    }

                    div { class: "flex justify-end gap-2 mt-2",
                        button {
                            style:"background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%); padding: 0.5rem 1.5rem; border-radius: 8px; color: white; font-weight: 500; cursor: pointer;",
                            class: "text-gray-500 text-sm hover:underline",
                            onclick: move |_| {
                                show_new_folder_input.set(false);
                                new_folder_name.set(String::new());
                                new_folder_description.set(String::new());
                            },
                            "Cancelar"
                        }
                        button {
                            style:"background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%); padding: 8px 12px; border-radius: 8px; color: white; font-weight: 500; cursor: pointer;",
                            class: "bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded",
                            onclick: move |_| {
                                let name = new_folder_name.read().trim().to_string();
                                let description = new_folder_description.read().trim().to_string();

                                if !name.is_empty() {
                                    files.write().create_folder_with_description(name.clone(), description.clone());
                                    new_folder_name.set(String::new());
                                    new_folder_description.set(String::new());
                                    show_new_folder_input.set(false);
                                }
                            },
                            "Criar Pasta"
                        }
                    }
                }
            }

            button {
                style:"background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%); padding: 0.5rem 1.5rem; border-radius: 8px; color: white; font-weight: 500; cursor: pointer;",
                class: "fixed bottom-6 right-6 bg-purple-100 hover:bg-purple-200 text-purple-600 shadow-lg p-4 rounded-full",
                title: "Nova Pasta",
                onclick: move |_| show_new_folder_input.set(true),
                i { class: "material-icons", "edit" }
            }

            button {
                style:"background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%); padding: 8px 12px; border-radius: 8px; color: white; font-weight: 500; cursor: pointer;",
                class: "fixed bottom-6 left-6 bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-full shadow-lg",
                onclick: move |_| {
                    send(Some(files.read().current_path.clone()));
                    dioxus::desktop::window().close();
                },
                "Selecionar Pasta"
            }
        }
    }
}