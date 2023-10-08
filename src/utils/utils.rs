use std::fs;
use std::path::Path;
use std::time::Duration;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub async fn sleep(secs: u64) {
    tokio::time::sleep(Duration::from_secs(secs)).await;
}

pub async fn sleep_millis(millis: u64) {
    tokio::time::sleep(Duration::from_millis(millis)).await;
}


pub async fn read_json_file<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let data: T = serde_json::from_str(&content)?;
    Ok(data)
}

pub async fn write_json_file<T: Serialize>(data: &T, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string(data)?;
    fs::write(path, content)?;
    Ok(())
}
