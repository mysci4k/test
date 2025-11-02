mod presentation;
mod shared;

use crate::{
    presentation::http::server::configure_server,
    shared::utils::constants::{SERVER_ADDRESS, SERVER_PORT},
};
use dotenvy::dotenv;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let server = configure_server(&SERVER_ADDRESS, *SERVER_PORT).await?;

    server.await
}
