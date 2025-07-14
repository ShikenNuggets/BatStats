use axum::{
    http::{HeaderValue, Method}, routing::get, Json, Router
};

use rand::Rng;
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::cors::{Any, Cors, CorsLayer };
use tokio::net::TcpListener;

#[derive(Serialize)]
struct RandomNumber{
    value: u32,
}

async fn get_random_number() -> Json<RandomNumber> {
    let random_value = rand::thread_rng().gen_range(1..=100);
    Json(RandomNumber { value: random_value })
}

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/random", get(get_random_number))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Backend listening on http://{}", addr);

    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
