use anyhow::Result;
use crate::interface::router;
use tracing::info;

pub async fn execute(host: String, port: u16) -> Result<()> {
    info!("Starting server on {}:{}", host, port);
    
    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await?;
    
    info!("Server listening on {}:{}", host, port);
    axum::serve(listener, router::init())
        .await?;
        
    Ok(())
} 