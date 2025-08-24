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

use std::{collections::{HashMap, HashSet}};


use speedrun_api::src_api;

use crate::speedrun_api::{src_api::get_category, types::{leaderboard::Leaderboard, run::{self, RunPlayer, RunPlayerType}, variable::Variable}};

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

	return Some(user.unwrap().names.international);
}

fn get_player_id_or_guest_name(player : &RunPlayer) -> Option<String>{
	if let RunPlayerType::Guest = player.rel{
		return player.name.clone();
	}

	return player.id.clone();
}

async fn get_world_records(leaderboards: &Vec<Leaderboard>) -> HashMap<String, i64>{
	let mut world_records: HashMap<String, i64> = HashMap::new();
	for lb in leaderboards{
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

	return world_records;
}

async fn get_all_runners(leaderboards: &Vec<Leaderboard>) -> HashSet<String>{
	let mut runners: HashSet<String> = HashSet::new();

	for leaderboard in leaderboards{
		for run in &leaderboard.runs{
			for player in &run.run.players{
				let player_name = get_player_name(&player).await;
				if player_name.is_none(){
					continue;
				}
				runners.insert(player_name.unwrap());
			}
		}
	}

	return runners;
}

fn get_fastest_time(leaderboard: &Leaderboard) -> Option<f64>{
	if leaderboard.runs.is_empty(){
		return None;
	}

	return Some(leaderboard.runs.first().unwrap().run.times.primary_t);
}

fn get_slowest_time(leaderboard: &Leaderboard) -> Option<f64>{
	if leaderboard.runs.is_empty(){
		return None;
	}

	return Some(leaderboard.runs.last().unwrap().run.times.primary_t);
}

fn get_last_place(leaderboard: &Leaderboard) -> i64{
	if leaderboard.runs.is_empty(){
		return 0;
	}
	
	return leaderboard.runs.last().unwrap().place.into();
}

async fn get_runner_times_map(leaderboard: &Leaderboard) -> HashMap<String, f64>{
	let mut run_times: HashMap<String, f64> = HashMap::new();

	let fastest_time = get_fastest_time(&leaderboard);
	let slowest_time = get_slowest_time(&leaderboard);

	if fastest_time.is_none() || slowest_time.is_none(){
		return run_times;
	}
	
	for run in &leaderboard.runs{
		let runner_name = get_player_name(run.run.players.first().unwrap()).await;
		if runner_name.is_none(){
			continue;
		}

		run_times.insert(runner_name.unwrap(), run.run.times.primary_t);
	}

	return run_times;
}

async fn get_runner_ranks_map(leaderboard: &Leaderboard) -> HashMap<String, i64>{
	let mut run_ranks: HashMap<String, i64> = HashMap::new();

	let last_place = get_last_place(leaderboard);
	if last_place <= 0{
		return run_ranks;
	}
	
	for run in &leaderboard.runs{
		let runner_name = get_player_name(run.run.players.first().unwrap()).await;
		if runner_name.is_none(){
			continue;
		}

		let rank_as_i64: i64 = run.place.into();
		run_ranks.insert(runner_name.unwrap(), rank_as_i64 - 1);
	}

	return run_ranks;
}

async fn get_variable_value_name(var: &Variable, value_id: &str) -> Option<String>{
	for val in &var.values.values{
		if val.1.label == value_id{
			return Some(val.1.label.clone());
		}
	}

	return None;
}

async fn get_leaderboard_name(leaderboard: &Leaderboard) -> String{
	let mut lb_name: String = String::new();

	let api_game = src_api::get_game(&leaderboard.game).await;
	if api_game.is_some(){
		lb_name += &api_game.unwrap().names.international;
		lb_name += " - ";
	}

	let api_category = src_api::get_category(&leaderboard.category).await;
	if api_category.is_some(){
		lb_name += &api_category.unwrap().name;
	}

	for var in &leaderboard.values{
		let api_var = src_api::get_variable(var.0).await;
		if api_var.is_none(){
			continue;
		}
		let api_var = api_var.unwrap();

		let value_name = get_variable_value_name(&api_var, var.1).await;
		if value_name.is_none(){
			continue;
		}
		let value_name = value_name.unwrap();

		lb_name += " (";
		lb_name += &api_var.name;
		lb_name += " = ";
		lb_name += &value_name;
		lb_name += ")";
	}

	return lb_name;

}

async fn get_total_runner_times(leaderboards: &Vec<Leaderboard>) -> HashMap<String, f64>{
	let mut runner_times: HashMap<String, f64> = HashMap::new();

	let all_runners = get_all_runners(leaderboards).await;
	for runner in &all_runners{
		runner_times.insert(runner.to_string(), 0.0);
	}

	for leaderboard in leaderboards{
		let api_category = src_api::get_category(&leaderboard.category).await;
		if api_category.is_none() || api_category.unwrap().miscellaneous{
			continue;
		}

		let fastest_time = get_fastest_time(&leaderboard);
		let slowest_time = get_slowest_time(&leaderboard);
		let times = get_runner_times_map(leaderboard).await;

		if fastest_time.is_none() || slowest_time.is_none() || times.is_empty(){
			println!("Something went wrong for leaderboard of category {}", leaderboard.category);
			continue;
		}

		let fastest_time = fastest_time.unwrap();
		let slowest_time = slowest_time.unwrap() - fastest_time;

		for runner in &all_runners{
			let mut time_to_add = slowest_time;
			if times.contains_key(runner){
				time_to_add = times[runner] - fastest_time;
			}

			runner_times.insert(runner.to_string(), runner_times[runner] + time_to_add);
		}
	}

	return runner_times;
}

async fn get_all_runner_ranks(leaderboards: &Vec<Leaderboard>) -> HashMap<String, i64>{
	let mut runner_ranks: HashMap<String, i64> = HashMap::new();

	let all_runners = get_all_runners(leaderboards).await;
	for runner in &all_runners{
		runner_ranks.insert(runner.to_string(), 0);
	}

	for leaderboard in leaderboards{
		let api_category = src_api::get_category(&leaderboard.category).await;
		if api_category.is_none() || api_category.unwrap().miscellaneous{
			continue;
		}

		let last_place = get_last_place(leaderboard);
		let ranks = get_runner_ranks_map(leaderboard).await;

		if last_place <= 0 || ranks.is_empty(){
			println!("Something went wrong for leaderboard of category {}", leaderboard.category);
			continue;
		}

		for runner in &all_runners{
			let mut rank_to_add = last_place;
			if ranks.contains_key(runner){
				rank_to_add = ranks[runner];
			}

			runner_ranks.insert(runner.to_string(), runner_ranks[runner] + rank_to_add);
		}
	}

	return runner_ranks;
}

#[tokio::main]
async fn main(){
	println!("Getting initial leaderboard data...");
	let asylum_leaderboards = src_api::get_all_fullgame_leaderboards(ASYLUM_GAME_ID).await;
	let city_leaderboards = src_api::get_all_fullgame_leaderboards(CITY_GAME_ID).await;
	let origins_leaderboards = src_api::get_all_fullgame_leaderboards(ORIGINS_GAME_ID).await;
	let knight_leaderboards = src_api::get_all_fullgame_leaderboards(KNIGHT_GAME_ID).await;
	let _multigame_leaderboards = src_api::get_all_fullgame_leaderboards(MULTI_GAME_ID).await;
	let _catext_leaderboards = src_api::get_all_fullgame_leaderboards(CATEXT_GAME_ID).await;

	let mut all_main_boards: Vec<Leaderboard> = Vec::new();
	all_main_boards.extend(asylum_leaderboards.clone());
	all_main_boards.extend(city_leaderboards.clone());
	all_main_boards.extend(origins_leaderboards.clone());
	all_main_boards.extend(knight_leaderboards.clone());

	println!("Processing world records...");
	let wrs = get_world_records(&all_main_boards).await;
	println!("World Records: {:?}", wrs);

	println!("Processing fastest runners...");
	let runner_times = get_total_runner_times(&all_main_boards).await;
	println!("All Runner Times: {:?}", runner_times);

	println!("Processing highest ranking runners...");
	let runner_ranks = get_all_runner_ranks(&all_main_boards).await;
	println!("All Runner Ranks: {:?}", runner_ranks);

	//println!("Asylum: ");
	//print_world_records_for_game(ASYLUM_GAME_ID).await;

	//println!("City: ");
	//print_world_records_for_game(CITY_GAME_ID).await;

	//println!("Origins: ");
	//print_world_records_for_game(ORIGINS_GAME_ID).await;

	//println!("Knight: ");
	//print_world_records_for_game(KNIGHT_GAME_ID).await;

	/* let asylum_leaderboards = src_api::get_all_fullgame_leaderboards(ASYLUM_GAME_ID).await;
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

	println!("{:?}", world_records); */

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
