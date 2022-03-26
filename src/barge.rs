use tonic::{Request, Response, Status};

use barge_proto::{JoinRequest, JoinResponse};
use barge_proto::barge_server::{Barge};

use tokio::sync::{mpsc::Sender, oneshot};

use crate::store::StoreMsg;

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

#[derive(Debug)]
pub enum StoreError {
  ChannelSendError(tokio::sync::mpsc::error::SendError<StoreMsg>),
  ChannelReceiveError(tokio::sync::oneshot::error::RecvError)
}

impl From<tokio::sync::oneshot::error::RecvError> for StoreError {
  fn from(e: tokio::sync::oneshot::error::RecvError) -> Self {
    Self::ChannelReceiveError(e)    
  }
}

impl From<tokio::sync::mpsc::error::SendError<StoreMsg>> for StoreError {
  fn from(e: tokio::sync::mpsc::error::SendError<StoreMsg>) -> Self {
    Self::ChannelSendError(e)    
  }
}

impl From<StoreError> for Status {
  fn from(e: StoreError) -> Self {
    match e {
        StoreError::ChannelSendError(_) => Self::internal("Failed to send request to store"),
        StoreError::ChannelReceiveError(_) => Self::internal("Falied to receive message from store")
    }
  }
}

async fn list_store(tx: Sender<StoreMsg>) -> Result<Vec<i32>, StoreError> {
  let (list_tx, list_rx) = oneshot::channel();
  let list_msg = StoreMsg::List { resp: list_tx };
  tx.send(list_msg).await?;
  Ok(list_rx.await?)
}

async fn add_peer(tx: Sender<StoreMsg>, peer: i32) -> Result<(), StoreError> {
    let (add_tx, add_rx) = oneshot::channel();
    let add_msg = StoreMsg::Add { peer, resp: add_tx };
    tx.send(add_msg).await?;
    Ok(add_rx.await?)
}

#[tonic::async_trait]
impl Barge for BargeService {

  async fn join(&self, request: Request<JoinRequest>) -> Result<Response<JoinResponse>, Status> {
    println!("Got a request from {:?}", request.remote_addr());
    let peers = list_store(self.tx.clone()).await?;
    add_peer(self.tx.clone(), request.into_inner().port).await?;

    let reply = barge_proto::JoinResponse {
      peers,
    };
    Ok(Response::new(reply))
  }
}
