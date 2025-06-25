use dioxus::prelude::*;
use std::collections::HashMap;
use rfd::AsyncFileDialog;
use std::path::{Path, PathBuf};
use std::fs;
use dioxus::prelude::Readable;
use std::process::{Command, Stdio};
use serde::Deserialize;
use dioxus_router::prelude::Link;
use dioxus_router::prelude::use_navigator;
use crate::Route; 
use tokio::time;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use crate::pages::create_project::ProjectStatus;
use crate::utils::file_manager::update_project_status;


#[derive(Props, Clone, PartialEq)]
pub struct ManualProcessorProps {
    pub project_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct FissuraData {
    pub name: String,
    pub confidence: f64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ImageAnalysisResult {
    pub path: String,
    pub fissura: Vec<FissuraData>,
}

#[derive(Clone, PartialEq)]
pub struct Building {
    pub name: String,
    pub facades: HashMap<String, Vec<ImageData>>,
}

#[derive(Clone, PartialEq)]
pub struct ImageData {
    pub path: String,
    pub name: String,
    pub preview_url: Option<String>,
}

fn create_data_uri(path: &Path) -> Option<String> {
    let mime_type = match path.extension().and_then(std::ffi::OsStr::to_str) {
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("bmp") => "image/bmp",
        Some("webp") => "image/webp",
        Some("tiff") | Some("tif") => "image/tiff",
        _ => "application/octet-stream",
    };

    let image_bytes = fs::read(path).ok()?;
    let base64_str = STANDARD.encode(&image_bytes);
    Some(format!("data:{};base64,{}", mime_type, base64_str))
}


#[component]
pub fn ManualProcessor(props: ManualProcessorProps) -> Element {
    let mut num_buildings = use_signal(|| 1);
    let mut buildings = use_signal(Vec::<Building>::new);
    let mut building_names = use_signal(|| vec!["Prédio 1".to_string()]);
    let mut facade_counts = use_signal(|| vec![1]);
    let mut facade_names = use_signal(|| vec![HashMap::new()]);
    let is_processing = use_signal(|| false);
    let mut status = use_signal(String::new);
    let navigator = use_navigator();

    let project_name_for_closure = props.project_name.clone();
    let project_name_for_detection_check = props.project_name.clone();

    use_effect(move || {
        let count = num_buildings();
        if count > 20 {
            num_buildings.set(20);
            return;
        }
        if count == 0 {
            num_buildings.set(1);
            return;
        }

        let current_buildings = buildings.read().clone();
        let current_building_names = building_names.read().clone();
        let current_facade_counts = facade_counts.read().clone();
        let current_facade_names = facade_names.read().clone();

        if current_buildings.len() == count {
            return;
        }

        let mut new_buildings_vec = Vec::with_capacity(count);
        let mut new_building_names_vec = Vec::with_capacity(count);
        let mut new_facade_counts_vec = Vec::with_capacity(count);
        let mut new_facade_names_vec = Vec::with_capacity(count);

        for i in 0..count {
            let name = current_building_names.get(i)
                .cloned()
                .unwrap_or_else(|| format!("Prédio {}", i + 1));
            
            let facades_data = current_buildings.get(i)
                .map(|b| b.facades.clone())
                .unwrap_or_else(HashMap::new);

            new_buildings_vec.push(Building {
                name: name.clone(),
                facades: facades_data,
            });
            new_building_names_vec.push(name);
            new_facade_counts_vec.push(current_facade_counts.get(i).cloned().unwrap_or(1));
            new_facade_names_vec.push(current_facade_names.get(i).cloned().unwrap_or_else(HashMap::new));
        }

        buildings.set(new_buildings_vec);
        building_names.set(new_building_names_vec);
        facade_counts.set(new_facade_counts_vec);
        facade_names.set(new_facade_names_vec);
    });

    let organize_folders = move |_: Event<MouseData>| {
        let current_buildings = buildings.read().clone();
        let mut is_processing_writer = is_processing;
        let mut status_writer = status;
        let project_name_clone = project_name_for_closure.clone();

        spawn(async move {
            is_processing_writer.set(true);
            status_writer.set("Organizando pastas...".to_string());
            
            let project_name_for_path = project_name_clone.clone();

            let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            let project_images_path = base_dir.join("Projects").join(&project_name_for_path).join("images");
            
            let mut folder_organization_successful = true;
            for building_detail in current_buildings.iter() {
                let building_folder_name = &building_detail.name;
                let building_path = project_images_path.join(building_folder_name);
                
                if let Err(e) = fs::create_dir_all(&building_path) {
                    status_writer.set(format!("Erro ao criar pasta do prédio {}: {}", building_folder_name, e));
                    is_processing_writer.set(false);
                    folder_organization_successful = false;
                    return;
                }
                
                for (facade_folder_name, images) in &building_detail.facades {
                    let facade_path = building_path.join(facade_folder_name);
                    
                    if let Err(e) = fs::create_dir_all(&facade_path) {
                        status_writer.set(format!("Erro ao criar pasta da fachada {}: {}", facade_folder_name, e));
                        is_processing_writer.set(false);
                        folder_organization_successful = false;
                        return;
                    }
                    
                    for image_data in images {
                        let source_path = PathBuf::from(&image_data.path);
                        let target_path = facade_path.join(&image_data.name);
                        
                        if !source_path.exists() {
                            status_writer.set(format!("Erro: Imagem de origem não encontrada {} para {}", image_data.path, facade_folder_name));
                            continue;
                        }

                        if let Err(e) = fs::copy(&source_path, &target_path) {
                            status_writer.set(format!("Erro ao copiar imagem {} para {}: {}", image_data.name, facade_folder_name, e));
                        }
                    }
                }
            }
            
            if folder_organization_successful {
                status_writer.set("Pastas organizadas com sucesso! Iniciando análise de imagens...".to_string());

                match run_yolo_script_and_parse_results(&project_name_clone, status_writer, &base_dir).await {
                    Ok(analysis_results) => {
                        if let Err(e) = update_project_status(&project_name_clone, ProjectStatus::ProcessingComplete) {
                             status_writer.set(format!("Análise concluída, mas falha ao atualizar status do projeto: {}", e));
                        } else {
                            status_writer.set(format!(
                                "Análise de imagens concluída. {} resultados. Redirecionando para validação...",
                                analysis_results.len()
                            ));
                        }
                        
                        time::sleep(time::Duration::from_millis(2000)).await;
                        
                        // MODIFICAÇÃO: Navega para a rota renomeada.
                        navigator.push(Route::ValidationPage {});
                    }
                    Err(e) => {
                        status_writer.set(format!("Erro durante a análise de imagens: {}", e));
                    }
                }
            }
            
            is_processing_writer.set(false);
        });
    };

    rsx! {
        div { class: "manual-processor-page",
            document::Stylesheet { href: asset!("/assets/styles.css") }
            document::Link {
                href: "https://fonts.googleapis.com/icon?family=Material+Icons",
                rel: "stylesheet"
            }

            div { class: "container py-8",
                                
                div { class: "card mb-6",
                    div { class: "form-group mb-6",
                        label { 
                            "Número de Prédios (1-20):" 
                        }
                        select {
                            class: "form-input",
                            value: "{num_buildings()}",
                            onchange: move |e| {
                                if let Ok(val) = e.value().parse::<usize>() {
                                    if val > 0 && val <= 20 {
                                        num_buildings.set(val);
                                    }
                                }
                            },
                            for i in 1..=20 {
                                option { key: "{i}", value: "{i}", "{i}" }
                            }
                        }
                    }

                    div { class: "space-y-6",
                        {(0..num_buildings()).map(|i| {
                            let building_names_data = building_names.read();
                            let facade_counts_data = facade_counts.read();
                            let facade_names_data = facade_names.read();
                            let buildings_data = buildings.read();

                            if i < building_names_data.len() && 
                               i < facade_counts_data.len() && 
                               i < facade_names_data.len() &&
                               i < buildings_data.len() {
                                let building_name_val = building_names_data[i].clone();
                                let facade_count_val = facade_counts_data[i];
                                
                                rsx! {
                                    div { key: "building-{i}", class: "card mb-6",
                                        div { class: "form-group mb-6",
                                            label { "Nome do Prédio:" }
                                            input {
                                                class: "form-input",
                                                value: "{building_name_val}",
                                                oninput: move |e| {
                                                    let new_building_name = e.value();
                                                    building_names.write()[i] = new_building_name.clone();
                                                    buildings.write()[i].name = new_building_name;
                                                }
                                            }
                                        }
            
                                        div { class: "form-group mb-6",
                                            label { "Número de Fachadas (1-8):" }
                                            select {
                                                class: "form-input",
                                                value: "{facade_count_val}",
                                                onchange: move |e| {
                                                    if let Ok(val) = e.value().parse::<usize>() {
                                                        if val > 0 && val <= 8 {
                                                            facade_counts.write()[i] = val;
                                                            facade_names.write()[i].retain(|&k, _| k < val);
                                                        }
                                                    }
                                                },
                                                for j_opt in 1..=8 {
                                                    option { key: "facade-opt-{i}-{j_opt}", value: "{j_opt}", "{j_opt}" }
                                                }
                                            }
                                        }
            
                                        div { class: "space-y-4",
                                            {(0..facade_count_val).map(|j| {
                                                let current_facade_name_for_ui = facade_names_data.get(i)
                                                    .and_then(|map| map.get(&j))
                                                    .cloned()
                                                    .unwrap_or_else(|| format!("Fachada {}", j + 1));
                                                
                                                let image_count = buildings_data.get(i)
                                                    .and_then(|b| b.facades.get(&current_facade_name_for_ui))
                                                    .map(|images| images.len())
                                                    .unwrap_or(0);

                                                rsx! {
                                                    div { key: "facade-{i}-{j}", class: "facade-card",
                                                        div { class: "d-flex items-center justify-between mb-4",
                                                            div { class: "form-group flex-1 mr-4",
                                                                label { "Nome da Fachada:" }
                                                                input {
                                                                    class: "form-input",
                                                                    placeholder: "Nome da fachada",
                                                                    value: "{current_facade_name_for_ui}",
                                                                    oninput: move |e| {
                                                                        let new_facade_name_ui = e.value();
                                                                        let mut current_facade_names_map_vec = facade_names.write();
                                                                        let old_name_ui = current_facade_names_map_vec[i]
                                                                            .get(&j)
                                                                            .cloned()
                                                                            .unwrap_or_else(|| format!("Fachada {}", j + 1));

                                                                        current_facade_names_map_vec[i].insert(j, new_facade_name_ui.clone());
                    
                                                                        let mut current_buildings_vec = buildings.write();
                                                                        if i < current_buildings_vec.len() {
                                                                            if old_name_ui != new_facade_name_ui {
                                                                                if let Some(images) = current_buildings_vec[i].facades.remove(&old_name_ui) {
                                                                                    current_buildings_vec[i].facades.insert(new_facade_name_ui.clone(), images);
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            button {
                                                                class: "btn btn-primary",
                                                                onclick: move |_| {
                                                                    let building_idx = i;
                                                                    let facade_ui_idx_for_add = j;
                                                                    spawn(async move {
                                                                        status.set("Selecionando imagens...".to_string());
                                                                        if let Some(files) = AsyncFileDialog::new()
                                                                            .add_filter("Todas as Imagens", &["jpg", "jpeg", "png", "gif", "bmp", "webp", "tiff", "tif"])
                                                                            .set_title("Selecionar Imagens")
                                                                            .set_directory("/")
                                                                            .pick_files()
                                                                            .await
                                                                        {
                                                                            status.set("Processando imagens selecionadas...".to_string());
                                                                            let facade_name_key = facade_names.read()[building_idx]
                                                                                .get(&facade_ui_idx_for_add)
                                                                                .cloned()
                                                                                .unwrap_or_else(|| format!("Fachada {}", facade_ui_idx_for_add + 1));
                    
                                                                            let image_data_vec: Vec<ImageData> = files
                                                                                .iter()
                                                                                .map(|f| {
                                                                                    let preview_url = create_data_uri(f.path());
                                                                                    
                                                                                    ImageData {
                                                                                        path: f.path().display().to_string(),
                                                                                        name: f.file_name(),
                                                                                        preview_url,
                                                                                    }
                                                                                })
                                                                                .collect();
                                                                            
                                                                            buildings.write()[building_idx].facades
                                                                                .entry(facade_name_key)
                                                                                .or_default()
                                                                                .extend(image_data_vec);
                                                                            
                                                                            status.set(format!("{} imagens adicionadas com sucesso!", files.len()));
                                                                        } else {
                                                                            status.set("Seleção de imagens cancelada.".to_string());
                                                                        }
                                                                    });
                                                                },
                                                                i { class: "material-icons", "add_photo_alternate" }
                                                                "Adicionar Imagens"
                                                            }
                                                        }
                                                        
                                                        div { class: "d-flex justify-between items-center mb-4",
                                                            p { class: "text-gray-600 text-sm", "Total de imagens: {image_count}" }
                                                        }
                    
                                                        if let Some(images_for_facade) = buildings_data[i].facades.get(&current_facade_name_for_ui) {
                                                            if !images_for_facade.is_empty() {
                                                                div { class: "image-grid",
                                                                    for (img_idx, img_data) in images_for_facade.iter().enumerate() {
                                                                        div { key: "img-{i}-{j}-{img_idx}", class: "image-preview-item",
                                                                            div { class: "image-wrapper",
                                                                                img {
                                                                                    src: "{img_data.preview_url.as_deref().unwrap_or_default()}",
                                                                                    class: "preview-image",
                                                                                    alt: "{img_data.name}"
                                                                                }
                                                                            }
                                                                            div { class: "delete-overlay",
                                                                                button {
                                                                                    class: "delete-button",
                                                                                    onclick: move |_| {
                                                                                        let building_idx_del = i;
                                                                                        let facade_ui_idx_del = j;
                                                                                        let image_index_to_remove = img_idx;
                    
                                                                                        let facade_name_key_for_delete = facade_names.read()[building_idx_del]
                                                                                            .get(&facade_ui_idx_del)
                                                                                            .cloned()
                                                                                            .unwrap_or_else(|| format!("Fachada {}", facade_ui_idx_del + 1));
                    
                                                                                        let mut all_buildings = buildings.write();
                                                                                        if building_idx_del < all_buildings.len() {
                                                                                            if let Some(facade_images_vec) = all_buildings[building_idx_del].facades.get_mut(&facade_name_key_for_delete) {
                                                                                                if image_index_to_remove < facade_images_vec.len() {
                                                                                                    facade_images_vec.remove(image_index_to_remove);
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    },
                                                                                    i { class: "material-icons", "delete" }
                                                                                }
                                                                            }
                                                                            p { class: "image-name", "{img_data.name}" }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }
                            } else {
                                rsx! {
                                    div { key: "loading-{i}", class: "status-box info",
                                        "Carregando dados do prédio..."
                                    }
                                }
                            }
                        })}
                    }

                    div { class: "mt-6 space-y-4",
                        if !status().is_empty() {
                            p { class: "status-message info", "{status()}" }
                        }
                        
                        div { class: "d-flex justify-end gap-4",
                            button {
                                class: "btn btn-primary",
                                disabled: is_processing(),
                                onclick: organize_folders,
                                if is_processing() { "Processando..." } else { "Organizar e Processar Imagens" }
                            }
                            
                            {
                                let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                                let detection_file = base_dir.join("Projects").join(&project_name_for_detection_check).join("detection_results.json");
                                if detection_file.exists() {
                                    rsx! {
                                        // MODIFICAÇÃO: Link para a rota renomeada.
                                        Link {
                                            to: Route::ValidationPage {},
                                            class: "btn btn-success",
                                            i { class: "material-icons", "verified" }
                                            "Validar Resultados"
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

pub async fn run_yolo_script_and_parse_results(
    project_name: &str,
    mut status: Signal<String>,
    app_rust_dir: &PathBuf,
) -> Result<Vec<ImageAnalysisResult>, String> {
    status.set("Preparando para executar script de análise de imagens...".to_string());

    let project_dir = app_rust_dir.join("Projects").join(project_name);
    let images_dir = project_dir.join("images");

    if !project_dir.exists() {
        return Err(format!("Diretório do projeto não encontrado: {}", project_dir.display()));
    }

    if !images_dir.exists() {
        return Err(format!("Diretório de imagens não encontrado: {}", images_dir.display()));
    }

    if !images_dir.is_dir() {
        return Err(format!("O caminho não é um diretório válido: {}", images_dir.display()));
    }

    let candidate_dirs = [
        app_rust_dir.join("..").join("model").join("Yolo").join("YOLO-Det-Py"),
        app_rust_dir.join("..").join("Yolo").join("YOLO-Det-Py"),
    ];

    let (script_path, model_path) = candidate_dirs
        .iter()
        .find_map(|dir| {
            let script = dir.join("rodar_modelo_prod.py");
            let model = dir.join("best.pt");
            if script.exists() && model.exists() {
                Some((script, model))
            } else {
                None
            }
        })
        .ok_or_else(|| {
            format!(
                "Script ou modelo YOLO não encontrados. Procurado em: {:?}",
                candidate_dirs
            )
        })?;

    status.set("Executando script de análise de imagens... (Isso pode levar um tempo)".to_string());

    let script_project_argument = project_name;

    let output = Command::new("python")
        .current_dir(app_rust_dir)
        .env("PATH", format!("{}:{}", 
            app_rust_dir.parent().unwrap().join("venv").join("bin").display(),
            std::env::var("PATH").unwrap_or_default()
        ))
        .arg(&script_path)
        .arg(&script_project_argument)
        .arg(&model_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("Falha ao executar o script Python: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        status.set(format!("Erro na execução do script Python: {}", stderr));
        return Err(format!("Script Python falhou: {}", stderr));
    }

    let stdout_str = String::from_utf8(output.stdout)
        .map_err(|e| format!("Saída do script Python não é UTF-8 válido: {}", e))?;
    
    status.set("Script executado. Processando resultados...".to_string());

    serde_json::from_str::<Vec<ImageAnalysisResult>>(&stdout_str)
        .map_err(|e| format!("Falha ao parsear JSON da saída do script: {}\nSaída: {}", e, stdout_str))
}