use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
    response::IntoResponse,
    Json};
use serde::{Deserialize, Serialize};
use std::env;
use tracing_subscriber;

#[tokio::main]
async fn main(){
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {:?}", listener.local_addr());

    axum::serve(listener, app)
    .await
    .unwrap();
}

async fn root() -> &'static str{
    "Hello, World!"
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse{
    let user = User{
        id: 1337,
        username: payload.username
    };
    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser{
    username: String
}

#[derive(Serialize)]
struct User{
    id: u64,
    username: String
}