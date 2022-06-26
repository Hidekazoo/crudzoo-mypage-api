use crate::configuration::{DatabaseSettings, Settings, self, get_configuration};
use crate::routes::{
    add_book, add_payment, create_user, find_payment, get_payment_types, store_daily_condition,
};
use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::{http, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(connection);
    let configuration = get_configuration().expect("Failed to read configuration");
    println!("allows: {:?}", configuration.application.allow_origin);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin(&configuration.application.allow_origin)
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(&[http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .expose_headers(&[http::header::CONTENT_DISPOSITION])
                    .max_age(3600),
            )
            .wrap(TracingLogger::default())
            .route("/payment_type", web::get().to(get_payment_types))
            .route("/user", web::post().to(create_user))
            .route("/payment", web::post().to(add_payment))
            .route("/user/{id}/payment", web::get().to(find_payment))
            .route("/book", web::post().to(add_book))
            .route("/daily_condition", web::post().to(store_daily_condition))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration.database)
            .await
            .expect("Failed to connect to Postgres.");
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        println!("{}", &address);
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool)?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn get_connection_pool(configuration: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_with(configuration.with_db())
        .await
}
