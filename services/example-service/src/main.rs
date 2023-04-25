use anyhow::Result;
use howlapp_protocol::hello::{
    greeter_server::{Greeter, GreeterServer},
    HelloReply, HelloRequest,
};

use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
struct HowlappGreeter;

#[tonic::async_trait]
impl Greeter for HowlappGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        Ok(Response::new(HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // setup logging
    howlapp_tracing::init("example-service")?;
    // setup grpc
    let greeter = HowlappGreeter::default();
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(([127, 0, 0, 1], 50051).into())
        .await?;
    Ok(())
}
