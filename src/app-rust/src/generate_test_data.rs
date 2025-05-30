// Arquivo: src/data_generator.rs
// Script para gerar dados de teste que simula a saída do modelo de classificação

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use chrono::Local;
use rand::Rng;

pub fn generate_sample_csv() -> Result<(), Box<dyn std::error::Error>> {
    // Cria a pasta data se não existir
    let data_dir = Path::new("./data");
    if !data_dir.exists() {
        fs::create_dir_all(data_dir)?;
        println!("📁 Pasta 'data' criada");
    }

    let file_path = data_dir.join("teste_fissuras.csv");
    let mut file = File::create(&file_path)?;

    // Cabeçalho CSV
    writeln!(file, "predio,termica,retracao")?;

    // Dados simulados mais realistas
    let predios = vec![
        ("Edifício Residencial Alpha", (8.0, 15.0), (12.0, 28.0)),
        ("Torre Comercial Beta", (5.0, 12.0), (15.0, 35.0)), 
        ("Complexo Industrial Gamma", (15.0, 30.0), (8.0, 20.0)),
        ("Centro Empresarial Delta", (6.0, 18.0), (18.0, 32.0)),
        ("Prédio Habitacional Epsilon", (9.0, 16.0), (10.0, 25.0)),
        ("Bloco Residencial Zeta", (4.0, 11.0), (14.0, 30.0)),
        ("Edifício Comercial Eta", (12.0, 25.0), (7.0, 18.0)),
        ("Torre Residencial Theta", (7.0, 14.0), (16.0, 28.0)),
        ("Complexo Misto Iota", (18.0, 32.0), (5.0, 15.0)),
        ("Centro Logístico Kappa", (3.0, 8.0), (20.0, 35.0)),
    ];

    let mut rng = rand::thread_rng();

    for (nome, (min_termica, max_termica), (min_retracao, max_retracao)) in predios {
        let termica = rng.gen_range(min_termica..=max_termica);
        let retracao = rng.gen_range(min_retracao..=max_retracao);
        
        writeln!(file, "{},{:.1},{:.1}", nome, termica, retracao)?;
    }

    println!("✅ Arquivo CSV gerado: {}", file_path.display());
    println!("📊 {} registros criados", 10);
    
    Ok(())
}

pub fn update_csv_periodically() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = Path::new("./data");
    let file_path = data_dir.join("teste_fiss)