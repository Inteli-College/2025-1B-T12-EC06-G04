use dioxus::prelude::*;
use std::f64::consts::PI;
use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
use crate::Route;
use dioxus_router::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

// MODIFICAÇÃO: A struct é importada de create_project para manter uma fonte única.
use crate::pages::create_project::{ProjectMetadata, ProjectStatus};


// --- Structs for parsing detection_results.json ---
#[derive(Deserialize, Debug, Clone)]
struct DetectionFissura {
    name: String,
    confidence: f64,
}

#[derive(Deserialize, Debug, Clone)]
struct ImageDetectionData {
    path: String,
    fissura: Vec<DetectionFissura>,
}

// --- Structs for aggregated data for bar chart ---
#[derive(Debug, Clone)]
struct BuildingFissuraSummary {
    building_name: String,
    termica_count: u32,
    retracao_count: u32,
}

// --- Struct for Box Plot statistics ---
#[derive(Debug, Clone, PartialEq)]
struct BoxPlotStats {
    min_whisker: f64,
    q1: f64,
    median: f64,
    q3: f64,
    max_whisker: f64,
    outliers: Vec<f64>,
}

// --- NEW: Type alias for Heatmap data ---
type HeatmapData = HashMap<String, HashMap<String, u32>>;


// --- Error type for JSON reading ---
#[derive(Debug)]
pub enum JsonReadError {
    Io(io::Error),
    Json(serde_json::Error),
    PathError(String),
}

impl From<io::Error> for JsonReadError {
    fn from(err: io::Error) -> JsonReadError {
        JsonReadError::Io(err)
    }
}

impl From<serde_json::Error> for JsonReadError {
    fn from(err: serde_json::Error) -> JsonReadError {
        JsonReadError::Json(err)
    }
}

// --- Função para obter o diretório base de projetos ---
fn get_projects_dir() -> Option<PathBuf> {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    Some(base_dir.join("Projects"))
}

// --- NOVA FUNÇÃO: Sanitizar nome para nome de pasta ---
fn sanitize_name(name: &str) -> String {
    name.replace(' ', "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect::<String>()
}

// --- MODIFICAÇÃO: Tornada pública para ser usada em outros módulos (homepage) ---
pub fn read_project_metadata(project_name: &str) -> Result<ProjectMetadata, JsonReadError> {
    let projects_dir = get_projects_dir().ok_or_else(|| JsonReadError::PathError("Diretório 'Projects' não encontrado".to_string()))?;
    let meta_path = projects_dir.join(project_name).join("project_meta.json");
    
    let file = File::open(&meta_path).map_err(JsonReadError::Io)?;
    let reader = BufReader::new(file);
    let metadata: ProjectMetadata = serde_json::from_reader(reader).map_err(JsonReadError::Json)?;

    Ok(metadata)
}

// --- NOVA FUNÇÃO: Salvar metadados do projeto ---
pub fn save_project_metadata(project_folder_name: &str, metadata: &ProjectMetadata) -> Result<(), io::Error> {
    if let Some(projects_dir) = get_projects_dir() {
        let project_path = projects_dir.join(project_folder_name);
        let metadata_path = project_path.join("project_meta.json");
        
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(metadata_path)?;
        
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, metadata)?;
        writer.flush()?;
    }
    Ok(())
}

// --- NOVA FUNÇÃO: Para deletar a pasta do projeto ---
fn delete_project_folder(project_name: &str) -> Result<(), std::io::Error> {
    if let Some(projects_dir) = get_projects_dir() {
        let project_path = projects_dir.join(project_name);
        if project_path.exists() {
            if project_path.is_dir() {
                fs::remove_dir_all(project_path)
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Diretório 'Projects' não encontrado."))
    }
}


// --- Function to read and parse detection_results.json ---
fn ler_json_detection_results(project_name: &str) -> Result<Vec<ImageDetectionData>, JsonReadError> {
    let projects_dir = get_projects_dir().ok_or_else(|| JsonReadError::PathError("Não foi possível encontrar o diretório 'Projects'".to_string()))?;
    let json_path = projects_dir
        .join(project_name)
        .join("detection_results.json");

    println!("[RUST graph.rs] Tentando ler JSON de: {}", json_path.display());

    let file = File::open(&json_path).map_err(|e| {
        eprintln!("[RUST graph.rs] Erro ao abrir arquivo JSON em '{}': {}", json_path.display(), e);
        JsonReadError::Io(e)
    })?;

    let reader = BufReader::new(file);
    let results: Vec<ImageDetectionData> = serde_json::from_reader(reader).map_err(|e| {
        eprintln!("[RUST graph.rs] Erro ao fazer parse do JSON em '{}': {}", json_path.display(), e);
        JsonReadError::Json(e)
    })?;
    Ok(results)
}

// Funções auxiliares (extract_building_name_from_path, polar_to_cartesian, etc.) permanecem as mesmas
// ... (O código das funções de geração de SVG não foi alterado e foi omitido aqui para brevidade) ...

// Helper to extract building name
fn extract_building_name_from_path(image_path_str: &str) -> Option<String> {
    let image_path = Path::new(image_path_str);
    image_path.parent()?.parent()?.file_name()?.to_str().map(String::from)
}

// --- NEW: Helper to extract facade name ---
fn extract_facade_name_from_path(image_path_str: &str) -> Option<String> {
    let image_path = Path::new(image_path_str);
    image_path.parent()?.file_name()?.to_str().map(String::from)
}


// --- Donut Chart Helpers ---
fn polar_to_cartesian(cx: f64, cy: f64, r: f64, angle_deg: f64) -> (f64, f64) {
    let angle_rad = (angle_deg - 90.0) * PI / 180.0;
    (cx + r * angle_rad.cos(), cy + r * angle_rad.sin())
}

fn describe_arc(cx: f64, cy: f64, r: f64, start_angle: f64, end_angle: f64) -> String {
    let (x1, y1) = polar_to_cartesian(cx, cy, r, end_angle);
    let (x2, y2) = polar_to_cartesian(cx, cy, r, start_angle);
    let large_arc_flag = if end_angle - start_angle > 180.0 { 1 } else { 0 };

    format!(
        "M {x1} {y1} A {r} {r} 0 {large_arc_flag} 0 {x2} {y2} L {cx} {cy} Z"
    )
}

fn donut_segment(
    cx: f64,
    cy: f64,
    r: f64,
    start_angle: f64,
    end_angle: f64,
    color_id: &str,
    label: &str
) -> String {
    let path = describe_arc(cx, cy, r, start_angle, end_angle);
    format!(
        r###"<path d="{path}" fill="url(#{color_id})" stroke="#FFFFFF" stroke-width="2" style="filter: drop-shadow(0px 2px 5px rgba(0,0,0,0.2)); opacity: 0;">
            <animate attributeName="opacity" from="0" to="1" dur="1s" fill="freeze" />
            <title>{label}</title>
        </path>"###
    )
}

fn gerar_svg_donut(total_termica: u32, total_retracao: u32) -> String {
    let total_fissuras = total_termica + total_retracao;
    if total_fissuras == 0 {
        return r##"<svg width="400" height="400" viewBox="0 0 400 400" xmlns="http://www.w3.org/2000/svg">
                   <text x="200" y="200" font-size="16" text-anchor="middle" fill="var(--gray-500)" dominant-baseline="middle">Sem dados para exibir</text>
                 </svg>"##.to_string();
    }
    let angle_termica = (total_termica as f64 / total_fissuras as f64) * 360.0;

    let cx = 200.0;
    let cy = 200.0;
    let raio_externo = 160.0;
    let raio_interno = 100.0;
    let label_termica = format!("Térmica: {}", total_termica);
    let label_retracao = format!("Retração: {}", total_retracao);

    let mut svg = String::from(r#"<svg width="400" height="400" viewBox="0 0 400 400" xmlns="http://www.w3.org/2000/svg">"#);
    svg.push_str(r###"<defs>
            <linearGradient id="grad_red" x1="0%" y1="0%" x2="100%" y2="100%"><stop offset="0%" style="stop-color:#c94a4a; stop-opacity:1" /><stop offset="100%" style="stop-color:#a93a3a; stop-opacity:1" /></linearGradient>
            <linearGradient id="grad_blue" x1="0%" y1="0%" x2="100%" y2="100%"><stop offset="0%" style="stop-color:#3a5a9c; stop-opacity:1" /><stop offset="100%" style="stop-color:#2c467a; stop-opacity:1" /></linearGradient>
        </defs>"###);
    svg.push_str(&donut_segment(cx, cy, raio_externo, 0.0, angle_termica, "grad_red", &label_termica));
    svg.push_str(&donut_segment(cx, cy, raio_externo, angle_termica, 360.0, "grad_blue", &label_retracao));
    svg.push_str(&format!(r###"<circle cx="{cx}" cy="{cy}" r="{raio_interno}" fill="var(--bg-light)"/>"###));
    svg.push_str(&format!(r###"<text x="{cx}" y="{cy}" font-size="18" text-anchor="middle" fill="var(--text-dark)" dominant-baseline="middle" font-family="Poppins, sans-serif" font-weight="600">Total Fissuras</text>"###));
    svg.push_str("</svg>");
    svg
}

// --- Bar Chart Helpers ---
fn gerar_svg_barras(building_summaries: &[BuildingFissuraSummary], media_total: f64) -> String {
    if building_summaries.is_empty() {
        return r##"<svg width="600" height="400" viewBox="0 0 600 400" xmlns="http://www.w3.org/2000/svg">
                   <text x="300" y="200" font-size="16" text-anchor="middle" fill="var(--gray-500)" dominant-baseline="middle">Sem dados para exibir</text>
                 </svg>"##.to_string();
    }
    let altura_total = 400;
    let largura_barra = 35;
    let espacamento = 90;
    let margem_esquerda = 60;
    let largura_total_svg = margem_esquerda + building_summaries.len() as i32 * espacamento + 40;
    let max_count_val = building_summaries.iter()
        .map(|s| s.termica_count + s.retracao_count)
        .max()
        .unwrap_or(1)
        .max(media_total.ceil() as u32) as f64;
    let max_bar_height = 250.0;
    let y_base = 320.0;

    let mut svg = format!(r###"<svg width="{largura_total_svg}" height="{altura_total}" viewBox="0 0 {largura_total_svg} {altura_total}" xmlns="http://www.w3.org/2000/svg" style="font-family: 'Inter', sans-serif;">"###);
    svg.push_str(r###"<defs>
            <linearGradient id="grad_red_bar" x1="0%" y1="0%" x2="0%" y2="100%"><stop offset="0%" style="stop-color:#c94a4a; stop-opacity:1" /><stop offset="100%" style="stop-color:#a93a3a; stop-opacity:1" /></linearGradient>
            <linearGradient id="grad_blue_bar" x1="0%" y1="0%" x2="0%" y2="100%"><stop offset="0%" style="stop-color:#3a5a9c; stop-opacity:1" /><stop offset="100%" style="stop-color:#2c467a; stop-opacity:1" /></linearGradient>
        </defs>"###);

    if media_total > 0.0 && max_count_val > 0.0 {
        let y_media = y_base - (media_total / max_count_val * max_bar_height);
        svg.push_str(&format!(
            r###"<line x1="{margem_esquerda}" y1="{y_media}" x2="{x2}" y2="{y_media}" stroke="var(--border-color)" stroke-width="2" stroke-dasharray="4,4" stroke-opacity="0.9"><title>Média por Edifício: {media_total:.2}</title></line>"###,
            x2 = largura_total_svg - 20
        ));
        svg.push_str(&format!(
            r###"<text x="{x}" y="{y}" font-size="11" fill="var(--text-dark)" text-anchor="end" dominant-baseline="middle">Média ({media_total:.1})</text>"###,
            x = largura_total_svg - 25,
            y = y_media
        ));
    }

    for (i, summary) in building_summaries.iter().enumerate() {
        let x_base = margem_esquerda + i as i32 * espacamento;
        let total_fissuras_edificio = summary.termica_count + summary.retracao_count;
        let h_total = if max_count_val == 0.0 { 0.0 } else { (total_fissuras_edificio as f64 / max_count_val * max_bar_height) };
        let proporcao_termica = if total_fissuras_edificio == 0 { 0.0 } else { summary.termica_count as f64 / total_fissuras_edificio as f64 };

        let h_termica = h_total * proporcao_termica;
        let h_retracao = h_total - h_termica;
        
        svg.push_str(&format!(r###"<rect x="{x}" y="{y}" width="{w}" height="0" fill="url(#grad_blue_bar)" rx="4"><animate attributeName="height" from="0" to="{h}" dur="0.8s" fill="freeze" /><animate attributeName="y" from="{y_plus}" to="{y}" dur="0.8s" fill="freeze" /><title>Retração: {val}</title></rect>"###, x = x_base, y = y_base - h_retracao, y_plus = y_base, w = largura_barra, h = h_retracao, val = summary.retracao_count));
        svg.push_str(&format!(r###"<rect x="{x}" y="{y}" width="{w}" height="0" fill="url(#grad_red_bar)" rx="4"><animate attributeName="height" from="0" to="{h}" dur="0.8s" fill="freeze" /><animate attributeName="y" from="{y_plus}" to="{y}" dur="0.8s" fill="freeze" /><title>Térmica: {val}</title></rect>"###, x = x_base, y = y_base - h_retracao - h_termica, y_plus = y_base - h_retracao, w = largura_barra, h = h_termica, val = summary.termica_count));
        
        svg.push_str(&format!(r###"<text x="{x_text}" y="340" font-size="12" text-anchor="middle" fill="var(--text-dark)">{name}</text>"###, x_text = x_base + largura_barra / 2, name = summary.building_name));
    }

    svg.push_str("</svg>");
    svg
}


// --- Box Plot Helpers ---
fn calculate_boxplot_stats(data: &mut Vec<f64>) -> Option<BoxPlotStats> {
    if data.is_empty() {
        return None;
    }
    data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let q1 = data[(data.len() as f64 * 0.25) as usize];
    let median = data[(data.len() as f64 * 0.50) as usize];
    let q3 = data[(data.len() as f64 * 0.75) as usize];
    let iqr = q3 - q1;

    let upper_whisker_bound = q3 + 1.5 * iqr;
    let lower_whisker_bound = q1 - 1.5 * iqr;

    let mut outliers = Vec::new();
    let mut non_outliers = Vec::new();
    for &val in data.iter() {
        if val > upper_whisker_bound || val < lower_whisker_bound {
            outliers.push(val);
        } else {
            non_outliers.push(val);
        }
    }

    let min_whisker = *non_outliers.first().unwrap_or(&q1);
    let max_whisker = *non_outliers.last().unwrap_or(&q3);

    Some(BoxPlotStats { min_whisker, q1, median, q3, max_whisker, outliers })
}

fn gerar_svg_boxplot(stats_termica: &Option<BoxPlotStats>, stats_retracao: &Option<BoxPlotStats>) -> String {
    if stats_termica.is_none() && stats_retracao.is_none() {
        return r##"<svg width="600" height="400" viewBox="0 0 600 400" xmlns="http://www.w3.org/2000/svg"><text x="300" y="200" font-size="16" text-anchor="middle" fill="var(--gray-500)" dominant-baseline="middle">Sem dados para exibir</text></svg>"##.to_string();
    }
    
    let width = 600;
    let height = 400;
    let margin = (50.0, 50.0, 50.0, 50.0);
    let plot_height = height as f64 - margin.0 - margin.2;

    let mut svg = format!(r###"<svg width="{width}" height="{height}" viewBox="0 0 {width} {height}" xmlns="http://www.w3.org/2000/svg" style="font-family: 'Inter', sans-serif;">"###);

    svg.push_str(&format!(r###"<line x1="{x1}" y1="50" x2="{x1}" y2="350" stroke="var(--border-color)" stroke-width="1"/>"###, x1 = margin.3));
    for i in 0..=10 {
        let y = 350.0 - (i as f64 * (plot_height / 10.0));
        let label = format!("{:.1}", i as f64 / 10.0);
        svg.push_str(&format!(r###"<text x="40" y="{y}" font-size="10" text-anchor="end" fill="var(--text-dark)" dominant-baseline="middle">{label}</text>"###));
        svg.push_str(&format!(r###"<line x1="50" y1="{y}" x2="{x2}" y2="{y}" stroke="var(--border-color)" stroke-width="0.5" stroke-dasharray="2,2"/>"###, x2 = width as f64 - margin.1));
    }
    svg.push_str(r###"<text x="15" y="200" font-size="12" fill="var(--text-dark)" transform="rotate(-90 15,200)">Confiança</text>"###);

    let draw_box = |stats: &BoxPlotStats, x_center: f64, color: &str, fill_color: &str| -> String {
        let mut parts = String::new();
        let scale = |v: f64| 350.0 - v * plot_height;

        let y_q1 = scale(stats.q1);
        let y_q3 = scale(stats.q3);
        let y_median = scale(stats.median);
        let y_min_w = scale(stats.min_whisker);
        let y_max_w = scale(stats.max_whisker);
        let box_width = 120.0;

        parts.push_str(&format!(r###"<line x1="{x_center}" y1="{y_max_w}" x2="{x_center}" y2="{y_q3}" stroke="{color}" stroke-width="2"/>"###));
        parts.push_str(&format!(r###"<line x1="{x_center}" y1="{y_q1}" x2="{x_center}" y2="{y_min_w}" stroke="{color}" stroke-width="2"/>"###));
        parts.push_str(&format!(r###"<line x1="{x_center_min}" y1="{y_max_w}" x2="{x_center_plus}" y2="{y_max_w}" stroke="{color}" stroke-width="2"/>"###, x_center_min = x_center - 25.0, x_center_plus = x_center + 25.0));
        parts.push_str(&format!(r###"<line x1="{x_center_min}" y1="{y_min_w}" x2="{x_center_plus}" y2="{y_min_w}" stroke="{color}" stroke-width="2"/>"###, x_center_min = x_center - 25.0, x_center_plus = x_center + 25.0));

        parts.push_str(&format!(r###"<rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{fill_color}" fill-opacity="0.5" stroke="{color}" stroke-width="2"/>"###, x = x_center - box_width / 2.0, y = y_q3, w = box_width, h = y_q1 - y_q3));
        
        parts.push_str(&format!(r###"<line x1="{x_start}" y1="{y_median}" x2="{x_end}" y2="{y_median}" stroke="{color}" stroke-width="3"/>"###, x_start = x_center - box_width / 2.0, x_end = x_center + box_width / 2.0));
        
        for &outlier in &stats.outliers {
            let y_outlier = scale(outlier);
            parts.push_str(&format!(r###"<circle cx="{x_center}" cy="{y_outlier}" r="4" fill="none" stroke="{color}" stroke-width="1.5"/>"###));
        }
        parts
    };

    if let Some(stats) = stats_termica {
        svg.push_str(&draw_box(stats, 200.0, "#c94a4a", "#f8d7da"));
        svg.push_str(r###"<text x="200" y="375" font-size="14" text-anchor="middle" fill="#c94a4a" font-weight="500">Térmica</text>"###);
    }

    if let Some(stats) = stats_retracao {
        svg.push_str(&draw_box(stats, 400.0, "#3a5a9c", "#d1ecf1"));
        svg.push_str(r###"<text x="400" y="375" font-size="14" text-anchor="middle" fill="#3a5a9c" font-weight="500">Retração</text>"###);
    }
    
    svg.push_str("</svg>");
    svg
}

// --- NEW: Heatmap Helpers ---
fn gerar_svg_heatmap(heatmap_data: &HeatmapData) -> String {
    if heatmap_data.is_empty() {
        return r##"<svg width="800" height="400" viewBox="0 0 800 400" xmlns="http://www.w3.org/2000/svg">
                   <text x="400" y="200" font-size="16" text-anchor="middle" fill="var(--gray-500)" dominant-baseline="middle">Sem dados para exibir</text>
                 </svg>"##.to_string();
    }

    let mut building_names: Vec<String> = heatmap_data.keys().cloned().collect();
    building_names.sort();

    let facade_name_set: HashSet<String> = heatmap_data.values().flat_map(|facades| facades.keys()).cloned().collect();
    let mut facade_names: Vec<String> = facade_name_set.into_iter().collect();
    facade_names.sort();

    let max_val = heatmap_data.values().flat_map(|f| f.values()).max().cloned().unwrap_or(0) as f64;

    let cell_size = 60.0;
    let x_offset = 150.0;
    let y_offset = 120.0;
    let svg_width = x_offset + (facade_names.len() as f64 * cell_size) + 120.0; // +120 for legend
    let svg_height = y_offset + (building_names.len() as f64 * cell_size);

    let mut svg = format!(r###"<svg width="{svg_width}" height="{svg_height}" viewBox="0 0 {svg_width} {svg_height}" xmlns="http://www.w3.org/2000/svg" style="font-family: 'Inter', sans-serif;">"###);

    let get_color = |count: u32| {
        if count == 0 { return "#f3f4f6".to_string(); }
        let intensity = (count as f64 / max_val).sqrt(); // Use sqrt for better color distribution
        // Interpolate from light orange (#fff5e6) to dark red (#c94a4a)
        let r = (255.0 + (201.0 - 255.0) * intensity).round() as u8;
        let g = (245.0 + (74.0 - 245.0) * intensity).round() as u8;
        let b = (230.0 + (74.0 - 230.0) * intensity).round() as u8;
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    };
    
    for (row, building) in building_names.iter().enumerate() {
        let y = y_offset + (row as f64 * cell_size);
        svg.push_str(&format!(r###"<text x="{x}" y="{y_center}" font-size="14" text-anchor="end" fill="var(--text-dark)" dominant-baseline="middle">{building}</text>"###, x = x_offset - 15.0, y_center = y + cell_size / 2.0));

        for (col, facade) in facade_names.iter().enumerate() {
            let count = heatmap_data.get(building).and_then(|f| f.get(facade)).cloned().unwrap_or(0);
            let color = get_color(count);
            let x = x_offset + (col as f64 * cell_size);
            
            svg.push_str(&format!(
                r###"<rect x="{x}" y="{y}" width="{size}" height="{size}" fill="{color}" stroke="rgba(0,0,0,0.05)" stroke-width="1" rx="4">
                    <title>Edifício: {building}\nFachada: {facade}\nFissuras: {count}</title>
                    <animate attributeName="opacity" from="0" to="1" dur="1.5s" fill="freeze" />
                </rect>"###,
                x=x, y=y, size=cell_size, color=color, building=building, facade=facade, count=count
            ));
        }
    }

    for (col, facade) in facade_names.iter().enumerate() {
        let x = x_offset + (col as f64 * cell_size) + (cell_size / 2.0);
        svg.push_str(&format!(r###"<text x="{x}" y="{y}" font-size="14" text-anchor="end" fill="var(--text-dark)" transform="rotate(-45, {x}, {y})">{facade}</text>"###, x = x, y = y_offset - 15.0));
    }
    
    let legend_x = svg_width - 90.0;
    svg.push_str(&format!(r###"<text x="{legend_x}" y="35" font-size="12" font-weight="500" fill="var(--text-dark)">Criticidade</text>"###));
    for i in 0..=5 {
        let ratio = i as f64 / 5.0;
        let count = (ratio * max_val).round() as u32;
        let color = get_color(count);
        let y = 50.0 + ratio * 150.0;
        svg.push_str(&format!(r###"<rect x="{legend_x}" y="{y}" width="25" height="25" rx="4" fill="{color}"/>"###));
        svg.push_str(&format!(r###"<text x="{x}" y="{y_text}" font-size="11" fill="var(--text-dark)" dominant-baseline="middle">{count}</text>"###, x = legend_x + 30.0, y_text= y + 12.5));
    }

    svg.push_str("</svg>");
    svg
}

#[derive(Props, PartialEq, Clone)]
pub struct GraphViewProps {
    pub project_name: String
}

#[component]
pub fn GraphView(props: GraphViewProps) -> Element {
    let mut show_edit_modal = use_signal(|| false);
    let mut show_delete_modal = use_signal(|| false);
    let mut project_display_name = use_signal(|| props.project_name.clone());

    use_effect({
        let project_name = props.project_name.clone();
        let mut display_name = project_display_name;
        move || {
            if let Ok(meta) = read_project_metadata(&project_name) {
                if !meta.name.is_empty() {
                    display_name.set(meta.name);
                }
            }
        }
    });

    match ler_json_detection_results(&props.project_name) {
        Ok(detection_data) => {
            let mut total_termica_overall = 0u32;
            let mut total_retracao_overall = 0u32;
            let mut building_fissura_map: HashMap<String, BuildingFissuraSummary> = HashMap::new();
            
            let mut confidences_termica: Vec<f64> = Vec::new();
            let mut confidences_retracao: Vec<f64> = Vec::new();
            
            let mut heatmap_data: HeatmapData = HashMap::new();

            for item_data in detection_data.iter() {
                let mut current_image_termica = 0u32;
                let mut current_image_retracao = 0u32;
                for fissura_item in item_data.fissura.iter() {
                    let name_lower = fissura_item.name.to_lowercase();
                    if name_lower == "termica" {
                        total_termica_overall += 1;
                        current_image_termica += 1;
                        confidences_termica.push(fissura_item.confidence);
                    } else if name_lower == "retracao" || name_lower == "retraçao" {
                        total_retracao_overall += 1;
                        current_image_retracao += 1;
                        confidences_retracao.push(fissura_item.confidence);
                    }
                }
                
                if let Some(building_name) = extract_building_name_from_path(&item_data.path) {
                    let summary = building_fissura_map.entry(building_name.clone()).or_insert_with(|| BuildingFissuraSummary {
                        building_name: building_name.clone(),
                        termica_count: 0,
                        retracao_count: 0,
                    });
                    summary.termica_count += current_image_termica;
                    summary.retracao_count += current_image_retracao;
                    
                    if let Some(facade_name) = extract_facade_name_from_path(&item_data.path) {
                        let total_fissures_in_image = item_data.fissura.len() as u32;
                        let building_entry = heatmap_data.entry(building_name.clone()).or_default();
                        let facade_entry = building_entry.entry(facade_name).or_insert(0);
                        *facade_entry += total_fissures_in_image;
                    }
                }
            }
            
            let mut building_summaries: Vec<BuildingFissuraSummary> = building_fissura_map.values().cloned().collect();
            building_summaries.sort_by(|a, b| a.building_name.cmp(&b.building_name));

            let media_fissuras_por_edificio = if !building_summaries.is_empty() {
                let total_fissuras: u32 = building_summaries.iter().map(|s| s.termica_count + s.retracao_count).sum();
                total_fissuras as f64 / building_summaries.len() as f64
            } else {
                0.0
            };

            let donut_svg = gerar_svg_donut(total_termica_overall, total_retracao_overall);
            let barras_svg = gerar_svg_barras(&building_summaries, media_fissuras_por_edificio);
            
            let stats_termica = calculate_boxplot_stats(&mut confidences_termica);
            let stats_retracao = calculate_boxplot_stats(&mut confidences_retracao);
            let boxplot_svg = gerar_svg_boxplot(&stats_termica, &stats_retracao);

            let heatmap_svg = gerar_svg_heatmap(&heatmap_data);

            let navigator = use_navigator();
            let report_target_building = building_summaries.first().map(|s| s.building_name.clone());
            let project_name_for_button = props.project_name.clone();
            let project_name_clone_for_delete = props.project_name.clone();
            
            rsx! {
                if show_edit_modal() {
                    EditProjectModal {
                        project_name: props.project_name.clone(),
                        on_close: move |_| show_edit_modal.set(false),
                        on_save: move |new_folder_name| {
                            if let Some(name) = new_folder_name {
                                navigator.replace(Route::GraphView { project_name: name });
                            }
                             if let Ok(meta) = read_project_metadata(&props.project_name) {
                                if !meta.name.is_empty() {
                                    project_display_name.set(meta.name);
                                }
                            }
                            show_edit_modal.set(false);
                        }
                    }
                }

                if show_delete_modal() {
                    DeleteConfirmationModal {
                        project_name: project_display_name(),
                        on_confirm: move |_| {
                            let nav = navigator.clone();
                            let name_to_delete = project_name_clone_for_delete.clone();
                            spawn(async move {
                                if delete_project_folder(&name_to_delete).is_ok() {
                                    println!("Projeto {} deletado.", name_to_delete);
                                    nav.push(Route::HomePage {});
                                } else {
                                    eprintln!("Falha ao deletar {}.", name_to_delete);
                                }
                            });
                            show_delete_modal.set(false);
                        },
                        on_cancel: move |_| show_delete_modal.set(false),
                    }
                }

                div {
                    class: "container py-8",
                    
                    div {
                        class: "text-center mb-4",
                        h1 { class: "page-header-title", "Análise Gráfica de Fissuras" }
                        p { class: "text-lg text-gray-600", "Projeto: {project_display_name()}"}
                    }

                    div {
                        class: "d-flex justify-between items-center w-full mb-8 flex-wrap gap-4",
                        div {
                            class: "d-flex",
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| { navigator.push(Route::HomePage {}); },
                                i { class: "material-icons", "arrow_back" }
                                "Voltar ao Início"
                            }
                        }
                        
                        div {
                            class: "d-flex gap-4",
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| show_edit_modal.set(true),
                                i { class: "material-icons", "edit" }
                                "Editar Projeto"
                            }
                            button {
                                class: "btn btn-danger",
                                onclick: move |_| show_delete_modal.set(true),
                                i { class: "material-icons", "delete" }
                                "Excluir Projeto"
                            }
                            if let Some(building_name) = report_target_building {
                                button {
                                    class: "btn btn-primary",
                                    onclick: move |_| {
                                        navigator.push(Route::ReportView { 
                                            project_name: project_name_for_button.clone(), 
                                            building_name: building_name.clone() 
                                        });
                                    },
                                    "Visualizar Relatório"
                                    i { class: "material-icons", "arrow_forward" }
                                }
                            }
                        }
                    }

                    div {
                        class: "dashboard-grid",
                        
                        div {
                            class: "card dashboard-card",
                            h2 { "Distribuição Geral" }
                            div {
                                class: "chart-container",
                                dangerous_inner_html: donut_svg
                            }
                            div {
                                class: "legend",
                                div { class: "legend-item",
                                    div { class: "legend-color-box", style: "background-color: #c94a4a;" }
                                    span { "Térmica ({total_termica_overall})" }
                                }
                                div { class: "legend-item",
                                    div { class: "legend-color-box", style: "background-color: var(--primary-blue);" }
                                    span { "Retração ({total_retracao_overall})" }
                                }
                            }
                        }

                         div {
                            class: "card dashboard-card",
                            h2 { "Confiança das Detecções" }
                            div { 
                                class: "chart-container", 
                                dangerous_inner_html: boxplot_svg 
                            }
                        }
                        
                        div {
                            class: "card dashboard-card dashboard-card-large",
                            h2 { "Fissuras por Edifício" }
                            div {
                                class: "chart-container-scroll",
                                div { dangerous_inner_html: barras_svg }
                            }
                        }

                        div {
                            class: "card dashboard-card dashboard-card-large",
                            h2 { "Heatmap de Criticidade por Fachada" },
                            div { 
                                class: "chart-container-scroll",
                                dangerous_inner_html: heatmap_svg 
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            let error_message = match e {
                JsonReadError::Io(io_err) => format!("Erro de I/O: {}. Verifique se o arquivo 'detection_results.json' existe e se o programa tem permissão para lê-lo.", io_err),
                JsonReadError::Json(json_err) => format!("Erro de Formato: {}. O arquivo 'detection_results.json' parece estar corrompido ou mal formatado.", json_err),
                JsonReadError::PathError(path_err) => format!("Erro de Caminho: {}", path_err),
            };

            let navigator = use_navigator();
            let project_name_clone = props.project_name.clone();

            rsx! {
                if show_delete_modal() {
                    DeleteConfirmationModal {
                        project_name: props.project_name.clone(),
                        on_confirm: move |_| {
                            let nav = navigator.clone();
                            let name_to_delete = project_name_clone.clone();
                             spawn(async move {
                                if delete_project_folder(&name_to_delete).is_ok() {
                                    nav.push(Route::HomePage {});
                                }
                            });
                        },
                        on_cancel: move |_| show_delete_modal.set(false),
                    }
                }

                div {
                    class: "status-screen-container",
                    div {
                        class: "status-card",
                        i { 
                            class: "material-icons status-card-icon", 
                            style: "color: var(--status-red);", 
                            "error_outline" 
                        }
                        h1 { 
                            class: "text-2xl font-bold mb-4",
                            "Erro ao Carregar Gráficos" 
                        }
                        p {
                            class: "text-gray-600 mb-6",
                            "Não foi possível carregar os dados para o projeto: ",
                            strong { "{props.project_name}" }
                        }
                        
                        div {
                            class: "status-box error",
                            style: "text-align: left; background-color: var(--status-red-light); border-color: var(--status-red);",
                            p { class: "status-box-text", style: "color: var(--status-red-dark); font-weight: 500;", "Detalhes do Erro:"}
                            p { class: "status-box-text", style: "color: var(--status-red-dark);", "{error_message}"}
                        }

                        div {
                            class: "d-flex justify-center gap-4",
                            style: "margin-top: 2rem; width: 100%;",
                            button {
                                class: "btn btn-secondary",
                                onclick: move |_| { navigator.push(Route::HomePage {}); },
                                i { class: "material-icons", "home" }
                                "Voltar ao Início"
                            }
                            button {
                                class: "btn btn-danger",
                                onclick: move |_| show_delete_modal.set(true),
                                i { class: "material-icons", "delete_forever" }
                                "Excluir Projeto"
                            }
                        }
                    }
                }
            }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
struct EditProjectModalProps {
    project_name: String,
    on_close: EventHandler<()>,
    on_save: EventHandler<Option<String>>,
}

#[component]
fn EditProjectModal(props: EditProjectModalProps) -> Element {
    let project_name_clone = props.project_name.clone();

    let initial_metadata = use_resource(move || {
        let name_for_resource = project_name_clone.clone();
        async move {
            read_project_metadata(&name_for_resource)
        }
    });

    let mut name = use_signal(|| String::new());
    let mut description = use_signal(|| String::new());
    let mut year = use_signal(|| String::new());
    let mut leader = use_signal(|| String::new());
    let mut structure_type = use_signal(|| String::new());
    let mut observations = use_signal(|| String::new());
    let mut status_message = use_signal(String::new);
    // ADIÇÃO: Sinal para manter o status atual, embora não seja editável pelo usuário.
    let mut current_status = use_signal(ProjectStatus::default);

    
    use_effect(move || {
        if let Some(Ok(data)) = initial_metadata.read().as_ref() {
            name.set(data.name.clone());
            description.set(data.description.clone());
            year.set(data.year.clone());
            leader.set(data.leader.clone());
            structure_type.set(data.structure_type.clone());
            observations.set(data.observations.clone());
            current_status.set(data.status.clone()); // Armazena o status atual
        }
    });

    let handle_save = move |_| {
        let original_folder_name = props.project_name.clone();
        
        // MODIFICAÇÃO: Inclui o status atual ao salvar.
        let new_metadata = ProjectMetadata {
            name: name(),
            description: description(),
            year: year(),
            leader: leader(),
            structure_type: structure_type(),
            observations: observations(),
            status: current_status.read().clone(), // Re-salva o status que não foi alterado
        };

        let new_sanitized_name = sanitize_name(&new_metadata.name);
        
        if new_sanitized_name.is_empty() {
            status_message.set("O nome do projeto não pode ser vazio.".to_string());
            return;
        }

        let name_changed = new_sanitized_name != original_folder_name;

        spawn({
            let on_save = props.on_save.clone();
            async move {
                if let Some(projects_dir) = get_projects_dir() {
                    let original_path = projects_dir.join(&original_folder_name);
                    let new_path = projects_dir.join(&new_sanitized_name);

                    if name_changed {
                        if new_path.exists() {
                            status_message.set(format!("Erro: Já existe um projeto com o nome '{}'.", new_sanitized_name));
                            return;
                        }
                        if let Err(e) = fs::rename(&original_path, &new_path) {
                            status_message.set(format!("Erro ao renomear pasta: {}", e));
                            return;
                        }
                    }

                    if let Err(e) = save_project_metadata(&new_sanitized_name, &new_metadata) {
                        status_message.set(format!("Erro ao salvar metadados: {}", e));
                        if name_changed {
                            _ = fs::rename(&new_path, &original_path);
                        }
                        return;
                    }
                    
                    let new_name_opt = if name_changed { Some(new_sanitized_name) } else { None };
                    on_save.call(new_name_opt);
                }
            }
        });
    };


    rsx! {
        div { class: "modal-overlay",
            div { 
                class: "modal-content",
                style: "max-width: 700px;",
                div {
                    style: "position: absolute; top: 1rem; left: 1rem;",
                    button {
                        class: "btn btn-icon",
                        onclick: move |_| props.on_close.call(()),
                        i { class: "material-icons", "close" }
                    }
                }
                
                h2 { class: "text-2xl font-bold mb-6 text-center", "Editar Projeto" }

                match initial_metadata.read().as_ref() {
                    Some(Ok(_)) => rsx! {
                        div { 
                            class: "card-body", 
                            div { class: "form-group",
                                label { "Nome do Projeto" }
                                input { class: "form-input", r#type: "text", value: "{name()}", oninput: move |e| name.set(e.value()) }
                            }
                            div { class: "form-group",
                                label { "Descrição" }
                                textarea { class: "form-textarea", rows: "4", value: "{description()}", oninput: move |e| description.set(e.value()) }
                            }
                            div { class: "form-group",
                                label { "Líder responsável pelo projeto" }
                                input { class: "form-input", r#type: "text", value: "{leader()}", oninput: move |e| leader.set(e.value()) }
                            }
                            div { class: "form-group",
                                label { "Tipo de estrutura do edifício" }
                                input { class: "form-input", r#type: "text", value: "{structure_type()}", oninput: move |e| structure_type.set(e.value()) }
                            }
                             div { class: "form-group",
                                label { "Ano" }
                                input { class: "form-input", r#type: "number", value: "{year()}", oninput: move |e| year.set(e.value()) }
                            }
                             div { class: "form-group",
                                label { "Observações gerais" }
                                input { class: "form-input", r#type: "text", value: "{observations()}", oninput: move |e| observations.set(e.value()) }
                            }
                            
                            if !status_message().is_empty() {
                                p { class: "status-message error", "{status_message()}" }
                            }

                            button {
                                class: "btn btn-primary mt-4 w-full",
                                onclick: handle_save,
                                "Salvar Mudanças"
                            }
                        }
                    },
                    Some(Err(e)) => rsx! { p { "Erro ao carregar dados do projeto: {e:?}" } },
                    None => rsx! { p { "Carregando..." } }
                }
            }
        }
    }
}


#[derive(Props, Clone, PartialEq)]
struct DeleteConfirmationModalProps {
    project_name: String,
    on_confirm: EventHandler<()>,
    on_cancel: EventHandler<()>,
}

#[component]
fn DeleteConfirmationModal(props: DeleteConfirmationModalProps) -> Element {
    rsx! {
        div { class: "modal-overlay",
            div { class: "modal-content",
                i { class: "modal-icon material-icons", "warning_amber" }
                h2 { class: "text-2xl font-bold mb-2", "Confirmar Exclusão" }
                p {
                    class: "text-gray-600 mb-6",
                    "Você tem certeza que deseja excluir o projeto ",
                    strong { "{props.project_name}" }
                    "? Esta ação não pode ser desfeita."
                }
                div {
                    class: "d-flex justify-center gap-4",
                    button {
                        class: "btn btn-secondary",
                        onclick: move |_| props.on_cancel.call(()),
                        "Cancelar"
                    }
                    button {
                        class: "btn btn-danger",
                        onclick: move |_| props.on_confirm.call(()),
                        "Sim, Excluir"
                    }
                }
            }
        }
    }
}