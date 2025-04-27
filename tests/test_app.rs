use mati_test_rust::log::{get_subscriber, init_subscriber};
use mati_test_rust::startup::run;
use once_cell::sync::Lazy;
use reqwest::{Client, StatusCode};
use rstest::{fixture, rstest};
use sqlx::{migrate, postgres::PgPoolOptions, PgPool};
use std::net::TcpListener;
use testcontainers_modules::testcontainers::{ContainerAsync, ImageExt};
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("test", "debug");
    init_subscriber(subscriber);
});

struct TestApp {
    pub client: Client,
    pub addr: String,
    pub db_pool: PgPool,
    pub _db_container: ContainerAsync<Postgres>,
}

#[fixture]
async fn test_app() -> TestApp {
    Lazy::force(&TRACING);
    let db_container = Postgres::default()
        .with_tag("latest")
        .start()
        .await
        .unwrap();
    let host = db_container.get_host().await.unwrap().to_string();
    let port = db_container.get_host_port_ipv4(5432).await.unwrap();
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&format!(
            "postgresql://postgres:postgres@{}:{}/postgres",
            host, port
        ))
        .await
        .unwrap();
    migrate!().run(&db_pool).await.unwrap();

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = run(listener, db_pool.clone()).unwrap();
    let _ = tokio::spawn(server);
    let addr = format!("http://127.0.0.1:{}", port);
    let client = Client::new();

    TestApp {
        client,
        addr,
        db_pool,
        _db_container: db_container,
    }
}

#[tokio::test]
#[rstest]
async fn test_healthcheck_works(#[future] test_app: TestApp) {
    let app = test_app.await;
    let response = app
        .client
        .get(format!("{}/healthcheck", app.addr))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

#[tokio::test]
#[rstest]
#[case::missing_name("email=mati%40gmail.com")]
#[case::missing_email("name=mati")]
#[case::missing_both("")]
async fn test_subscribe_invalid(#[future] test_app: TestApp, #[case] body: String) {
    let app = test_app.await;
    let response = app
        .client
        .post(format!("{}/subscriptions", app.addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::BAD_REQUEST, response.status());
}

#[tokio::test]
#[rstest]
async fn test_subscribe_valid(#[future] test_app: TestApp) {
    let app = test_app.await;
    let response = app
        .client
        .post(format!("{}/subscriptions", app.addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body("name=mati&email=mati%40gmail.com")
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());

    let saved = sqlx::query!("select email, name from subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .unwrap();

    assert_eq!(saved.email, "mati@gmail.com");
    assert_eq!(saved.name, "mati");
}
