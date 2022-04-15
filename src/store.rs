pub mod store_client;
pub mod store_server;

use tokio::sync::oneshot;
use crate::routes::{Routes, Route};

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
  pub fn new() -> Self{
    Self{ peers: vec![], routes: Routes::new() }
  }

  pub fn add_peer(&mut self, peer: i32) {
    if !self.peers.contains(&peer) {
      self.peers.push(peer);

      let route = Route::new(peer, 1, peer);
      self.routes.add(route);
    }
  }

  pub fn add_route(&mut self, node: i32, direction: i32, hops: i32) {
    if let Some(existing) = self.routes.get(&direction) {
      let route = Route::new(node, existing.hops + hops, direction);
      self.routes.add(route);
    }
  }

  pub fn list(&self) -> Vec<i32> {
    // should we return reference instead?
    self.peers.clone()
  }

  pub fn to_store_data(&self) -> StoreData {
    StoreData { peers: self.peers.clone(), routes: self.routes.clone() }
  }

  pub fn remove(&mut self, peer: i32) {
    self.peers.retain(|&item| item != peer)
  }
}

#[derive(Debug)]
pub enum StoreMsg {
  Add {
    peer: i32,
    resp: oneshot::Sender<()>,
  },
  List {
    resp: oneshot::Sender<StoreData>
  },
  Remove {
    peer: i32,
    resp: oneshot::Sender<()>
  }
}

#[cfg(test)]
mod tests {
  use crate::store::Store;

  #[test]
  fn add_should_add_peer() {
    let mut store = Store::new();
    store.add_peer(16);
    assert!(store.list().contains(&16));
  }

  #[test]
  fn add_should_not_add_duplicate_peers() {
    let mut store = Store::new();
    store.add_peer(16);
    store.add_peer(16);
    store.add_peer(16);
    assert!(store.list().contains(&16));
    assert!(store.list().len() == 1);
  }

  #[test]
  fn remove_should_remove_peer() {
    let mut store = Store::new();
    store.add_peer(16);
    assert!(store.list().contains(&16));
    assert!(store.list().len() == 1);
    store.remove(16);
    assert!(store.list().is_empty())
  }
}
