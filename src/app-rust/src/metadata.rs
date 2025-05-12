use std::path::Path;
use exif::{In, Tag};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub path: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

pub fn get_image_metadata(path: &Path) -> Result<ImageMetadata> {
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

fn convert_to_decimal_degrees(rational: &[exif::Rational]) -> f64 {
    let degrees = rational[0].to_f64();
    let minutes = rational[1].to_f64();
    let seconds = rational[2].to_f64();
    
    degrees + minutes / 60.0 + seconds / 3600.0
} 