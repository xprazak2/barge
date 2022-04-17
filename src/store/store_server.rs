use crate::store::{StoreMsg, Store};
use tokio::{sync::mpsc::Receiver, task::JoinHandle};

pub async fn run(mut rx: Receiver<StoreMsg>) -> () {
    let mut store = Store::new();

    while let Some(msg) = rx.recv().await {
        match msg {
            StoreMsg::AddPeer { peer, resp } => {
                log::info!("Adding peer to store: {}", peer);
                store.add_peer(peer);
                if let Err(_) = resp.send(()) {
                    log::error!("Error sending response when adding peer to store")
                }
            },
            StoreMsg::List { resp } => {
                if let Err(_) = resp.send(store.to_store_data()) {
                    log::error!("Error sending response when listing store")
                }
            },
            StoreMsg::Remove { peer, resp } => {
                log::info!("Removing peer from store: {}", peer);
                store.remove(peer);
                if let Err(_) = resp.send(()) {
                    log::error!("Error sending response when removing peer from store")
                }
            },
            StoreMsg::OnBootstrap { peer, routes, resp } => {
                log::info!("Adding new routes via peer: {}", peer);
                store.add_routes(peer, routes);
                if let Err(_) = resp.send(()) {
                    log::error!("Error sending response when adding routes to store")
                }

            },
            StoreMsg::OnHeartbeat { peer, routes, resp } => {
                log::info!("Heartbeat, peer: {}", peer);
                store.add_routes(peer, routes);
                log::debug!("Store state: {:?}", store.to_store_data());
                if let Err(_) = resp.send(()) {
                    log::error!("Error sending response on heartbeat")
                }

            },
        }
    }
}

pub fn start_store(rx: Receiver<StoreMsg>) -> JoinHandle<()> {
    tokio::spawn(async move {
        run(rx).await
    })
}
