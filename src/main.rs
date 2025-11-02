mod presentation;

use crate::presentation::http::server::configure_server;
use dotenvy::dotenv;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let server = configure_server("127.0.0.1", 8080).await?;

    server.await
}
