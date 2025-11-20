use crate::application::services::{AuthService, BoardService, UserService, WebSocketService};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub auth_service: Arc<AuthService>,
    pub user_service: Arc<UserService>,
    pub board_service: Arc<BoardService>,
    pub websocket_service: Arc<WebSocketService>,
}

impl AppState {
    pub fn new(
        auth_service: Arc<AuthService>,
        user_service: Arc<UserService>,
        board_service: Arc<BoardService>,
        websocket_service: Arc<WebSocketService>,
    ) -> Self {
        Self {
            auth_service,
            user_service,
            board_service,
            websocket_service,
        }
    }
}
