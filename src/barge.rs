use tonic::{Request, Response, Status};

use barge_proto::{JoinRequest, JoinResponse};
use barge_proto::barge_server::{Barge};

pub mod barge_proto {
    tonic::include_proto!("barge_proto");
}

#[derive(Debug, Default)]
pub struct BargeService {}

#[tonic::async_trait]
impl Barge for BargeService {
  async fn join(&self, request: Request<JoinRequest>) -> Result<Response<JoinResponse>, Status> {
    println!("Got a request from {:?}", request.remote_addr());

    let reply = barge_proto::JoinResponse {
      peers: vec![54677],
    };
    Ok(Response::new(reply))
  }
}
