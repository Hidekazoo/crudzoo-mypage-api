use std::net::SocketAddr;

use api_axum::auth::claims::Claims;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let app = Router::new().route("/health", get(health));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health(claims: Claims) -> impl IntoResponse {
    StatusCode::OK
}
