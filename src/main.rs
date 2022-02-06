pub mod cli;
pub mod barge;

use clap::Parser;
use cli::Cli;
use barge::BargeService;

use barge::barge_proto::barge_server::{BargeServer};
use tonic::{transport::Server};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("Args: {:?}", args);

    let barge = BargeService::default();
    let addr = format!("[::1]:{}", args.port).parse()?;

    Server::builder().add_service(BargeServer::new(barge)).serve(addr).await?;

    Ok(())
}

