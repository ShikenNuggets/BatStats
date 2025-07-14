use axum::{
    routing::get,
    Json, Router,
};

use rand::Rng;
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer };
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
    let app = Router::new()
        .route("/random", get(get_random_number))
        .layer(CorsLayer::new().allow_origin(Any));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Backend listening on https://{}", addr);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on https://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
