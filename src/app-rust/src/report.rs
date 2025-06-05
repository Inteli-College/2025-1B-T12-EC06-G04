use dioxus::prelude::*;
use dioxus_router::prelude::*;
use serde::Deserialize;
use tempfile::NamedTempFile;
use pulldown_cmark::{Parser, Options, html};
use std::{
    io::{
        Read,
        Write,
        BufWriter,
        BufReader
    },
    process::Command,
    fs::File,
    path::{Path, PathBuf},
    env
};
use chrono::Local;
use rand::Rng;
use crate::Route;

#[path = "./report_generator.rs"]
pub mod report_generator;
use report_generator::generate_report;

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

fn get_report(project_name_prop: &str, building_name_prop: &str) -> Result<String, handlebars::RenderError> {
    let template: &str = include_str!("Template/report_template.md");

    let cwd_string = match env::current_dir() {
        Ok(cwd) => {
            let s = cwd.display().to_string();
            println!("[RUST report.rs] Current Working Directory: {}", s);
            s
        }
        Err(e) => {
            let err_msg = format!("Failed to get CWD for report: {}", e);
            eprintln!("[RUST report.rs] {}", err_msg);
            return Err(handlebars::RenderError::from(handlebars::RenderErrorReason::Other(err_msg)));
        }
    };
    
    let detection_json_path_str = format!("Projects/{}/detection_results.json", project_name_prop);
    let absolute_detection_json_path = Path::new(&cwd_string).join(&detection_json_path_str);

    println!("[RUST report.rs] Attempting to read detection_results.json from (absolute constructed): {:?}", absolute_detection_json_path);

    if !absolute_detection_json_path.exists() {
        let err_msg = format!("Arquivo detection_results.json não existe em: {:?}. Verifique o CWD e o caminho relativo.", absolute_detection_json_path);
        eprintln!("[RUST report.rs] {}", err_msg);
        return Err(handlebars::RenderError::from(handlebars::RenderErrorReason::Other(err_msg)));
    }

    let file = File::open(&absolute_detection_json_path)
        .map_err(|e| handlebars::RenderError::from(handlebars::RenderErrorReason::Other(format!("Erro ao abrir detection_results.json (path: {:?}): {}", absolute_detection_json_path, e))))?;
    
    let detection_data_vec: Vec<ImageDetectionData> = serde_json::from_reader(BufReader::new(file))
        .map_err(|e| handlebars::RenderError::from(handlebars::RenderErrorReason::Other(format!("Erro ao ler/parsear detection_results.json (path: {:?}): {}", absolute_detection_json_path, e))))?;

    let mut fissuras_flat_for_template = Vec::new();
    let mut rng = rand::thread_rng();

    for image_data in detection_data_vec {
        let path_obj = Path::new(&image_data.path);
        let facade_name = path_obj.parent().and_then(|p| p.file_name()).and_then(|os| os.to_str()).unwrap_or("N/A").to_string();

        for fissura_item in image_data.fissura {
            let mut fissura_obj_for_template = serde_json::Map::new();
            fissura_obj_for_template.insert("caminho_imagem".to_string(), serde_json::Value::String(image_data.path.clone()));
            fissura_obj_for_template.insert("classificacao".to_string(), serde_json::Value::String(fissura_item.name.clone()));
            
            let confidence_number = serde_json::Number::from_f64(fissura_item.confidence)
                                      .unwrap_or_else(|| serde_json::Number::from(0));
            fissura_obj_for_template.insert("confianca".to_string(), serde_json::Value::Number(confidence_number));
            
            fissura_obj_for_template.insert("faceta_id".to_string(), serde_json::Value::String(facade_name.clone()));
            fissura_obj_for_template.insert("orientacao".to_string(), serde_json::Value::String("N/A".to_string()));
            fissura_obj_for_template.insert("observacoes".to_string(), serde_json::Value::String("N/A".to_string()));
            fissura_obj_for_template.insert("id_fissura".to_string(), serde_json::Value::String(format!("f_{}", rng.gen::<u32>())));

            fissuras_flat_for_template.push(serde_json::Value::Object(fissura_obj_for_template));
        }
    }

    let mut template_data = serde_json::Map::new();
    template_data.insert("nome_projeto".to_string(), serde_json::Value::String(project_name_prop.to_string()));
    template_data.insert("nome_predio".to_string(), serde_json::Value::String(building_name_prop.to_string()));
    template_data.insert("fissuras".to_string(), serde_json::Value::Array(fissuras_flat_for_template));
    
    let now = Local::now();
    template_data.insert("data_geracao".to_string(), serde_json::Value::String(now.format("%Y-%m-%d %H:%M:%S").to_string()));

    let final_json_for_template = serde_json::Value::Object(template_data);

    let report_output_dir: PathBuf = ["Report", project_name_prop].iter().collect();
    let report_md_filename: String = format!("Relatorio-{}-{}.md", project_name_prop.replace(' ', "_"), building_name_prop.replace(' ', "_"));
    let report_md_filepath: PathBuf = report_output_dir.join(&report_md_filename);

    let report_markdown_content: String;

    if !report_md_filepath.exists() {
        std::fs::create_dir_all(&report_output_dir).map_err(|e| handlebars::RenderError::from(handlebars::RenderErrorReason::Other(format!("Erro ao criar pasta Report '{:?}': {}", report_output_dir, e))))?;
        report_markdown_content = generate_report(template, &final_json_for_template)?;
        let mut file = File::create(&report_md_filepath)
            .map_err(|e| handlebars::RenderError::from(handlebars::RenderErrorReason::Other(format!("Erro ao criar arquivo MD '{:?}': {}", report_md_filepath, e))))?;
        file.write_all(report_markdown_content.as_bytes())
            .map_err(|e| handlebars::RenderError::from(handlebars::RenderErrorReason::Other(format!("Erro ao escrever no arquivo MD '{:?}': {}", report_md_filepath, e))))?;
        println!("[RUST report.rs] Novo relatório MD gerado em: {:?}", report_md_filepath);
    } else {
        let mut file = File::open(&report_md_filepath)
            .map_err(|e| handlebars::RenderError::from(handlebars::RenderErrorReason::Other(format!("Erro ao abrir arquivo MD existente '{:?}': {}", report_md_filepath, e))))?;
        let mut md_content = String::new();
        file.read_to_string(&mut md_content)
            .map_err(|e| handlebars::RenderError::from(handlebars::RenderErrorReason::Other(format!("Erro ao ler arquivo MD existente '{:?}': {}", report_md_filepath, e))))?;
        report_markdown_content = md_content;
        println!("[RUST report.rs] Relatório MD existente carregado de: {:?}", report_md_filepath);
    }

    Ok(render_markdown(&report_markdown_content))
}

#[derive(Props, PartialEq, Clone)]
pub struct ReportViewProps {
    pub project_name: String,
    pub building_name: String
}

#[allow(non_snake_case)]
pub fn ReportView(props: ReportViewProps) -> Element {
    let report_md_filename: String = format!("Relatorio-{}-{}.md", &props.project_name.replace(' ', "_"), &props.building_name.replace(' ', "_"));
    let report_md_filepath: PathBuf = ["Report", &props.project_name, &report_md_filename].iter().collect();
    
    if let Ok(cwd) = env::current_dir() {
        println!("[RUST ReportView Render] CWD: {:?}", cwd);
    }
    println!("[RUST ReportView Render] Tentando usar MD de: {:?}", report_md_filepath);

    let common_button_min_width = "min-w-[180px]"; 

    rsx! {
        document::Link {
            rel: "stylesheet",
            href: asset!("/src/Template/report_page.css")
        }
        body {
            header {
                class: " bg-blue-600 text-white p-4 shadow font-bold text-center",
                i { class: "material-icons icon", "description" }
                h1 { class: "inline-block align-middle", "Relatório de Inspeção - 14 BIS" }
            }
            main {
                div {
                    class: "flex flex-wrap p-6 gap-6 max-w-7xl mx-auto",
                    Link {
                        to: Route::HomePage {},
                        class: format!("flex-1 {} px-4 py-2 bg-blue-500 text-white hover:bg-blue-600 active:bg-blue-700 transition-colors duration-200 shadow rounded-lg flex items-center justify-center gap-2 text-sm md:text-base", common_button_min_width),
                        i { class: "material-icons", "home" }
                        "Voltar para Home"
                    }
                    button {
                        class: format!("flex-1 {} px-4 py-2 bg-blue-500 text-white hover:bg-blue-600 active:bg-blue-700 transition-colors duration-200 shadow rounded-lg flex items-center justify-center text-sm md:text-base", common_button_min_width),
                        onclick: {
                            let path_clone = report_md_filepath.clone();
                            move |_| {
                                if path_clone.exists() {
                                    match std::fs::read_to_string(&path_clone) {
                                        Ok(content) => export(&content, "MD"),
                                        Err(e) => eprintln!("Erro ao ler arquivo MD para exportação: {}", e),
                                    }
                                } else {
                                    eprintln!("Arquivo de relatório MD não encontrado para exportação: {:?}", path_clone);
                                }
                            }
                        },
                        "Exportar em MD"
                    }
                    button {
                        class: format!("flex-1 {} px-4 py-2 bg-blue-500 text-white hover:bg-blue-600 active:bg-blue-700 transition-colors duration-200 shadow rounded-lg flex items-center justify-center text-sm md:text-base", common_button_min_width),
                        onclick: {
                            let path_clone = report_md_filepath.clone();
                            move |_| {
                                if path_clone.exists() {
                                    match std::fs::read_to_string(&path_clone) {
                                        Ok(content) => export(&content, "PDF"),
                                        Err(e) => eprintln!("Erro ao ler arquivo MD para exportação: {}", e),
                                    }
                                } else {
                                    eprintln!("Arquivo de relatório MD não encontrado para exportação: {:?}", path_clone);
                                }
                            }
                        },
                        "Exportar em PDF"
                    }
                    button {
                        class: format!("flex-1 {} px-4 py-2 bg-blue-500 text-white hover:bg-blue-600 active:bg-blue-700 transition-colors duration-200 shadow rounded-lg flex items-center justify-center text-sm md:text-base", common_button_min_width),
                        onclick: {
                            let path_clone = report_md_filepath.clone();
                            move |_| {
                                if path_clone.exists() {
                                    match std::fs::read_to_string(&path_clone) {
                                        Ok(content) => export(&content, "DOCX"),
                                        Err(e) => eprintln!("Erro ao ler arquivo MD para exportação: {}", e),
                                    }
                                } else {
                                     eprintln!("Arquivo de relatório MD não encontrado para exportação: {:?}", path_clone);
                                }
                            }
                        },
                        "Exportar em DOCX"
                    }
                }
                div {
                    class: "text-viewer w-full",
                    div {
                        class: "text-content",
                        dangerous_inner_html: get_report(&props.project_name, &props.building_name)
                            .unwrap_or_else(|e| format!("<h1>Erro ao gerar relatório</h1><p>Detalhes: {}</p><p>Verifique o console para mais informações sobre caminhos de arquivos.</p>", e))
                    }
                }
            }
        }
    }
}