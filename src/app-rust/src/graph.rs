use dioxus::prelude::*;
use std::f64::consts::PI;
use serde::Deserialize;
use std::fs::File;
use std::io::{self, BufReader};
use crate::Route;
use dioxus_router::prelude::*;
use std::collections::HashMap;
use std::path::Path;
use std::env; // For checking Current Working Directory

// --- Structs for parsing detection_results.json ---
#[derive(Deserialize, Debug, Clone)]
struct DetectionFissura {
    name: String,
    confidence: f64, // Confidence is f64 in the JSON
}

#[derive(Deserialize, Debug, Clone)]
struct ImageDetectionData {
    path: String,
    fissura: Vec<DetectionFissura>,
}
// Type alias for the root JSON structure (a list of ImageDetectionData)
// type DetectionResults = Vec<ImageDetectionData>; // This will be the return type of our JSON reader

// --- Structs for aggregated data for bar chart ---
#[derive(Debug, Clone)]
struct BuildingFissuraSummary {
    building_name: String,
    termica_count: u32,
    retracao_count: u32,
}

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
    // --- BEGIN CWD DEBUG ---
    let cwd_string = match env::current_dir() {
        Ok(cwd) => {
            let s = cwd.display().to_string();
            println!("[RUST graph.rs] Current Working Directory: {}", s);
            s
        }
        Err(e) => {
            eprintln!("[RUST graph.rs] Failed to get Current Working Directory: {}", e);
            return Err(JsonReadError::PathError("Failed to get CWD".to_string()));
        }
    };
    // --- END CWD DEBUG ---

    // Construct path relative to the CWD printed above.
    // If CWD is ".../src/app-rust/src", then "../Projects/..." should be correct
    // if the target is ".../src/app-rust/Projects/..."
    let json_path_str = format!("../Projects/{}/detection_results.json", project_name);
    
    // Resolve this potentially relative path against the CWD to get an absolute path for File::open
    // std::fs::canonicalize can also be used for more robust absolute path resolution.
    let absolute_json_path = Path::new(&cwd_string).join(&json_path_str);
    // It's often better to canonicalize to resolve symlinks, .., etc., fully.
    let canonical_path = match absolute_json_path.canonicalize() {
        Ok(p) => p,
        Err(e) => {
            // If canonicalize fails, it often means the path doesn't exist or isn't accessible at some point.
            // Fallback to the joined path for the error message, but the open will likely fail.
            eprintln!("[RUST graph.rs] Error canonicalizing path '{:?}': {}. Using non-canonical path for open attempt.", absolute_json_path, e);
            // return Err(JsonReadError::PathError(format!("Error canonicalizing path {:?}: {}", absolute_json_path, e)));
            // For the purpose of File::open, the non-canonicalized absolute_json_path might still be what we need if canonicalize fails due to non-existence
             absolute_json_path.clone() // Use the joined path if canonicalization fails (e.g. because file doesn't exist yet for canonicalize to work on the final component)
        }
    };


    println!("[RUST graph.rs] Constructed relative path (from CWD): {}", json_path_str);
    println!("[RUST graph.rs] Resolved absolute path for open: {:?}", absolute_json_path); // Path before canonicalization
    println!("[RUST graph.rs] Canonical path (if successful): {:?}", canonical_path); // Path after canonicalization, might differ
    
    // Try opening with the path that seems most likely to be what File::open expects, which is the directly joined one. 
    // Canonicalize is good for verifying, but File::open takes the direct path.
    let file = File::open(&absolute_json_path).map_err(|e| { 
        eprintln!("[RUST graph.rs] Error opening JSON file at '{:?}': {}", absolute_json_path, e);
        eprintln!("[RUST graph.rs] Verifique se o arquivo 'detection_results.json' existe no diretório do projeto");
        JsonReadError::Io(e)
    })?;
    let reader = BufReader::new(file);
    let results: Vec<ImageDetectionData> = serde_json::from_reader(reader).map_err(|e| {
        eprintln!("[RUST graph.rs] Error parsing JSON from '{:?}': {}", absolute_json_path, e);
        JsonReadError::Json(e)
    })?;
    Ok(results)
}

//  Helpers do gráfico do Donut 

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

    let mut svg = String::new();
    svg.push_str(r#"<svg width="500" height="500" viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg">"#);

    svg.push_str(
        r###"
        <defs>
            <linearGradient id="grad_red" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#ff5a5f; stop-opacity:1" />
                <stop offset="100%" style="stop-color:#d62828; stop-opacity:1" />
            </linearGradient>
            <linearGradient id="grad_blue" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#0077ff; stop-opacity:1" />
                <stop offset="100%" style="stop-color:#0055aa; stop-opacity:1" />
            </linearGradient>
        </defs>
        "###
    );

    svg.push_str(&donut_segment(cx, cy, raio_externo, 0.0, angle_termica, "grad_red", &label_termica));
    svg.push_str(&donut_segment(cx, cy, raio_externo, angle_termica, 360.0, "grad_blue", &label_retracao));

    svg.push_str(&format!(
        r###"<circle cx="{cx}" cy="{cy}" r="{r}" fill="#242526"/>"###,
        cx = cx,
        cy = cy,
        r = raio_interno
    ));

    svg.push_str(&format!(
        r###"<text x="{cx}" y="{cy}" font-size="20" text-anchor="middle" fill="#ffffff" dominant-baseline="middle">Total Fissuras</text>"###,
        cx = cx,
        cy = cy
    ));

    svg.push_str("</svg>");
    svg
}

// Helper to extract building name like "Prédio X" from path
// Assumes path structure like ".../images/Prédio X/Fachada Y/image.jpg"
fn extract_building_name_from_path(image_path_str: &str) -> Option<String> {
    let image_path = Path::new(image_path_str);
    // Go up from file: Fachada Y -> Prédio X -> images
    image_path.parent()?.parent()?.file_name()?.to_str().map(String::from)
}

//  Gráfico de Barras 

fn gerar_svg_barras(building_summaries: &[BuildingFissuraSummary]) -> String {
    if building_summaries.is_empty() {
        return r##"<svg width="600" height="450" viewBox="0 0 600 450" xmlns="http://www.w3.org/2000/svg">
                   <text x="300" y="225" font-size="20" text-anchor="middle" fill="#ffffff" dominant-baseline="middle">Sem dados para Barras</text>
                 </svg>"##.to_string();
    }
    let altura_total = 450;
    let largura_barra = 30;
    let espacamento = 80;
    let largura_total_svg = 60 + building_summaries.len() as i32 * espacamento;

    let max_count_val = building_summaries.iter()
        .flat_map(|s| vec![s.termica_count, s.retracao_count])
        .max()
        .unwrap_or(1) as f64;
    let max_bar_height = 200.0; // Max height for a bar

    let mut svg = format!(
        r###"<svg width="{largura_total_svg}" height="{altura_total}" viewBox="0 0 {largura_total_svg} {altura_total}" xmlns="http://www.w3.org/2000/svg">"###
    );

    svg.push_str(
        r###"
        <defs>
            <linearGradient id="grad_red" x1="0%" y1="0%" x2="0%" y2="100%">
                <stop offset="0%" style="stop-color:#ff5a5f; stop-opacity:1" />
                <stop offset="100%" style="stop-color:#d62828; stop-opacity:1" />
            </linearGradient>
            <linearGradient id="grad_blue" x1="0%" y1="0%" x2="0%" y2="100%">
                <stop offset="0%" style="stop-color:#0077ff; stop-opacity:1" />
                <stop offset="100%" style="stop-color:#0055aa; stop-opacity:1" />
            </linearGradient>
        </defs>
        "###
    );

    for (i, summary) in building_summaries.iter().enumerate() {
        let x_base = 60 + i as i32 * espacamento;
        let h_termica = if max_count_val == 0.0 { 0.0 } else { (summary.termica_count as f64 / max_count_val * max_bar_height) };
        let h_retracao = if max_count_val == 0.0 { 0.0 } else { (summary.retracao_count as f64 / max_count_val * max_bar_height) };

        // Térmica bar
        svg.push_str(&format!(
            r###"<rect x="{x}" y="{y}" width="{w}" height="0" fill="url(#grad_red)" rx="3">
                    <animate attributeName="height" from="0" to="{h}" dur="0.8s" fill="freeze" />
                    <animate attributeName="y" from="{y_plus}" to="{y}" dur="0.8s" fill="freeze" />
                    <title>Térmica: {val}</title>
                </rect>"###,
            x = x_base,
            y = 250.0 - h_termica,
            y_plus = 250.0,
            w = largura_barra,
            h = h_termica,
            val = summary.termica_count
        ));

        // Retração bar
        svg.push_str(&format!(
            r###"<rect x="{x}" y="{y}" width="{w}" height="0" fill="url(#grad_blue)" rx="3">
                    <animate attributeName="height" from="0" to="{h}" dur="0.8s" fill="freeze" />
                    <animate attributeName="y" from="{y_plus}" to="{y}" dur="0.8s" fill="freeze" />
                    <title>Retração: {val}</title>
                </rect>"###,
            x = x_base + largura_barra + 2, // Small gap between bars
            y = 250.0 - h_retracao,
            y_plus = 250.0,
            w = largura_barra,
            h = h_retracao,
            val = summary.retracao_count
        ));

        // Building name label
        svg.push_str(&format!(
            r###"<text x="{x_text}" y="270" font-size="10" text-anchor="middle" fill="#f0f0f0">{name}</text>"###,
            x_text = x_base + largura_barra + 1, // Centered under the pair of bars
            name = summary.building_name
        ));
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

            for item_data in detection_data {
                let mut current_image_termica = 0u32;
                let mut current_image_retracao = 0u32;
                for fissura_item in item_data.fissura {
                    if fissura_item.name.to_lowercase() == "termica" {
                        total_termica_overall += 1;
                        current_image_termica +=1;
                    } else if fissura_item.name.to_lowercase() == "retracao" || fissura_item.name.to_lowercase() == "retraçao" {
                        total_retracao_overall += 1;
                        current_image_retracao +=1;
                    }
                }
                
                // Aggregate for bar chart by building
                if let Some(building_name) = extract_building_name_from_path(&item_data.path) {
                    let summary = building_fissura_map.entry(building_name.clone()).or_insert_with(|| BuildingFissuraSummary {
                        building_name,
                        termica_count: 0,
                        retracao_count: 0,
                    });
                    summary.termica_count += current_image_termica;
                    summary.retracao_count += current_image_retracao;
                }
            }
            
            let building_summaries: Vec<BuildingFissuraSummary> = building_fissura_map.values().cloned().collect();

            let donut_svg = gerar_svg_donut(total_termica_overall, total_retracao_overall);
            let barras_svg = gerar_svg_barras(&building_summaries);
            let navigator = use_navigator();

            rsx! {
                div {
                    style: "
                        background-color: #242526;
                        color: #f0f0f0;
                        font-family: 'Segoe UI', sans-serif;
                        min-height: 100vh;
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        padding: 40px;
                        position: relative;
                    ",

                    // Botão home
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
                        style: "
                            display: flex;
                            justify-content: space-between;
                            align-items: flex-start;
                            gap: 40px;
                            flex-wrap: wrap;
                            max-width: 1400px;
                        ",

                        h1 {
                            style: "font-size: 32px; color: #ff5a5f; margin-bottom: 10px; text-align: center; width: 100%;",
                            "Gráficos das Fissuras (Projeto: {props.project_name})"
                        }

                        // gráfico Donut
                        div {
                            style: "flex: 1; min-width: 400px; text-align: center;",
                            h2 { style: "font-size: 24px; color: #ffffff;", "Distribuição Total de Fissuras" }
                            div { dangerous_inner_html: donut_svg }
                            p { style: "margin-top: 10px; font-size: 14px;", "Térmicas: {total_termica_overall} | Retração: {total_retracao_overall}" }
                            div {
                                style: "margin-top: 10px; font-size: 14px;",
                                span { style: "color: #ff5a5f; margin-right: 10px;", "⬤ Térmica" }
                                span { style: "color: #0077ff;", "⬤ Retração" }
                            }
                        }

                        // Barras e  botão 
                        div {
                            style: "flex: 1; min-width: 600px; position: relative;",
                            h2 { style: "font-size: 24px; color: #ffffff;", "Fissuras por Edifício" }
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
                                    bottom: 10px;
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
