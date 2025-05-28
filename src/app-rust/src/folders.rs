use dioxus::prelude::*;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};
use dioxus_router::prelude::Link;
use crate::Route;


fn display_from_projects(path: &Path) -> Option<PathBuf> {
    for ancestor in path.ancestors() {
        if ancestor.file_name().map_or(false, |name| name == "projects") {
            return path.strip_prefix(ancestor).ok().map(|p| p.to_path_buf());
        }
    }
    None
}

#[allow(non_snake_case)]
pub fn Folders() -> Element {
    let processed_folder_signal = use_context::<Signal<Option<PathBuf>>>();
    let initial_path_from_state = processed_folder_signal.read().clone();

    let mut files = use_signal(|| Files::new(initial_path_from_state));

    // variáveis para o filtros de ordenação alfabética e por data
    let mut sort_alphabetical_order = use_signal(|| SortAlphabeticOrder::AZ);
    let mut sort_date_order = use_signal(|| SortDateOrder::MaisRecente);

    let alphabetical_order = sort_alphabetical_order.read();
    let date_order = sort_date_order.read();

    let binding = files.read();
    let mut entries: Vec<_> = binding.path_names.iter().collect();

    entries.sort_by(|a, b| {
        let date_a = a.created.as_ref().and_then(|s| DateTime::parse_from_rfc3339(s).ok());
        let date_b = b.created.as_ref().and_then(|s| DateTime::parse_from_rfc3339(s).ok());
    
        // Aplica o filtro de data
        let date_cmp = match *date_order {
            SortDateOrder::MaisRecente => date_b.cmp(&date_a),
            SortDateOrder::MaisAntigo => date_a.cmp(&date_b),
        };
    
        // Se as datas forem iguais ou inexistentes, aplica o filtro alfabético
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

    let mut new_folder_name = use_signal(|| String::new());
    let mut new_folder_description = use_signal(|| String::new());
    let mut show_new_folder_input = use_signal(|| false);

    // pesquisa do usuário
    let mut search_input = use_signal(|| String::new());


    let folder_cards = entries.iter().enumerate()
    .filter_map(|(dir_id, entry)| {
        let path = &entry.path;
        let folder_name = path.file_name()?.to_string_lossy();
        let path_display = display_from_projects(path)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| path.display().to_string());
        let created = entry.created.clone().unwrap_or_default();
        let description = entry.description.clone().unwrap_or_else(|| "Sem descrição".to_string());

        // Aplicando a pesquisa do usuário
        let search = search_input.read().to_lowercase();
        if !search.is_empty() && !folder_name.to_lowercase().contains(&search) {
            return None;
        }

        Some(rsx!(
            div {
                class: "flex flex-col items-center text-center bg-white shadow rounded-lg p-4 cursor-pointer hover:shadow-lg hover:bg-blue-50 transition duration-300 ease-in-out",
                key: "{path_display}",
                onclick: move |_| files.write().enter_dir(dir_id),

                i { class: "material-icons text-6xl text-blue-500 mb-2", "folder" }
                h2 { class: "mt-2 font-semibold text-base text-gray-900 truncate max-w-full", "{folder_name}" }
                p { class: "text-xs text-gray-400 mt-1", "{created}" }
                p { class: "text-xs text-gray-600 mt-1", "{description}" }
            }
        ))
    })
    .filter_map(Result::ok)
    .collect::<Vec<_>>();

    // aqui começa o front
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") } // puxa as classes do tailwind

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

            // barra de pesquisa
            div {
                class: "w-full p-4",
                input {
                    r#type: "text",
                    class: "w-full p-2 border rounded",
                    placeholder: "Buscar pasta...",
                    oninput: move |e| {
                        search_input.set(e.value().clone());
                    },
                    value: "{search_input}",
                }
            }

            // botões de filtro de ordenação alfabética
            div {
                class: "flex gap-2",
                button {
                    class: "px-4 py-2 bg-blue-500 text-white hover:bg-blue-600 transition-colors duration-200 shadow-md rounded-full",
                    onclick: move |_| sort_date_order.set(SortDateOrder::MaisRecente),
                    "Mais recente"
                }
                button {
                    class: "px-4 py-2 bg-blue-500 text-white hover:bg-blue-600 transition-colors duration-200 shadow-md rounded-full",
                    onclick: move |_| sort_date_order.set(SortDateOrder::MaisAntigo),
                    "Mais antigo"
                }
            }

            // botões de filtro de ordenação por data
            div {
                class: "flex gap-2",
                button {
                    class: "px-4 py-2 bg-blue-500 text-white hover:bg-blue-600 transition-colors duration-200 shadow-md rounded-full",
                    onclick: move |_| sort_alphabetical_order.set(SortAlphabeticOrder::AZ),
                    "A-Z"
                }
                button {
                    class: "px-4 py-2 bg-blue-500 text-white hover:bg-blue-600 transition-colors duration-200 shadow-md rounded-full",
                    onclick: move |_| sort_alphabetical_order.set(SortAlphabeticOrder::ZA),
                    "Z-A"
                }
            }
            
        

            main {
                class: "p-6 grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6 max-w-7xl mx-auto",
                { folder_cards.into_iter() }

                Link {
                    to: Route::ReportView {},  // ajuste para o nome da rota correta
                    class: "fixed bottom-6 left-6 bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-full shadow-lg",
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

            // se o botão para criar pasta for clicado, aciona o pop-up abaixo
            if *show_new_folder_input.read() {
                div {
                    class: "fixed bottom-24 right-6 bg-white border shadow-lg rounded-lg p-4 flex flex-col gap-2 w-80 max-w-full z-50",
    
                    h2 { class: "text-lg font-semibold text-gray-800", "Novo Projeto" }
    
                    input {
                        class: "border rounded px-3 py-2 w-full",
                        r#type: "text",
                        placeholder: "Nome da nova pasta",
                        value: "{new_folder_name.read()}",
                        oninput: move |e| new_folder_name.set(e.value()) // callback que acessa e atualiza o valor do input sempre que o usuário escrever algo
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
                            class: "text-gray-500 text-sm hover:underline",
                            onclick: move |_| {
                                show_new_folder_input.set(false);
                                new_folder_name.set(String::new());
                                new_folder_description.set(String::new());
                            },
                            "Cancelar"
                        }
                        button {
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
    
            // Botão para criar o projeto
            Link {
                to: Route::Home {},
                button {
                    class: "fixed bottom-6 right-6 bg-purple-100 hover:bg-purple-200 text-purple-600 shadow-lg p-4 rounded-full",
                    title: "Nova Pasta",
                    onclick: move |_| show_new_folder_input.set(true),
                    i { class: "material-icons", "edit" }
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
            None => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("projects"),
        };

        if let Err(e) = std::fs::create_dir_all(&base_path) {
            eprintln!("Falha ao criar diretório base em Files::new: {} ({:?})", base_path.display(), e);
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
            None => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("projects"),
        };

        if self.base_path != new_base_path {
            self.base_path = new_base_path.clone();
            self.current_path = new_base_path;
            if let Err(e) = std::fs::create_dir_all(&self.base_path) {
                self.err = Some(format!("Falha ao criar novo diretório base {}: {:?}", self.base_path.display(), e));
            } else {
                self.err = None;
            }
            self.reload_path_list();
        }
    }

    pub fn create_folder_with_description(&mut self, name: String, description: String) {
        let path = self.current_path.join(&name);
        if let Err(err) = std::fs::create_dir_all(&path) {
            self.err = Some(format!("Erro ao criar pasta: {err}"));
            return;
        }

        let desc_path = path.join("description.txt");
        if let Err(err) = std::fs::write(&desc_path, description) {
            self.err = Some(format!("Erro ao salvar descrição: {err}"));
            return;
        }

        self.reload_path_list();
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
                let created = entry.metadata()
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

                self.path_names.push(FileEntry { path, created, description });
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

    fn enter_dir(&mut self, dir_id: usize) {
        if let Some(entry) = self.path_names.get(dir_id) {
            let path = &entry.path;
            if path.is_dir() && path.starts_with(&self.base_path) {
                self.current_path = path.clone();
                self.reload_path_list();
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
