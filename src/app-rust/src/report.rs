use dioxus::prelude::*;
use serde_json::Value;
use tempfile::NamedTempFile;
use pulldown_cmark::{Parser, Options, html};
use std::{
    io::{
        Read,
        Write,
        BufWriter
    },
    process::Command,
    fs::File,
    path::PathBuf
};

#[path = "./report_generator.rs"]
mod report_generator;
use report_generator::generate_report;

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
    let md_content = md_content.to_string();
    let file_type = file_type.to_string();

    std::thread::spawn(move || {
        let file_type_lower = file_type.to_lowercase();
        let file_type_upper = file_type.to_uppercase();

        if let Some(path) = rfd::FileDialog::new()
            .set_title(&format!("Salvar arquivo {} como...", file_type_lower))
            .add_filter(&file_type_upper, &[&file_type_lower])
            .set_file_name(&format!("Relatorio.{}", &file_type_lower))
            .save_file()
        {
            let path = PathBuf::from(path);
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
    });
}

fn get_report(report_data_path: PathBuf) -> Result<String, handlebars::RenderError> {
    // Importa o template do relatório
    let template: &str = include_str!("Template/report_template.md");

    println!("report_data_path: {:?}", report_data_path.to_str());

    if !report_data_path.exists() {
        return Err(handlebars::RenderError::from(
            handlebars::RenderErrorReason::Other(format!(
                "O arquivo de dados do relatório não existe: {:?}",
                report_data_path
            )),
        ));
    }

    // Pegar arquivo JSON
    let file = File::open(&report_data_path)
        .map_err(|e| handlebars::RenderError::from(
            handlebars::RenderErrorReason::Other(format!("Erro ao abrir JSON: {}", e))
        ))?;

    // Deserialize do JSON
    let mut json_data: Value = serde_json::from_reader(file)
        .map_err(|e| handlebars::RenderError::from(
            handlebars::RenderErrorReason::Other(format!("Erro ao ler JSON: {}", e))
        ))?;

    // Extrai os nomes antes do empréstimo mutável
    let project_name = json_data.get("nome_projeto")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .replace(' ', "_")
        .to_string();
    let building_name = json_data.get("nome_predio")
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .replace(' ', "_")
        .to_string();

    // --- FLATTEN dados_fissuras INTO fissuras ARRAY FOR TEMPLATE COMPATIBILITY ---
    // Remove any existing top-level fissuras array to avoid conflicts
    json_data.as_object_mut().map(|obj| obj.remove("fissuras"));

    let mut fissuras_flat = Vec::new();
    if let Some(dados_fissuras) = json_data.get("dados_fissuras").and_then(|v| v.as_array()) {
        for faceta in dados_fissuras {
            let faceta_id = faceta.get("id_faceta").cloned().unwrap_or(Value::Null);
            let orientacao = faceta.get("orientacao").cloned().unwrap_or(Value::Null);
            let observacoes = faceta.get("observacoes").cloned().unwrap_or(Value::Null);
            if let Some(imagens) = faceta.get("imagens_fissuras").and_then(|v| v.as_array()) {
                for imagem in imagens {
                    let caminho_imagem = imagem.get("caminho_imagem").cloned().unwrap_or(Value::Null);
                    if let Some(fissuras) = imagem.get("fissuras").and_then(|v| v.as_array()) {
                        for fissura in fissuras {
                            let classificacao = fissura.get("classificacao_fissura").cloned().unwrap_or(Value::Null);
                            let confianca = fissura.get("porcentagem_confianca_modelo").cloned().unwrap_or(Value::Null);
                            let id_fissura = fissura.get("id_fissura").cloned().unwrap_or(Value::Null);
                            // Compose a flat object for the template
                            let mut fissura_obj = serde_json::Map::new();
                            fissura_obj.insert("faceta_id".to_string(), faceta_id.clone());
                            fissura_obj.insert("orientacao".to_string(), orientacao.clone());
                            fissura_obj.insert("observacoes".to_string(), observacoes.clone());
                            fissura_obj.insert("caminho_imagem".to_string(), caminho_imagem.clone());
                            fissura_obj.insert("classificacao".to_string(), classificacao);
                            fissura_obj.insert("confianca".to_string(), confianca);
                            fissura_obj.insert("id_fissura".to_string(), id_fissura);
                            fissuras_flat.push(Value::Object(fissura_obj));
                        }
                    }
                }
            }
        }
    }
    json_data["fissuras"] = Value::Array(fissuras_flat);

    // Ajusta o caminho das imagens das fissuras (now in the new flat array)
    if let Some(fissuras) = json_data.get_mut("fissuras").and_then(|v| v.as_array_mut()) {
        for fissura in fissuras {
            if let Some(caminho) = fissura.get_mut("caminho_imagem") {
                if let Some(str_path) = caminho.as_str() {
                    let new_path = format!("Projects/{}/{}", project_name, str_path);
                    *caminho = Value::String(new_path);
                }
            }
        }
    }
    let file_name = format!("Relatorio-{}-{}.md", project_name, building_name);
    let file_path: PathBuf = ["Report", &project_name, &file_name].iter().collect();

    let report: String;

    // Verifica se o arquivo do relatório já existe; caso não, gera o relatório
    if !file_path.exists() {
        let report_dir: PathBuf = ["Report", &project_name].iter().collect();
        std::fs::create_dir_all(&report_dir).map_err(|e| handlebars::RenderError::from(
            handlebars::RenderErrorReason::Other(format!("Erro ao criar pasta Report: {}", e))
        ))?;

        // Gera o report, passando o template e os dados
        report = generate_report(template, &json_data)?;

        // Salva o relatório gerado em um arquivo Markdown
        let mut file = File::create(&file_path)
            .map_err(|e| handlebars::RenderError::from(
                handlebars::RenderErrorReason::Other(format!("Erro ao criar arquivo MD: {}", e))
            ))?;
        file.write_all(report.as_bytes())
            .map_err(|e| handlebars::RenderError::from(
                handlebars::RenderErrorReason::Other(format!("Erro ao escrever no arquivo MD: {}", e))
            ))?;
    } else {
        // Se o arquivo já existe, lê o conteúdo Markdown
        let mut file = File::open(&file_path)
            .map_err(|e| handlebars::RenderError::from(
                handlebars::RenderErrorReason::Other(format!("Erro ao abrir arquivo MD existente: {}", e))
            ))?;
        let mut md_content = String::new();
        file.read_to_string(&mut md_content)
            .map_err(|e| handlebars::RenderError::from(
                handlebars::RenderErrorReason::Other(format!("Erro ao ler arquivo MD existente: {}", e))
            ))?;
        report = md_content;
    }

    // Retorna o relatório renderizado como HTML
    Ok(render_markdown(&report))
}

#[derive(Props, PartialEq, Clone)]
pub struct ReportViewProps {
    pub project_name: String,
    pub building_name: String
}

#[allow(non_snake_case)]
pub fn ReportView(props: ReportViewProps) -> Element {
    // Corrigir caminho do arquivo de dados para buscar em 'Projects' ao invés de 'src/Report'
    let data_file_name: String = format!("Dados-{}-{}.json", &props.project_name, &props.building_name);
    let data_file_path: PathBuf = ["Projects", &props.project_name, &data_file_name].iter().collect();
    let report_file_name: String = format!("Relatorio-{}-{}.md", &props.project_name, &props.building_name);
    let report_file_path: PathBuf = ["Report", &props.project_name, &report_file_name].iter().collect();

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/Template/report_page.css")
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
                        onclick: {
                            let report_file_path = report_file_path.clone();
                            move |_| {
                                let report_file_path = report_file_path.clone();
                                match std::fs::read_to_string(&report_file_path) {
                                    Ok(content) => export(&content, "MD"),
                                    Err(e) => eprintln!("Erro ao ler arquivo MD: {}", e),
                                }
                            }
                        },
                        "Exportar em MD"
                    }
                    button {
                        onclick: {
                            let report_file_path = report_file_path.clone();
                            move |_| {
                                let report_file_path = report_file_path.clone();
                                match std::fs::read_to_string(&report_file_path) {
                                    Ok(content) => export(&content, "PDF"),
                                    Err(e) => eprintln!("Erro ao ler arquivo MD: {}", e),
                                }
                            }
                        },
                        "Exportar em PDF"
                    },
                    button {
                        onclick: {
                            let report_file_path = report_file_path.clone();
                            move |_| {
                                let report_file_path = report_file_path.clone();
                                match std::fs::read_to_string(&report_file_path) {
                                    Ok(content) => export(&content, "DOCX"),
                                    Err(e) => eprintln!("Erro ao ler arquivo MD: {}", e),
                                }
                            }
                        },
                        "Exportar em DOCX"
                    }
                }
                div {
                    class: "text-viewer",
                    div {
                        class: "text-content",
                        dangerous_inner_html: get_report(data_file_path.clone())
                            .unwrap_or_else(|e| format!("Erro ao gerar relatório: {}", e))
                    }
                }
            }
        }
    }
}
