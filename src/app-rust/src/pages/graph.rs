use dioxus::prelude::*;
use std::f64::consts::PI;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader};
use crate::Route;
use dioxus_router::prelude::*;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::path::PathBuf;

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
enum JsonReadError {
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

// --- Function to read and parse detection_results.json ---
fn ler_json_detection_results(project_name: &str) -> Result<Vec<ImageDetectionData>, JsonReadError> {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let json_path = base_dir
        .join("Projects")
        .join(project_name)
        .join("detection_results.json");

    println!("[RUST graph.rs] Base dir (CARGO_MANIFEST_DIR): {}", base_dir.display());
    println!("[RUST graph.rs] JSON path (relativo ao base_dir): {}", json_path.display());

    let file = File::open(&json_path).map_err(|e| {
        eprintln!("[RUST graph.rs] Erro ao abrir arquivo JSON em '{}': {}", json_path.display(), e);
        eprintln!("[RUST graph.rs] Verifique se o arquivo existe e se as permissões estão corretas.");
        JsonReadError::Io(e)
    })?;

    let reader = BufReader::new(file);
    let results: Vec<ImageDetectionData> = serde_json::from_reader(reader).map_err(|e| {
        eprintln!("[RUST graph.rs] Erro ao fazer parse do JSON em '{}': {}", json_path.display(), e);
        JsonReadError::Json(e)
    })?;
    Ok(results)
}

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
        r###"<path d="{path}" fill="url(#{color_id})" stroke="#121212" stroke-width="2" style="filter: drop-shadow(0px 2px 5px rgba(0,0,0,0.6)); opacity: 0;">
            <animate attributeName="opacity" from="0" to="1" dur="1s" fill="freeze" />
            <title>{label}</title>
        </path>"###
    )
}

fn gerar_svg_donut(total_termica: u32, total_retracao: u32) -> String {
    let total_fissuras = total_termica + total_retracao;
    if total_fissuras == 0 {
        return r##"<svg width="500" height="500" viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg">
                   <text x="250" y="250" font-size="20" text-anchor="middle" fill="#ffffff" dominant-baseline="middle">Sem dados para Donut</text>
                 </svg>"##.to_string();
    }
    let angle_termica = (total_termica as f64 / total_fissuras as f64) * 360.0;

    let cx = 250.0;
    let cy = 250.0;
    let raio_externo = 200.0;
    let raio_interno = 120.0;
    let label_termica = format!("Térmica: {}", total_termica);
    let label_retracao = format!("Retração: {}", total_retracao);

    let mut svg = String::from(r#"<svg width="500" height="500" viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg">"#);
    svg.push_str(r###"<defs>
            <linearGradient id="grad_red" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#ff5a5f; stop-opacity:1" />
                <stop offset="100%" style="stop-color:#d62828; stop-opacity:1" />
            </linearGradient>
            <linearGradient id="grad_blue" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#0077ff; stop-opacity:1" />
                <stop offset="100%" style="stop-color:#0055aa; stop-opacity:1" />
            </linearGradient>
        </defs>"###);
    svg.push_str(&donut_segment(cx, cy, raio_externo, 0.0, angle_termica, "grad_red", &label_termica));
    svg.push_str(&donut_segment(cx, cy, raio_externo, angle_termica, 360.0, "grad_blue", &label_retracao));
    svg.push_str(&format!(r###"<circle cx="{cx}" cy="{cy}" r="{raio_interno}" fill="#242526"/>"###));
    svg.push_str(&format!(r###"<text x="{cx}" y="{cy}" font-size="20" text-anchor="middle" fill="#ffffff" dominant-baseline="middle">Total Fissuras</text>"###));
    svg.push_str("</svg>");
    svg
}

// --- Bar Chart Helpers ---
fn gerar_svg_barras(building_summaries: &[BuildingFissuraSummary], media_total: f64) -> String {
    if building_summaries.is_empty() {
        return r##"<svg width="600" height="450" viewBox="0 0 600 450" xmlns="http://www.w3.org/2000/svg">
                   <text x="300" y="225" font-size="20" text-anchor="middle" fill="#ffffff" dominant-baseline="middle">Sem dados para Barras</text>
                 </svg>"##.to_string();
    }
    let altura_total = 450;
    let largura_barra = 30;
    let espacamento = 80;
    let margem_esquerda = 60;
    let largura_total_svg = margem_esquerda + building_summaries.len() as i32 * espacamento + 40; // Adiciona espaço à direita
    let max_count_val = building_summaries.iter()
        .map(|s| s.termica_count + s.retracao_count)
        .max()
        .unwrap_or(1)
        .max(media_total.ceil() as u32) as f64;
    let max_bar_height = 200.0;
    let y_base = 250.0;

    let mut svg = format!(r###"<svg width="{largura_total_svg}" height="{altura_total}" viewBox="0 0 {largura_total_svg} {altura_total}" xmlns="http://www.w3.org/2000/svg">"###);
    svg.push_str(r###"<defs>
            <linearGradient id="grad_red_bar" x1="0%" y1="0%" x2="0%" y2="100%"><stop offset="0%" style="stop-color:#ff5a5f; stop-opacity:1" /><stop offset="100%" style="stop-color:#d62828; stop-opacity:1" /></linearGradient>
            <linearGradient id="grad_blue_bar" x1="0%" y1="0%" x2="0%" y2="100%"><stop offset="0%" style="stop-color:#0077ff; stop-opacity:1" /><stop offset="100%" style="stop-color:#0055aa; stop-opacity:1" /></linearGradient>
        </defs>"###);

    // --- DESENHA A LINHA DA MÉDIA ---
    if media_total > 0.0 && max_count_val > 0.0 {
        let y_media = y_base - (media_total / max_count_val * max_bar_height);
        svg.push_str(&format!(
            r###"<line x1="{margem_esquerda}" y1="{y_media}" x2="{x2}" y2="{y_media}" stroke="#f0f0f0" stroke-width="2" stroke-dasharray="5,5" stroke-opacity="0.8">
                <title>Média por Edifício: {media_total:.2}</title>
            </line>"###,
            x2 = largura_total_svg - 40
        ));
        svg.push_str(&format!(
            r###"<text x="{x}" y="{y}" font-size="10" fill="#f0f0f0" text-anchor="end" dominant-baseline="middle">Média ({media_total:.1})</text>"###,
            x = largura_total_svg - 45,
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
        
        // Barra de Retração (azul, embaixo)
        svg.push_str(&format!(r###"<rect x="{x}" y="{y}" width="{w}" height="0" fill="url(#grad_blue_bar)" rx="3"><animate attributeName="height" from="0" to="{h}" dur="0.8s" fill="freeze" /><animate attributeName="y" from="{y_plus}" to="{y}" dur="0.8s" fill="freeze" /><title>Retração: {val}</title></rect>"###, x = x_base, y = y_base - h_retracao, y_plus = y_base, w = largura_barra, h = h_retracao, val = summary.retracao_count));

        // Barra de Térmica (vermelha, em cima)
        svg.push_str(&format!(r###"<rect x="{x}" y="{y}" width="{w}" height="0" fill="url(#grad_red_bar)" rx="3"><animate attributeName="height" from="0" to="{h}" dur="0.8s" fill="freeze" /><animate attributeName="y" from="{y_plus}" to="{y}" dur="0.8s" fill="freeze" /><title>Térmica: {val}</title></rect>"###, x = x_base, y = y_base - h_retracao - h_termica, y_plus = y_base - h_retracao, w = largura_barra, h = h_termica, val = summary.termica_count));
        
        // Rótulo do edifício
        svg.push_str(&format!(r###"<text x="{x_text}" y="270" font-size="10" text-anchor="middle" fill="#f0f0f0">{name}</text>"###, x_text = x_base + largura_barra / 2, name = summary.building_name));
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
        return r##"<svg width="800" height="400" viewBox="0 0 800 400" xmlns="http://www.w3.org/2000/svg"><text x="400" y="200" font-size="20" text-anchor="middle" fill="#ffffff" dominant-baseline="middle">Sem dados para Box Plot</text></svg>"##.to_string();
    }
    
    let width = 800;
    let height = 400;
    let margin = (50.0, 50.0, 50.0, 50.0); // top, right, bottom, left
    let plot_height = height as f64 - margin.0 - margin.2;

    let mut svg = format!(r###"<svg width="{width}" height="{height}" viewBox="0 0 {width} {height}" xmlns="http://www.w3.org/2000/svg">"###);

    // Draw Y-axis and labels
    svg.push_str(r###"<line x1="50" y1="50" x2="50" y2="350" stroke="#f0f0f0" stroke-width="1"/>"###);
    for i in 0..=10 {
        let y = 350.0 - (i as f64 * (plot_height / 10.0));
        let label = format!("{:.1}", i as f64 / 10.0);
        svg.push_str(&format!(r###"<text x="40" y="{y}" font-size="10" text-anchor="end" fill="#f0f0f0" dominant-baseline="middle">{label}</text>"###));
        svg.push_str(&format!(r###"<line x1="50" y1="{y}" x2="750" y2="{y}" stroke="#444" stroke-width="0.5" stroke-dasharray="2,2"/>"###));
    }
    svg.push_str(r###"<text x="10" y="200" font-size="12" fill="#f0f0f0" transform="rotate(-90 15,200)">Confiança</text>"###);

    // Helper to draw one box plot
    let draw_box = |stats: &BoxPlotStats, x_center: f64, color: &str| -> String {
        let mut parts = String::new();
        let scale = |v: f64| 350.0 - v * plot_height;

        let y_q1 = scale(stats.q1);
        let y_q3 = scale(stats.q3);
        let y_median = scale(stats.median);
        let y_min_w = scale(stats.min_whisker);
        let y_max_w = scale(stats.max_whisker);
        let box_width = 100.0;

        // Whiskers
        parts.push_str(&format!(r###"<line x1="{x_center}" y1="{y_max_w}" x2="{x_center}" y2="{y_q3}" stroke="{color}" stroke-width="2"/>"###));
        parts.push_str(&format!(r###"<line x1="{x_center}" y1="{y_q1}" x2="{x_center}" y2="{y_min_w}" stroke="{color}" stroke-width="2"/>"###));
        parts.push_str(&format!(r###"<line x1="{x_center_min}" y1="{y_max_w}" x2="{x_center_plus}" y2="{y_max_w}" stroke="{color}" stroke-width="2"/>"###, x_center_min = x_center - 20.0, x_center_plus = x_center + 20.0));
        parts.push_str(&format!(r###"<line x1="{x_center_min}" y1="{y_min_w}" x2="{x_center_plus}" y2="{y_min_w}" stroke="{color}" stroke-width="2"/>"###, x_center_min = x_center - 20.0, x_center_plus = x_center + 20.0));

        // Box
        parts.push_str(&format!(r###"<rect x="{x}" y="{y}" width="{w}" height="{h}" fill="{color}" fill-opacity="0.3" stroke="{color}" stroke-width="2"/>"###, x = x_center - box_width / 2.0, y = y_q3, w = box_width, h = y_q1 - y_q3));
        
        // Median line
        parts.push_str(&format!(r###"<line x1="{x_start}" y1="{y_median}" x2="{x_end}" y2="{y_median}" stroke="{color}" stroke-width="3"/>"###, x_start = x_center - box_width / 2.0, x_end = x_center + box_width / 2.0));
        
        // Outliers
        for &outlier in &stats.outliers {
            let y_outlier = scale(outlier);
            parts.push_str(&format!(r###"<circle cx="{x_center}" cy="{y_outlier}" r="3" fill="none" stroke="{color}" stroke-width="1.5"/>"###));
        }
        parts
    };

    // Draw Térmica box plot
    if let Some(stats) = stats_termica {
        svg.push_str(&draw_box(stats, 250.0, "#ff5a5f"));
        svg.push_str(r###"<text x="250" y="370" font-size="14" text-anchor="middle" fill="#ff5a5f">Térmica</text>"###);
    }

    // Draw Retração box plot
    if let Some(stats) = stats_retracao {
        svg.push_str(&draw_box(stats, 550.0, "#0077ff"));
        svg.push_str(r###"<text x="550" y="370" font-size="14" text-anchor="middle" fill="#0077ff">Retração</text>"###);
    }
    
    svg.push_str("</svg>");
    svg
}

// --- NEW: Heatmap Helpers ---
fn gerar_svg_heatmap(heatmap_data: &HeatmapData) -> String {
    if heatmap_data.is_empty() {
        return r##"<svg width="800" height="400" viewBox="0 0 800 400" xmlns="http://www.w3.org/2000/svg">
                   <text x="400" y="200" font-size="20" text-anchor="middle" fill="#ffffff" dominant-baseline="middle">Sem dados para Heatmap</text>
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
    let svg_width = x_offset + (facade_names.len() as f64 * cell_size) + 100.0; // +100 for legend
    let svg_height = y_offset + (building_names.len() as f64 * cell_size);

    let mut svg = format!(r###"<svg width="{svg_width}" height="{svg_height}" viewBox="0 0 {svg_width} {svg_height}" xmlns="http://www.w3.org/2000/svg">"###);

    // Color interpolation function
    let get_color = |count: u32| {
        if count == 0 { return "#4a4a4a".to_string(); }
        let intensity = count as f64 / max_val;
        // Interpolate from light yellow (#ffffcc) to dark red (#b30000)
        let r = (255.0 + (179.0 - 255.0) * intensity).round() as u8;
        let g = (255.0 + (0.0 - 255.0) * intensity).round() as u8;
        let b = (204.0 + (0.0 - 204.0) * intensity).round() as u8;
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    };
    
    // Draw cells and Y-axis labels
    for (row, building) in building_names.iter().enumerate() {
        let y = y_offset + (row as f64 * cell_size);
        svg.push_str(&format!(r###"<text x="{x}" y="{y_center}" font-size="14" text-anchor="end" fill="#f0f0f0" dominant-baseline="middle">{building}</text>"###, x = x_offset - 10.0, y_center = y + cell_size / 2.0));

        for (col, facade) in facade_names.iter().enumerate() {
            let count = heatmap_data.get(building).and_then(|f| f.get(facade)).cloned().unwrap_or(0);
            let color = get_color(count);
            let x = x_offset + (col as f64 * cell_size);
            
            svg.push_str(&format!(
                r###"<rect x="{x}" y="{y}" width="{size}" height="{size}" fill="{color}" stroke="#242526" stroke-width="1">
                    <title>Edifício: {building}\nFachada: {facade}\nFissuras: {count}</title>
                    <animate attributeName="opacity" from="0" to="1" dur="1.5s" fill="freeze" />
                </rect>"###,
                x=x, y=y, size=cell_size, color=color, building=building, facade=facade, count=count
            ));
        }
    }

    // Draw X-axis labels
    for (col, facade) in facade_names.iter().enumerate() {
        let x = x_offset + (col as f64 * cell_size) + (cell_size / 2.0);
        svg.push_str(&format!(r###"<text x="{x}" y="{y}" font-size="14" text-anchor="end" fill="#f0f0f0" transform="rotate(-45, {x}, {y})">{facade}</text>"###, x = x, y = y_offset - 10.0));
    }
    
    // Draw Legend
    let legend_x = svg_width - 80.0;
    svg.push_str(&format!(r###"<text x="{legend_x}" y="30" font-size="12" fill="#f0f0f0">Criticidade</text>"###));
    for i in 0..=10 {
        let ratio = i as f64 / 10.0;
        let count = (ratio * max_val).round() as u32;
        let color = get_color(count);
        let y = 50.0 + ratio * 150.0;
        svg.push_str(&format!(r###"<rect x="{legend_x}" y="{y}" width="20" height="15" fill="{color}"/>"###));
        svg.push_str(&format!(r###"<text x="{x}" y="{y_text}" font-size="10" fill="#f0f0f0" dominant-baseline="middle">{count}</text>"###, x = legend_x + 25.0, y_text= y + 7.5));
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
    match ler_json_detection_results(&props.project_name) {
        Ok(detection_data) => {
            let mut total_termica_overall = 0u32;
            let mut total_retracao_overall = 0u32;
            let mut building_fissura_map: HashMap<String, BuildingFissuraSummary> = HashMap::new();
            
            // Data collection for Box Plot
            let mut confidences_termica: Vec<f64> = Vec::new();
            let mut confidences_retracao: Vec<f64> = Vec::new();
            
            // --- NEW: Data collection for Heatmap ---
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
                
                // --- MODIFIED: Aggregate data for all charts ---
                if let Some(building_name) = extract_building_name_from_path(&item_data.path) {
                    // Bar chart data
                    let summary = building_fissura_map.entry(building_name.clone()).or_insert_with(|| BuildingFissuraSummary {
                        building_name: building_name.clone(),
                        termica_count: 0,
                        retracao_count: 0,
                    });
                    summary.termica_count += current_image_termica;
                    summary.retracao_count += current_image_retracao;
                    
                    // Heatmap data
                    if let Some(facade_name) = extract_facade_name_from_path(&item_data.path) {
                        let total_fissures_in_image = item_data.fissura.len() as u32;
                        let building_entry = heatmap_data.entry(building_name.clone()).or_default();
                        let facade_entry = building_entry.entry(facade_name).or_insert(0);
                        *facade_entry += total_fissures_in_image;
                    }
                }
            }
            
            let mut building_summaries: Vec<BuildingFissuraSummary> = building_fissura_map.values().cloned().collect();
            // Ordenar para consistência
            building_summaries.sort_by(|a, b| a.building_name.cmp(&b.building_name));


            // --- CÁLCULO DA MÉDIA PARA O GRÁFICO DE BARRAS ---
            let media_fissuras_por_edificio = if !building_summaries.is_empty() {
                let total_fissuras: u32 = building_summaries.iter().map(|s| s.termica_count + s.retracao_count).sum();
                total_fissuras as f64 / building_summaries.len() as f64
            } else {
                0.0
            };

            let donut_svg = gerar_svg_donut(total_termica_overall, total_retracao_overall);
            // Passa a média para a função de geração do SVG
            let barras_svg = gerar_svg_barras(&building_summaries, media_fissuras_por_edificio);
            
            // Calculate stats and generate Box Plot SVG
            let stats_termica = calculate_boxplot_stats(&mut confidences_termica);
            let stats_retracao = calculate_boxplot_stats(&mut confidences_retracao);
            let boxplot_svg = gerar_svg_boxplot(&stats_termica, &stats_retracao);

            // --- NEW: Generate Heatmap SVG ---
            let heatmap_svg = gerar_svg_heatmap(&heatmap_data);

            let navigator = use_navigator();

            rsx! {
                div {
                    style: "
                        background-color: #242526;
                        color: #f0f0f0;
                        font-family: 'Segoe UI', sans-serif;
                        min-height: 100vh;
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        padding: 40px;
                        gap: 40px; // Add gap between sections
                        position: relative;
                    ",

                    button {
                        onclick: move |_| {
                            navigator.push(Route::HomePage {});  
                        },
                        style: "
                            position: absolute;
                            top: 20px;
                            left: 20px;
                            background-color: #ff5a5f;
                            color: white;
                            border: none;
                            padding: 10px 16px;
                            border-radius: 6px;
                            cursor: pointer;
                            font-size: 14px;
                        ",
                        "← Início"
                    }

                    div {
                        style: "width: 100%; max-width: 1400px; text-align: center;",
                        h1 {
                            style: "font-size: 32px; color: #ff5a5f; margin-bottom: 20px;",
                            "Gráficos das Fissuras (Projeto: {props.project_name})"
                        }
                    }

                    div {
                        style: "
                            display: flex;
                            justify-content: space-around;
                            align-items: flex-start;
                            gap: 40px;
                            flex-wrap: wrap;
                            width: 100%;
                            max-width: 1400px;
                        ",

                        div {
                            style: "flex: 1; min-width: 400px; text-align: center; background: #3a3b3c; padding: 20px; border-radius: 8px;",
                            h2 { style: "font-size: 24px; color: #ffffff; margin-top:0;", "Distribuição Total" }
                            div { dangerous_inner_html: donut_svg }
                            p { style: "margin-top: 10px; font-size: 14px;", "Térmicas: {total_termica_overall} | Retração: {total_retracao_overall}" }
                            div {
                                style: "margin-top: 10px; font-size: 14px;",
                                span { style: "color: #ff5a5f; margin-right: 10px;", "⬤ Térmica" }
                                span { style: "color: #0077ff;", "⬤ Retração" }
                            }
                        }

                        div {
                            style: "flex: 2; min-width: 600px; position: relative; background: #3a3b3c; padding: 20px; border-radius: 8px;",
                            h2 { style: "font-size: 24px; color: #ffffff; margin-top:0;", "Fissuras por Edifício" }
                            div {
                                style: "margin-top: 10px; overflow-x: auto;",
                                div { dangerous_inner_html: barras_svg }
                            }
                            button {
                                onclick: move |_| {
                                    let building_name = "Galpão_3".to_string(); 
                                    navigator.push(Route::ReportView { project_name: props.project_name.clone(), building_name });
                                },
                                style: "
                                    position: absolute;
                                    bottom: -50px;
                                    right: 20px;
                                    background-color: #0077ff;
                                    color: white;
                                    border: none;
                                    padding: 10px 16px;
                                    border-radius: 6px;
                                    cursor: pointer;
                                    font-size: 16px;
                                ",
                                "Visualizar relatório detalhado →"
                            }
                        }
                    }
                    
                    // --- NEW: Heatmap Section ---
                    div {
                        style: "
                            width: 100%;
                            max-width: 1400px;
                            background: #3a3b3c; padding: 20px; border-radius: 8px;
                            text-align: center;
                        ",
                        h2 { style: "font-size: 24px; color: #ffffff; margin-bottom: 20px; margin-top:0;", "Heatmap de Criticidade por Fachada" },
                        div { 
                            style: "overflow-x: auto;",
                            dangerous_inner_html: heatmap_svg 
                        }
                    }

                    // Box Plot Section
                    div {
                        style: "
                            width: 100%;
                            max-width: 1400px;
                            background: #3a3b3c; padding: 20px; border-radius: 8px;
                            text-align: center;
                        ",
                        h2 { style: "font-size: 24px; color: #ffffff; margin-bottom: 20px; margin-top:0;", "Distribuição de Confiança das Detecções" }
                        div { dangerous_inner_html: boxplot_svg }
                    }
                }
            }
        }
        Err(e) => {
            let error_message = match e {
                JsonReadError::Io(io_err) => format!("Erro de I/O ao ler arquivo JSON: {}. Verifique o caminho e permissões.", io_err),
                JsonReadError::Json(json_err) => format!("Erro ao parsear JSON: {}. Verifique o formato do arquivo.", json_err),
                JsonReadError::PathError(path_err) => format!("Erro no caminho do arquivo: {}", path_err),
            };
            rsx! {
                div {
                    style: "padding: 20px; color: red; text-align: center; font-family: 'Segoe UI', sans-serif; background-color: #242526; min-height: 100vh; display: flex; flex-direction: column; justify-content: center; align-items: center;",
                    h1 { "Erro ao carregar dados para o gráfico" },
                    p { "{error_message}" },
                    p { "Verifique se o arquivo 'Projects/{props.project_name}/detection_results.json' existe e está no formato correto."}
                }
            }
        }
    }
}