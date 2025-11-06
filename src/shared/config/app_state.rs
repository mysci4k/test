use crate::application::services::{AuthService, UserService};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
}

impl AppState {
    pub fn new(auth_service: Arc<AuthService>, user_service: Arc<UserService>) -> Self {
        Self {
            auth_service,
            user_service,
        }
    }
}
