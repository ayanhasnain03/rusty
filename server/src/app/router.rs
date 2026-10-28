
use axum::Router;

use crate::modules::auth::routes::routes as auth_routes;

pub fn create_router() -> Router {
    Router::new()
        .nest("/api/v1/auth", auth_routes())
}
