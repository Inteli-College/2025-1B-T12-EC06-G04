use dioxus::prelude::*;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};
use dioxus_router::prelude::Link;
use crate::Route;

use crate::{
    utils::file_manager::{
        display_from_projects,
        Files,
        FileEntry
    }
};

use dioxus_router::prelude::use_navigator;
use crate::pages::create_project::{PROJECT_NAME, ProjectStatus};
use crate::pages::graph::read_project_metadata;

#[allow(non_snake_case)]
pub fn HomePage() -> Element {
    let processed_folder_signal = use_context::<Signal<Option<PathBuf>>>();
    let initial_path_from_state = processed_folder_signal.read().clone();

    let mut files = use_signal(|| Files::new(initial_path_from_state));
    let navigator = use_navigator();

    use_effect(move || {
        let new_path = processed_folder_signal.read().clone();
        let mut files_mut = files.write();
        files_mut.update_base_path_if_different(new_path);
        files_mut.reload_path_list();
    });

    let mut sort_alphabetical_order = use_signal(|| SortAlphabeticOrder::AZ);
    let mut sort_date_order = use_signal(|| SortDateOrder::MaisRecente);
    let mut show_filters = use_signal(|| false);

    let alphabetical_order = sort_alphabetical_order.read();
    let date_order = sort_date_order.read();

    let binding = files.read();
    let mut entries: Vec<_> = binding.path_names.iter().collect();

    entries.sort_by(|a, b| {
        let date_a = a.created.as_ref().and_then(|s| DateTime::parse_from_rfc3339(s).ok());
        let date_b = b.created.as_ref().and_then(|s| DateTime::parse_from_rfc3339(s).ok());

        let date_cmp = match *date_order {
            SortDateOrder::MaisRecente => date_b.cmp(&date_a),
            SortDateOrder::MaisAntigo => date_a.cmp(&date_b),
        };

        if date_cmp == std::cmp::Ordering::Equal {
            let name_a = a.path.file_name()
                .map(|n| n.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            let name_b = b.path.file_name()
                .map(|n| n.to_string_lossy().to_lowercase())
                .unwrap_or_default();

            match *alphabetical_order {
                SortAlphabeticOrder::AZ => name_a.cmp(&name_b),
                SortAlphabeticOrder::ZA => name_b.cmp(&name_a),
            }
        } else {
            date_cmp
        }
    });

    if *alphabetical_order == SortAlphabeticOrder::ZA {
        entries.reverse();
    }

    use_effect(move || {
        let new_path = processed_folder_signal.read().clone();
        files.write().update_base_path_if_different(new_path);
    });

    let mut search_input = use_signal(|| String::new());

    let folder_cards = entries.iter().enumerate()
        .filter_map(|(_dir_id, entry)| {
            let path = &entry.path;
            let folder_name = path.file_name()?.to_string_lossy().to_string();
            let path_display = display_from_projects(path)
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| path.display().to_string());
            
            let created = entry.created.as_ref()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                .map(|dt| {
                    let local_dt: DateTime<Local> = dt.into();
                    local_dt.format("%d/%m/%Y").to_string()
                })
                .unwrap_or_else(|| "Sem data".to_string());

            let description = entry.description.clone().unwrap_or_else(|| "Sem descrição".to_string());

            let search = search_input.read().to_lowercase();
            if !search.is_empty() && !folder_name.to_lowercase().contains(&search) {
                return None;
            }

            let folder_name_clone = folder_name.clone();
            let nav = navigator.clone();

            Some(rsx!(
                div {
                    class: "folder-card",
                    key: "{path_display}",
                    onclick: move |_| {
                        let project_folder = folder_name_clone.clone();
                        let navigator_clone = nav.clone();
                        
                        *PROJECT_NAME.write() = Some(project_folder.clone());

                        spawn(async move {
                            let route = match read_project_metadata(&project_folder) {
                                Ok(metadata) => {
                                    match metadata.status {
                                        ProjectStatus::Created => Route::Process {},
                                        // MODIFICAÇÃO: Navega para a rota renomeada.
                                        ProjectStatus::ProcessingComplete => Route::ValidationPage {},
                                        ProjectStatus::ValidationComplete => Route::GraphView { project_name: project_folder },
                                    }
                                }
                                Err(_) => {
                                    Route::Process {} 
                                }
                            };
                            navigator_clone.push(route);
                        });
                    },
                    i { class: "material-icons", "folder" }
                    h2 { title: "{folder_name}", "{folder_name}" }
                    p { class: "date", "{created}" }
                    p { class: "description", "{description}" }
                }
            ))
        })
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    rsx! {
        document::Stylesheet { href: asset!("/assets/styles.css") }
        document::Link {
            href: "https://fonts.googleapis.com/icon?family=Material+Icons",
            rel: "stylesheet"
        }

        body {
            header { class: "page-header",
                div { class: "header-group",
                    h1 { "Projetos" }
                }
            }

            div { class: "controls-bar",
                div {
                    input {
                        r#type: "text",
                        class: "search-input",
                        placeholder: "Buscar projeto...",
                        oninput: move |e| search_input.set(e.value().clone()),
                        value: "{search_input}",
                    }
                }

                div { class: "filter-controls",
                    button {
                        class: if *show_filters.read() { "filter-toggle active" } else { "filter-toggle" },
                        onclick: move |_| show_filters.toggle(),
                        i { class: "material-icons", "filter_list" }
                    }

                    if *show_filters.read() {
                        div {
                            class: "filter-buttons-container",
                            
                            button {
                                class: format!("filter-button {}",
                                    if *date_order == SortDateOrder::MaisRecente { "selected" } else { "unselected" }
                                ),
                                onclick: move |_| sort_date_order.set(SortDateOrder::MaisRecente),
                                "Mais recente"
                            }
                            button {
                                class: format!("filter-button {}",
                                    if *date_order == SortDateOrder::MaisAntigo { "selected" } else { "unselected" }
                                ),
                                onclick: move |_| sort_date_order.set(SortDateOrder::MaisAntigo),
                                "Mais antigo"
                            }
                            button {
                                class: format!("filter-button {}",
                                    if *alphabetical_order == SortAlphabeticOrder::AZ { "selected" } else { "unselected" }
                                ),
                                onclick: move |_| sort_alphabetical_order.set(SortAlphabeticOrder::AZ),
                                "A-Z"
                            }
                            button {
                                class: format!("filter-button {}",
                                    if *alphabetical_order == SortAlphabeticOrder::ZA { "selected" } else { "unselected" }
                                ),
                                onclick: move |_| sort_alphabetical_order.set(SortAlphabeticOrder::ZA),
                                "Z-A"
                            }
                        }
                    }
                }
            }

            main {
                class: "folder-grid",
                if folder_cards.is_empty() {
                    div { 
                        class: "empty-state",
                        div { class: "empty-state-content",
                            div { class: "empty-state-icon-wrapper",
                                i { class: "material-icons empty-icon", "folder_open" }
                            }
                            h3 { class: "empty-state-title", "Nenhum projeto encontrado" }
                            p { 
                                class: "empty-state-description", 
                                "Não existem projetos criados.",
                                br {},
                                "Que tal criar seu primeiro projeto para começar?"
                            }
                            Link { 
                                to: Route::NewProject {},
                                class: "btn btn-primary create-folder-btn",
                                i { class: "material-icons", "add" }
                                "Criar Novo Projeto"
                            }
                        }
                    }
                } else {
                    { folder_cards.into_iter() }
                }
            }

            if let Some(err) = files.read().err.as_ref() {
                div { class: "status-message error",
                    code { class: "text-sm", "{err}" }
                    button {
                        class: "text-red-500 hover:text-red-700",
                        onclick: move |_| files.write().clear_err(),
                        "x"
                    }
                }
            }
            
            Link {
                to: Route::NewProject {},
                class: "fab btn-secondary",
                title: "Novo Projeto",
                i { class: "material-icons", "add" }
            }
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum SortAlphabeticOrder {
    AZ,
    ZA,
}

#[derive(PartialEq, Clone, Copy)]
enum SortDateOrder {
    MaisRecente,
    MaisAntigo,
}