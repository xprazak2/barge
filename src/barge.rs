use tonic::{Request, Response, Status};

use barge_proto::{JoinRequest, JoinResponse};
use barge_proto::barge_server::{Barge};

use tokio::sync::{mpsc::Sender};

use crate::store::StoreMsg;

use crate::store::store_client;

pub mod barge_proto {
    tonic::include_proto!("barge_proto");
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
    let peers = store_client::list_store(self.tx.clone()).await?;
    store_client::add_peer(self.tx.clone(), request.into_inner().port).await?;

    let reply = barge_proto::JoinResponse {
      peers,
    };
    Ok(Response::new(reply))
  }
}
