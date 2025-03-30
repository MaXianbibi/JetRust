use std::net::SocketAddr;
use anyhow::{Context, Result}; 

mod routes;
mod handlers;
mod element;

use axum::routing::get;
use element::HtmlElement;
use tower_http::services::ServeDir;



async fn bind_address(addr: SocketAddr) -> Result<tokio::net::TcpListener> {
    let listener = tokio::net::TcpListener::bind(addr).await
    .with_context(|| format!("Failed to bind to address {}", addr))?;


    Ok(listener)
}

#[tokio::main]
async fn main() -> Result<()>  {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = bind_address(addr).await?;
    
    
    let body: HtmlElement = HtmlElement::new("body");
    let app = routes::create_routes()
    .route("/", get(handlers::root))
    // Ajoute un gestionnaire pour les fichiers statiques
    .nest_service("/static", ServeDir::new("static"));

    axum::serve(listener, app).await
    .context("Axum server encountered an error")?;

    Ok(())
}