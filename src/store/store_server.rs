use crate::store::{StoreMsg, Store};
use tokio::sync::mpsc::Receiver;

pub async fn run(mut rx: Receiver<StoreMsg>) -> () {
    let mut store = Store::new();

    while let Some(msg) = rx.recv().await {
        match msg {
            StoreMsg::Add { peer, resp } => {
                store.add(peer);
                resp.send(()).expect("error add");
            },
            StoreMsg::List { resp } => {
                let peers = store.list();
                resp.send(peers).expect("Error list");
            },
            StoreMsg::Remove { peer, resp } => {
                store.remove(peer);
                resp.send(()).expect("error remove");
            }
        }
    }
}
