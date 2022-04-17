use crate::routes::Route;
use crate::store::{StoreData, StoreMsg};
use std::{error::Error, fmt};
use tokio::sync::{mpsc::Sender, oneshot};
use tonic::Status;

#[derive(Debug)]
pub enum StoreError {
    ChannelSendError(tokio::sync::mpsc::error::SendError<StoreMsg>),
    ChannelReceiveError(tokio::sync::oneshot::error::RecvError),
}

impl Error for StoreError {}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something went wrong and I am a terrible error message.")
    }
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
            StoreError::ChannelReceiveError(_) => {
                Self::internal("Falied to receive message from store")
            }
        }
    }
}

pub async fn list_store(tx: Sender<StoreMsg>) -> Result<StoreData, StoreError> {
    let (list_tx, list_rx) = oneshot::channel();
    let list_msg = StoreMsg::List { resp: list_tx };
    tx.send(list_msg).await?;
    Ok(list_rx.await?)
}

pub async fn add_peer(tx: Sender<StoreMsg>, peer: i32) -> Result<(), StoreError> {
    let (add_tx, add_rx) = oneshot::channel();
    let add_msg = StoreMsg::AddPeer { peer, resp: add_tx };
    tx.send(add_msg).await?;
    Ok(add_rx.await?)
}

pub async fn remove_peer(tx: Sender<StoreMsg>, peer: i32) -> Result<(), StoreError> {
    let (remove_tx, remove_rx) = oneshot::channel();
    let remove_msg = StoreMsg::Remove {
        peer,
        resp: remove_tx,
    };
    tx.send(remove_msg).await?;
    Ok(remove_rx.await?)
}

pub async fn on_bootstrap(
    tx: Sender<StoreMsg>,
    routes: Vec<Route>,
    peer: i32,
) -> Result<(), StoreError> {
    let (add_tx, add_rx) = oneshot::channel();
    let add_msg = StoreMsg::OnBootstrap {
        peer,
        routes,
        resp: add_tx,
    };
    tx.send(add_msg).await?;
    Ok(add_rx.await?)
}

pub async fn on_heartbeat(
    tx: Sender<StoreMsg>,
    routes: Vec<Route>,
    peer: i32,
) -> Result<(), StoreError> {
    let (add_tx, add_rx) = oneshot::channel();
    let add_msg = StoreMsg::OnHeartbeat {
        peer,
        routes,
        resp: add_tx,
    };
    tx.send(add_msg).await?;
    Ok(add_rx.await?)
}
