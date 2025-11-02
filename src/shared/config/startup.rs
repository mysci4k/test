use crate::infrastructure::persistence::database;
use sea_orm::DatabaseConnection;
use tracing::info;

pub async fn initialize_infrastructure() -> Result<DatabaseConnection, Box<dyn std::error::Error>> {
    let database = database::run().await.map_err(|err| {
        eprintln!("Failed to connect to the database: {}", err);
        err
    })?;
    info!("Successfully connected to the PostgreSQL database");

    Ok(database)
}
