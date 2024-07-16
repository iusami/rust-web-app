use axum::{
    http::StatusCode,
    routing::{get, post},
    Router,
    response::IntoResponse,
    Json};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock}
    };
use thiserror::Error;
use tracing_subscriber;

#[derive(Error, Debug)]
enum RepositoryError{
    #[error("Not Found, id is {0}")]
    NotFound(i32),
}

pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static{
    fn create(&self, payload: CreateTodo) -> Todo;
    fn find(&self, id: i32) -> Option<Todo>;
    fn all(&self) -> Vec<Todo>;
    fn update(&self, id: i32) -> anyhow::Result<()>;
    fn delete(&self, id: i32) -> anyhow::Result<()>;
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Todo{
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CreateTodo{
    text: String
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateTodo{
    text: Option<String>,
    completed: Option<bool>,
}

impl Todo{
    pub fn new(id: i32, text: String) -> Self{
        Self {
            id,
            text,
            completed: false,
        }
    }
}

type TodoData = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory{
    store: Arc<RwLock<TodoData>>,
}

impl TodoRepositoryForMemory{
    pub fn new() -> Self{
        TodoRepositoryForMemory{
            store: Arc::default(),
        }
    }
}

impl TodoRepository for TodoRepositoryForMemory{
    fn create(&self, payload: CreateTodo) -> Todo {
        todo!();
    }
    fn find(&self, id: i32) -> Option<Todo> {
        todo!();
    }
    fn all(&self) -> Vec<Todo>{
        todo!();
    }
    fn update(&self, id: i32) -> anyhow::Result<()> {
        todo!();
    }
    fn delete(&self, id: i32) -> anyhow::Result<()> {
        todo!();
    }
}

#[tokio::main]
async fn main(){
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let app = create_app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {:?}", listener.local_addr());

    axum::serve(listener, app)
    .await
    .unwrap();
}

fn create_app() -> Router{
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct CreateUser{
    username: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct User{
    id: u64,
    username: String
}

#[cfg(test)]
mod test{
    use super::*;
    use axum::{
        body::Body,
        http::{header, Method, Request},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world(){
        let req = Request::builder().uri("/").body(Body::empty()).unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Hello, World!")
    }
    #[tokio::test]
    async fn should_return_user_data(){
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(r#"{"username":"foo"}"#))
            .unwrap();
        let res = create_app().oneshot(req).await.unwrap();
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        let body = String::from_utf8(bytes.to_vec()).unwrap();
        let user = serde_json::from_str::<User>(&body).expect("cannot convet User instance.");
        assert_eq!(user,
                   User{
                      id: 1337,
                      username: "foo".to_string()});
    }
}