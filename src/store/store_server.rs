use crate::store::{StoreMsg, Store};
use tokio::sync::mpsc::Receiver;

pub async fn run(mut rx: Receiver<StoreMsg>) -> () {
    let mut store = Store::new();

    while let Some(msg) = rx.recv().await {
        match msg {
            StoreMsg::Add { peer, resp } => {
                store.add(peer);
                resp.send(());
            },
            StoreMsg::List { resp } => {
                let peers = store.list();
                resp.send(peers);
            },
            StoreMsg::Remove { peer, resp } => {
                store.remove(peer);
                resp.send(());
            }
        }
    }
}
