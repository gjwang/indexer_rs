use redis::aio::Connection;
use redis::AsyncCommands;
use tokio;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con: Connection = client.get_async_connection().await?;

    con.set("my_key", "Hello, Async Redis!").await?;
    let value: String = con.get("my_key").await?;

    println!("my_key: {}", value);

    Ok(())
}
