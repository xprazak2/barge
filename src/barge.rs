use tonic::{Request, Response, Status};

use barge_proto::{JoinRequest, JoinResponse};
use barge_proto::barge_server::{Barge};

use tokio::sync::{mpsc::Sender};

use crate::routes;
use crate::store::StoreMsg;

use crate::store::store_client;

pub mod barge_proto {
    tonic::include_proto!("barge_proto");
}

impl From<routes::Route> for barge_proto::Route {
  fn from(item: routes::Route) -> Self {
    Self { node_name: item.node_name, hops: item.hops, direction: item.direction}
  }
}

#[derive(Debug)]
pub struct BargeService {
  tx: Sender<StoreMsg>
}

impl BargeService {
  pub fn new(tx: Sender<StoreMsg>) -> Self {
    Self{ tx }
  }
}

#[tonic::async_trait]
impl Barge for BargeService {

  async fn join(&self, request: Request<JoinRequest>) -> Result<Response<JoinResponse>, Status> {
    println!("Got a request from {:?}", request.remote_addr());
    let data = store_client::list_store(self.tx.clone()).await?;
    store_client::add_peer(self.tx.clone(), request.into_inner().port).await?;

    let reply = barge_proto::JoinResponse {
      peers: data.peers,
      routes: data.routes.list().into_iter().map(|route| barge_proto::Route::from(route)).collect()
    };
    Ok(Response::new(reply))
  }
}
