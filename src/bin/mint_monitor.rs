use std::time::Duration;

extern crate web3;
extern crate ethabi;

#[macro_use]
extern crate fstrings;

use web3::transports::Http;
use web3::Web3;
use web3::types::{BlockNumber, Transaction, Address, U64, BlockId};

extern crate dotenv;

mod json_storage;
use json_storage::{read_json_file, write_json_file, BlockchainInfo};


async fn monitor_mint_event(block_number: U64, web3: &Web3<Http>) -> web3::Result<()>   {    // Fetch the block data
    let block = web3.eth().block_with_txs(web3::types::BlockId::Number(block_number.into())).await?;

    if let Some(b) = block {
        for tx in b.transactions {
            let receipt = web3.eth().transaction_receipt(tx.hash).await?;
            if let Some(r) = receipt {
                for log in r.logs {
                    let raw_log = (log.topics, log.data.0);
                    // Assuming you have the ERC-721 contract ABI
                    let contract = ethabi::Contract::load(ERC721_ABI.as_bytes()).unwrap();

                    // Try to decode the Transfer event
                    if let Ok(event) = contract.event("Transfer") {
                        if let Ok(decoded) = event.parse_log(raw_log.into()) {
                            let from: ethabi::Address = decoded.params[0].value.clone().into_address().unwrap();
                            let to: ethabi::Address = decoded.params[1].value.clone().into_address().unwrap();

                            if from == ethabi::Address::default() {
                                println!("Mint event detected!");
                                println!("Minted to: {:?}", to);
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("Block data not found.");
    }

    Ok(())
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let infura_api_key = std::env::var("INFURA_API_KEY").expect("INFURA_API_KEY not found");
    println!("INFURA_API_KEY: {}", infura_api_key);

    // let rpc: &String = &format!("https://mainnet.infura.io/v3/{}", infura_api_key);
    let rpc: &String = &f!("https://mainnet.infura.io/v3/{infura_api_key}");

    let http = Http::new(rpc)?;
    let web3 = Web3::new(http);

    loop {
        let path = "data.json";
        let mut  eth_last_blk_num = 0;
        let mut data = read_json_file(&path).await.unwrap();

        for blk_chian in data.iter_mut() {
            println!("{:?}", blk_chian);
            if blk_chian.network == "eth" {
                eth_last_blk_num = blk_chian.block_num;
            }
        }
        println!("eth_last_blk_num={}", eth_last_blk_num);

        let latest_block = web3.eth().block_number().await?.as_u64();
        let start_block = eth_last_blk_num;

        println!("start to scan from{} to {}, late={} blocks", start_block, latest_block, latest_block-start_block);

        for block_num in start_block..=latest_block {
            println!("Scan block_num: {:?}", block_num);
            let block_number = U64::from(block_num);
            monitor_mint_event(block_number, &web3).await?;

            //save last_blk_num
            for blk_chian in data.iter_mut() {
                println!("{:?}", blk_chian);
                if blk_chian.network == "eth" {
                    blk_chian.block_num = block_num;
                    eth_last_blk_num = block_num;
                    println!("Updated ETH block_num: {}", blk_chian.block_num);
                }
            }
            write_json_file(&path, &data).await.unwrap();
            println!("Updated data: {:?}", data);
        }

        let latest_block = web3.eth().block_number().await?.as_u64();
        let delay_blk_num = latest_block - eth_last_blk_num;
        println!("delay_blk_num data: {}", delay_blk_num);

        let mut sleep_sec  = 10;
        if delay_blk_num == 0 {
            sleep_sec = 0;
        }

        // Wait for a specified duration before polling again.
        // You can adjust this duration as needed.
        tokio::time::sleep(Duration::from_secs(sleep_sec)).await;
    }

    // Ok(())
}

// ERC-721 ABI for the Transfer event
// This is a very minimal ABI. In a real application, you'd probably load this from a file or an external source.
const ERC721_ABI: &str = r#"
[
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "name": "_from",
                "type": "address"
            },
            {
                "indexed": true,
                "name": "_to",
                "type": "address"
            },
            {
                "indexed": true,
                "name": "_tokenId",
                "type": "uint256"
            }
        ],
        "name": "Transfer",
        "type": "event"
    }
]
"#;
