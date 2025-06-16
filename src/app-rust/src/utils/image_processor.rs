// image_processor.rs
use std::path::{Path, PathBuf};
use std::fs;
use std::io::BufReader;
use walkdir::WalkDir;
use anyhow::{Result, Context, anyhow};
use regex::Regex;
use std::collections::HashMap;
use std::process::Command;
use exif::{Tag, In, Reader, Value};
use crate::pages::create_project::PROJECT_NAME;
use dioxus::prelude::Readable;

// Representa uma localização geográfica
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

// Metadados extraídos de uma imagem
#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub path: PathBuf,
    pub file_name: String,
    pub location: Option<Location>,
    pub gps_img_direction: Option<f64>, // Em graus, 0-359.99, Norte verdadeiro
}

// Representa uma fachada de um prédio
#[derive(Debug, Clone)]
pub struct Fachada {
    pub _nome: String, // "Norte", "Sul", "Leste", "Oeste", "Indefinida"
    pub imagens: Vec<ImageMetadata>,
}

// Representa um prédio
#[derive(Debug, Clone)]
pub struct Predio {
    pub id: String, // Ex: "Predio-1"
    pub centroide: Location,
    pub fachadas: HashMap<String, Fachada>,
    pub todas_imagens_no_predio: Vec<ImageMetadata>,
}

// Estatísticas do processamento de imagens (mantida para compatibilidade e informação)
#[derive(Debug, Clone, Default)]
pub struct ProcessingStats {
    pub total_images: usize,
    pub images_with_gps: usize,
    pub images_without_gps: usize,
    pub images_with_direction: usize,
    pub predio_groups: usize, // Renomeado de location_groups
    pub errors: Vec<String>,
}

// Mapeamento de nomes de tags para Tags EXIF (atualizado)
fn nome_para_tag() -> HashMap<&'static str, Tag> {
    let mut m = HashMap::new();
    m.insert("GPSLatitude", Tag::GPSLatitude);
    m.insert("GPSLongitude", Tag::GPSLongitude);
    m.insert("GPSLatitudeRef", Tag::GPSLatitudeRef);
    m.insert("GPSLongitudeRef", Tag::GPSLongitudeRef);
    m.insert("GPSAltitude", Tag::GPSAltitude);
    m.insert("GPSAltitudeRef", Tag::GPSAltitudeRef);
    m.insert("GPSDateStamp", Tag::GPSDateStamp);
    m.insert("GPSTimeStamp", Tag::GPSTimeStamp);
    m.insert("GPSProcessingMethod", Tag::GPSProcessingMethod);
    m.insert("GPSMapDatum", Tag::GPSMapDatum);
    m.insert("GPSImgDirection", Tag::GPSImgDirection); // Adicionado
    m.insert("GPSImgDirectionRef", Tag::GPSImgDirectionRef); // Adicionado
    m
}

// Calcula a distância Haversine entre duas localizações em metros
fn haversine_distance(loc1: &Location, loc2: &Location) -> f64 {
    const R: f64 = 6371000.0; // Raio da Terra em metros
    let d_lat = (loc2.latitude - loc1.latitude).to_radians();
    let d_lon = (loc2.longitude - loc1.longitude).to_radians();
    let lat1_rad = loc1.latitude.to_radians();
    let lat2_rad = loc2.latitude.to_radians();

    let a = (d_lat / 2.0).sin().powi(2) + lat1_rad.cos() * lat2_rad.cos() * (d_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    R * c
}

// Calcula o centroide de uma lista de localizações
fn calculate_centroid(locations: &[Location]) -> Option<Location> {
    if locations.is_empty() {
        return None;
    }
    let mut sum_lat = 0.0;
    let mut sum_lon = 0.0;
    for loc in locations {
        sum_lat += loc.latitude;
        sum_lon += loc.longitude;
    }
    Some(Location {
        latitude: sum_lat / locations.len() as f64,
        longitude: sum_lon / locations.len() as f64,
    })
}

// Determina a fachada com base na direção da imagem
fn determinar_fachada_nome(direction: Option<f64>) -> String {
    match direction {
        Some(dir) => {
            if (dir >= 315.0 && dir < 360.0) || (dir >= 0.0 && dir < 45.0) {
                "Norte".to_string()
            } else if dir >= 45.0 && dir < 135.0 {
                "Leste".to_string()
            } else if dir >= 135.0 && dir < 225.0 {
                "Sul".to_string()
            } else if dir >= 225.0 && dir < 315.0 {
                "Oeste".to_string()
            } else {
                "Indefinida".to_string() // Caso de valor inválido fora de 0-360
            }
        }
        None => "Indefinida".to_string(),
    }
}

// Verifica se o exiftool está disponível no sistema
fn is_exiftool_available() -> bool {
    match Command::new("exiftool").arg("-ver").output() {
        Ok(_) => true,
        Err(_) => false,
    }
}

// Função para converter coordenadas no formato "X deg Y' Z.ZZ" S/N/E/W para decimal
fn parse_dms_to_decimal(dms_str: &str) -> Option<f64> {
    // Regex para capturar graus, minutos, segundos e direção
    // Formato: "16 deg 38' 18.20" S" ou variações
    let re = Regex::new(r#"(\d+)\s*deg\s*(\d+)'\s*([\d.]+)"?\s*([NSEW])?"#).ok()?;
    
    if let Some(caps) = re.captures(dms_str) {
        let degrees: f64 = caps.get(1)?.as_str().parse().ok()?;
        let minutes: f64 = caps.get(2)?.as_str().parse().ok()?;
        let seconds: f64 = caps.get(3)?.as_str().parse().ok()?;
        
        let mut decimal = degrees + (minutes / 60.0) + (seconds / 3600.0);
        
        // Aplicar sinal negativo se for Sul ou Oeste
        if let Some(direction) = caps.get(4) {
            let dir = direction.as_str();
            if dir == "S" || dir == "W" {
                decimal = -decimal;
            }
        }
        
        return Some(decimal);
    }
    
    // Tenta outro formato: "16.6384 S" ou "161.1255 E"
    let re_decimal = Regex::new(r#"([\d.-]+)\s*([NSEW])"#).ok()?;
    if let Some(caps) = re_decimal.captures(dms_str) {
        let mut decimal: f64 = caps.get(1)?.as_str().parse().ok()?;
        let dir = caps.get(2)?.as_str();
        
        if dir == "S" || dir == "W" {
            decimal = -decimal;
        }
        
        return Some(decimal);
    }
    
    // Tenta formato puramente numérico
    if let Ok(decimal) = dms_str.trim().parse::<f64>() {
        return Some(decimal);
    }
    
    None
}

// Extrai metadados usando exiftool (mais robusto)
fn extract_image_metadata_exiftool(path: &Path) -> Result<ImageMetadata> {
    let file_name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
    let mut image_meta = ImageMetadata {
        path: path.to_path_buf(),
        file_name,
        location: None,
        gps_img_direction: None,
    };

    // Executa exiftool para obter metadados completos
    let output = Command::new("exiftool")
        .arg(path)
        .output()
        .with_context(|| format!("Falha ao executar exiftool para {}", path.display()))?;

    if !output.status.success() {
        return Err(anyhow!("exiftool falhou com código de saída: {}", output.status));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Primeiro tenta extrair GPS Position (que contém latitude e longitude juntos)
    let re_gps_position = Regex::new(r#"GPS Position\s*:\s*(.+)"#).unwrap();
    if let Some(caps) = re_gps_position.captures(&stdout) {
        if let Some(position_str) = caps.get(1) {
            // Tenta extrair latitude e longitude do GPS Position
            let position = position_str.as_str();
            
            // Formato típico: "16 deg 38' 18.20\" S, 161 deg 7' 31.68\" E"
            let parts: Vec<&str> = position.split(',').collect();
            if parts.len() >= 2 {
                let lat_str = parts[0].trim();
                let lon_str = parts[1].trim();
                
                if let (Some(lat), Some(lon)) = (parse_dms_to_decimal(lat_str), parse_dms_to_decimal(lon_str)) {
                    image_meta.location = Some(Location { latitude: lat, longitude: lon });
                }
            }
        }
    } else {
        // Se não encontrou GPS Position, tenta extrair latitude e longitude separadamente
        let re_lat = Regex::new(r#"GPS Latitude\s*:\s*(.+)"#).unwrap();
        let re_lon = Regex::new(r#"GPS Longitude\s*:\s*(.+)"#).unwrap();
        let re_lat_ref = Regex::new(r#"GPS Latitude Ref\s*:\s*([NS])"#).unwrap();
        let re_lon_ref = Regex::new(r#"GPS Longitude Ref\s*:\s*([EW])"#).unwrap();

        let mut lat_val: Option<f64> = None;
        let mut lon_val: Option<f64> = None;
        let mut lat_ref: Option<&str> = None;
        let mut lon_ref: Option<&str> = None;

        // Extrai latitude
        if let Some(caps) = re_lat.captures(&stdout) {
            if let Some(lat_str) = caps.get(1) {
                lat_val = parse_dms_to_decimal(lat_str.as_str());
            }
        }

        // Extrai longitude
        if let Some(caps) = re_lon.captures(&stdout) {
            if let Some(lon_str) = caps.get(1) {
                lon_val = parse_dms_to_decimal(lon_str.as_str());
            }
        }

        // Extrai referências (N/S, E/W)
        if let Some(caps) = re_lat_ref.captures(&stdout) {
            if let Some(ref_str) = caps.get(1) {
                lat_ref = Some(ref_str.as_str());
            }
        }

        if let Some(caps) = re_lon_ref.captures(&stdout) {
            if let Some(ref_str) = caps.get(1) {
                lon_ref = Some(ref_str.as_str());
            }
        }

        // Combina os valores e referências
        if let (Some(mut lat), Some(mut lon)) = (lat_val, lon_val) {
            // Aplica referências se não foram aplicadas pelo parser
            if let Some("S") = lat_ref {
                if lat > 0.0 { lat = -lat; }
            }
            if let Some("W") = lon_ref {
                if lon > 0.0 { lon = -lon; }
            }
            
            image_meta.location = Some(Location { latitude: lat, longitude: lon });
        }
    }

    // Extrai GPSImgDirection
    let re_direction = Regex::new(r#"GPS Img Direction\s*:\s*(.+)"#).unwrap();
    if let Some(caps) = re_direction.captures(&stdout) {
        if let Some(dir_str) = caps.get(1) {
            // Tenta converter para float
            if let Ok(direction) = dir_str.as_str().trim().parse::<f64>() {
                image_meta.gps_img_direction = Some(direction);
            }
        }
    }

    // Debug: Imprime informações sobre a extração
    if image_meta.location.is_some() {
        println!("ExifTool extraiu GPS para {}: {:?}", path.display(), image_meta.location);
    } else {
        println!("ExifTool não encontrou GPS para {}", path.display());
    }

    Ok(image_meta)
}

// Extrai metadados (localização e direção) de um arquivo de imagem usando a biblioteca exif
fn extract_image_metadata_lib(path: &Path, tag_map: &HashMap<&str, Tag>) -> Result<ImageMetadata> {
    let file_name = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
    let mut image_meta = ImageMetadata {
        path: path.to_path_buf(),
        file_name,
        location: None,
        gps_img_direction: None,
    };

    let file = fs::File::open(path)
        .with_context(|| format!("Falha ao abrir {}", path.display()))?;
    let mut buf_reader = BufReader::new(file);

    let exif_reader = Reader::new();
    match exif_reader.read_from_container(&mut buf_reader) {
        Ok(exif_data) => {
            // Extrair localização (usando lógica similar à original)
            if let Some(loc) = extract_gps_rational(&exif_data, tag_map) {
                image_meta.location = Some(loc);
            } else if let Some(loc) = extract_gps_string(&exif_data, tag_map) {
                image_meta.location = Some(loc);
            }

            // Extrair GPSImgDirection
            if let Some(direction_field) = exif_data.get_field(Tag::GPSImgDirection, In::PRIMARY) {
                match &direction_field.value {
                    Value::Rational(ref v) if !v.is_empty() => {
                        image_meta.gps_img_direction = Some(v[0].to_f64());
                    }
                    _ => { /* não é rational ou está vazio */ }
                }
            }
        }
        Err(_e) => { /* Falha ao ler EXIF, metadados permanecem None */ }
    };

    Ok(image_meta)
}

// Função principal para extrair metadados, tentando primeiro com a biblioteca e depois com exiftool
fn extract_image_metadata(path: &Path, tag_map: &HashMap<&str, Tag>, use_exiftool: bool) -> Result<ImageMetadata> {
    // Se exiftool está disponível, tenta primeiro com ele (mais confiável)
    if use_exiftool {
        let exiftool_result = extract_image_metadata_exiftool(path);
        if exiftool_result.is_ok() && exiftool_result.as_ref().unwrap().location.is_some() {
            return exiftool_result;
        }
    }
    
    // Se exiftool falhou ou não está disponível, tenta com a biblioteca exif
    let lib_result = extract_image_metadata_lib(path, tag_map);
    
    // Se ambos falharam, retorna o resultado da biblioteca (que provavelmente é um erro ou sem GPS)
    lib_result
}

// Funções auxiliares para extrair GPS (mantidas e usadas por extract_image_metadata_lib)
fn extract_gps_rational(exif: &exif::Exif, tag_map: &HashMap<&str, Tag>) -> Option<Location> {
    let lat_tag = tag_map.get("GPSLatitude")?;
    let lat_ref_tag = tag_map.get("GPSLatitudeRef")?;
    let lon_tag = tag_map.get("GPSLongitude")?;
    let lon_ref_tag = tag_map.get("GPSLongitudeRef")?;

    let lat_f = exif.get_field(*lat_tag, In::PRIMARY)?;
    let lat_ref_f = exif.get_field(*lat_ref_tag, In::PRIMARY)?;
    let lon_f = exif.get_field(*lon_tag, In::PRIMARY)?;
    let lon_ref_f = exif.get_field(*lon_ref_tag, In::PRIMARY)?;

    let lat_val = match &lat_f.value {
        Value::Rational(r) if r.len() >= 3 => r.clone(),
        _ => return None,
    };
    let mut lat = lat_val[0].to_f64() + lat_val[1].to_f64()/60.0 + lat_val[2].to_f64()/3600.0;
    if let Value::Ascii(ref ascii_val) = lat_ref_f.value {
        if !ascii_val.is_empty() && ascii_val[0].as_slice() == b"S" {
            lat = -lat;
        }
    }

    let lon_val = match &lon_f.value {
        Value::Rational(r) if r.len() >= 3 => r.clone(),
        _ => return None,
    };
    let mut lon = lon_val[0].to_f64() + lon_val[1].to_f64()/60.0 + lon_val[2].to_f64()/3600.0;
    if let Value::Ascii(ref ascii_val) = lon_ref_f.value {
        if !ascii_val.is_empty() && ascii_val[0].as_slice() == b"W" {
            lon = -lon;
        }
    }
    Some(Location { latitude: lat, longitude: lon })
}

fn extract_gps_string(exif: &exif::Exif, tag_map: &HashMap<&str, Tag>) -> Option<Location> {
    let lat_tag = tag_map.get("GPSLatitude")?;
    let lat_ref_tag = tag_map.get("GPSLatitudeRef")?;
    let lon_tag = tag_map.get("GPSLongitude")?;
    let lon_ref_tag = tag_map.get("GPSLongitudeRef")?;

    let lat_f = exif.get_field(*lat_tag, In::PRIMARY)?;
    let lat_ref = exif.get_field(*lat_ref_tag, In::PRIMARY)?;
    let lon_f = exif.get_field(*lon_tag, In::PRIMARY)?;
    let lon_ref = exif.get_field(*lon_ref_tag, In::PRIMARY)?;

    let lat_str = match &lat_f.value {
        Value::Ascii(v) if !v.is_empty() => String::from_utf8_lossy(&v[0]).to_string(),
        _ => return None,
    };
    let lon_str = match &lon_f.value {
        Value::Ascii(v) if !v.is_empty() => String::from_utf8_lossy(&v[0]).to_string(),
        _ => return None,
    };
    let lat_ref_str = match &lat_ref.value {
        Value::Ascii(v) if !v.is_empty() => String::from_utf8_lossy(&v[0]).to_string(),
        _ => return None,
    };
    let lon_ref_str = match &lon_ref.value {
        Value::Ascii(v) if !v.is_empty() => String::from_utf8_lossy(&v[0]).to_string(),
        _ => return None,
    };

    // Usa a função parse_dms_to_decimal para converter strings DMS para decimal
    let lat_with_ref = format!("{} {}", lat_str, lat_ref_str);
    let lon_with_ref = format!("{} {}", lon_str, lon_ref_str);
    
    let lat = parse_dms_to_decimal(&lat_with_ref)?;
    let lon = parse_dms_to_decimal(&lon_with_ref)?;

    Some(Location { latitude: lat, longitude: lon })
}

// Função para sanitizar nomes de arquivos/diretórios
fn sanitize_filename(name: &str) -> String {
    let forbidden_chars: &[char] = &['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    name.replace(' ', "_")
        .chars()
        .filter(|c| !forbidden_chars.contains(c))
        .collect()
}

// Função principal de processamento (MODIFICADA SIGNIFICATIVAMENTE)
pub fn process_folder(folder_path_str: &str, distance_threshold_meters: f64) -> Result<ProcessingStats> {
    let project_name = match PROJECT_NAME.try_read() {
        Ok(guard) => match &*guard {
            Some(name) => name.clone(),
            None => return Err(anyhow!("Nome do projeto não definido")),
        },
        Err(_) => return Err(anyhow!("Erro ao ler nome do projeto")),
    };

    // Construct path relative to CARGO_MANIFEST_DIR (src/app-rust/Projects)
    let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let images_base_path = base_dir.join("Projects").join(&project_name).join("images");

    let input_folder_path = Path::new(folder_path_str); // Path for input images
    let tag_map = nome_para_tag();
    
    // Verifica se exiftool está disponível
    let use_exiftool = is_exiftool_available();
    if use_exiftool {
        println!("ExifTool encontrado, usando para extração de metadados.");
    } else {
        println!("ExifTool não encontrado, usando apenas biblioteca exif.");
    }

    let mut stats = ProcessingStats::default();
    let mut all_image_metadata: Vec<ImageMetadata> = Vec::new();

    for entry in WalkDir::new(input_folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension()
                .and_then(|ext| ext.to_str())
                .map_or(false, |ext| matches!(ext.to_lowercase().as_str(), 
                    "jpg" | "jpeg" | "png" | "gif" | "bmp" | "webp" | "tiff" | "tif"
                ))
        })
    {
        stats.total_images += 1;
        let path = entry.path();
        
        // Tenta extrair metadados com biblioteca e fallback para exiftool
        match extract_image_metadata(path, &tag_map, use_exiftool) {
            Ok(metadata) => {
                if metadata.location.is_some() {
                    stats.images_with_gps += 1;
                }
                if metadata.gps_img_direction.is_some() {
                    stats.images_with_direction += 1;
                }
                all_image_metadata.push(metadata);
            }
            Err(e) => {
                stats.errors.push(format!("Erro ao processar metadados de {}: {}", path.display(), e));
            }
        }
    }

    let images_with_location: Vec<ImageMetadata> = all_image_metadata
        .into_iter()
        .filter(|meta| meta.location.is_some())
        .collect();

    if images_with_location.is_empty() {
        stats.errors.push("Nenhuma imagem com dados GPS encontrada.".to_string());
        stats.images_without_gps = stats.total_images;
        return Ok(stats);
    }

    let mut predios: Vec<Predio> = Vec::new();
    
    // Agrupamento de imagens em prédios
    let mut images_to_assign = images_with_location.clone();

    for image_meta in images_to_assign.drain(..) {
        let mut assigned_to_existing = false;
        for predio in predios.iter_mut() {
            if haversine_distance(&image_meta.location.unwrap(), &predio.centroide) <= distance_threshold_meters {
                predio.todas_imagens_no_predio.push(image_meta.clone());
                let locs: Vec<Location> = predio.todas_imagens_no_predio.iter().map(|im| im.location.unwrap()).collect();
                predio.centroide = calculate_centroid(&locs).unwrap_or(predio.centroide);
                assigned_to_existing = true;
                break;
            }
        }
        if !assigned_to_existing {
            let new_predio_id = format!("Predio-{}", predios.len() + 1);
            predios.push(Predio {
                id: new_predio_id,
                centroide: image_meta.location.unwrap(),
                fachadas: HashMap::new(),
                todas_imagens_no_predio: vec![image_meta.clone()],
            });
        }
    }

    // Classificação de Fachadas e Criação de Pastas
    for predio in predios.iter_mut() {
        let sanitized_predio_id = sanitize_filename(&predio.id); // Sanitizar ID do prédio
        let predio_target_dir = images_base_path.join(&sanitized_predio_id); // Usar ID sanitizado
        if let Err(e) = fs::create_dir_all(&predio_target_dir) {
            stats.errors.push(format!("Erro ao criar pasta do prédio {}: {}", sanitized_predio_id, e));
            continue;
        }

        for image_data in &predio.todas_imagens_no_predio {
            let fachada_nome_str = determinar_fachada_nome(image_data.gps_img_direction);
            let base_fachada_dir_name = format!("fachada-{}", fachada_nome_str);
            let sanitized_fachada_dir_name = sanitize_filename(&base_fachada_dir_name); // Sanitizar nome da fachada
            
            let fachada_entry = predio.fachadas.entry(fachada_nome_str.clone()).or_insert_with(|| Fachada {
                _nome: fachada_nome_str.clone(),
                imagens: Vec::new(),
            });
            fachada_entry.imagens.push(image_data.clone());

            // Criar pasta da fachada e copiar imagem
            let fachada_target_dir = predio_target_dir.join(&sanitized_fachada_dir_name); // Usar nome sanitizado
            if let Err(e) = fs::create_dir_all(&fachada_target_dir) {
                stats.errors.push(format!("Erro ao criar diretório {}: {}", fachada_target_dir.display(), e));
                continue; // Pula para a próxima imagem se não puder criar a pasta da fachada
            }

            let sanitized_image_filename = sanitize_filename(&image_data.file_name); // Sanitizar nome do arquivo da imagem
            let target_image_path = fachada_target_dir.join(&sanitized_image_filename); // Usar nome do arquivo sanitizado
            
            // Tentar copiar o arquivo
            match fs::copy(&image_data.path, &target_image_path) {
                Ok(_) => {
                    // Se a cópia foi bem-sucedida, tentar apagar o arquivo original
                    if let Err(e_remove) = fs::remove_file(&image_data.path) {
                        stats.errors.push(format!(
                            "Falha ao apagar arquivo original {}: {:?}",
                            image_data.path.display(),
                            e_remove
                        ));
                    }
                }
                Err(e_copy) => {
                    stats.errors.push(format!(
                        "Falha ao copiar {} para {}: {:?}",
                        image_data.path.display(),
                        target_image_path.display(),
                        e_copy
                    ));
                }
            }
        }
    }

    stats.predio_groups = predios.len();
    stats.images_without_gps = stats.total_images - stats.images_with_gps;

    Ok(stats)
}
