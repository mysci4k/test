use crate::{
    application::services::AuthService,
    domain::repositories::UserRepository,
    infrastructure::persistence::{SeaOrmUserRepository, database},
    shared::{
        config::AppState,
        utils::{constants::REDIS_URL, email::EmailService},
    },
};
use redis::Client as RedisClient;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tracing::info;

pub async fn initialize_infrastructure()
-> Result<(DatabaseConnection, RedisClient, Arc<EmailService>), Box<dyn std::error::Error>> {
    let database = database::run().await.map_err(|err| {
        eprintln!("Failed to connect to the database: {}", err);
        err
    })?;
    info!("Successfully connected to the PostgreSQL database");

    let redis_client = RedisClient::open(REDIS_URL.as_str()).map_err(|err| {
        eprintln!("Failed to create Redis client: {}", err);
        err
    })?;
    info!("Successfully connected to the Redis server");

    let email_service = Arc::new(EmailService::new().map_err(|err| {
        eprintln!("Failed to initialize email service: {}", err);
        err
    })?);
    info!("Successfully initialized email service");

    Ok((database, redis_client, email_service))
}

pub fn initialize_repositories(database: DatabaseConnection) -> Arc<dyn UserRepository> {
    let user_repository =
        Arc::new(SeaOrmUserRepository::new(database.clone())) as Arc<dyn UserRepository>;

    info!("Successfully initialized repositories");

    user_repository
}

pub fn initialize_services(
    user_repository: Arc<dyn UserRepository>,
    redis_client: RedisClient,
    email_service: Arc<EmailService>,
) -> AppState {
    let auth_service = Arc::new(AuthService::new(
        user_repository.clone(),
        redis_client,
        email_service,
    ));

    info!("Successfully initialized services");

    AppState::new(auth_service)
}
