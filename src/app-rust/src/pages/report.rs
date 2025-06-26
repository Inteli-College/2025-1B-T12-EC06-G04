use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::Deserialize;
use std::{
    env,
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
    thread,
    sync::mpsc::{self, Receiver, Sender},
    time::Duration,
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

#[derive(Clone, Debug)]
enum ExportType {
    Markdown,
    Pdf,
    Docx,
}

#[derive(Clone, Debug)]
enum ExportMessage {
    Started,
    Success(String),
    Error(String),
    Finished,
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

    // Estado para controlar mensagens de status
    let mut status_message = use_signal(|| String::new());
    let mut is_exporting = use_signal(|| false);

    // Canal para receber mensagens das threads
    let mut export_receiver = use_signal(|| -> Option<Receiver<ExportMessage>> { None });

    // Effect para escutar mensagens do canal
    use_future(move || {
        let mut status = status_message.clone();
        let mut exporting = is_exporting.clone();
        let mut receiver = export_receiver.clone();
        
        async move {
            loop {
                if let Some(rx) = receiver.read().as_ref() {
                    if let Ok(message) = rx.try_recv() {
                        match message {
                            ExportMessage::Started => {
                                exporting.set(true);
                                status.set("Abrindo diálogo de salvamento...".to_string());
                            }
                            ExportMessage::Success(path) => {
                                status.set(format!("✅ Arquivo salvo em: {}", path));
                            }
                            ExportMessage::Error(err) => {
                                status.set(format!("❌ Erro: {}", err));
                            }
                            ExportMessage::Finished => {
                                exporting.set(false);
                            }
                        }
                    }
                }
                
                // Pequena pausa para não sobrecarregar
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    });

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

                // Mensagem de status
                if !status_message.read().is_empty() {
                    div {
                        class: if status_message.read().contains("❌") { "status-message error" } else { "status-message info" },
                        style: "margin-bottom: 1rem; padding: 0.75rem; border-radius: 0.25rem; background-color: #f8f9fa; border: 1px solid #dee2e6;",
                        {status_message.read().as_str()}
                    }
                }

                // Indicador de carregamento
                if *is_exporting.read() {
                    div {
                        class: "loading-indicator",
                        style: "margin-bottom: 1rem; padding: 1rem; text-align: center; background-color: #fff3cd; border: 1px solid #ffeaa7; border-radius: 0.25rem; color: #856404;",
                        "🔄 Processando exportação... Por favor, aguarde."
                    }
                }

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
                        disabled: *is_exporting.read(),
                        onclick: {
                            let path = report_md_path.clone();
                            let project = props.project_name.clone();
                            let building = props.building_name.clone();
                            let mut receiver = export_receiver.clone();
                            move |_| {
                                let (tx, rx) = mpsc::channel();
                                receiver.set(Some(rx));
                                
                                export_with_native_dialog(
                                    path.clone(), 
                                    project.clone(), 
                                    building.clone(), 
                                    ExportType::Markdown,
                                    tx
                                );
                            }
                        },
                        if *is_exporting.read() { "Exportando..." } else { "Exportar em MD" }
                    }

                    button {
                        class: "btn btn-primary",
                        disabled: *is_exporting.read(),
                        onclick: {
                            let path = report_md_path.clone();
                            let project = props.project_name.clone();
                            let building = props.building_name.clone();
                            let mut receiver = export_receiver.clone();
                            move |_| {
                                let (tx, rx) = mpsc::channel();
                                receiver.set(Some(rx));
                                
                                export_with_native_dialog(
                                    path.clone(), 
                                    project.clone(), 
                                    building.clone(), 
                                    ExportType::Pdf,
                                    tx
                                );
                            }
                        },
                        if *is_exporting.read() { "Exportando..." } else { "Exportar em PDF" }
                    }

                    button {
                        class: "btn btn-primary",
                        disabled: *is_exporting.read(),
                        onclick: {
                            let path = report_md_path.clone();
                            let project = props.project_name.clone();
                            let building = props.building_name.clone();
                            let mut receiver = export_receiver.clone();
                            move |_| {
                                let (tx, rx) = mpsc::channel();
                                receiver.set(Some(rx));
                                
                                export_with_native_dialog(
                                    path.clone(), 
                                    project.clone(), 
                                    building.clone(), 
                                    ExportType::Docx,
                                    tx
                                );
                            }
                        },
                        if *is_exporting.read() { "Exportando..." } else { "Exportar em DOCX" }
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

fn export_with_native_dialog(
    report_path: PathBuf,
    project: String,
    building: String,
    export_type: ExportType,
    sender: Sender<ExportMessage>
) {
    thread::spawn(move || {
        let _ = sender.send(ExportMessage::Started);
        
        let content = match fs::read_to_string(&report_path) {
            Ok(content) => content,
            Err(e) => {
                let _ = sender.send(ExportMessage::Error(format!("Erro ao ler arquivo: {}", e)));
                let _ = sender.send(ExportMessage::Finished);
                return;
            }
        };

        let result = match export_type {
            ExportType::Markdown => {
                let default_name = format!(
                    "Relatorio-{}-{}-{}.md",
                    project.replace(" ", "_"),
                    building.replace(" ", "_"),
                    Local::now().format("%Y%m%d-%H%M%S")
                );
                
                if let Some(path) = show_save_dialog(&default_name, "md") {
                    save_md_file(&content, &path)
                } else {
                    Err("Operação cancelada".into())
                }
            }
            ExportType::Pdf => {
                let default_name = format!(
                    "Relatorio-{}-{}-{}.pdf",
                    project.replace(" ", "_"),
                    building.replace(" ", "_"),
                    Local::now().format("%Y%m%d-%H%M%S")
                );
                
                if let Some(path) = show_save_dialog(&default_name, "pdf") {
                    save_pdf_file(&content, &path)
                } else {
                    Err("Operação cancelada".into())
                }
            }
            ExportType::Docx => {
                let default_name = format!(
                    "Relatorio-{}-{}-{}.docx",
                    project.replace(" ", "_"),
                    building.replace(" ", "_"),
                    Local::now().format("%Y%m%d-%H%M%S")
                );
                
                if let Some(path) = show_save_dialog(&default_name, "docx") {
                    save_docx_file(&content, &path)
                } else {
                    Err("Operação cancelada".into())
                }
            }
        };

        match result {
            Ok(path) => {
                let _ = sender.send(ExportMessage::Success(path));
            }
            Err(e) => {
                let _ = sender.send(ExportMessage::Error(e.to_string()));
            }
        }
        
        let _ = sender.send(ExportMessage::Finished);
    });
}

// Usa comando nativo do sistema para abrir diálogo
fn show_save_dialog(default_name: &str, extension: &str) -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        
        let script = format!(r#"
            Add-Type -AssemblyName System.Windows.Forms
            $dialog = New-Object System.Windows.Forms.SaveFileDialog
            $dialog.Filter = "Arquivos {} (*.{})|*.{}|Todos os arquivos (*.*)|*.*"
            $dialog.FileName = "{}"
            $dialog.Title = "Salvar Relatório"
            if ($dialog.ShowDialog() -eq [System.Windows.Forms.DialogResult]::OK) {{
                Write-Output $dialog.FileName
            }}
        "#, extension.to_uppercase(), extension, extension, default_name);

        let output = Command::new("powershell")
            .args(&["-Command", &script])
            .output()
            .ok()?;

        let path = String::from_utf8(output.stdout).ok()?.trim().to_string();
        if path.is_empty() { None } else { Some(path) }
    }
    
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        
        let output = Command::new("osascript")
            .args(&[
                "-e",
                &format!(r#"choose file name with prompt "Salvar Relatório" default name "{}" default location (path to downloads folder)"#, default_name)
            ])
            .output()
            .ok()?;

        let path = String::from_utf8(output.stdout).ok()?.trim().to_string();
        if path.is_empty() || path.contains("User canceled") { None } else { Some(path) }
    }
    
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        
        let output = Command::new("zenity")
            .args(&[
                "--file-selection",
                "--save",
                "--confirm-overwrite",
                &format!("--filename={}", default_name),
                &format!("--file-filter=Arquivos {} | *.{}", extension.to_uppercase(), extension),
                "--file-filter=Todos os arquivos | *",
                "--title=Salvar Relatório"
            ])
            .output()
            .ok()?;

        let path = String::from_utf8(output.stdout).ok()?.trim().to_string();
        if path.is_empty() { None } else { Some(path) }
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        None
    }
}

fn save_md_file(content: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(path.to_string())
}

fn save_pdf_file(content: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let (doc, page1, layer1) = PdfDocument::new("Relatório", Mm(210.0), Mm(297.0), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_builtin_font(BuiltinFont::Helvetica)?;

    let lines = content.lines();
    let mut y = Mm(280.0);
    for line in lines {
        current_layer.use_text(line, 12.0, Mm(10.0), y, &font);
        y -= Mm(7.0);
        
        if y < Mm(20.0) {
            break; // Evita overflow na página
        }
    }

    let file = File::create(path)?;
    doc.save(&mut BufWriter::new(file))?;
    Ok(path.to_string())
}

fn save_docx_file(content: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut doc = Docx::new();
    for line in content.lines() {
        doc = doc.add_paragraph(Paragraph::new().add_run(Run::new().add_text(line)));
    }

    let mut file = File::create(path)?;
    doc.build().pack(&mut file)?;
    Ok(path.to_string())
}

// Função para exportação rápida na pasta Downloads
fn quick_export_all(
    report_path: PathBuf,
    project: String,
    building: String,
    sender: Sender<ExportMessage>
) {
    thread::spawn(move || {
        let _ = sender.send(ExportMessage::Started);
        
        let content = match fs::read_to_string(&report_path) {
            Ok(content) => content,
            Err(e) => {
                let _ = sender.send(ExportMessage::Error(format!("Erro ao ler arquivo: {}", e)));
                let _ = sender.send(ExportMessage::Finished);
                return;
            }
        };

        let downloads_dir = dirs::download_dir().unwrap_or_else(|| {
            dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
        });

        let timestamp = Local::now().format("%Y%m%d-%H%M%S");
        let base_name = format!(
            "Relatorio-{}-{}-{}",
            project.replace(" ", "_"),
            building.replace(" ", "_"),
            timestamp
        );

        let mut results = Vec::new();

        // Salva MD
        let md_path = downloads_dir.join(format!("{}.md", base_name));
        if let Err(e) = save_md_file(&content, &md_path.to_string_lossy()) {
            results.push(format!("❌ MD: {}", e));
        } else {
            results.push("✅ MD salvo".to_string());
        }

        // Salva PDF
        let pdf_path = downloads_dir.join(format!("{}.pdf", base_name));
        if let Err(e) = save_pdf_file(&content, &pdf_path.to_string_lossy()) {
            results.push(format!("❌ PDF: {}", e));
        } else {
            results.push("✅ PDF salvo".to_string());
        }

        // Salva DOCX
        let docx_path = downloads_dir.join(format!("{}.docx", base_name));
        if let Err(e) = save_docx_file(&content, &docx_path.to_string_lossy()) {
            results.push(format!("❌ DOCX: {}", e));
        } else {
            results.push("✅ DOCX salvo".to_string());
        }

        let message = format!(
            "Exportação rápida concluída em Downloads: {}",
            results.join(", ")
        );
        
        let _ = sender.send(ExportMessage::Success(message));
        let _ = sender.send(ExportMessage::Finished);
    });
}