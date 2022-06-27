use api::configuration::get_configuration;
use api::startup::run;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use tracing::info;
use tracing_subscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    info!("Start Process!");
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await
}
