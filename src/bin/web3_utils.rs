use web3::transports::Http;
use web3::Web3;

extern crate dotenv;


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


    Ok(())
}
