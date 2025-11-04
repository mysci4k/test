use crate::application::services::AuthService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
}

impl AppState {
    pub fn new(auth_service: Arc<AuthService>) -> Self {
        Self { auth_service }
    }
}
