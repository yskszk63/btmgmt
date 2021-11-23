use futures::StreamExt;

use btmgmt::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init();

    let client = Client::open().unwrap();

    let mut events = client.events().await;
    tokio::spawn(async move {
        while let Some(event) = events.next().await {
            println!("1 {:?}", event);
        }
    });

    let mut events = client.events().await;
    tokio::spawn(async move {
        while let Some(event) = events.next().await {
            println!("2 {:?}", event);
        }
    });

    std::future::pending::<()>().await;
}
