use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use std::net::TcpListener;
use sqlx::{PgConnection, query};
use uuid::Uuid;
use chrono::Utc;
use crate::routes::health_check;
#[derive(serde::Deserialize)]
pub struct FormatData {
    email: String,
    name: String,
}

pub async fn subscribe(
    form: web::Form<FormatData>,
    connection: web::Data<PgConnection>,
) -> HttpResponse {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
        // We use `get_ref` to get an immutable reference to the `PgConnection`
        // wrapped by `web::Data`.
        .execute(connection.get_ref())
        .await;
    HttpResponse::Ok().finish()
}
pub fn run(listener: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error>  {
    let connection = web::Data::new(connection);
    let server = HttpServer::new(move ||  {
        App::new()
            .route("/health_check", web::get().to(health_check::health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)

}