mod speedrun_utils;

const ASYLUM_GAME_ID: &str = "4pd0p06e";
const CITY_GAME_ID: &str = "x3692ldl";
const ORIGINS_GAME_ID: &str = "4pdvp4dw";
const KNIGHT_GAME_ID: &str = "4d7p4rd7";
const MULTI_GAME_ID: &str = "nd2eyoed";
const CATEXT_GAME_ID: &str = "m1mnnv3d";

use axum::{
    http::{HeaderValue, Method}, routing::get, Json, Router
};

use rand::Rng;
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;

use std::future;

use futures::{ StreamExt, TryStreamExt };

#[derive(Serialize)]
struct RandomNumber{
    value: u32,
}

async fn get_random_number() -> Json<RandomNumber> {
    let random_value = rand::thread_rng().gen_range(1..=100);
    println!("Received request for random number, outputting {}", random_value);
    Json(RandomNumber { value: random_value })
}

#[tokio::main]
async fn main() {
    let _ = speedrun_utils::read_run_data_from_file("data.json");
    let _ = speedrun_utils::get_runs_for_game(ASYLUM_GAME_ID).await.unwrap();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/random", get(get_random_number))
        .layer(cors);

    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("Backend istening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    //let client = SpeedrunApiBuilder::new().build_async()?;

    //let endpoint = FullGameLeaderboard::builder()
    //    .game("xldev513") // example game
    //    .category("rklg3rdn")
    //    .build()
    //    .unwrap();

    //let leaderboard: types::Leaderboard = endpoint.query_async(&client).await?;

    // let client = SpeedrunApiBuilder::default().build_async().unwrap();
    // let endpoint = Runs::builder().build().unwrap();
    // endpoint.stream(&client)
    //     .take(30)
    //     .try_for_each_concurrent(10, |run: types::Run| {
    //         println!("{}", run.weblink);
    //         future::ready(Ok(()))
    //     })
    //     .await.unwrap();

    // let endpoint = Runs::builder()
    //     .status(RunStatus::Verified)
    //     .orderby(RunsSorting::VerifyDate)
    //     .direction(Direction::Desc)
    //     .build()
    //     .unwrap();

    // endpoint.stream(&client)
    //     .take(10)
    //     .try_for_each_concurrent(5, |run: types::Run|{
    //         println!("{}", run.weblink);
    //         future::ready(Ok(()))
    //     })
    //     .await.unwrap();

    //Ok(())
}
