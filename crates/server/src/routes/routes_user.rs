use std::sync::Arc;

use axum::{
    middleware,
    routing::{delete, get, put},
    Router,
};
use lib_core::model::ModelManager;
use lib_web::{handlers::handlers_user, middlewares};

pub async fn routes(mm: Arc<ModelManager>) -> Router {
    Router::new()
        .route("/", get(handlers_user::get_all_users))
        .route("/{nickname}", get(handlers_user::get_user_profile))
        .route("/{nickname}", put(handlers_user::update_user_profile))
        .route("/{nickname}", delete(handlers_user::delete_user_profile))
        .with_state(mm)
        .layer(middleware::from_fn(middlewares::require_auth))
}
