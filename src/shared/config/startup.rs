use crate::{
    application::services::AuthService,
    domain::repositories::UserRepository,
    infrastructure::persistence::{SeaOrmUserRepository, database},
    shared::config::AppState,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tracing::info;

pub async fn initialize_infrastructure() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let database = database::run().await.map_err(|err| {
        eprintln!("Failed to connect to the database: {}", err);
        err
    })?;
    info!("Successfully connected to the PostgreSQL database");

    Ok(database)
}

pub fn initialize_repositories(database: DatabaseConnection) -> Arc<dyn UserRepository> {
    let user_repository =
        Arc::new(SeaOrmUserRepository::new(database.clone())) as Arc<dyn UserRepository>;

    info!("Successfully initialized repositories");

    user_repository
}

pub fn initialize_services(user_repository: Arc<dyn UserRepository>) -> AppState {
    let auth_service = Arc::new(AuthService::new(user_repository.clone()));

    info!("Successfully initialized services");

    AppState::new(auth_service)
}
