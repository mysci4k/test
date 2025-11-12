mod application;
mod domain;
mod infrastructure;
mod presentation;
mod shared;

use crate::{
    presentation::http::configure_server,
    shared::{
        config::{initialize_infrastructure, initialize_repositories, initialize_services},
        utils::constants::{SERVER_ADDRESS, SERVER_PORT},
    },
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

    let (database, redis_client) = initialize_infrastructure()
        .await
        .expect("Failed to initialize infrastructure");

    let (user_repository, board_repository, board_member_repository) =
        initialize_repositories(database);

    let app_state = initialize_services(
        user_repository,
        board_repository,
        board_member_repository,
        redis_client,
    );

    let server = configure_server(app_state, &SERVER_ADDRESS, *SERVER_PORT).await?;

    server.await
}
