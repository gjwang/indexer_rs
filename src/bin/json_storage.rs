/**
 @author: 0xMaster

 Created on: 2023-10-06
 */


use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub(crate) network: String,
    pub(crate) block_num: u64,
}


pub async fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Vec<BlockchainInfo>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let data:Vec<BlockchainInfo> = serde_json::from_str(&content).unwrap();
    Ok(data)
}

pub async fn write_json_file<P: AsRef<Path>>(path: P, data: &Vec<BlockchainInfo> ) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string_pretty(data)?;
    fs::write(path, content)?;
    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let path = "data.json";
//
//     // Read the JSON file
//     let mut data = read_json_file(&path).await?;
//     for blk_chian in data.iter_mut() {
//         println!("{:?}", blk_chian);
//         // Modify the data (for demonstration purposes)
//         blk_chian.block_num += 1;
//     }
//
//     // Write the modified data back to the JSON file
//     write_json_file(&path, &data).await?;
//
//     println!("Updated data: {:?}", data);
//
//     Ok(())
// }