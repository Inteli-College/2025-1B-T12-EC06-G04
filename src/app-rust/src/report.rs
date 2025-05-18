use dioxus::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

use pulldown_cmark::{Parser, Options, html};

fn render_markdown(md: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(md, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

fn export(md_content: &str, file_type: &str) {
    let file_type_lower = file_type.to_lowercase();
    let file_type_upper = file_type.to_uppercase();

    if let Some(path) = rfd::FileDialog::new()
        .set_title(&format!("Salvar arquivo {} como...", file_type_lower))
        .add_filter(&file_type_upper, &[&file_type_lower])
        .set_file_name(&format!("Relatorio.{}", &file_type_lower))
        .save_file()
    {
        if &file_type_lower == "md" {
            let new_file = File::create(&path).unwrap();
            let mut writer = BufWriter::new(new_file);
            writer.write_all(md_content.as_bytes()).unwrap();
        } else {
            let mut temp_md = NamedTempFile::new().unwrap();
            write!(temp_md, "{}", md_content).unwrap();
            temp_md.flush().unwrap();

            let status = Command::new("pandoc")
                .arg(temp_md.path())
                .arg("-o")
                .arg(&path)
                .status()
                .expect(&format!("Falha ao gerar {} com pandoc", &file_type_upper));

            if !status.success() {
                eprintln!("Erro ao converter Markdown para {} com pandoc", &file_type_upper);
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn ReportView() -> Element {
    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/Report/report_page.css")
        }
        body {
            header {
                i { class: "material-icons icon", "logo" }
                h1 { "14 BIS" }
            }
            main {
                div {
                    class: "button-area",
                    button {
                        onclick: |_| {
                            export(include_str!("Report/relatorio.md"), "MD");
                        },
                        "Exportar em MD"
                    }
                    button {
                        onclick: |_| {
                            export(include_str!("Report/relatorio.md"), "PDF");
                        },
                        "Exportar em PDF"
                    }
                    button {
                        onclick: |_| {
                            export(include_str!("Report/relatorio.md"), "DOCX");
                        },
                        "Exportar em DOCX"
                    }
                }
                div {
                    class: "text-viewer",
                    div {
                        class: "text-content",
                        dangerous_inner_html: "{render_markdown(include_str!(\"Report/relatorio.md\"))}"
                    }
                }
            }
        }
    }
}