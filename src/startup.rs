use std::net::TcpListener;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use tracing_actix_web::TracingLogger;
use sqlx::PgPool;
use crate::routes::{healthcheck, subscribe};

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> 
where 
{
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(Data::new(pool.clone()))
            .route("/healthcheck", web::get().to(healthcheck))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener)?
        .run();

    Ok(server)
}
