mod speedrun_utils;
mod speedrun_api;

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
use serde::{Serialize};
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;

use std::{collections::HashMap};


use speedrun_api::src_api;

use crate::speedrun_api::types::{run::{RunPlayer, RunPlayerType}};

#[derive(Serialize)]
struct RandomNumber{
    value: u32,
}

async fn get_random_number() -> Json<RandomNumber> {
    let random_value = rand::thread_rng().gen_range(1..=100);
    println!("Received request for random number, outputting {}", random_value);
    Json(RandomNumber { value: random_value })
}

async fn get_player_name(player : &RunPlayer) -> Option<String>{
    if let RunPlayerType::Guest = player.rel{
        return player.name.clone();
    }

    if player.id.is_none(){
        return player.name.clone();
    }

    let user = src_api::get_user(&player.id.clone().unwrap()).await;
    if user.is_none(){
        return None;
    }

    return Some(user.unwrap().data.names.international);
}

#[tokio::main]
async fn main(){
    let asylum_leaderboards = src_api::get_all_fullgame_leaderboards(ASYLUM_GAME_ID).await;
    println!("Processing {} leaderboards...", asylum_leaderboards.len());

    let mut world_records: HashMap<String, i64> = HashMap::new();
    for lb in asylum_leaderboards{
        let wr_run = lb.runs.first();
        if !wr_run.is_some(){
            continue;
        }

        let player = &wr_run.unwrap().run.players.first();
        if !player.is_some(){
            continue;
        }

        let player_name = get_player_name(player.unwrap()).await;
        if player_name.is_none(){
            continue;
        }

        let player_name = player_name.unwrap();

        if let Some(value) = world_records.get_mut(&player_name){
            *value += 1;
        }else if !world_records.contains_key(&player_name){
            world_records.insert(player_name, 1);
        }
    }

    println!("{:?}", world_records);

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/random", get(get_random_number))
        .layer(cors);

    let listener = TcpListener::bind("127.0.0.1:3001").await.unwrap();
    println!("Backend istening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
