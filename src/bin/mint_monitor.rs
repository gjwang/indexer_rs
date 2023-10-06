extern crate web3;
extern crate ethabi;

use web3::transports::Http;
use web3::Web3;

#[macro_use]extern crate fstrings;
extern crate dotenv;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let infura_api_key = std::env::var("INFURA_API_KEY").expect("INFURA_API_KEY not found");
    println!("INFURA_API_KEY: {}", infura_api_key);

    // let rpc: &String = &format!("https://mainnet.infura.io/v3/{}", infura_api_key);
    let rpc: &String = &f!("https://mainnet.infura.io/v3/{infura_api_key}");

    let http = Http::new(rpc)?;
    let web3 = Web3::new(http);


    // Fetch the latest block number
    let block_number = web3.eth().block_number().await?;

    // Fetch the block data
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
