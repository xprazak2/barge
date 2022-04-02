pub mod store_client;
pub mod store_server;

use tokio::sync::oneshot;

#[derive(Debug)]
pub struct Store {
  peers: Vec<i32>
}

impl Store {
  pub fn new() -> Self{
    Self{ peers: vec![] }
  }

  pub fn add(&mut self, peer: i32) {
    if !self.peers.contains(&peer) {
      self.peers.push(peer)
    }
  }

  pub fn list(&self) -> Vec<i32> {
    // should we return reference instead?
    self.peers.clone()
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
    resp: oneshot::Sender<Vec<i32>>
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
    store.add(16);
    assert!(store.list().contains(&16));
  }

  #[test]
  fn add_should_not_add_duplicate_peers() {
    let mut store = Store::new();
    store.add(16);
    store.add(16);
    store.add(16);
    assert!(store.list().contains(&16));
    assert!(store.list().len() == 1);
  }

  #[test]
  fn remove_should_remove_peer() {
    let mut store = Store::new();
    store.add(16);
    assert!(store.list().contains(&16));
    assert!(store.list().len() == 1);
    store.remove(16);
    assert!(store.list().len() == 0)
  }
}
