use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Faceta {
    pub id: String,
    pub orientacao: String,
    #[serde(rename = "qtd_rachaduras")]
    pub qtd_rachaduras: u32,
    pub observacoes: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fissura {
    pub faceta_id: String,
    pub localizacao: String,
    pub classificacao: String, // "Estrutural", "Térmica", "Retração plástica", "Deslocamento"
    pub descricao: String,
    pub caminho_imagem: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportData {
    pub nome_projeto: String,
    pub data_analise: String, // e.g., "YYYY-MM-DD"
    pub nome_responsavel: String,
    pub nome_predio: String,
    pub endereco_predio: String,
    pub numero_andares: u32,
    pub ano_construcao: u32,
    pub tipo_estrutura: String,
    pub observacoes_gerais: String,
    pub facetas: Vec<Faceta>,
    pub fissuras: Vec<Fissura>,
    pub conclusao_geral: String,
    pub recomendacoes: String,
    pub funcao_responsavel: String,
    pub nome_empresa: String,
} 