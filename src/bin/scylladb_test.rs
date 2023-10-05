use std::path::Path;

use anyhow::Result;
use scylla::CloudSessionBuilder;
extern crate dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Connecting to SNI proxy as described in cloud config yaml ...");
    dotenv::dotenv().expect("Failed to read .env file");

    let bundle_config_yaml_file: &String = &std::env::var("SCYLLA_BUNDLE_CONFIG_YAML_FILE")
        .expect("scylla_bundle_config_file not found");

    let session = CloudSessionBuilder::new(Path::new(bundle_config_yaml_file))
        .unwrap()
        .build()
        .await
        .unwrap();

    session.query("CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = {'class' : 'SimpleStrategy', 'replication_factor' : 1}",
                  &[]).await.unwrap();
    session
        .query("DROP TABLE IF EXISTS ks.t;", &[])
        .await
        .unwrap();

    println!("Ok.");

    Ok(())
}

