use dioxus::prelude::*;
use std::collections::HashMap;
use rfd::AsyncFileDialog;
use std::path::{PathBuf, Path};
use std::fs;
use dioxus::prelude::Readable;
use std::process::{Command, Stdio};
use serde::Deserialize;

pub async fn run_yolo_script_and_parse_results(
    project_name: &str,
    mut status: Signal<String>,
    app_rust_dir: &PathBuf,
) -> Result<Vec<ImageAnalysisResult>, String> {
    status.set("Preparando para executar script de análise de imagens...".to_string());

    let script_path = app_rust_dir.join("..").join("Yolo").join("YOLO-Det-Py").join("rodar_modelo_prod.py");
    let model_path = app_rust_dir.join("..").join("Yolo").join("YOLO-Det-Py").join("best.pt");

    if !script_path.exists() {
        return Err(format!("Script Python não encontrado em: {}", script_path.display()));
    }
    if !model_path.exists() {
        return Err(format!("Modelo YOLO não encontrado em: {}", model_path.display()));
    }

    status.set("Executando script de análise de imagens... (Isso pode levar um tempo)".to_string());

    let script_project_argument = format!("../app-rust/Projects/{}", project_name);

    let output = Command::new("python3")
        .current_dir(app_rust_dir)
        .arg(&script_path)
        .arg(script_project_argument)
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