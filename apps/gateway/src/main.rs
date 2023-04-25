use std::{env, net::Ipv4Addr};

use anyhow::Result;
use macrostack_protocol::hello::{greeter_client::GreeterClient, HelloReply, HelloRequest};
use serde::Serialize;
use tonic::Response;
use warp::Filter;

#[derive(Serialize)]
struct Message {
    message: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    macrostack_tracing::init("example-service")?;

    // greeter service config
    let greeter_host: Ipv4Addr = env::var("EXAMPLE_SERVICE_CONNECT_SERVICE_HOST")?.parse()?;
    let greeter_port: u16 = env::var("EXAMPLE_SERVICE_CONNECT_SERVICE_PORT")?.parse()?;
    // connect to greeter service
    let greeter = macrostack_protocol::hello::greeter_client::GreeterClient::connect(format!(
        "http://{}:{}",
        greeter_host, greeter_port
    ))
    .await?;

    let greeter = warp::any().map(move || greeter.clone());

    let routes = warp::path!("hello" / String)
        .and(greeter)
        .and_then(|name, mut greeter: GreeterClient<_>| async move {
            greeter
                .say_hello(HelloRequest { name })
                .await
                .map_err(|_| warp::reject())
        })
        .map(|reply: Response<HelloReply>| {
            warp::reply::json(&Message {
                message: reply.into_inner().message,
            })
        });

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;

    Ok(())
}
