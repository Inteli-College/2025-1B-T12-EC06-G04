use dioxus::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn main() {
    dioxus::launch(app);
}

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

fn export_pdf(md_content: &str) {
    if let Some(path) = rfd::FileDialog::new()
        .set_title("Salvar arquivo PDF como...")
        .add_filter("PDF", &["pdf"])
        .set_file_name("Relatorio.pdf")
        .save_file()
    {
        let mut temp_md = NamedTempFile::new().unwrap();
        write!(temp_md, "{}", md_content).unwrap();
        temp_md.flush().unwrap();

        let status = Command::new("pandoc")
            .arg(temp_md.path())
            .arg("-o")
            .arg(&path)
            .status()
            .expect("Falha ao gerar PDF com pandoc");

        if !status.success() {
            eprintln!("Erro ao converter Markdown para PDF com pandoc");
        }
    }
}

fn export_md(md_content: &str) {
    if let Some(path) = rfd::FileDialog::new()
        .set_title("Salvar arquivo MD como...")
        .add_filter("MD", &["md"])
        .set_file_name("Relatorio.md")
        .save_file()
    {
        let new_file = File::create(&path).unwrap();
        let mut writer = BufWriter::new(new_file);
        writer.write_all(md_content.as_bytes()).unwrap();
    }
}

fn app() -> Element {
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
                            export_md(include_str!("Report/relatorio.md"));
                        },
                        "Exportar em MD"
                    }
                    button {
                        onclick: |_| {
                            export_pdf(include_str!("Report/relatorio.md"));
                        },
                        "Exportar em PDF"
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