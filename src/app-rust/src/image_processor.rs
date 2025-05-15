use std::path::{Path, PathBuf};
use std::fs;
use std::io::{BufReader, BufRead};
use walkdir::WalkDir;
use anyhow::{Result, Context};
use regex::Regex;
use std::collections::HashMap;

use exif::{Tag, In, Reader, Value};

/// Representa uma localização geográfica
#[derive(Debug, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

/// Metadados extraídos de uma imagem
#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub path: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

/// Estatísticas do processamento de imagens
#[derive(Debug)]
pub struct ProcessingStats {
    pub total_images: usize,
    pub images_with_gps: usize,
    pub images_without_gps: usize,
    pub location_groups: usize,
    pub errors: Vec<String>,
}

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
    m
}

/// Processa todas as imagens JPEG em `folder_path`, agrupa por proximidade e copia para pastas
pub fn process_folder(folder_path: &str, threshold: f64) -> Result<ProcessingStats> {
    let mut stats = ProcessingStats {
        total_images: 0,
        images_with_gps: 0,
        images_without_gps: 0,
        location_groups: 0,
        errors: Vec::new(),
    };

    let mut groups: Vec<Vec<(PathBuf, Location)>> = Vec::new();

    // Caminho para arquivo de metadados ExifTool (se existir)
    let exiftool_metadata_path = Path::new(folder_path).join("exiftool_metadata.txt");
    let exiftool_metadata = if exiftool_metadata_path.exists() {
        read_exiftool_metadata(&exiftool_metadata_path)
            .with_context(|| format!("falha ao ler metadados ExifTool de {}", exiftool_metadata_path.display()))?
    } else {
        HashMap::new()
    };

    for entry in WalkDir::new(folder_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().extension()
                .and_then(|ext| ext.to_str())
                .map_or(false, |ext| matches!(ext.to_lowercase().as_str(), "jpg" | "jpeg"))
        })
    {
        stats.total_images += 1;
        let path = entry.path();
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Tenta buscar metadados do ExifTool primeiro
        let location = if let Some(metadata) = exiftool_metadata.get(file_name) {
            get_gps_from_exiftool_string(metadata)
        } else {
            // Caso contrário, usa EXIF diretamente do arquivo
            get_gps_location(path)?
        };

        match location {
            Some(loc) => {
                stats.images_with_gps += 1;
                // agrupa por threshold
                let mut placed = false;
                for grp in &mut groups {
                    if let Some((_, first_loc)) = grp.first() {
                        if are_locations_close(first_loc, &loc, threshold) {
                            grp.push((path.to_path_buf(), loc.clone()));
                            placed = true;
                            break;
                        }
                    }
                }
                if !placed {
                    groups.push(vec![(path.to_path_buf(), loc.clone())]);
                }
            }
            None => stats.images_without_gps += 1,
        }
    }

    // cria pastas por grupo e copia arquivos
    for (i, grp) in groups.iter().enumerate() {
        let dir_name = format!("location_group_{}", i + 1);
        let target_dir = Path::new(folder_path).join(&dir_name);
        if let Err(err) = fs::create_dir_all(&target_dir) {
            stats.errors.push(format!("falha criar {}: {}", target_dir.display(), err));
            continue;
        }
        for (src, _) in grp {
            if let Some(name) = src.file_name() {
                let dst = target_dir.join(name);
                if let Err(err) = fs::copy(src, &dst) {
                    stats.errors.push(format!("falha copiar {} para {}: {}", src.display(), dst.display(), err));
                }
            } else {
                stats.errors.push(format!("nome inválido: {}", src.display()));
            }
        }
    }

    stats.location_groups = groups.len();
    Ok(stats)
}

/// Extrai GPS de um arquivo, via EXIF ou métodos alternativos
fn get_gps_location(path: &Path) -> Result<Option<Location>> {
    // Tenta ler metadados do ExifTool se existir um arquivo com o mesmo nome + .txt
    let exiftool_path = path.with_extension("txt");
    if exiftool_path.exists() {
        if let Ok(metadata) = fs::read_to_string(&exiftool_path) {
            if let Some(loc) = get_gps_from_exiftool_string(&metadata) {
                return Ok(Some(loc));
            }
        }
    }

    // Se não encontrou no arquivo de metadados ExifTool, tenta via EXIF diretamente
    let file = fs::File::open(path)
        .with_context(|| format!("falha abrir {}", path.display()))?;
    let mut buf = BufReader::new(file);

    let exif = match Reader::new().read_from_container(&mut buf) {
        Ok(e) => e,
        Err(_) => return Ok(None),
    };

    let tag_map = nome_para_tag();

    // tenta rationale
    if let Some(loc) = extract_gps_rational(&exif, &tag_map) {
        return Ok(Some(loc));
    }
    // tenta string
    if let Some(loc) = extract_gps_string(&exif, &tag_map) {
        return Ok(Some(loc));
    }
    Ok(None)
}

/// Lê metadados do ExifTool de um arquivo e retorna mapeamento de nome de arquivo para metadados
fn read_exiftool_metadata(path: &Path) -> Result<HashMap<String, String>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    
    let mut result = HashMap::new();
    let mut current_file = String::new();
    let mut current_metadata = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("File Name") {
            // Salva o arquivo anterior (se houver)
            if !current_file.is_empty() && !current_metadata.is_empty() {
                result.insert(current_file.clone(), current_metadata.join("\n"));
                current_metadata.clear();
            }
            
            // Começa um novo arquivo
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                current_file = parts[1].trim().to_string();
            }
        }
        
        if !current_file.is_empty() {
            current_metadata.push(line);
        }
    }
    
    // Salva o último arquivo
    if !current_file.is_empty() && !current_metadata.is_empty() {
        result.insert(current_file, current_metadata.join("\n"));
    }
    
    Ok(result)
}

/// Extrai coordenadas GPS do formato de texto do ExifTool
fn get_gps_from_exiftool_string(metadata: &str) -> Option<Location> {
    let mut gps_position_str: Option<String> = None;
    let mut lat_str: Option<String> = None;
    let mut lon_str: Option<String> = None;
    let mut lat_ref_str: Option<String> = None;
    let mut lon_ref_str: Option<String> = None;

    for line in metadata.lines() {
        let trimmed_line = line.trim();
        if let Some(val) = trimmed_line.strip_prefix("GPS Position") {
            if let Some(actual_val) = val.strip_prefix(':') {
                gps_position_str = Some(actual_val.trim().to_string());
                // Strategy 1: Try to parse GPS Position directly
                if let Some(loc) = parse_position_string(gps_position_str.as_ref().unwrap()) {
                    return Some(loc);
                }
            }
        } else if let Some(val) = trimmed_line.strip_prefix("GPS Latitude") {
            if let Some(actual_val) = val.strip_prefix(':') {
                lat_str = Some(actual_val.trim().to_string());
            }
        } else if let Some(val) = trimmed_line.strip_prefix("GPS Longitude") {
            if let Some(actual_val) = val.strip_prefix(':') {
                lon_str = Some(actual_val.trim().to_string());
            }
        } else if let Some(val) = trimmed_line.strip_prefix("GPS Latitude Ref") {
            if let Some(actual_val) = val.strip_prefix(':') {
                lat_ref_str = Some(actual_val.trim().to_string());
            }
        } else if let Some(val) = trimmed_line.strip_prefix("GPS Longitude Ref") {
            if let Some(actual_val) = val.strip_prefix(':') {
                lon_ref_str = Some(actual_val.trim().to_string());
            }
        }
    }

    // Strategy 2: Fallback to separate Latitude/Longitude fields if GPS Position failed or wasn't present
    if let (Some(lat_s), Some(lon_s)) = (lat_str, lon_str) {
        // Regex to parse "4 deg 23' 40.08\"" or "4 deg 23' 40.08\" S"
        // The direction (S, N, E, W) is optional here as it should ideally come from Ref
        let re_coord_val = Regex::new(r#"(\d+)\s*deg\s*(\d+)'\s*([\d.]+)"(?:\\s*[NSEW])?"#).ok()?;

        let parse_coord_val = |s: &str| -> Option<f64> {
            let caps = re_coord_val.captures(s)?;
            let deg: f64 = caps.get(1)?.as_str().parse().ok()?;
            let min: f64 = caps.get(2)?.as_str().parse().ok()?;
            let sec: f64 = caps.get(3)?.as_str().parse().ok()?;
            Some(deg + (min / 60.0) + (sec / 3600.0))
        };

        if let (Some(mut lat), Some(mut lon)) = (parse_coord_val(&lat_s), parse_coord_val(&lon_s)) {
            let final_lat_ref = lat_ref_str.as_deref().unwrap_or("N");
            let final_lon_ref = lon_ref_str.as_deref().unwrap_or("E");

            if final_lat_ref.contains("South") || final_lat_ref == "S" {
                lat = -lat;
            }
            if final_lon_ref.contains("West") || final_lon_ref == "W" {
                lon = -lon;
            }
            return Some(Location { latitude: lat, longitude: lon });
        }
    }
    
    None
}

fn extract_gps_rational(exif: &exif::Exif, tag_map: &HashMap<&str, Tag>) -> Option<Location> {
    let lat_tag = tag_map.get("GPSLatitude")?;
    let lat_ref_tag = tag_map.get("GPSLatitudeRef")?;
    let lon_tag = tag_map.get("GPSLongitude")?;
    let lon_ref_tag = tag_map.get("GPSLongitudeRef")?;

    let lat_f = exif.get_field(*lat_tag, In::PRIMARY)?;
    let lat_ref = exif.get_field(*lat_ref_tag, In::PRIMARY)?;
    let lon_f = exif.get_field(*lon_tag, In::PRIMARY)?;
    let lon_ref = exif.get_field(*lon_ref_tag, In::PRIMARY)?;

    let degs = match &lat_f.value {
        Value::Rational(r) if r.len() >= 3 => r.clone(),
        _ => return None,
    };
    let lat = degs[0].to_f64() + degs[1].to_f64()/60.0 + degs[2].to_f64()/3600.0;
    let lat_sign = if matches!(&lat_ref.value, Value::Ascii(vals) if vals[0] == b"S") { -1.0 } else { 1.0 };

    let degs2 = match &lon_f.value {
        Value::Rational(r) if r.len() >= 3 => r.clone(),
        _ => return None,
    };
    let lon = degs2[0].to_f64() + degs2[1].to_f64()/60.0 + degs2[2].to_f64()/3600.0;
    let lon_sign = if matches!(&lon_ref.value, Value::Ascii(vals) if vals[0] == b"W") { -1.0 } else { 1.0 };

    Some(Location { latitude: lat * lat_sign, longitude: lon * lon_sign })
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
    let lat_ref = match &lat_ref.value {
        Value::Ascii(v) if !v.is_empty() => String::from_utf8_lossy(&v[0]).to_string(),
        _ => return None,
    };
    let lon_ref = match &lon_ref.value {
        Value::Ascii(v) if !v.is_empty() => String::from_utf8_lossy(&v[0]).to_string(),
        _ => return None,
    };

    let re = Regex::new(r#"(\d+)\s*deg\s*(\d+)'\s*([\d.]+)"#).ok()?;
    let parse_coord = |s: &str| -> Option<f64> {
        let caps = re.captures(s)?;
        let deg: f64 = caps.get(1)?.as_str().parse().ok()?;
        let min: f64 = caps.get(2)?.as_str().parse().ok()?;
        let sec: f64 = caps.get(3)?.as_str().parse().ok()?;
        Some(deg + (min / 60.0) + (sec / 3600.0))
    };

    let lat = parse_coord(&lat_str)?;
    let lon = parse_coord(&lon_str)?;
    let lat = if lat_ref == "S" { -lat } else { lat };
    let lon = if lon_ref == "W" { -lon } else { lon };

    Some(Location { latitude: lat, longitude: lon })
}

fn parse_position_string(s: &str) -> Option<Location> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() < 2 { return None; }
    
    let re = Regex::new(r#"(\d+)\s*deg\s*(\d+)'\s*([\d.]+)"\s*([NSEW])"#).ok()?;
    let parse_coord = |s: &str| -> Option<(f64, char)> {
        let caps = re.captures(s.trim())?;
        let deg: f64 = caps.get(1)?.as_str().parse().ok()?;
        let min: f64 = caps.get(2)?.as_str().parse().ok()?;
        let sec: f64 = caps.get(3)?.as_str().parse().ok()?;
        let dir = caps.get(4)?.as_str().chars().next()?;
        Some((deg + (min / 60.0) + (sec / 3600.0), dir))
    };

    let (lat, lat_dir) = parse_coord(parts[0])?;
    let (lon, lon_dir) = parse_coord(parts[1])?;
    let lat = if lat_dir == 'S' { -lat } else { lat };
    let lon = if lon_dir == 'W' { -lon } else { lon };
    Some(Location { latitude: lat, longitude: lon })
}

/// Função adicional para processar arquivos de metadados do ExifTool diretamente
pub fn process_exiftool_metadata_file(metadata_path: &str, output_folder: &str, threshold: f64) -> Result<ProcessingStats> {
    let mut stats = ProcessingStats {
        total_images: 0,
        images_with_gps: 0,
        images_without_gps: 0,
        location_groups: 0,
        errors: Vec::new(),
    };

    let file = fs::File::open(metadata_path)
        .with_context(|| format!("Falha ao abrir arquivo de metadados: {}", metadata_path))?;
    let reader = BufReader::new(file);

    let mut current_image: Option<String> = None;
    let mut current_metadata = String::new();
    let mut images_with_metadata: Vec<(String, Option<Location>)> = Vec::new();
    
    for line in reader.lines() {
        let line = line.with_context(|| "Erro ao ler linha do arquivo de metadados")?;
        
        if line.starts_with("---") || line.trim().is_empty() {
            if let Some(filename) = &current_image {
                stats.total_images += 1;
                let location = get_gps_from_exiftool_string(&current_metadata);
                if location.is_some() {
                    stats.images_with_gps += 1;
                } else {
                    stats.images_without_gps += 1;
                }
                images_with_metadata.push((filename.clone(), location));
                current_metadata.clear();
            }
            current_image = None;
            continue;
        }
        if line.starts_with("File Name") {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                current_image = Some(parts[1].trim().to_string());
            }
        }
        if current_image.is_some() {
            current_metadata.push_str(&line);
            current_metadata.push('\n');
        }
    }
    if let Some(filename) = &current_image {
        stats.total_images += 1;
        let location = get_gps_from_exiftool_string(&current_metadata);
        if location.is_some() {
            stats.images_with_gps += 1;
        } else {
            stats.images_without_gps += 1;
        }
        images_with_metadata.push((filename.clone(), location));
    }

    let mut groups: Vec<Vec<String>> = Vec::new();
    for (filename, location) in &images_with_metadata {
        if let Some(loc) = location {
            let mut placed = false;
            for group in &mut groups {
                if let Some(first_filename) = group.first() {
                    if let Some((_, Some(first_loc))) = images_with_metadata.iter().find(|(name, _)| name == first_filename) {
                        if are_locations_close(first_loc, loc, threshold) {
                            group.push(filename.clone());
                            placed = true;
                            break;
                        }
                    }
                }
            }
            if !placed {
                groups.push(vec![filename.clone()]);
            }
        }
    }
    stats.location_groups = groups.len();

    for (i, group) in groups.iter().enumerate() {
        let dir_name = format!("location_group_{}", i + 1);
        let target_dir = Path::new(output_folder).join(&dir_name);
        fs::create_dir_all(&target_dir)
            .with_context(|| format!("Erro ao criar diretório {}", target_dir.display()))?;
        let info_path = target_dir.join("group_info.txt");
        let mut info_content = format!("Grupo de localização {}\n", i + 1);
        info_content.push_str("Arquivos neste grupo:\n");
        for filename in group {
            info_content.push_str(&format!("- {}\n", filename));
            let src_path = Path::new(output_folder).join(filename);
            if src_path.exists() {
                let dst_path = target_dir.join(filename);
                fs::copy(&src_path, &dst_path)
                    .with_context(|| format!("Erro ao copiar {} para {}", src_path.display(), dst_path.display()))?;
            }
        }
        fs::write(&info_path, info_content)
            .with_context(|| format!("Erro ao escrever informações do grupo em {}", info_path.display()))?;
    }
    Ok(stats)
}

fn are_locations_close(a: &Location, b: &Location, thr: f64) -> bool {
    (a.latitude - b.latitude).abs() <= thr && (a.longitude - b.longitude).abs() <= thr
}
