use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::net::TcpListener;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[derive(serde::Deserialize)]
struct FormatData {
    email: String,
    name: String,
}

async fn subscribe(_form: web::Form<FormatData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error>  {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
        .listen(listener).unwrap()
        .run();
    Ok(server)
   
}
 