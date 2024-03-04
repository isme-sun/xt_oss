#[allow(unused)]
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

mod routes {
    pub(super) async fn index() -> &'static str {
        "xtoss callback demo"
    }
    // pub(super) async fn oss_callback(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    //     (StatusCode::CREATED, Json(user))
    // }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(routes::index));
    // .route("/cb",post(routes::oss_callback));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
