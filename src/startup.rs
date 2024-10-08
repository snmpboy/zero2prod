use crate::routes::{health_check, subscriptions};
use actix_web::dev::{ConnectionInfo, Server};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    //let connection = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions::subscribe))
            .app_data(db_pool.clone())
            //.app_data(db_pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}  