use std::net::SocketAddr;

use api_axum::{
    auth::claims::Claims,
    configuration::get_configuration,
    routes::{get_payment_types, post_iteration},
};
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use sqlx::postgres::PgPoolOptions;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres");

    let app = Router::new()
        .route("/health", get(health))
        .route("/payment_type", get(get_payment_types))
        .route("/iteration", post(post_iteration))
        .layer(Extension(connection_pool));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health(_claims: Claims) -> impl IntoResponse {
    StatusCode::OK
}

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        Comparison::Equal
    } else {
        if _first_list.len() > _second_list.len() {
            for w in _first_list.windows(_second_list.len()) {
                if w == _second_list {
                    return Comparison::Superlist;
                }
            }
        } else {
            for w in _second_list.windows(_first_list.len()) {
                if w == _first_list {
                    return Comparison::Sublist;
                }
            }
        }
        Comparison::Unequal
    }
}
