use std::fs;

use settings::Settings;

pub mod settings;

pub fn load(path: &str) -> Result<Settings, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let settings: Settings = toml::from_str(&contents)?;
    Ok(settings)
}
