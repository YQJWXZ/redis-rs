use anyhow::Result;
use redis_rs::{network, Backend};
use tokio::net::TcpListener;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";
    info!("Simple Redis server listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;

    let backend = Backend::new();
    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from {}", raddr);
        let cloned_backend = backend.clone();
        tokio::spawn(async move {
            match network::stream_handler(stream, cloned_backend).await {
                Ok(_) => info!("Connection from {} exited", raddr),
                Err(e) => warn!("Connection from {} closed with error: {}", raddr, e),
            }
        });
    }
}
