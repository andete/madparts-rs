// (c) 2018 Joost Yervante Damad <joost@damad.be>

use serde_json;
use std::fs;

#[derive(Debug, Default, Deserialize)]
pub struct Settings {
    pub klc_dir: Option<String>,
}

pub fn load_settings() -> Settings {
    let filename = format!("{}/.madparts.conf", env!("HOME"));
    match fs::read_to_string(&filename) {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|e| {
            warn!("Error json decoding file {}: {}", filename, e);
            Settings::default()
        }),
        Err(e) => {
            info!("Error reading file {}: {:?}", filename, e);
            info!("Using default settings");
            Settings::default()
        }
    }
}
