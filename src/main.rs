use std::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::*;
//use sqlx::{Connection, PgConnection};
use tokio_postgres::{Connection};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Could not get the configuration.");
    let connection = Connection::connect(
        &configuration.database.connection_string()
    )
        .await
        .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port); 
    let listener = TcpListener::bind(address)?;
    run(listener, connection)?.await
}