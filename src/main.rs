use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // tracing_subscriber::fmt::init();
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;
    let listeners = "localhost:9991";
    let server = nucleum::NucleumServer::new(listeners);

    server.run().await?;

    Ok(())
}
