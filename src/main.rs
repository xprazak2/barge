pub mod cli;
pub mod barge;
pub mod store;

use clap::Parser;
use cli::Cli;
use store::{Store, StoreMsg};

use barge::BargeService;
use barge::barge_proto::barge_client::BargeClient;
use barge::barge_proto::barge_server::{BargeServer};
use barge::barge_proto::JoinRequest;

use tonic::{transport::Server};

use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let (tx, mut rx) = mpsc::channel::<StoreMsg>(10);
    let barge = BargeService::new(tx.clone());
    let addr = format!("[::1]:{}", args.port).parse()?;

    let _store_actor = tokio::spawn(async move {
        let mut store = Store::new();

        while let Some(msg) = rx.recv().await {
            match msg {
                StoreMsg::Add { peer, resp } => {
                    store.add_peer(peer);
                    resp.send(());
                },
                StoreMsg::List { resp } => {
                    let peers = store.list();
                    resp.send(peers);
                }
            }
        }
    });

    if let Some(peer) = args.bootstrap_peer {
        let mut client = BargeClient::connect(format!("http://[::1]:{}", peer)).await?;

        let request = tonic::Request::new(JoinRequest { port: args.port });
        let response = client.join(request).await?;

        println!("RESPONSE={:?}", response);
    }

    Server::builder().add_service(BargeServer::new(barge)).serve(addr).await?;

    Ok(())
}
