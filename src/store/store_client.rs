use tokio::sync::{mpsc::Sender, oneshot};
use crate::store::{StoreMsg};
use tonic::{Status};

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

pub async fn list_store(tx: Sender<StoreMsg>) -> Result<Vec<i32>, StoreError> {
  let (list_tx, list_rx) = oneshot::channel();
  let list_msg = StoreMsg::List { resp: list_tx };
  tx.send(list_msg).await?;
  Ok(list_rx.await?)
}

pub async fn add_peer(tx: Sender<StoreMsg>, peer: i32) -> Result<(), StoreError> {
  let (add_tx, add_rx) = oneshot::channel();
  let add_msg = StoreMsg::Add { peer, resp: add_tx };
  tx.send(add_msg).await?;
  Ok(add_rx.await?)
}

pub async fn remove_peer(tx: Sender<StoreMsg>, peer: i32) -> Result<(), StoreError> {
  let (remove_tx, remove_rx) = oneshot::channel();
  let remove_msg = StoreMsg::Remove { peer, resp: remove_tx };
  tx.send(remove_msg).await?;
  Ok(remove_rx.await?)  
}
