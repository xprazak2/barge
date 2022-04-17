pub mod store_client;
pub mod store_server;

use crate::routes::{Route, Routes};
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct Store {
    peers: Vec<i32>,
    routes: Routes,
}

#[derive(Debug, Clone)]
pub struct StoreData {
    pub peers: Vec<i32>,
    pub routes: Routes,
}

impl Store {
    pub fn new() -> Self {
        Self {
            peers: vec![],
            routes: Routes::new(),
        }
    }

    pub fn add_peer(&mut self, peer: i32) {
        if !self.peers.contains(&peer) {
            self.peers.push(peer);

            let route = Route::new(peer, 1, peer);
            self.routes.add(route);
        }
    }

    pub fn add_routes(&mut self, peer: i32, routes: Vec<Route>) {
        for mut route in routes {
            route.hops += 1;
            route.direction = peer;
            self.routes.add(route);
        }
    }

    pub fn on_bootstrap(&mut self, peer: i32, routes: Vec<Route>) {
        self.add_peer(peer);
        self.add_routes(peer, routes);
    }

    pub fn list_peers(&self) -> Vec<i32> {
        // should we return reference instead?
        self.peers.clone()
    }

    pub fn to_store_data(&self) -> StoreData {
        StoreData {
            peers: self.peers.clone(),
            routes: self.routes.clone(),
        }
    }

    pub fn remove(&mut self, peer: i32) {
        self.peers.retain(|&item| item != peer)
    }
}

#[derive(Debug)]
pub enum StoreMsg {
    AddPeer {
        peer: i32,
        resp: oneshot::Sender<()>,
    },
    List {
        resp: oneshot::Sender<StoreData>,
    },
    Remove {
        peer: i32,
        resp: oneshot::Sender<()>,
    },
    OnBootstrap {
        peer: i32,
        routes: Vec<Route>,
        resp: oneshot::Sender<()>,
    },
    OnHeartbeat {
        peer: i32,
        routes: Vec<Route>,
        resp: oneshot::Sender<()>,
    },
}

#[cfg(test)]
mod tests {
    use crate::routes::Route;
    use crate::store::Store;

    #[test]
    fn add_should_add_peer() {
        let mut store = Store::new();
        store.add_peer(16);
        assert!(store.list_peers().contains(&16));
        let route = Route::new(16, 1, 16);
        let res = store.routes.get(&16).unwrap();
        assert_eq!(res, &route);
    }

    #[test]
    fn add_should_not_add_duplicate_peers() {
        let mut store = Store::new();
        store.add_peer(16);
        store.add_peer(16);
        store.add_peer(16);
        assert!(store.list_peers().contains(&16));
        assert!(store.list_peers().len() == 1);
    }

    #[test]
    fn remove_should_remove_peer() {
        let mut store = Store::new();
        store.add_peer(16);
        assert!(store.list_peers().contains(&16));
        assert!(store.list_peers().len() == 1);
        store.remove(16);
        assert!(store.list_peers().is_empty())
    }
}
