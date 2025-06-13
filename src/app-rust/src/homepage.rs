use crate::Route;
use chrono::{DateTime, Local};
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use std::path::{Path, PathBuf};

fn display_from_projects(path: &Path) -> Option<PathBuf> {
    for ancestor in path.ancestors() {
        if ancestor
            .file_name()
            .map_or(false, |name| name == "Projects")
        {
            return path.strip_prefix(ancestor).ok().map(|p| p.to_path_buf());
        }
    }
    None
}

#[allow(non_snake_case)]
pub fn HomePage() -> Element {
    let processed_folder_signal = use_context::<Signal<Option<PathBuf>>>();
    let initial_path_from_state = processed_folder_signal.read().clone();

    let mut files = use_signal(|| Files::new(initial_path_from_state));

    let mut sort_alphabetical_order = use_signal(|| SortAlphabeticOrder::AZ);
    let mut sort_date_order = use_signal(|| SortDateOrder::MaisRecente);

    let mut show_filters = use_signal(|| false);

    let alphabetical_order = sort_alphabetical_order.read();
    let date_order = sort_date_order.read();

    let binding = files.read();
    let mut entries: Vec<_> = binding.path_names.iter().collect();

    entries.sort_by(|a, b| {
        let date_a = a
            .created
            .as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok());
        let date_b = b
            .created
            .as_ref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok());

        let date_cmp = match *date_order {
            SortDateOrder::MaisRecente => date_b.cmp(&date_a),
            SortDateOrder::MaisAntigo => date_a.cmp(&date_b),
        };

        if date_cmp == std::cmp::Ordering::Equal {
            let name_a = a
                .path
                .file_name()
                .map(|n| n.to_string_lossy().to_lowercase())
                .unwrap_or_default();
            let name_b = b
                .path
                .file_name()
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
            let folder_name = path.file_name()?.to_string_lossy();
            let path_display = display_from_projects(path)
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| path.display().to_string());
            let created = entry.created.clone().unwrap_or_default();
            let description = entry.description.clone().unwrap_or_else(|| "Sem descrição".to_string());

            let search = search_input.read().to_lowercase();
            if !search.is_empty() && !folder_name.to_lowercase().contains(&search) {
                return None;
            }

            Some(rsx!(
                Link {
                    to: Route::GraphView { project_name: folder_name.to_string() },
                    class: "folders flex flex-col items-center text-center cursor-pointer",
                    key: "{path_display}",
                    i { class: "material-icons text-6xl text-blue-500 mb-2", "folder" }
                    h2 { class: "mt-2 font-semibold text-base text-gray-900 truncate max-w-full", "{folder_name}" }
                    p { class: "text-xs text-gray-400 mt-1", "{created}" }
                    p { class: "text-xs text-gray-600 mt-1", "{description}" }
                }
            ))
        })
        .filter_map(Result::ok)
        .collect::<Vec<_>>();


    // --- Início do frontend ---
    rsx! {
        style { // Estilização para o 
            ".selected-filter {{ 
                background: linear-gradient(135deg, oklch(54.6% 0.245 262.881) 0%, oklch(0.4753 0.2363 262.881) 100%) !important; 
                color:oklch(98.5% 0 0) !important; 
                border: none !important; 
                border: 2px solid oklch(54.6% 0.245 262.881) !important;
                box-sizing: border-box;
            }}

             .unselected-filter {{ 
                background: linear-gradient(135deg, #ffffff 0%, #f8f9fa 100%);
                color: oklch(42.4% 0.199 265.638);  
                border: 2px solid oklch(54.6% 0.245 262.881); 
                box-sizing: border-box; 
                hover:bg-blue-200
            }}

            .selected-filter:hover, .unselected-filter:hover {{
                transform: translateY(-1px) scale(1.01);
                box-shadow: 
                    0 8px 25px rgba(0, 0, 0, 0.08),
                    0 4px 10px rgba(0, 0, 0, 0.03);
                transition: all 0.2s cubic-bezier(0.2, 0, 0.1, 0.5);
            }}

            .unselected-filter:hover {{ 
                background: oklch(93.2% 0.032 255.585)
            }}

            .filter-icon {{
                transition: transform 0.3s ease-in-out;
            }}
            
            .filter-icon-active {{
                transform: rotate(180deg);
            }}

            .folders {{
                background: linear-gradient(145deg, 
                    rgba(255, 255, 255, 0.9) 0%,
                    rgba(255, 255, 255, 0.7) 25%,
                    rgba(248, 250, 252, 0.8) 50%,
                    rgba(241, 245, 249, 0.7) 75%,
                    rgba(236, 240, 244, 0.8) 100%);
                border: 1px solid rgba(255, 255, 255, 0.4);
                border-radius: 12px;
                padding: 1.5rem;
                backdrop-filter: blur(10px);
                box-shadow: 
                    0 2px 8px rgba(0, 0, 0, 0.05),
                    0 1px 3px rgba(0, 0, 0, 0.1);
                transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
                cursor: pointer;
            }}

            .folders:hover {{
                transform: translateY(-4px) scale(1.02);
                box-shadow: 0 8px 25px rgba(0, 0, 0, 0.12), 0 4px 10px rgba(0, 0, 0, 0.08);
                border-color: rgba(59, 130, 246, 0.3);
            }}
            
            .box {{
                backdrop-filter: blur(10px);
                box-shadow: 
                    0 2px 8px rgba(0, 0, 0, 0.05),
                    0 1px 3px rgba(0, 0, 0, 0.1);
                transition: all 0.2s cubic-bezier(0.3, 0, 0.1, 1);
                cursor: pointer;
            }}

            .box:hover {{
                transform: translateY(-1.5px) scale(1.009);
                box-shadow: 
                    0 8px 25px rgba(0, 0, 0, 0.12),
                    0 4px 10px rgba(0, 0, 0, 0.08);
                border-color: rgba(59, 130, 246, 0.3);
            }}

            .filter-buttons-container {{
                display: flex;
                flex-wrap: wrap;
                gap: 8px; 
                animation: fadeIn 0.4s ease-out forwards;
            }}
            @keyframes fadeIn {{
                from {{
                    opacity: 0;
                    transform: translateY(-10px);
                }}
                to {{
                    opacity: 1;
                    transform: translateY(0);
                }}
            }}"
        }

        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        body { class: "min-h-screen  text-gray-900 font-sans",
            style:"background: radial-gradient(circle at center, #ffffff, #f0f8ff, #e0f2e1)",
            document::Link {
                href: "https://fonts.googleapis.com/icon?family=Material+Icons",
                rel: "stylesheet"
            }

            header { class: "flex items-center justify-between bg-blue-600 text-white p-4 shadow",
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

            div {
                style: "display: flex; flex-direction: row-reverse; align-items: center; justify-content: space-between; width: 100%; padding: 16px 24px;",
                // Barra de pesquisa
                div {
                    class: "w-4",
                    input {
                        r#type: "text",
                        class: "px-4 py-2 border rounded-lg shadow",
                        style: "width: 304px;",
                        placeholder: "Buscar pasta...",
                        oninput: move |e| search_input.set(e.value().clone()),
                        value: "{search_input}",
                    }
                }

                div {
                    style: "display: flex; align-items: center; gap: 10px;",
                    button {
                        class: "bg-gray-200 hover:bg-gray-300 rounded-full shadow box",
                        style: "display: flex; align-items: center; justify-content: center; padding: 8px;",
                        onclick: move |_| show_filters.toggle(),
                        
                        i {
                            class: if *show_filters.read() {
                                "material-icons filter-icon filter-icon-active"
                            } else {
                                "material-icons filter-icon"
                            },
                            "filter_list"
                        }
                    }

                    if *show_filters.read() {
                        div {
                            class: "filter-buttons-container",
                            
                            button {
                                class: format!("unselected-filter px-4 py-2 text-white transition-colors duration-200 shadow rounded-lg {}",
                                    if *date_order == SortDateOrder::MaisRecente { "selected-filter" } else { "bg-blue-500 hover:bg-blue-200" }
                                ),
                                onclick: move |_| sort_date_order.set(SortDateOrder::MaisRecente),
                                "Mais recente"
                            }
                            button {
                                class: format!("unselected-filter px-4 py-2 text-white transition-colors duration-200 shadow rounded-lg {}",
                                    if *date_order == SortDateOrder::MaisAntigo { "selected-filter" } else { "bg-blue-500 hover:bg-blue-200" }
                                ),
                                onclick: move |_| sort_date_order.set(SortDateOrder::MaisAntigo),
                                "Mais antigo"
                            }
                            button {
                                class: format!("unselected-filter px-4 py-2 text-white transition-colors duration-200 shadow rounded-lg {}",
                                    if *alphabetical_order == SortAlphabeticOrder::AZ { "selected-filter" } else { "bg-blue-500 hover:bg-blue-200" }
                                ),
                                onclick: move |_| sort_alphabetical_order.set(SortAlphabeticOrder::AZ),
                                "A-Z"
                            }
                            button {
                                class: format!("unselected-filter px-4 py-2 text-white transition-colors duration-200 shadow rounded-lg {}",
                                    if *alphabetical_order == SortAlphabeticOrder::ZA { "selected-filter" } else { "bg-blue-500 hover:bg-blue-200" }
                                ),
                                onclick: move |_| sort_alphabetical_order.set(SortAlphabeticOrder::ZA),
                                "Z-A"
                            }
                        }
                    }
                }
            }


            main {
                class: "p-6 grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6",
                style:"background-color:blue, max width: 90%;",
                { folder_cards.into_iter() }

                Link {
                    class: "fixed bottom-6 left-6 hover:bg-blue-600 text-white px-4 py-2 rounded-full shadow-lg box",
                    style:"background: linear-gradient(135deg, #3b82f6 0%, #1d4ed8 100%);",
                    to: Route::ReportView { project_name: "Galpão_Logístico_XPTO".to_string(), building_name: "Galpão_3".to_string() },
                    button {
                        class: "flex items-center gap-2",
                        i { class: "material-icons", "assessment" }
                        span { "Relatório" }
                    }
                }

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

            Link {
                to: Route::NewProject {},
                button {
                    class: "fixed bottom-6 right-6 shadow-lg rounded-full box",
                    style: "padding: 10px 10px 5px 10px; background: linear-gradient(135deg, #fefefe 0%, #f5f5f5 100%); color: oklch(28.2% 0.091 267.935); border: 1px solid #ddd",
                    title: "Nova Pasta",
                    i { class: "material-icons", "add" }
                }
            }
        }
    }
}

#[derive(PartialEq)]
enum SortAlphabeticOrder {
    AZ,
    ZA,
}

#[derive(PartialEq)]
enum SortDateOrder {
    MaisRecente,
    MaisAntigo,
}

struct FileEntry {
    path: PathBuf,
    created: Option<String>,
    description: Option<String>,
}

struct Files {
    base_path: PathBuf,
    current_path: PathBuf,
    path_names: Vec<FileEntry>,
    err: Option<String>,
}

impl Files {
    fn new(initial_path_option: Option<PathBuf>) -> Self {
        let base_path = match initial_path_option {
            Some(path) => path,
            None => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Projects"),
        };

        if let Err(e) = std::fs::create_dir_all(&base_path) {
            eprintln!(
                "Falha ao criar diretório base em Files::new: {} ({:?})",
                base_path.display(),
                e
            );
        }

        let current_path = base_path.clone();

        let mut files_instance = Self {
            base_path,
            current_path,
            path_names: vec![],
            err: None,
        };

        files_instance.reload_path_list();
        files_instance
    }

    fn update_base_path_if_different(&mut self, new_initial_path_option: Option<PathBuf>) {
        let new_base_path = match new_initial_path_option {
            Some(path) => path,
            None => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Projects"),
        };

        if self.base_path != new_base_path {
            self.base_path = new_base_path.clone();
            self.current_path = new_base_path;
            if let Err(e) = std::fs::create_dir_all(&self.base_path) {
                self.err = Some(format!(
                    "Falha ao criar novo diretório base {}: {:?}",
                    self.base_path.display(),
                    e
                ));
            } else {
                self.err = None;
            }
            self.reload_path_list();
        }
    }

    fn reload_path_list(&mut self) {
        let paths = match std::fs::read_dir(&self.current_path) {
            Ok(e) => e,
            Err(err) => {
                self.err = Some(format!("Erro ao ler diretório: {err:?}"));
                return;
            }
        };

        let collected = paths.collect::<Vec<_>>();
        self.clear_err();
        self.path_names.clear();

        for entry in collected {
            if let Ok(entry) = entry {
                let path = entry.path();
                let created = entry
                    .metadata()
                    .and_then(|m| m.created())
                    .ok()
                    .and_then(|time| {
                        let datetime: DateTime<Local> = time.into();
                        Some(datetime.to_rfc3339())
                    });
                let description = if path.is_dir() {
                    let desc_path = path.join("description.txt");
                    std::fs::read_to_string(&desc_path).ok()
                } else {
                    None
                };

                self.path_names.push(FileEntry {
                    path,
                    created,
                    description,
                });
            }
        }
    }

    fn go_up(&mut self) {
        if self.current_path != self.base_path {
            if let Some(parent) = self.current_path.parent() {
                if parent.starts_with(&self.base_path) {
                    self.current_path = parent.to_path_buf();
                    self.reload_path_list();
                }
            }
        }
    }

    fn current(&self) -> String {
        self.current_path.display().to_string()
    }

    fn clear_err(&mut self) {
        self.err = None;
    }
}