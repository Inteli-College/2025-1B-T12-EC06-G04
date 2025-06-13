use std::path::{Path, PathBuf};
use chrono::{DateTime, Local};

pub fn display_from_projects(path: &Path) -> Option<PathBuf> {
    for ancestor in path.ancestors() {
        if ancestor.file_name().map_or(false, |name| name == "Projects") {
            return path.strip_prefix(ancestor).ok().map(|p| p.to_path_buf());
        }
    }
    None
}

pub struct FileEntry {
    pub path: PathBuf,
    pub created: Option<String>,
    pub description: Option<String>,
}

pub struct Files {
    pub base_path: PathBuf,
    pub current_path: PathBuf,
    pub path_names: Vec<FileEntry>,
    pub err: Option<String>,
}

impl Files {
    pub fn new(initial_path_option: Option<PathBuf>) -> Self {
        let base_path = match initial_path_option {
            Some(path) => path,
            None => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Projects"),
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

    pub fn update_base_path_if_different(&mut self, new_initial_path_option: Option<PathBuf>) {
        let new_base_path = match new_initial_path_option {
            Some(path) => path,
            None => PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Projects"),
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

    pub fn reload_path_list(&mut self) {
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
            
                let description = Some("Descrição qualquer".to_string());
                self.path_names.push(FileEntry { path, created, description });
            }
        }
    }

    pub fn go_up(&mut self) {
        if self.current_path != self.base_path {
            if let Some(parent) = self.current_path.parent() {
                if parent.starts_with(&self.base_path) {
                    self.current_path = parent.to_path_buf();
                    self.reload_path_list();
                }
            }
        }
    }

    pub fn enter_dir(&mut self, dir_id: usize) {
        if let Some(entry) = self.path_names.get(dir_id) {
            let path = &entry.path;
            if path.is_dir() && path.starts_with(&self.base_path) {
                self.current_path = path.clone();
                self.reload_path_list();
            }
        }
    }

    pub fn current(&self) -> String {
        self.current_path.display().to_string()
    }

    pub fn clear_err(&mut self) {
        self.err = None;
    }
}
