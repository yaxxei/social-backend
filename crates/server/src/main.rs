use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{
    http::{self, header, HeaderValue},
    Router,
};
use lib_core::model::ModelManager;
use lib_web::handlers::AppState;
use routes::{
    routes_auth, routes_chat, routes_comment, routes_community, routes_like, routes_post,
    routes_profile, routes_user, routes_ws,
};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

static PORT: u16 = 3030;

mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mm = Arc::new(ModelManager::new().await?);

    let state = Arc::new(AppState {
        mm: mm.clone(),
        notification_conns: Arc::new(Mutex::new(HashMap::new())),
        chat_conns: Arc::new(Mutex::new(HashMap::new())),
    });

    let auth_app = routes_auth::routes(mm.clone()).await;
    let user_app = routes_user::routes(mm.clone()).await;
    let community_app = routes_community::routes(mm.clone()).await;
    let post_app = routes_post::routes(mm.clone()).await;
    let comment_app = routes_comment::routes(mm.clone()).await;
    let like_app = routes_like::routes(mm.clone()).await;
    let profile_app = routes_profile::routes(mm.clone()).await;
    let chat_app = routes_chat::routes(state.clone()).await;
    let ws_app = routes_ws::routes(state).await;

    let app = Router::new()
        .route("/", axum::routing::get(handler))
        .nest("/api/auth", auth_app)
        .nest("/api/users", user_app)
        .nest("/api/communities", community_app)
        .nest("/api/posts", post_app)
        .nest("/api/comments", comment_app)
        .nest("/api/likes", like_app)
        .nest("/api/profile", profile_app)
        .nest("/api/chats", chat_app)
        .nest("/api/ws", ws_app)
        .layer(
            CorsLayer::new()
                .allow_origin(HeaderValue::from_static("http://localhost:5173"))
                .allow_credentials(true)
                .allow_methods([
                    http::Method::GET,
                    http::Method::POST,
                    http::Method::PUT,
                    http::Method::PATCH,
                    http::Method::DELETE,
                ])
                .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]),
        );

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], PORT));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    tracing::info!("Server started on port = {}", PORT);

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(())
}

async fn handler() -> &'static str {
    "Hello, world"
}
