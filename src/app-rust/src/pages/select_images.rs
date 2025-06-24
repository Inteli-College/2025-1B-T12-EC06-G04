use dioxus::prelude::*;
use rfd::AsyncFileDialog;
use crate::utils::image_processor::{process_folder, ProcessingStats};
use std::path::PathBuf;
use std::rc::Rc;
use std::path::Path;
use chrono::{DateTime, Local};
use crate::manual_processor::ManualProcessor;
use dioxus_router::prelude::use_navigator;
use crate::Route as AppRoute;
use crate::manual_processor::ManualProcessorProps;
use crate::pages::create_project::PROJECT_NAME;
use dioxus::prelude::Readable;
use tokio;

#[component]
pub fn SelectImages() -> Element {
    let mut folder_path = use_signal(|| None::<String>);
    let mut status = use_signal(String::new);
    let mut threshold = use_signal(|| 200.0_f64);
    let mut stats = use_signal(|| None::<ProcessingStats>);
    let mut is_processing = use_signal(|| false);
    let mut is_selecting_folder = use_signal(|| false);
    let navigator = use_navigator();

    let mut processed_folder_signal = use_context::<Signal<Option<PathBuf>>>();

    rsx! {
        // Replaced tailwind.css with styles.css
        document::Stylesheet { href: asset!("/assets/styles.css") }
        document::Link {
            href: "https://fonts.googleapis.com/icon?family=Material+Icons",
            rel: "stylesheet"
        }

        div { class: "container",
            h1 { class: "text-center", style: "font-size: 1.8rem; margin-bottom: 2rem;",
                "Organizador de Fotos por Localização"
            }
            div { class: "card",
                div { class: "input-group mb-6",
                    input {
                        class: "form-input",
                        r#type: "text",
                        value: folder_path().unwrap_or_default(),
                        readonly: true,
                        placeholder: "Selecione uma pasta..."
                    }
                    button {
                        class: "btn btn-primary",
                        disabled: is_selecting_folder(),
                        onclick: move |_| {
                            is_selecting_folder.set(true);
                            spawn(async move {
                                if let Some(file_handle) = AsyncFileDialog::new().pick_folder().await {
                                    folder_path.set(Some(file_handle.path().display().to_string()));
                                    processed_folder_signal.set(None);
                                }
                                is_selecting_folder.set(false);
                            });
                        },
                        i { class: "material-icons", "folder" }
                        if is_selecting_folder() { "Selecionando..." } else { "Selecionar Pasta" }
                    }
                }
                div { class: "form-group mb-6",
                    label {
                        "Distância máxima entre imagens do mesmo prédio (metros):"
                    }
                    input {
                        class: "form-input",
                        style: "max-width: 120px;", // Added style for smaller width
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
                div { class: "d-flex gap-4",
                    button {
                        class: "btn btn-primary",
                        style: "flex: 1;",
                        disabled: is_processing() || folder_path().is_none(),
                        onclick: move |_| {
                            if let Some(path_str) = folder_path() {
                                is_processing.set(true);
                                status.set("Processando imagens...".to_string());

                                let path_clone_for_processing = path_str.clone();
                                let threshold_value = threshold();
                                let path_clone_for_state = path_str.clone();

                                spawn(async move {
                                    let result = process_folder(&path_clone_for_processing, threshold_value);

                                    match result {
                                        Ok(result_data) => {
                                            stats.set(Some(result_data.clone()));
                                            if result_data.images_with_gps > 0 {
                                                status.set(format!("Processamento concluído! {} imagens com GPS organizadas em {} prédios. Redirecionando...",
                                                    result_data.images_with_gps, result_data.predio_groups));
                                                processed_folder_signal.set(Some(PathBuf::from(path_clone_for_state)));
                                                tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
                                                navigator.push(AppRoute::ValidationScreen {});
                                            } else {
                                                status.set("Nenhuma imagem com GPS foi encontrada.".to_string());
                                                processed_folder_signal.set(None);
                                            }
                                        }
                                        Err(e) => {
                                            status.set(format!("Erro: {}", e));
                                            processed_folder_signal.set(None);
                                        }
                                    }
                                    is_processing.set(false);
                                });
                            }
                        },
                        i { class: "material-icons", "sync" }
                        if is_processing() { "Processando..." } else { "Processar automaticamente" }
                    }
                    button {
                        class: "btn btn-secondary",
                        style: "flex: 1;",
                        disabled: is_processing(),
                        onclick: move |_| {
                            match PROJECT_NAME.try_read() {
                                Ok(guard) => match &*guard {
                                    Some(name) => {
                                        if !name.is_empty() {
                                            dioxus::desktop::window().new_window(
                                                VirtualDom::new_with_props(
                                                    ManualProcessor,
                                                    ManualProcessorProps { project_name: name.clone() }
                                                ),
                                                Default::default(),
                                            );
                                        } else {
                                            status.set("Erro: Nome do projeto está vazio.".to_string());
                                        }
                                    }
                                    None => {
                                        status.set("Erro: Nenhum projeto selecionado.".to_string());
                                    }
                                },
                                Err(_) => {
                                    status.set("Erro ao ler nome do projeto.".to_string());
                                }
                            }
                        },
                        "Abrir Processador Manual"
                    }
                }

                if is_processing() {
                    div { class: "text-center py-4 text-gray-600",
                        "Carregando... Por favor, aguarde."
                    }
                }
            }

            if let Some(stats_data) = stats.read().as_ref() {
                div { class: "card", style: "margin-top: 2rem;",
                    h2 { style: "font-family: 'Poppins', sans-serif; font-weight: 600; font-size: 1.25rem; margin-bottom: 1rem;", "Estatísticas" }
                    div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;",
                        p { "Total de imagens: {stats_data.total_images}" }
                        p { "Imagens com GPS: {stats_data.images_with_gps}" }
                        p { "Imagens sem GPS: {stats_data.images_without_gps}" }
                        p { "Imagens com direção: {stats_data.images_with_direction}" }
                        p { "Prédios identificados: {stats_data.predio_groups}" }
                    }
                    if !stats_data.errors.is_empty() {
                        div { class: "mt-4",
                            h3 { style: "font-family: 'Poppins', sans-serif; font-weight: 600; color: var(--status-red); margin-top: 1rem; margin-bottom: 0.5rem;", "Erros:" }
                            ul { style: "list-style-type: disc; list-style-position: inside; color: var(--status-red);",
                                {stats_data.errors.iter().map(|error| rsx! {
                                    li { "{error}" }
                                })}
                            }
                        }
                    }

                    if !is_processing() && stats_data.images_with_gps > 0 {
                         div { class: "text-center", style: "margin-top: 1.5rem;",
                            button {
                                class: "btn",
                                style: "background-color: var(--status-green); color: var(--text-light);",
                                onclick: move |_| {
                                    match PROJECT_NAME.try_read() {
                                        Ok(guard) => match &*guard {
                                            Some(project_name) => {
                                                let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                                                let detection_file = base_dir.join("Projects").join(project_name).join("detection_results.json");
                                                if detection_file.exists() {
                                                    navigator.push(AppRoute::ValidationScreen {});
                                                } else {
                                                    status.set("Erro: Resultados não encontrados. Execute o processamento de IA.".to_string());
                                                }
                                            }
                                            None => status.set("Erro: Nenhum projeto selecionado.".to_string()),
                                        },
                                        Err(_) => status.set("Erro ao ler nome do projeto.".to_string()),
                                    }
                                },
                                i { class: "material-icons", "verified" }
                                "Validar Fissuras Detectadas"
                            }
                        }
                    }
                }
            }


            if !status.read().is_empty() {
                div {
                    class: "status-message error",
                    style: "position: fixed; bottom: 1.5rem; right: 1.5rem; z-index: 100;",
                    onclick: move |_| status.set(String::new()),
                    "{status.read().clone()}"
                }
            }
        }
    }
}

// NOTE: The `folders_popup` component was also updated to use the new style classes.
fn folders_popup(send: Rc<dyn Fn(Option<PathBuf>)>) -> Element {
    let processed_folder_signal = use_context::<Signal<Option<PathBuf>>>();
    let initial_path_from_state = processed_folder_signal.read().clone();
    let mut files = use_signal(|| Files::new(initial_path_from_state));

    use_effect(move || {
        let new_path = processed_folder_signal.read().clone();
        files.write().update_base_path_if_different(new_path);
    });

    let mut new_folder_name = use_signal(String::new);
    let mut new_folder_description = use_signal(String::new);
    // Corrected line:
    let mut show_new_folder_input = use_signal(|| false);

    let file_cards = files.read().path_names.iter().enumerate()
    .filter_map(|(dir_id, entry)| {
        let path = &entry.path;
        let path_end = path.file_name()?.to_string_lossy();
        let created = entry.created.clone().unwrap_or_default();

        Some(rsx!(
            div {
                class: "folder-card",
                key: "{path.display()}",
                onclick: move |_| files.write().enter_dir(dir_id),

                i { class: "material-icons", "folder" }
                h2 { "{path_end}" }
                p { class: "date", "{created}" }
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

        header { class: "page-header",
            div { style: "display: flex; align-items: center; gap: 1rem;",
                i { class: "material-icons", "menu" }
                h1 { "{files.read().current()}" }
            }
            i {
                class: "material-icons icon-button",
                onclick: move |_| files.write().go_up(),
                "logout" // This icon probably means "go up" or "exit"
            }
        }

        main {
            class: "folder-grid",
            style: "padding: 1.5rem; max-width: 1200px; margin: 0 auto;",
            { file_cards.into_iter() }
        }

        if let Some(err) = files.read().err.as_ref() {
            div { class: "status-message error", style: "margin: 1.5rem;",
                "{err}"
                button {
                    style: "margin-left: 1rem; cursor: pointer; font-weight: bold;",
                    onclick: move |_| files.write().clear_err(),
                    "x"
                }
            }
        }

        if *show_new_folder_input.read() {
            div {
                class: "card",
                style: "position: fixed; bottom: 5.5rem; right: 1.5rem; z-index: 1001; width: 320px;",

                h2 { style: "font-family: 'Poppins', sans-serif; font-size: 1.2rem; font-weight: 600;", "Novo Projeto" }

                div { class: "form-group",
                    label { "Nome da nova pasta" }
                    input {
                        class: "form-input",
                        r#type: "text",
                        placeholder: "Nome...",
                        value: "{new_folder_name.read()}",
                        oninput: move |e| new_folder_name.set(e.value())
                    }
                }
                div { class: "form-group",
                    label { "Descrição do projeto" }
                    textarea {
                        class: "form-textarea",
                        rows: "3",
                        placeholder: "Descrição...",
                        value: "{new_folder_description.read()}",
                        oninput: move |e| new_folder_description.set(e.value())
                    }
                }

                div { style: "display: flex; justify-content: flex-end; gap: 0.5rem; margin-top: 1rem;",
                    button {
                        class: "btn-secondary", // Assuming a secondary button style exists
                        style: "border: none;",
                        onclick: move |_| {
                            show_new_folder_input.set(false);
                            new_folder_name.set(String::new());
                            new_folder_description.set(String::new());
                        },
                        "Cancelar"
                    }
                    button {
                        class: "btn btn-primary",
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
                        "Criar"
                    }
                }
            }
        }


        button {
            class: "fab",
            title: "Nova Pasta",
            onclick: move |_| show_new_folder_input.set(true),
            i { class: "material-icons", "create_new_folder" }
        }

        button {
            class: "btn btn-primary",
            style: "position: fixed; bottom: 1.5rem; left: 1.5rem;",
            onclick: move |_| {
                send(Some(files.read().current_path.clone()));
                dioxus::desktop::window().close();
            },
            "Selecionar Pasta Atual"
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