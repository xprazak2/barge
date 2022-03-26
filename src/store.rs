use tokio::sync::oneshot;

#[derive(Debug)]
pub struct Store {
  peers: Vec<i32>
}

impl Store {
  pub fn new() -> Self{
    Self{ peers: vec![] }
  }

  pub fn add_peer(&mut self, peer: i32) {
    self.peers.push(peer)
  }

  pub fn list(&self) -> Vec<i32> {
    self.peers.clone()
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
  }
}
