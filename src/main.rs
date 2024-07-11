use axum::{routing::get, Router};
use std::env;
use tracing_subscriber;

#[tokio::main]
async fn main(){
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {:?}", listener.local_addr());

    axum::serve(listener, app)
    .await
    .unwrap();
}

async fn root() -> &'static str{
    "Hello, World!"
}