use mati_test_rust::{
    log::{get_subscriber, init_subscriber},
    settings::{get_settings, Settings},
    startup::run,
};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    let settings = get_settings().unwrap();

    let subscriber = get_subscriber("mati-test-rust", "info");
    init_subscriber(subscriber);

    let listener = get_listener(&settings).await;
    let pool = get_pool(&settings).await;

    run(listener, pool)?.await?;
    Ok(())
}

async fn get_listener(settings: &Settings) -> TcpListener {
    TcpListener::bind(format!("127.0.0.1:{}", settings.app_port)).unwrap()
}

async fn get_pool(settings: &Settings) -> PgPool {
    PgPoolOptions::new()
        .max_connections(settings.pool_max_connections)
        .connect(settings.database_url.expose_secret())
        .await
        .unwrap()
}
