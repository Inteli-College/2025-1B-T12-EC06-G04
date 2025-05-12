use std::path::Path;
use std::fs;
use walkdir::WalkDir;
use exif::{In, Tag};
use anyhow::Result;

// Estrutura para armazenar metadados da imagem
#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub path: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

// Função principal para processar as imagens
pub fn process_images(folder_path: &str, threshold: f64) -> Result<()> {
    let mut images = Vec::new();
    
    // Coleta todas as imagens e seus metadados
    for entry in WalkDir::new(folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let path = e.path();
            path.extension()
                .map(|ext| {
                    let ext = ext.to_string_lossy().to_lowercase();
                    ext == "jpg" || ext == "jpeg" || ext == "png"
                })
                .unwrap_or(false)
        }) {
        if let Ok(metadata) = get_image_metadata(entry.path()) {
            images.push(metadata);
        }
    }

    // Agrupa imagens por proximidade
    let mut groups = Vec::new();
    let mut processed = vec![false; images.len()];

    for i in 0..images.len() {
        if processed[i] {
            continue;
        }

        let mut group = Vec::new();
        group.push(images[i].clone());
        processed[i] = true;

        for j in (i + 1)..images.len() {
            if processed[j] {
                continue;
            }

            if let (Some(lat1), Some(lon1), Some(lat2), Some(lon2)) = (
                images[i].latitude,
                images[i].longitude,
                images[j].latitude,
                images[j].longitude,
            ) {
                if calculate_distance(lat1, lon1, lat2, lon2) <= threshold {
                    group.push(images[j].clone());
                    processed[j] = true;
                }
            }
        }

        if !group.is_empty() {
            groups.push(group);
        }
    }

    // Cria pastas e move as imagens
    for (i, group) in groups.iter().enumerate() {
        let group_folder = Path::new(folder_path).join(format!("grupo_{}", i + 1));
        fs::create_dir_all(&group_folder)?;

        for image in group {
            let source_path = Path::new(&image.path);
            let file_name = source_path.file_name().unwrap();
            let destination = group_folder.join(file_name);
            fs::copy(source_path, destination)?;
        }
    }

    Ok(())
}

// Função para extrair metadados da imagem
fn get_image_metadata(path: &Path) -> Result<ImageMetadata> {
    let file = std::fs::File::open(path)?;
    let mut bufreader = std::io::BufReader::new(&file);
    let exif = exif::Reader::new().read_from_container(&mut bufreader)?;

    let mut latitude = None;
    let mut longitude = None;

    if let Some(lat) = exif.get_field(Tag::GPSLatitude) {
        if let Some(lon) = exif.get_field(Tag::GPSLongitude) {
            if let (In::Rational(lat_rat), In::Rational(lon_rat)) = (lat.value, lon.value) {
                if lat_rat.len() >= 3 && lon_rat.len() >= 3 {
                    latitude = Some(convert_to_decimal_degrees(&lat_rat));
                    longitude = Some(convert_to_decimal_degrees(&lon_rat));
                }
            }
        }
    }

    Ok(ImageMetadata {
        path: path.to_string_lossy().into_owned(),
        latitude,
        longitude,
    })
}

// Converte coordenadas de graus/minutos/segundos para graus decimais
fn convert_to_decimal_degrees(rational: &[exif::Rational]) -> f64 {
    let degrees = rational[0].to_f64();
    let minutes = rational[1].to_f64();
    let seconds = rational[2].to_f64();
    
    degrees + minutes / 60.0 + seconds / 3600.0
}

// Calcula a distância entre duas coordenadas
fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    ((lat2 - lat1).powi(2) + (lon2 - lon1).powi(2)).sqrt()
} 