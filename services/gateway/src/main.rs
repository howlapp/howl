use std::{convert::Infallible, env};

use anyhow::Result;
use serde::Serialize;

use warp::{http::StatusCode, Filter, Rejection, Reply};

#[derive(Serialize)]
struct VersionInformation {
    commit: String,
    version: String,
}

enum ErrorCode {
    NotFound,
}

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    msg: String,
}

async fn handle_rejection(r: Rejection) -> Result<impl Reply, Infallible> {
    if r.is_not_found() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&ErrorMessage {
                code: ErrorCode::NotFound as u16,
                msg: "Error: Not Found".to_string(),
            }),
            StatusCode::NOT_FOUND,
        ));
    }
    todo!()
}

#[tokio::main]
async fn main() -> Result<()> {
    howlapp_tracing::init("gateway")?;
    // return api version
    let version = warp::path::end().and(warp::get()).map(|| {
        warp::reply::json(&VersionInformation {
            commit: "".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
    });
    // create router
    let routes = version.recover(handle_rejection).with(warp::log("gateway"));
    // serve
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
    Ok(())
}
