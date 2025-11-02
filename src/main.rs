mod presentation;
mod shared;

use crate::{
    presentation::http::server::configure_server,
    shared::utils::constants::{SERVER_ADDRESS, SERVER_PORT},
};
use dotenvy::dotenv;
use std::io::Result;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global default subscriber");

    let server = configure_server(&SERVER_ADDRESS, *SERVER_PORT).await?;

    server.await
}
