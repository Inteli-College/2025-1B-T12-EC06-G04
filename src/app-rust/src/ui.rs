use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use rfd::AsyncFileDialog;
use crate::image_processor::{process_folder, ProcessingStats};
use crate::Route as AppRoute;
use std::path::PathBuf;
use std::rc::Rc;
use futures_util::StreamExt;
use std::path::Path;
use chrono::{DateTime, Local};
use crate::manual_processor::{ManualProcessor, ManualProcessorProps};
use crate::create_project::PROJECT_NAME;
use dioxus::prelude::Readable;

#[component]
pub fn Home() -> Element {
    let mut folder_path = use_signal(|| None::<String>);
    let mut status = use_signal(String::new);
    let mut threshold = use_signal(|| 200.0_f64);
    let mut stats = use_signal(|| None::<ProcessingStats>);
    let mut is_processing = use_signal(|| false);
    let mut is_selecting_folder = use_signal(|| false);

    let mut processed_folder_signal = use_context::<Signal<Option<PathBuf>>>();

    let project_name_available = use_memo(move || {
        PROJECT_NAME.try_read().map_or(false, |guard| guard.is_some())
    });

    // Handle for the folders popup
    let handle = use_coroutine(move |mut rx: UnboundedReceiver<Option<PathBuf>>| async move {
        use futures_util::StreamExt;
        while let Some(path) = rx.next().await {
            processed_folder_signal.set(path);
        }
    });

    let open_folders_window = move |_evt: MouseEvent| {
        let tx = handle.tx();
        dioxus::desktop::window().new_window(
            VirtualDom::new_with_props(folders_popup, Rc::new(move |path| tx.unbounded_send(path).unwrap())),
            Default::default(),
        );
    };

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

        div { class: "min-h-screen bg-gray-100 text-gray-900 font-sans",
            document::Link {
                href: "https://fonts.googleapis.com/icon?family=Material+Icons",
                rel: "stylesheet"
            }

            div { class: "container mx-auto px-4 py-8 max-w-4xl",
                h1 { class: "text-3xl font-bold text-center mb-8 text-gray-800", 
                    "Organizador de Fotos por Localização" 
                }
                div { class: "bg-white rounded-lg shadow-md p-6 mb-6",
                    div { class: "flex gap-4 mb-6",
                input {
                            class: "flex-1 px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
                    r#type: "text",
                    value: folder_path().unwrap_or_default(),
                    readonly: true,
                    placeholder: "Selecione uma pasta de imagens para processar..."
                }
                button {
                            class: "px-6 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2",
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
                            i { class: "material-icons", "folder" }
                    if is_selecting_folder() { "Selecionando..." } else { "Selecionar Pasta de Imagens" }
                }
            }
                    div { class: "mb-6",
                        label { class: "block text-gray-700 mb-2", 
                            "Distância máxima entre imagens do mesmo prédio (metros):" 
                        }
                input {
                            class: "w-full px-4 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500",
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

                    if !project_name_available() {
                        p { class: "text-center text-red-500 mb-4 py-2 px-4 border border-red-300 bg-red-50 rounded-md",
                            "Para habilitar o processamento, por favor, primeiro crie um projeto na tela 'Criar Novo Projeto'."
                        }
                    }

                    div { class: "flex gap-4",
            button {
                            class: "flex-1 px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2",
                disabled: is_processing() || folder_path().is_none() || !project_name_available(),
                onclick: move |_| {
                                if let Some(path_str) = folder_path() {
                        if !project_name_available() {
                            status.set("Erro: Crie um projeto antes de processar.".to_string());
                            return;
                        }
                        is_processing.set(true);
                        status.set("Processando imagens...".to_string());
                        
                                    let path_clone_for_processing = path_str.clone();
                        let threshold_value = threshold();
                        
                        spawn(async move {
                                        let result = process_folder(&path_clone_for_processing, threshold_value);
                            
                            match result {
                                            Ok(result_data) => {
                                                stats.set(Some(result_data.clone()));
                                                if result_data.images_with_gps > 0 {
                                        status.set(format!("Processamento concluído! {} imagens com GPS organizadas em {} prédios.", 
                                                        result_data.images_with_gps, result_data.predio_groups));
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
                    } else {
                         status.set("Erro: Selecione uma pasta de imagens primeiro.".to_string());
                    }
                },
                            i { class: "material-icons", "sync" }
                            if is_processing() { "Processando..." } else { "Processar Automaticamente" }
                        }
                        button {
                            class: "flex-1 px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2",
                            disabled: is_processing() || !project_name_available(),
                            onclick: move |_| {
                                if project_name_available() {
                                    if let Ok(guard) = PROJECT_NAME.try_read() {
                                        if let Some(name) = &*guard {
                                            dioxus::desktop::window().new_window(
                                                VirtualDom::new_with_props(
                                                    ManualProcessor,
                                                    ManualProcessorProps { project_name: name.clone() }
                                                ),
                                                Default::default(),
                                            );
                                        } else {
                                            status.set("Erro: Nome do projeto é None, não deveria acontecer aqui.".to_string());
                                        }
                                    } else {
                                        status.set("Erro: Falha ao ler o nome do projeto global.".to_string());
                                    }
                                } else {
                                    status.set("Erro: Crie um projeto antes de processar manualmente.".to_string());
                                }
                            },
                            i { class: "material-icons", "folder_open" }
                            "Processar Manualmente"
                        }
                    }

            if is_processing() {
                        div { class: "text-center py-4 text-gray-600",
                            "Carregando... Por favor, aguarde."
                        }
                    }

                    if let Some(stats_data) = stats.read().as_ref() {
                        div { class: "bg-white rounded-lg shadow-md p-6 mb-6",
                            h2 { class: "text-2xl font-semibold mb-4 text-gray-800", "Estatísticas" }
                            div { class: "grid grid-cols-2 gap-4",
                                p { class: "text-gray-700", "Total de imagens: {stats_data.total_images}" }
                                p { class: "text-gray-700", "Imagens com GPS: {stats_data.images_with_gps}" }
                                p { class: "text-gray-700", "Imagens sem GPS: {stats_data.images_without_gps}" }
                                p { class: "text-gray-700", "Imagens com direção: {stats_data.images_with_direction}" }
                                p { class: "text-gray-700", "Prédios identificados: {stats_data.predio_groups}" }
                            }
                            if !stats_data.errors.is_empty() {
                                div { class: "mt-4",
                                    h3 { class: "text-xl font-semibold mb-2 text-red-600", "Erros:" }
                                    ul { class: "list-disc list-inside text-red-500",
                                        {stats_data.errors.iter().map(|error| rsx! {
                                    li { "{error}" }
                                })}
                                    }
                                }
                            }
                        }

                        if !is_processing() && stats_data.images_with_gps > 0 {
                            div { class: "text-center",
                                button { 
                                    class: "px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 flex items-center gap-2",
                                    onclick: open_folders_window,
                                    i { class: "material-icons", "folder" }
                                    "Visualizar Pastas Organizadas"
                                }
                            }
                        }
                    }

                    if !status.read().is_empty() {
                         p { class: "text-center mt-4", "{status}" }
                    }
                }
            }
        }
    }
}

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
        document::Stylesheet { href: asset!("/assets/tailwind.css") }

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
    
            button {
                class: "fixed bottom-6 right-6 bg-purple-100 hover:bg-purple-200 text-purple-600 shadow-lg p-4 rounded-full",
                title: "Nova Pasta",
                onclick: move |_| show_new_folder_input.set(true),
                i { class: "material-icons", "edit" }
            }

            button {
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

// Helper function from folders.rs
fn display_from_projects(path: &Path) -> Option<PathBuf> {
    for ancestor in path.ancestors() {
        if ancestor.file_name().map_or(false, |name| name == "projects") {
            return path.strip_prefix(ancestor).ok().map(|p| p.to_path_buf());
        }
    }
    None
}

// FileEntry and Files structs from folders.rs
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
                        Some(datetime.format("%d/%m/%Y %H:%M").to_string())
                    });

                self.path_names.push(FileEntry { path, created });
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