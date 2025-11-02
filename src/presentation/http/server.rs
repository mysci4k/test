use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use std::io::Result;

#[get("/")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is up!")
}

pub async fn configure_server(
    server_address: &str,
    server_port: u16,
) -> Result<actix_web::dev::Server> {
    let server = HttpServer::new(move || App::new().service(health_check))
        .bind((server_address, server_port))?;

    Ok(server.run())
}
