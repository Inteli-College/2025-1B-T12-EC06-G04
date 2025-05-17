use dioxus::prelude::*;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};

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
    let mut files = use_signal(Files::new);
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
    
            // Botão para criar as pastas
            button {
                class: "fixed bottom-6 right-6 bg-purple-100 hover:bg-purple-200 text-purple-600 shadow-lg p-4 rounded-full",
                title: "Nova Pasta",
                onclick: move |_| show_new_folder_input.set(true),
                i { class: "material-icons", "edit" }
            }

        }
    }
}



struct FileEntry {
    path: PathBuf,
    created: Option<String>,
}

struct Files {
    base_path: PathBuf,
    current_path: PathBuf,
    path_names: Vec<FileEntry>,
    err: Option<String>,
}

impl Files {
    fn new() -> Self {
        let base_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("projects");
        std::fs::create_dir_all(&base_path).ok();

        let current_path = base_path.clone();

        let mut files = Self {
            base_path,
            current_path,
            path_names: vec![],
            err: None,
        };

        files.reload_path_list();
        files
    }

    // precisamos refatorar, porque essa função cria uma pasta com um arquivo para a descrição dela
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
                        Some(datetime.format("%d/%m/%Y %H:%M").to_string())
                    });

                self.path_names.push(FileEntry { path, created });
            }
        }
    }

    // criei essa função para voltar as pastas
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

    // criei essa aqui para entrar nas pastas
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
