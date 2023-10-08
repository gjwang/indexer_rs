/**
@author: 0xMaster

Created on: 2023-10-06
 */

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockchainInfo {
    pub(crate) network: String,
    pub(crate) block_num: u64,
}


pub async fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Vec<BlockchainInfo>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let data: Vec<BlockchainInfo> = serde_json::from_str(&content).unwrap();
    Ok(data)
}

pub async fn write_json_file<P: AsRef<Path>>(path: P, data: &Vec<BlockchainInfo>) -> Result<(), Box<dyn std::error::Error>> {
    let content = serde_json::to_string_pretty(data)?;
    fs::write(path, content)?;
    Ok(())
}

fn main() {}