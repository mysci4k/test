pub mod auth_controller;
pub mod server;
pub mod user_controller;

pub use auth_controller::configure as configure_auth_roures;
pub use server::configure_server;
pub use user_controller::configure as configure_user_routes;
