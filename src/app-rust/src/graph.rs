use dioxus::prelude::*;
use std::f64::consts::PI;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use csv::Reader;
use crate::Route;
use dioxus_router::prelude::*;


//  Modelo de Dados 
#[derive(Deserialize, Debug, Clone)]
struct FissuraData {
    predio: String,
    termica: u32,
    retracao: u32,
}

//  Leitura do CSV(Depois ajustar para Dicionário)
fn ler_csv(path: &str) -> Vec<FissuraData> {
    let file = File::open(path).expect("Erro ao abrir CSV");
    let mut rdr = Reader::from_reader(BufReader::new(file));
    rdr.deserialize().filter_map(Result::ok).collect()
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

fn gerar_svg_donut(termica: u32, retracao: u32) -> String {
    let total = termica + retracao;
    let angle_termica = (termica as f64 / total as f64) * 360.0;

    let cx = 250.0;
    let cy = 250.0;
    let raio_externo = 200.0;
    let raio_interno = 120.0;

    let label_termica = format!("Térmica: {}", termica);
    let label_retracao = format!("Retração: {}", retracao);

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
        r###"<text x="{cx}" y="{cy}" font-size="20" text-anchor="middle" fill="#ffffff" dominant-baseline="middle">Total</text>"###,
        cx = cx,
        cy = cy
    ));

    svg.push_str("</svg>");
    svg
}

//  Gráfico de Barras 

fn gerar_svg_barras(dados: &[FissuraData]) -> String {
    let altura_total = 450;
    let largura_barra = 30;
    let espacamento = 80;
    let largura_total = 60 + dados.len() as i32 * espacamento;

    let max_valor = dados.iter()
        .flat_map(|d| vec![d.termica, d.retracao])
        .max()
        .unwrap_or(1) as f64;

    let mut svg = format!(
        r###"<svg width="{largura_total}" height="{altura_total}" viewBox="0 0 {largura_total} {altura_total}" xmlns="http://www.w3.org/2000/svg">"###
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

    for (i, d) in dados.iter().enumerate() {
        let x_base = 60 + i as i32 * espacamento;

        let h_termica = (d.termica as f64 / max_valor * 200.0) as i32;
        let h_retracao = (d.retracao as f64 / max_valor * 200.0) as i32;

        // Térmica
        svg.push_str(&format!(
            r###"<rect x="{x}" y="{y}" width="{w}" height="0" fill="url(#grad_red)" rx="3">
                    <animate attributeName="height" from="0" to="{h}" dur="0.8s" fill="freeze" />
                    <animate attributeName="y" from="{y_plus}" to="{y}" dur="0.8s" fill="freeze" />
                    <title>Térmica: {val}</title>
                </rect>"###,
            x = x_base,
            y = 250 - h_termica,
            y_plus = 250,
            w = largura_barra,
            h = h_termica,
            val = d.termica
        ));

        // Retração
        svg.push_str(&format!(
            r###"<rect x="{x}" y="{y}" width="{w}" height="0" fill="url(#grad_blue)" rx="3">
                    <animate attributeName="height" from="0" to="{h}" dur="0.8s" fill="freeze" />
                    <animate attributeName="y" from="{y_plus}" to="{y}" dur="0.8s" fill="freeze" />
                    <title>Retração: {val}</title>
                </rect>"###,
            x = x_base + largura_barra + 2,
            y = 250 - h_retracao,
            y_plus = 250,
            w = largura_barra,
            h = h_retracao,
            val = d.retracao
        ));

        // texto prédio
        svg.push_str(&format!(
            r###"<text x="{x}" y="270" font-size="10" text-anchor="start" fill="#f0f0f0">{nome}</text>"###,
            x = x_base,
            nome = d.predio
        ));
    }

    svg.push_str("</svg>");
    svg
}

//Frontend de backend

#[component]
pub fn GraphView() -> Element {
    let dados = ler_csv("teste_fissuras.csv"); 

    let total_termica: u32 = dados.iter().map(|d| d.termica).sum();
    let total_retracao: u32 = dados.iter().map(|d| d.retracao).sum();

    let donut_svg = gerar_svg_donut(total_termica, total_retracao);
    let barras_svg = gerar_svg_barras(&dados);

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
                    navigator.push(Route::Home {});  
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
                    "Gráficos das Fissuras"
                }

                // gráfico Donut
                div {
                    style: "flex: 1; min-width: 400px; text-align: center;",
                    h2 { style: "font-size: 24px; color: #ffffff;", "Distribuição Total de Fissuras" }
                    div { dangerous_inner_html: donut_svg }
                    p { style: "margin-top: 10px; font-size: 14px;", "Térmicas: {total_termica} | Retração: {total_retracao}" }
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
                        style: "margin-top: 10px;",
                        div { dangerous_inner_html: barras_svg }
                    }

                    button {
                        onclick: move |_| {
                            navigator.push(Route::ReportView {});
                        },                    
                        style: "
                            position: absolute;
                            bottom: 60px;
                            right: 20px;
                            background-color: #0077ff;
                            color: white;
                            border: none;
                            padding: 10px 16px;
                            border-radius: 6px;
                            cursor: pointer;
                            font-size: 18px;
                        ",
                        "Válidar Fotos →"
                    }
                }
            }
        }
    }
}