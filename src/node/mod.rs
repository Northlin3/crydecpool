// src/node/mod.rs
use tokio::signal;

pub async fn run() -> anyhow::Result<()> {
    tracing::info!("Node runtime ready. Press Ctrl-C to exit.");
    signal::ctrl_c().await?;
    tracing::info!("Shutdown complete.");
    Ok(())
}
