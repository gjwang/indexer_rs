/**
 @author: 0xMaster

 Created on: 2023-10-06
 */


use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    name: String,
    age: u32,
}

async fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Data, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let data: Data = serde_json::from_str(&content)?;
    Ok(data)
}

async fn write_json_file<P: AsRef<Path>>(path: P, data: &Data) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string_pretty(data)?;
    fs::write(path, content)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = "data.json";

    // Read the JSON file
    let mut data = read_json_file(&path).await?;

    // Modify the data (for demonstration purposes)
    data.age += 1;

    // Write the modified data back to the JSON file
    write_json_file(&path, &data).await?;

    println!("Updated data: {:?}", data);

    Ok(())
}
