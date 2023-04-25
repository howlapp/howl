use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // setup logging
    howlapp_tracing::init("guild-service")?;
    // setup grpc
    Ok(())
}
