// use std::fmt::format;
use std::time::Duration;
use futures::future::join_all;

extern crate web3;
extern crate ethabi;

#[macro_use]
extern crate fstrings;

use web3::transports::Http;
use web3::Web3;
use web3::types::{BlockNumber, Transaction, Address, U64, BlockId, Block};
use dirs;
extern crate dotenv;


mod json_storage;
use json_storage::{read_json_file, write_json_file, BlockchainInfo};
use std::fs;

const ERC721_ABI_FILE: &str = "src/abi/erc721_abi.json";


async fn fetch_block(web3: &Web3<Http>, block_num: u64) -> web3::Result<Option<Block<Transaction>>> {
    let block_id = BlockId::Number(BlockNumber::Number(U64::from(block_num)));
    let block = web3.eth().block_with_txs(block_id).await?;
    Ok(block)
}


async fn process_mint_event(block:Block<Transaction>, web3: &Web3<Http>) -> web3::Result<()> {    // Fetch the block data
    let path = ERC721_ABI_FILE;
    let content = fs::read_to_string(path)?;
    let contract = ethabi::Contract::load(content.as_bytes()).unwrap();

    for tx in block.transactions {
        let receipt = web3.eth().transaction_receipt(tx.hash).await?;
        if let Some(r) = receipt {
            for log in r.logs {
                let raw_log = (log.topics, log.data.0);

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

    Ok(())
}

// async fn monitor_mint_event(block_number: U64, web3: &Web3<Http>) -> web3::Result<()> {    // Fetch the block data
//     //TODO batch poll
//     let block = web3.eth().block_with_txs(web3::types::BlockId::Number(block_number.into())).await?;
//
//     let path = ERC721_ABI_FILE;
//     let content = fs::read_to_string(path)?;
//
//     let contract = ethabi::Contract::load(content.as_bytes()).unwrap();
//
//     if let Some(b) = block {
//         for tx in b.transactions {
//             let receipt = web3.eth().transaction_receipt(tx.hash).await?;
//             if let Some(r) = receipt {
//                 for log in r.logs {
//                     let raw_log = (log.topics, log.data.0);
//
//                     // Try to decode the Transfer event
//                     if let Ok(event) = contract.event("Transfer") {
//                         if let Ok(decoded) = event.parse_log(raw_log.into()) {
//                             let from: ethabi::Address = decoded.params[0].value.clone().into_address().unwrap();
//                             let to: ethabi::Address = decoded.params[1].value.clone().into_address().unwrap();
//
//                             if from == ethabi::Address::default() {
//                                 println!("Mint event detected!");
//                                 println!("Minted to: {:?}", to);
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     } else {
//         println!("Block data not found.");
//     }
//
//     Ok(())
// }

async fn batch_request_blocks(start_block: u64, end_block: u64, web3: &Web3<Http>)
    -> Result<Vec<web3::Result<Option<Block<Transaction>>>>, web3::Error> {
    let mut tasks = Vec::new();
    for block_num in start_block..=end_block {
        let web3_clone = web3.clone();
        let task = tokio::spawn(async move {
            fetch_block(&web3_clone, block_num).await
        });
        tasks.push(task);
    }

    let results: Vec<_> = join_all(tasks).await.into_iter().map(|x| x.unwrap()).collect();
    // for block in results {
    //     if let Ok(blk) = block {
    //         if let Some(b) = blk {
    //             println!("Block number: {:?}", b.number);
    //             // println!("Block number: {:?}", b.transactions);
    //         }
    //     }
    // }
    Ok(results)
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let infura_api_key = std::env::var("INFURA_API_KEY").expect("INFURA_API_KEY not found");
    println!("INFURA_API_KEY: {}", infura_api_key);

    let rpc: &String = &f!("https://mainnet.infura.io/v3/{infura_api_key}");

    let http = Http::new(rpc)?;
    let web3 = Web3::new(http);

    let home_dir = dirs::home_dir().unwrap();
    const BASE_PATH: &str = "data/indexer_rs";
    const FILE_NAME: &str = "block_high_data.json";
    let full_filename = home_dir.join(f!("{BASE_PATH}/{FILE_NAME}"));
    println!("full_filename {:?}", full_filename);


    loop {
        let mut  eth_last_blk_num = 0;
        let mut data = read_json_file(&full_filename).await.unwrap();

        for blk_chian in data.iter_mut() {
            println!("{:?}", blk_chian);
            if blk_chian.network == "eth" {
                eth_last_blk_num = blk_chian.block_num;
            }
        }

        let latest_block = web3.eth().block_number().await?.as_u64();
        let start_block = eth_last_blk_num;

        let end_blk = (start_block + 10).min(latest_block);
        println!("start_block={} end_blk={}, latest_block={} late_blk={}", end_blk, start_block, latest_block, latest_block-end_blk);

        let block_vec = batch_request_blocks(start_block, end_blk, &web3).await?;
        for block in block_vec {
            if let Ok(blk) = block {
                if let Some(b) = blk {
                    let block_num = b.number.unwrap().as_u64();
                    println!("Block number: {:?}", b.number);
                    process_mint_event(b, &web3);

                    //TODO: maybe block num will has some gap, missing some blocks
                    //Log it, find it out, handle it
                    //save last_blk_num
                    for blk_chian in data.iter_mut() {
                        println!("{:?}", blk_chian);
                        if blk_chian.network == "eth" {
                            blk_chian.block_num = block_num;
                            eth_last_blk_num = block_num;
                            println!("Updated ETH block_num: {}", blk_chian.block_num);
                        }
                    }
                    write_json_file(&full_filename, &data).await.unwrap();
                    println!("Updated data: {:?}", data);
                }
            }
        }

        let latest_block = web3.eth().block_number().await?.as_u64();
        let delay_blk_num = latest_block - eth_last_blk_num;
        println!("delay_blk_num data: {}", delay_blk_num);

        let mut sleep_sec  = 10;
        if delay_blk_num == 0 {
            sleep_sec = 0;
        }
        // Wait for a specified duration before polling again.
        tokio::time::sleep(Duration::from_secs(sleep_sec)).await;
    }

    // Ok(())
}
