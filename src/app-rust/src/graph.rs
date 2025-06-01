use dioxus::prelude::*;
use std::f64::consts::PI;
use std::fs::File;
use std::io::BufReader;
use crate::Route;
use dioxus_router::prelude::*;
use crate::report_structures::{ReportData, Fissura, Faceta};
use serde_json;

// Function to read the JSON report
fn ler_json_report(base_projects_dir: &str, project_name: &str, building_name: &str) -> Result<ReportData, Box<dyn std::error::Error>> {
    let report_file_name = format!("Dados-{}-{}.json", project_name, building_name);
    let report_file_path = std::path::Path::new(base_projects_dir)
        .join(project_name)
        .join(&report_file_name);

    println!("Attempting to read report from: {:?}", report_file_path); // For debugging

    if !report_file_path.exists() {
        // Before failing, attempt to generate it if it doesn't exist.
        // This is a crucial part of the flow.
        println!("Report not found, attempting to generate: {} for building {}", project_name, building_name);
        match crate::report_generator::generate_json_report(base_projects_dir, project_name, building_name) {
            Ok(generated_data) => {
                println!("Report generated successfully on demand.");
                return Ok(generated_data);
            }
            Err(e) => {
                eprintln!("Error generating report on demand: {}", e);
                return Err(format!("Failed to generate report {}: {}", report_file_name, e).into());
            }
        }
    }

    let file = File::open(report_file_path)?;
    let reader = BufReader::new(file);
    let report_data: ReportData = serde_json::from_reader(reader)?;
    Ok(report_data)
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

fn gerar_svg_donut(fissuras: &[Fissura]) -> String {
    let mut counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
    for fissura in fissuras {
        *counts.entry(fissura.classificacao.clone()).or_insert(0) += 1;
    }

    let total_fissuras: u32 = counts.values().sum();
    if total_fissuras == 0 {
        // Handle case with no fissuras to avoid division by zero
        return r#"<svg width="500" height="500" viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg">
                   <text x="250" y="250" font-size="20" text-anchor="middle" fill="#ffffff" dominant-baseline="middle">Nenhuma fissura encontrada</text>
                 </svg>"#.to_string();
    }

    let cx = 250.0;
    let cy = 250.0;
    let raio_externo = 200.0;
    let raio_interno = 120.0;

    // Define colors for classifications - can be expanded
    let colors = vec![\"grad_red\", \"grad_blue\", \"grad_green\", \"grad_yellow\", \"grad_purple\"];
    let mut color_idx = 0;

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
            <linearGradient id="grad_green" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#50C878; stop-opacity:1" />
                <stop offset="100%" style="stop-color:#38761D; stop-opacity:1" />
            </linearGradient>
            <linearGradient id="grad_yellow" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#FFD700; stop-opacity:1" />
                <stop offset="100%" style="stop-color:#B8860B; stop-opacity:1" />
            </linearGradient>
            <linearGradient id="grad_purple" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" style="stop-color:#9370DB; stop-opacity:1" />
                <stop offset="100%" style="stop-color:#4B0082; stop-opacity:1" />
            </linearGradient>
        </defs>
        "###
    );

    let mut current_angle = 0.0;
    for (classificacao, count) in &counts {
        let angle = (*count as f64 / total_fissuras as f64) * 360.0;
        let color_id = colors[color_idx % colors.len()];
        color_idx += 1;
        let label = format!("{}: {}", classificacao, count);
        svg.push_str(&donut_segment(cx, cy, raio_externo, current_angle, current_angle + angle, color_id, &label));
        current_angle += angle;
    }

    svg.push_str(&format!(
        r###"<circle cx="{cx}" cy="{cy}" r="{r}" fill="#242526"/>"###,
        cx = cx,
        cy = cy,
        r = raio_interno
    ));
    svg.push_str(&format!(
        r###"<text x="{cx}" y="{cy}" font-size="20" text-anchor="middle" fill="#ffffff" dominant-baseline="middle">Total: {total_fissuras}</text>"###,
        cx = cx,
        cy = cy
    ));
    svg.push_str("</svg>");
    svg
}

// Modified gerar_svg_barras to use Faceta data
fn gerar_svg_barras(facetas: &[Faceta]) -> String {
    if facetas.is_empty() {
        return r#"<svg width="600" height="450" viewBox="0 0 600 450" xmlns="http://www.w3.org/2000/svg">
                   <text x="300" y="225" font-size="20" text-anchor="middle" fill="#ffffff" dominant-baseline="middle">Nenhuma faceta para exibir</text>
                 </svg>"#.to_string();
    }
    let altura_total = 450;
    let largura_barra_grupo = 60; // Increased width for group
    let espacamento = 80;
    let largura_total_svg = 60 + facetas.len() as i32 * espacamento;

    let max_valor = facetas.iter()
        .map(|f| f.qtd_rachaduras)
        .max()
        .unwrap_or(1) as f64;
    
    let max_bar_height = 200.0; // Max height for a bar

    let mut svg = format!(
        r###"<svg width="{largura_total_svg}" height="{altura_total}" viewBox="0 0 {largura_total_svg} {altura_total}" xmlns="http://www.w3.org/2000/svg">"###
    );

    svg.push_str(
        r###"
        <defs>
            <linearGradient id="grad_orange" x1="0%" y1="0%" x2="0%" y2="100%">
                <stop offset="0%" style="stop-color:#FFA500; stop-opacity:1" /> <!-- Orange -->
                <stop offset="100%" style="stop-color:#FF8C00; stop-opacity:1" /> <!-- DarkOrange -->
            </linearGradient>
             <linearGradient id="grad_teal" x1="0%" y1="0%" x2="0%" y2="100%">
                <stop offset="0%" style="stop-color:#20B2AA; stop-opacity:1" /> <!-- LightSeaGreen -->
                <stop offset="100%" style="stop-color:#008080; stop-opacity:1" /> <!-- Teal -->
            </linearGradient>
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
    // Axis line
    svg.push_str(&format!(
        r###"<line x1="50" y1="250" x2="{}" y2="250" stroke="#f0f0f0" stroke-width="1"/>"###,
        largura_total_svg - 10
    ));


    for (i, faceta) in facetas.iter().enumerate() {
        let x_base = 60 + i as i32 * espacamento;
        let h_rachaduras = if max_valor == 0.0 { 0.0 } else { (faceta.qtd_rachaduras as f64 / max_valor * max_bar_height) };
        
        let bar_color = match faceta.orientacao.to_lowercase().as_str() {
            "norte" => "grad_red",
            "sul" => "grad_blue",
            "leste" => "grad_orange",
            "oeste" => "grad_teal",
            _ => "grad_grey", // Default or add more
        };

        svg.push_str(&format!(
            r###"<rect x="{x}" y="{y}" width="{w}" height="0" fill="url(#{color})" rx="3">
                    <animate attributeName="height" from="0" to="{h}" dur="0.8s" fill="freeze" />
                    <animate attributeName="y" from="{y_plus}" to="{y}" dur="0.8s" fill="freeze" />
                    <title>Faceta {id} ({orientacao}): {val} rachaduras</title>
                </rect>"###,
            x = x_base,
            y = 250.0 - h_rachaduras,
            y_plus = 250.0,
            w = largura_barra_grupo, // Use group width
            h = h_rachaduras,
            val = faceta.qtd_rachaduras,
            id = faceta.id,
            orientacao = faceta.orientacao,
            color = bar_color
        ));

        svg.push_str(&format!(
            r###"<text x="{x_text}" y="270" font-size="12" text-anchor="middle" fill="#f0f0f0">{label}</text>"###,
            x_text = x_base + largura_barra_grupo / 2,
            label = format!("{} ({})", faceta.id, faceta.orientacao)
        ));
         // Value on top of bar
        svg.push_str(&format!(
            r###"<text x="{x_text}" y="{y_val}" font-size="12" text-anchor="middle" fill="#f0f0f0" opacity="0">
                {val}
                <animate attributeName="opacity" from="0" to="1" begin="0.8s" dur="0.5s" fill="freeze" />
            </text>"###,
            x_text = x_base + largura_barra_grupo / 2,
            y_val = 250.0 - h_rachaduras - 5.0,
            val = faceta.qtd_rachaduras
        ));
    }

    svg.push_str("</svg>");
    svg
}

#[derive(Props, PartialEq, Clone)]
pub struct GraphViewProps {
    pub project_name: String,
    pub building_name: String,
}

#[component]
pub fn GraphView(props: GraphViewProps) -> Element {
    // Define the base directory for projects.
    // This might need to be configurable or determined dynamically in a real app.
    let base_projects_dir = "src/app-rust/Projects"; 

    let report_data_result = use_signal(|| ler_json_report(base_projects_dir, &props.project_name, &props.building_name));

    let navigator = use_navigator();

    match &*report_data_result.read() {
        Ok(report_data) => {
            let donut_svg = gerar_svg_donut(&report_data.fissuras);
            let barras_svg = gerar_svg_barras(&report_data.facetas);

            let mut classification_summary: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
            for fissura in &report_data.fissuras {
                *classification_summary.entry(fissura.classificacao.clone()).or_insert(0) += 1;
            }
            let summary_texts: Vec<String> = classification_summary.iter()
                .map(|(class, count)| format!("{}: {}", class, count))
                .collect();
            
            // Define colors for legend - should match donut chart logic
            let legend_colors = vec![
                ("#ff5a5f", "Est./Térm./Retr."), // Simplified, assuming order or specific names
                ("#0077ff", "Desloc./Outra"),
                ("#50C878", "Tipo C"),
                ("#FFD700", "Tipo D"),
                ("#9370DB", "Tipo E")
            ];
            let mut legend_idx = 0;

            rsx! {
                div {
                    style: "
                        background-color: #242526;
                        color: #f0f0f0;
                        font-family: 'Segoe UI', sans-serif;
                        min-height: 100vh;
                        display: flex;
                        flex-direction: column;
                        justify-content: flex-start;
                        align-items: center;
                        padding: 40px;
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

                    h1 {
                        style: "font-size: 32px; color: #ff5a5f; margin-bottom: 30px; text-align: center; width: 100%;",
                        "Análise de Fissuras: {props.project_name} - {props.building_name}"
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
                            style: "flex: 1; min-width: 450px; max-width: 500px; text-align: center; padding: 20px; background-color: #2c2d2e; border-radius: 8px; box-shadow: 0 4px 8px rgba(0,0,0,0.3);",
                            h2 { style: "font-size: 24px; color: #ffffff; margin-bottom: 20px;", "Distribuição por Classificação" }
                            div { dangerous_inner_html: donut_svg }
                            div {
                                style: "margin-top: 20px; font-size: 14px; display: flex; flex-wrap: wrap; justify-content: center; gap: 15px;",
                                for (class_name, count) in classification_summary.iter() {
                                    {
                                        let color_map = [("Estrutural", "#ff5a5f"), ("Térmica", "#0077ff"), ("Retração plástica", "#50C878"), ("Deslocamento", "#FFD700")];
                                        let current_color = color_map.iter().find(|(c, _)| *c == class_name.as_str()).map_or("#9370DB", |(_, hex)| *hex);
                                        rsx! {
                                            span { 
                                                style: "display: flex; align-items: center;",
                                                span { style: "width: 12px; height: 12px; background-color: {current_color}; border-radius: 50%; margin-right: 5px;" }
                                                "{class_name}: {count}"
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        div {
                            style: "flex: 1; min-width: 550px; max-width: 700px; text-align: center; padding: 20px; background-color: #2c2d2e; border-radius: 8px; box-shadow: 0 4px 8px rgba(0,0,0,0.3);",
                            h2 { style: "font-size: 24px; color: #ffffff; margin-bottom: 20px;", "Rachaduras por Faceta" }
                            div {
                                style: "margin-top: 10px; overflow-x: auto;",
                                div { dangerous_inner_html: barras_svg }
                            }
                        }
                    }
                    
                    button {
                        onclick: move |_| {
                            navigator.push(Route::ReportView { 
                                project_name: props.project_name.clone(), 
                                building_name: props.building_name.clone() 
                            });
                        },
                        style: "
                            margin-top: 40px;
                            background-color: #0077ff;
                            color: white;
                            border: none;
                            padding: 12px 20px;
                            border-radius: 6px;
                            cursor: pointer;
                            font-size: 18px;
                            transition: background-color 0.3s;
                        ",
                        onmouseenter: "this.style.backgroundColor='#0055aa'",
                        onmouseleave: "this.style.backgroundColor='#0077ff'",
                        "Visualizar Relatório Detalhado →"
                    }
                }
            }
        },
        Err(e) => {
            rsx! {
                div {
                    style: "color: red; padding: 20px; text-align: center;",
                    p { "Erro ao carregar os dados do relatório para {props.project_name} - {props.building_name}:" },
                    p { "{e}" },
                     button {
                        onclick: move |_| {
                             // Attempt to reload or re-fetch data.
                             // Forcing a re-render of the component might trigger use_signal to re-evaluate.
                             // Or, more robustly, have a specific state/method to trigger refetch.
                             // For now, simple navigation back might be an option or a dedicated retry button.
                             report_data_result.set(ler_json_report(base_projects_dir, &props.project_name, &props.building_name));
                        },
                        "Tentar Novamente"
                    },
                    button {
                        onclick: move |_| {
                            navigator.push(Route::HomePage {});
                        },
                        "Voltar para Início"
                    }
                }
            }
        }
    }
}