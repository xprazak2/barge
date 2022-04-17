use crate::barge::barge_proto::{barge_client::BargeClient, HeartbeatRequest};
use crate::helper;
use crate::store::StoreMsg;
use tokio::time;
use tokio::{sync::mpsc::Sender, task::JoinHandle};

use crate::store::store_client;

pub fn start_sync(tx: Sender<StoreMsg>, interval: u64) -> JoinHandle<()> {
    tokio::spawn(async move { run(tx, interval).await })
}

async fn run(tx: Sender<StoreMsg>, interval: u64) -> ! {
    let mut interval = time::interval(time::Duration::from_secs(interval));
    loop {
        interval.tick().await;
        // This will probably not give us meaningful errors...
        if let Err(msg) = work(tx.clone()).await {
            log::error!("Heartbeat error: {:?}", msg)
        }
    }
}

async fn work(tx: Sender<StoreMsg>) -> Result<(), Box<dyn std::error::Error>> {
    let store_data = store_client::list_store(tx.clone()).await?;
    for peer in store_data.peers {
        let mut client = BargeClient::connect(format!("http://[::1]:{}", peer)).await?;
        let request = tonic::Request::new(HeartbeatRequest {});
        let response = client.heartbeat(request).await?;
        let routes = helper::routes_from_proto(response.into_inner().routes);
        // should we aggregate responses from all peers to send only one message to store per heartbeat?
        store_client::on_heartbeat(tx.clone(), routes, peer).await?;
    }

    Ok(())
}
