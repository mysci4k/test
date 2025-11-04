use crate::{presentation::configure_auth_roures, shared::config::AppState};
use actix_web::{App, HttpResponse, HttpServer, Responder, get, web};
use std::io::Result;

#[get("/")]
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is up!")
}

pub async fn configure_server(
    app_state: AppState,
    server_address: &str,
    server_port: u16,
) -> Result<actix_web::dev::Server> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.auth_service.clone()))
            .service(
                web::scope("/api")
                    .service(health_check)
                    .configure(configure_auth_roures),
            )
    })
    .bind((server_address, server_port))?;

    Ok(server.run())
}
