use std::net::TcpListener;

use actix_web::{web, dev::Server, HttpServer, App};
use sqlx::PgPool;
use crate::routes::{health_check, subscribe};

pub fn run(
    listener: TcpListener,
    connection: PgPool 
) -> Result<Server, std::io::Error> {
    let web_data = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/subscriptions", web::post().to(subscribe))
        .app_data(web_data.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}