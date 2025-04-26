use std::net::TcpListener;
use sqlx::postgres::PgPoolOptions;
use mati_test_rust::config::get_config;
use mati_test_rust::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenvy::dotenv().ok();
    env_logger::init();
    let config = get_config().unwrap();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", config.app_port))?;
    let pool = PgPoolOptions::new()
        .max_connections(config.pool_max_connections)
        .connect(&config.database_url)
        .await
        .unwrap();
    
    run(listener, pool)?.await?;
    Ok(())
}
