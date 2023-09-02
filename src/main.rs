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
    let height = client
        .repository()
        .read_last_finalization_info()
        .await
        .unwrap()
        .header
        .height;
    println!("last block height: {}", height);

    // let test = client.repository().read_finalization_info(1).await.unwrap();

    // let test2 = client
    //     .repository()
    //     .read_governance_approved_agendas()
    //     .await
    //     .unwrap();

    let commit_1 = "7e2c04181d63b75c1060a016c16d53fea7eb3301".to_string();
    // let commit_2 = "7e2c04181d63b75c1060a016c16d53fea7eb3301".to_string();

    let hash = hex::decode(&commit_1).unwrap().try_into().unwrap();
    let data = client
        .repository()
        .read_commit(CommitHash { hash })
        .await
        .unwrap();

    match data {
        Commit::Transaction(data) => {
            let body = data.body;
            dbg!(body);
        }
        _ => {
            println!("this is not transaction type");
        }
    }

    // dbg!("first commit", data);
    // data.clone().
    // let test = data.into();

    let hash = hex::decode(&commit_1).unwrap().try_into().unwrap();
    let data = client
        .repository()
        .read_commit(CommitHash { hash })
        .await
        .unwrap();

    dbg!("second commit", data);
}
