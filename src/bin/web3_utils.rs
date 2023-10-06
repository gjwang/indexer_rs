use web3::Web3;
use web3::transports::Http;
use web3::types::{Block, Transaction, Address, U64};
use std::str::FromStr;

extern crate dotenv;

async fn fetch_transactions(web3: &web3::Web3<Http>, address: Address) -> web3::Result<Vec<Transaction>> {
    let mut transactions: Vec<Transaction> = Vec::new();

    // Assuming we're only fetching the last 10 blocks, adjust as needed.
    // Convert U64 to usize for looping
    let latest_block = web3.eth().block_number().await?.as_usize();
    let start_block = (latest_block as isize - 10).max(0) as usize;

    for i in start_block..=latest_block {
        let block_number = web3::types::BlockNumber::Number(U64::from(i));
        // let block: Block<Transaction> = web3.eth().block_with_txs(web3::types::BlockId::Number(i.into())).await?;
        let block_opt = web3.eth().block_with_txs(web3::types::BlockId::Number(block_number)).await?;
        if let Some(block) = block_opt {
            for tx in block.transactions {
                if tx.from == Some(address) || tx.to == Some(address) {
                    transactions.push(tx);
                }
            }
        }
    }

    Ok(transactions)
}


#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    let infura_api_key = std::env::var("INFURA_API_KEY").expect("INFURA_API_KEY not found");
    println!("INFURA_API_KEY: {}", infura_api_key);
    let rpc: &String = &format!("https://mainnet.infura.io/v3/{}", infura_api_key);

    let http = Http::new(rpc)?;
    let web3 = Web3::new(http);

    let block_number = web3.eth().block_number().await?;
    println!("Latest block number: {:?}", block_number);

    println!("Calling accounts.");
    let mut accounts = web3.eth().accounts().await?;
    println!("Accounts: {:?}", accounts);
    accounts.push("00a329c0648769a73afac7f9381e08fb43dbea72".parse().unwrap());

    println!("Calling balance.");
    for account in accounts {
        let balance = web3.eth().balance(account, None).await?;
        println!("Balance of {:?}: {}", account, balance);
    }


    let address_str = "YOUR_ETH_ADDRESS";  // Replace with desired Ethereum address
    let address = Address::from_str(address_str).expect("Invalid address");

    let transactions = fetch_transactions(&web3, address).await?;
    for tx in transactions {
        println!("Transaction: {:?}", tx);
    }

    Ok(())
}
