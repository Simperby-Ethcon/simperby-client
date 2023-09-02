use simperby::types::{Auth, Config};
use simperby::*;
use simperby_core::*;

async fn read_config<T: serde::de::DeserializeOwned>(path: &str) -> Option<T> {
    let content = tokio::fs::read_to_string(path).await.ok()?;
    serde_spb::from_str(&content).ok()
}

#[tokio::main]
async fn main() {
    let path = "/Users/jeongseup/Study/simperby-devnet";
    let config: Config = read_config(&format!("{path}/.simperby/config.json"))
        .await
        .unwrap();

    let auth: Auth = read_config(&format!("{path}/.simperby/auth.json"))
        .await
        .unwrap();

    println!("Hello, world!");

    let client = Client::open(path, Config {}, auth.clone()).await.unwrap();
    // let test = client
    //     .repository()
    //     .read_last_finalization_info()
    //     .await
    //     .unwrap()
    //     .header;

    let test = client.repository().read_agendas().await.unwrap();

    match client.repository().read_blocks().await {
        Ok(blocks) => {
            for (commit_hash, block_hash) in blocks {
                println!("Commit Hash: {:?}", commit_hash);
                println!("Block Hash: {:?}", block_hash);
            }
        }
        Err(err) => {
            eprintln!("Error reading blocks: {:?}", err);
        }
    }
    // dbg!(blocks);
}
