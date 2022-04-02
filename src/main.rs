use clap::Parser;
use barge::cli::Cli;
use barge::store::{StoreMsg};
use barge::store::{store_server};

use barge::barge::BargeService;
use barge::barge::barge_proto::barge_client::BargeClient;
use barge::barge::barge_proto::barge_server::{BargeServer};
use barge::barge::barge_proto::JoinRequest;

use tonic::{transport::Server};

use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Cli::parse();

    let (tx, rx) = mpsc::channel::<StoreMsg>(10);
    let barge = BargeService::new(tx.clone());
    let addr = format!("[::1]:{}", args.port).parse()?;

    let _store_actor = tokio::spawn(async move {
        store_server::run(rx).await
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
