use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::Deserialize;
use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

use chrono::Local;
use pulldown_cmark::{html, Options, Parser};
use printpdf::*;
use docx_rs::*;

use crate::Route;
use crate::utils::report_generator::generate_report;

#[derive(Props, PartialEq, Clone)]
pub struct ReportViewProps {
    pub project_name: String,
    pub building_name: String,
}

#[allow(non_snake_case)]
pub fn ReportView(props: ReportViewProps) -> Element {
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let report_md_filename = format!(
        "Relatorio-{}-{}.md",
        props.project_name.replace(' ', "_"),
        props.building_name.replace(' ', "_")
    );
    let report_md_path = base_dir.join("Report").join(&props.project_name).join(&report_md_filename);

    rsx! {
        body {
            header {
                class: "page-header",
                style: "justify-content: center;",
                i { class: "material-icons", "description" }
                h1 { "Relatório de Inspeção - 14 BIS" }
            }

            main {
                class: "container",

                div {
                    class: "report-button-bar",

                    Link {
                        to: Route::GraphView { project_name: props.project_name.clone() },
                        class: "btn btn-secondary",
                        i { class: "material-icons", "arrow_back" }
                        "Voltar para Análise"
                    }

                    button {
                        class: "btn btn-primary",
                        onclick: {
                            let path = report_md_path.clone();
                            move |_| {
                                if let Ok(content) = fs::read_to_string(&path) {
                                    export_md(&content);
                                }
                            }
                        },
                        "Exportar em MD"
                    }

                    button {
                        class: "btn btn-primary",
                        onclick: {
                            let path = report_md_path.clone();
                            let project = props.project_name.clone();
                            let building = props.building_name.clone();
                            move |_| {
                                if let Ok(content) = fs::read_to_string(&path) {
                                    if let Err(e) = export_pdf(&content, &project, &building) {
                                        eprintln!("Erro ao exportar PDF: {}", e);
                                    }
                                }
                            }
                        },
                        "Exportar em PDF"
                    }

                    button {
                        class: "btn btn-primary",
                        onclick: {
                            let path = report_md_path.clone();
                            let project = props.project_name.clone();
                            let building = props.building_name.clone();
                            move |_| {
                                if let Ok(content) = fs::read_to_string(&path) {
                                    if let Err(e) = export_docx(&content, &project, &building) {
                                        eprintln!("Erro ao exportar DOCX: {}", e);
                                    }
                                }
                            }
                        },
                        "Exportar em DOCX"
                    }
                }

                div {
                    class: "report-viewer",
                    div {
                        class: "report-content",
                        dangerous_inner_html: get_report(&props.project_name, &props.building_name)
                            .unwrap_or_else(|e| format!("<div class='status-message error'><h1>Erro ao gerar relatório</h1><p>{}</p>", e))
                    }
                }
            }
        }
    }
}

// ======= Funções auxiliares =======

fn get_report(project_name: &str, building_name: &str) -> Result<String, String> {
    let filename = format!(
        "Relatorio-{}-{}.md",
        project_name.replace(' ', "_"),
        building_name.replace(' ', "_")
    );
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let file_path = base_dir.join("Report").join(project_name).join(&filename);

    let md_content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Erro ao ler arquivo MD: {}", e))?;

    let parser = Parser::new_ext(&md_content, Options::all());
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(html_output)
}

fn export_md(content: &str) {
    let filename = format!("Relatorio-{}.md", Local::now().format("%Y%m%d-%H%M%S"));
    if let Ok(mut file) = File::create(&filename) {
        let _ = file.write_all(content.as_bytes());
        println!("Arquivo MD exportado: {}", filename);
    }
}

fn export_pdf(content: &str, project: &str, building: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new("Relatório", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;

    let lines = content.lines();
    let mut y = Mm(280.0);
    for line in lines {
        current_layer.use_text(line, 12.0, Mm(10.0), y, &font);
        y -= Mm(7.0);
    }

    let filename = format!("Relatorio-{}-{}.pdf", project.replace(" ", "_"), building.replace(" ", "_"));
    let file = File::create(&filename)?;
    doc.save(&mut BufWriter::new(file))?;
    println!("Arquivo PDF exportado: {}", filename);
    Ok(())
}

fn export_docx(content: &str, project: &str, building: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Docx::new();
    for line in content.lines() {
        doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text(line)));
    }

    let filename = format!("Relatorio-{}-{}.docx", project.replace(" ", "_"), building.replace(" ", "_"));
    let mut file = File::create(&filename)?;
    doc.build().pack(&mut file)?;
    println!("Arquivo DOCX exportado: {}", filename);
    Ok(())
}
