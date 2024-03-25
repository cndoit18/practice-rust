use mini_redis::{client, Result};

const ADDRESS: &'static str = "127.0.0.1:6379";

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect(ADDRESS).await?;
    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;
    println!("从服务端获得到结果={:?}", result);
    Ok(())
}