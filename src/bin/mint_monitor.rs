extern crate web3;
extern crate ethabi;

use web3::transports::Http;
use web3::types::U64;
use web3::Web3;

#[macro_use]
extern crate fstrings;
extern crate dotenv;

async fn monitor_mint_event(block_number: U64, web3: Web3<Http>) -> web3::Result<()>   {    // Fetch the block data
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
        // Fetch the latest block number
        match web3.eth().block_number().await {
            Ok(block_num) => {
                // Fetch the latest block number
                let block_number = web3.eth().block_number().await?;
                println!("block_number {:?}", block_number);


                //How to reuse web3?
                let http = Http::new(rpc)?;
                let web3 = Web3::new(http);
                monitor_mint_event(block_number, web3).await?;

                println!("Latest Ethereum block number: {}", block_num);
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await; // Sleep for 10 seconds before querying again
            }

            Err(e) => {
                eprintln!("Failed to fetch latest block number: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await; // If there's an error, sleep for 60 seconds before retrying
            }
        }
    }


    Ok(())
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
