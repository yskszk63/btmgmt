use btmgmt::command::*;
use btmgmt::*;

use tokio::stream::StreamExt;

#[tokio::main]
async fn main() {
    let (client, handle) = Client::open().unwrap();

    let mut events = client.events().await;
    tokio::spawn(async move {
        while let Some(event) = events.next().await {
            dbg!(event);
        }
    });

    let r = client
        .call(None, ReadManagementVersionInformation::new())
        .await
        .unwrap();
    dbg!(r);
    let r = client
        .call(None, ReadManagementSupportedCommands::new())
        .await
        .unwrap();
    dbg!(r);
    let r = client
        .call(None, ReadControllerIndexList::new())
        .await
        .unwrap();
    for index in r {
        let r = client
            .call(index.clone(), ReadControllerInformation::new())
            .await
            .unwrap();
        println!("{:?}", r);
    }

    drop(client);
    handle.await.unwrap().unwrap()
}
