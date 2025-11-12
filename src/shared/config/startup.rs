use crate::{
    application::services::{AuthService, BoardService, UserService},
    domain::{
        repositories::{BoardMemberRepository, BoardRepository, UserRepository},
        services::{EmailService, TokenService},
    },
    infrastructure::{
        cache::RedisTokenService,
        email::SmtpEmailService,
        persistence::{
            SeaOrmBoardMemberRepository, SeaOrmBoardRepository, SeaOrmUserRepository, database,
        },
    },
    shared::{config::AppState, utils::constants::REDIS_URL},
};
use redis::Client as RedisClient;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tracing::info;

pub async fn initialize_infrastructure()
-> Result<(DatabaseConnection, RedisClient), Box<dyn std::error::Error>> {
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

    Ok((database, redis_client))
}

pub fn initialize_repositories(
    database: DatabaseConnection,
) -> (
    Arc<dyn UserRepository>,
    Arc<dyn BoardRepository>,
    Arc<dyn BoardMemberRepository>,
) {
    let user_repository =
        Arc::new(SeaOrmUserRepository::new(database.clone())) as Arc<dyn UserRepository>;
    let board_repository =
        Arc::new(SeaOrmBoardRepository::new(database.clone())) as Arc<dyn BoardRepository>;
    let board_member_repository = Arc::new(SeaOrmBoardMemberRepository::new(database.clone()))
        as Arc<dyn BoardMemberRepository>;

    info!("Successfully initialized repositories");

    (user_repository, board_repository, board_member_repository)
}

pub fn initialize_services(
    user_repository: Arc<dyn UserRepository>,
    board_repository: Arc<dyn BoardRepository>,
    board_member_repository: Arc<dyn BoardMemberRepository>,
    redis_client: RedisClient,
) -> AppState {
    let token_service = Arc::new(RedisTokenService::new(redis_client)) as Arc<dyn TokenService>;
    let email_service =
        Arc::new(SmtpEmailService::new().expect("Failed to initialize email service"))
            as Arc<dyn EmailService>;

    let auth_service = Arc::new(AuthService::new(
        user_repository.clone(),
        token_service,
        email_service,
    ));
    let user_service = Arc::new(UserService::new(user_repository));
    let board_service = Arc::new(BoardService::new(board_repository, board_member_repository));

    info!("Successfully initialized services");

    AppState::new(auth_service, user_service, board_service)
}
