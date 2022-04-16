use clap::Parser;
use barge::cli::Cli;
use barge::store::{StoreMsg};
use barge::store::{store_server, store_client};

use barge::routes;

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

    let _store_actor = store_server::start_store(rx);

    if let Some(peer) = args.bootstrap_peer {
        let mut client = BargeClient::connect(format!("http://[::1]:{}", peer)).await?;

        let request = tonic::Request::new(JoinRequest { port: args.port });
        let response = client.join(request).await?;

        println!("RESPONSE={:?}", response);
        let routes = response.into_inner().routes.into_iter().map(|br| routes::Route::from(br)).collect();
        store_client::add_routes(tx.clone(), routes, peer).await?;
    }

    Server::builder().add_service(BargeServer::new(barge)).serve(addr).await?;

    Ok(())
}
