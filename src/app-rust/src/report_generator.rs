use handlebars::Handlebars;
use serde_json::Value;
use crate::report_structures::{ReportData, Faceta, Fissura};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Write;
use chrono::Local;

// This function simulates calling the C++ model and processing images
// In a real scenario, this would involve FFI or inter-process communication
fn simulate_cpp_model_and_image_processing(
    images_dir: &Path,
    project_name: &str,
    building_name: &str,
) -> Result<(Vec<Faceta>, Vec<Fissura>), Box<dyn std::error::Error>> {
    let mut fissuras = Vec::new();
    let mut facetas = Vec::new();
    let mut faceta_id_counter = 1;
    let mut fissura_counter = 1;

    // Mock facetas - let's create two for now
    let faceta1_id = format!("F{:02}", faceta_id_counter);
    facetas.push(Faceta {
        id: faceta1_id.clone(),
        orientacao: "Norte".to_string(),
        qtd_rachaduras: 0, // Will be updated based on found fissuras
        observacoes: format!("Observações da faceta {} do {}", faceta1_id, building_name),
    });
    faceta_id_counter += 1;

    let faceta2_id = format!("F{:02}", faceta_id_counter);
    facetas.push(Faceta {
        id: faceta2_id.clone(),
        orientacao: "Leste".to_string(),
        qtd_rachaduras: 0, // Will be updated based on found fissuras
        observacoes: format!("Observações da faceta {} do {}", faceta2_id, building_name),
    });

    if images_dir.exists() && images_dir.is_dir() {
        for entry in fs::read_dir(images_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                // Simulate data for each image found
                let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                let image_path_in_report = format!("images/{}", file_name);

                // Assign to a faceta (e.g., alternating or based on some logic)
                let target_faceta_id = if fissura_counter % 2 == 0 { &faceta2_id } else { &faceta1_id };

                fissuras.push(Fissura {
                    faceta_id: target_faceta_id.clone(),
                    localizacao: format!("Localização simulada {} ({})", fissura_counter, file_name),
                    classificacao: if fissura_counter % 3 == 0 {
                        "Estrutural".to_string()
                    } else if fissura_counter % 3 == 1 {
                        "Térmica".to_string()
                    } else {
                        "Retração plástica".to_string()
                    },
                    descricao: format!("Descrição simulada da fissura {} para a imagem {}.", fissura_counter, file_name),
                    caminho_imagem: image_path_in_report,
                });

                // Update qtd_rachaduras for the faceta
                if let Some(faceta) = facetas.iter_mut().find(|f| f.id == *target_faceta_id) {
                    faceta.qtd_rachaduras += 1;
                }
                fissura_counter += 1;
            }
        }
    }

    Ok((facetas, fissuras))
}

pub fn generate_json_report(
    base_projects_dir_str: &str, // e.g., "src/app-rust/Projects"
    project_name: &str,          // e.g., "Galpão_Logístico_XPTO"
    building_name: &str,         // e.g., "Galpão_3"
) -> Result<ReportData, Box<dyn std::error::Error>> {
    let project_root = Path::new(base_projects_dir_str).join(project_name);
    let images_dir = project_root.join("images");

    println!("Checking for images in: {:?}", images_dir);

    let (facetas, fissuras) = simulate_cpp_model_and_image_processing(&images_dir, project_name, building_name)?;

    let current_date = Local::now().format("%Y-%m-%d").to_string();

    let report_data = ReportData {
        nome_projeto: project_name.to_string(),
        data_analise: current_date,
        nome_responsavel: "Eng. Responsável Simulado".to_string(),
        nome_predio: building_name.to_string(),
        endereco_predio: "Endereço Simulado, 123".to_string(),
        numero_andares: 1, // Mock data
        ano_construcao: 2000, // Mock data
        tipo_estrutura: "Tipo de Estrutura Simulada".to_string(),
        observacoes_gerais: format!("Observações gerais simuladas para o {} - {}.", project_name, building_name),
        facetas,
        fissuras,
        conclusao_geral: "Conclusão geral simulada. Análise requer especialista.".to_string(),
        recomendacoes: "Recomendações simuladas. Verificar todas as fissuras.".to_string(),
        funcao_responsavel: "Engenheiro de Simulação".to_string(),
        nome_empresa: "Simulações & Análises Ltda.".to_string(),
    };

    let report_file_name = format!("Dados-{}-{}.json", project_name, building_name);
    let report_file_path = project_root.join(report_file_name.clone());

    let json_string = serde_json::to_string_pretty(&report_data)?;

    let mut file = File::create(&report_file_path)?;
    file.write_all(json_string.as_bytes())?;

    println!("Report generated successfully at: {:?}", report_file_path);
    Ok(report_data)
}

// The original generate_report function using Handlebars. Keep or remove based on usage.
pub fn generate_report(
    template_text: &str,
    origin_data: &Value
) -> Result<String, handlebars::RenderError> {
    let mut handlebars = Handlebars::new();

    handlebars.register_template_string("t1", template_text)?;

    handlebars.render("t1", origin_data)
}
