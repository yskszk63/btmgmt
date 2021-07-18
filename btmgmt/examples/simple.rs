use futures::StreamExt;

use btmgmt::command::*;
use btmgmt::*;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init();

    let client = Client::new().unwrap();

    let mut events = client.events().await;
    tokio::spawn(async move {
        while let Some(event) = events.next().await {
            dbg!(event);
        }
    });

    let r = client
        .call(None, ReadManagementVersionInformation)
        .await
        .unwrap();
    dbg!(r);
    let r = client
        .call(None, ReadManagementSupportedCommands)
        .await
        .unwrap();
    dbg!(r);
    let r = client
        .call(None, ReadControllerIndexList)
        .await
        .unwrap();
    for index in r.0 {
        let r = client
            .call(index.clone(), ReadControllerInformation)
            .await
            .unwrap();
        println!("{:?}", r);
    }
}
