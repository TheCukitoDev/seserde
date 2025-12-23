use eserde::Deserialize;
use eserde_axum::Json;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let router = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        .route("/user", axum::routing::post(create_user));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

#[derive(Deserialize)]
pub struct Contact {
    pub email: String,
    pub phone: String,
}

#[derive(Deserialize)]
pub struct User {
    pub name: String,
    pub age: u8,
    pub contact: Contact,
}

async fn create_user(Json(payload): Json<User>) -> axum::http::StatusCode {
    println!("User: {}", payload.name);
    axum::http::StatusCode::CREATED
}
