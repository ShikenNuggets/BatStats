mod asylum;
mod city;
mod origins;
mod knight;

mod mastery;
mod speedrun_api;
mod utils;

const MULTI_GAME_ID: &str = "nd2eyoed";
const CATEXT_GAME_ID: &str = "m1mnnv3d";

use axum::{
	http::{HeaderValue, Method}, routing::get, Json, Router
};

use rand::Rng;
use serde::{Serialize};
use tower_http::cors::CorsLayer;
use tokio::{fs, net::TcpListener};

use std::{collections::{HashMap, HashSet}, path::Path};


use speedrun_api::src_api;

use crate::speedrun_api::types::{leaderboard::Leaderboard, run::{self, RunPlayer, RunPlayerType}};

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

async fn get_total_runner_times(leaderboards: &Vec<Leaderboard>, subtract_from_wr: bool) -> HashMap<String, f64>{
	let mut runner_times: HashMap<String, f64> = HashMap::new();

	let all_runners = get_all_runners(leaderboards).await;
	for runner in &all_runners{
		runner_times.insert(runner.to_string(), 0.0);
	}

	for leaderboard in leaderboards{
		let api_category = src_api::get_category(&leaderboard.category).await;
		if api_category.is_none() || api_category.as_ref().unwrap().miscellaneous{
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
				time_to_add = times[runner];
				if subtract_from_wr{
					time_to_add -= fastest_time;
				}
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

async fn combine_times(asylum_times: &HashMap<String, f64>, city_times: &HashMap<String, f64>, origins_times: &HashMap<String, f64>, knight_times: &HashMap<String, f64>) -> HashMap<String, f64>{
	let mut final_times: HashMap<String, f64> = HashMap::new();

	for time in asylum_times{
		if !city_times.contains_key(time.0) || !origins_times.contains_key(time.0) || !knight_times.contains_key(time.0){
			continue;
		}

		let time_val = time.1 + city_times[time.0] + origins_times[time.0] + knight_times[time.0];
		final_times.insert(time.0.to_string(), time_val);
	}

	return final_times;
}

async fn get_all_any_percent_times() -> HashMap<String, f64>{
	let asylum_any_times = asylum::get_best_any_percent_times().await;
	let city_any_times = city::get_best_any_percent_times().await;
	let origins_any_times = origins::get_best_any_percent_times().await;
	let knight_any_times = knight::get_best_any_percent_times().await;

	return combine_times(&asylum_any_times, &city_any_times, &origins_any_times, &knight_any_times).await;
}

async fn get_all_glitchless_times() -> HashMap<String, f64>{
	let asylum_any_times = asylum::get_best_glitchless_times().await;
	let city_any_times = city::get_best_glitchless_times().await;
	let origins_any_times = origins::get_best_glitchless_times().await;
	let knight_any_times = knight::get_best_glitchless_times().await;

	return combine_times(&asylum_any_times, &city_any_times, &origins_any_times, &knight_any_times).await;
}

async fn get_all_hundo_times() -> HashMap<String, f64>{
	let asylum_any_times = asylum::get_best_hundo_times().await;
	let city_any_times = city::get_best_hundo_times().await;
	let origins_any_times = origins::get_best_hundo_times().await;
	let knight_any_times = knight::get_best_hundo_times().await;

	return combine_times(&asylum_any_times, &city_any_times, &origins_any_times, &knight_any_times).await;
}

fn merge_mastery(mastery: &HashMap<String, f64>, overall_mastery: &mut HashMap<String, f64>, divisor: f64){
	for entry in mastery{
		let name = entry.0.clone();
		let value = entry.1 / divisor;
		if overall_mastery.contains_key(&name){
			let cur_val = overall_mastery[&name];
			overall_mastery.insert(name,  cur_val + value);
		}else{
			overall_mastery.insert(name, value);
		}
	}
}

async fn get_overall_mastery() -> HashMap<String, f64>{
	let asylum_mastery = mastery::get_mastery_ranks_for_game(asylum::GAME_ID).await;
	let city_mastery = mastery::get_mastery_ranks_for_game(city::GAME_ID).await;
	let origins_mastery = mastery::get_mastery_ranks_for_game(origins::GAME_ID).await;
	let knight_mastery = mastery::get_mastery_ranks_for_game(knight::GAME_ID).await;

	let mut overall_mastery = HashMap::new();
	merge_mastery(&asylum_mastery, &mut overall_mastery, 4.0);
	merge_mastery(&city_mastery, &mut overall_mastery, 4.0);
	merge_mastery(&origins_mastery, &mut overall_mastery, 4.0);
	merge_mastery(&knight_mastery, &mut overall_mastery, 4.0);

	return overall_mastery;
}

enum Ordering{
	LowerIsBetter,
	HigherIsBetter
}

fn get_sorted_values<T: PartialOrd>(map: HashMap<String, T>, order: Ordering) -> Vec<(String, T)>{
	let mut vec: Vec<(String, T)> = map.into_iter().collect();
	vec.sort_by(|a, b| {
		if let Ordering::LowerIsBetter = order{
			a.1.partial_cmp(&b.1).unwrap()
		}else{
			b.1.partial_cmp(&a.1).unwrap()
		}
	});

	return vec;
}

fn serialize_to_json<T: PartialOrd + Serialize>(map: HashMap<String, T>, order: Ordering) -> String{
	let sorted = get_sorted_values(map, order);
	return serde_json::to_string(&sorted).unwrap();
}

async fn serialize_to_file<T: PartialOrd + Serialize>(file_name: &str, map: HashMap<String, T>, order: Ordering) -> bool{
	let dir = Path::new(file_name).parent().unwrap();
	if !dir.exists(){
		fs::create_dir_all(dir).await.unwrap();
	}

	let content = serialize_to_json(map, order);
	if let Err(e) = fs::write(file_name, content).await{
		eprintln!("Could not write to file! Error: {}", e);
		return false;
	}

	return true;
}

#[tokio::main]
async fn main(){
	println!("Getting initial leaderboard data...");
	let asylum_leaderboards = src_api::get_all_fullgame_leaderboards(asylum::GAME_ID).await;
	let city_leaderboards = src_api::get_all_fullgame_leaderboards(city::GAME_ID).await;
	let origins_leaderboards = src_api::get_all_fullgame_leaderboards(origins::GAME_ID).await;
	let knight_leaderboards = src_api::get_all_fullgame_leaderboards(knight::GAME_ID).await;
	let _multigame_leaderboards = src_api::get_all_fullgame_leaderboards(MULTI_GAME_ID).await;
	let _catext_leaderboards = src_api::get_all_fullgame_leaderboards(CATEXT_GAME_ID).await;

	let mut all_main_boards: Vec<Leaderboard> = Vec::new();
	all_main_boards.extend(asylum_leaderboards.clone());
	all_main_boards.extend(city_leaderboards.clone());
	all_main_boards.extend(origins_leaderboards.clone());
	all_main_boards.extend(knight_leaderboards.clone());

	println!("Processing world records...");
	let wrs = get_world_records(&all_main_boards).await;
	serialize_to_file("Data/WorldRecords.json", wrs, Ordering::HigherIsBetter).await;

	println!("Processing fastest runners...");
	let runner_times = get_total_runner_times(&all_main_boards, true).await;
	serialize_to_file("Data/RunnerTimes.json", runner_times, Ordering::LowerIsBetter).await;

	println!("Processing highest ranking runners...");
	let runner_ranks = get_all_runner_ranks(&all_main_boards).await;
	serialize_to_file("Data/RunnerRanks.json", runner_ranks, Ordering::LowerIsBetter).await;

	println!("Processing Any% times...");
	let any_times = get_all_any_percent_times().await;
	serialize_to_file("Data/AnyTimes.json", any_times, Ordering::LowerIsBetter).await;

	println!("Processing Glitchless times...");
	let glitchless_times = get_all_glitchless_times().await;
	serialize_to_file("Data/GlitchlessTimes.json", glitchless_times, Ordering::LowerIsBetter).await;

	println!("Processing 100% times...");
	let hundo_times = get_all_hundo_times().await;
	serialize_to_file("Data/HundoTimes.json", hundo_times, Ordering::LowerIsBetter).await;

	println!("Processing Asylum mastery ranks...");
	let asylum_mastery = mastery::get_mastery_ranks_for_game(asylum::GAME_ID).await;
	serialize_to_file("Data/AsylumMastery.json", asylum_mastery, Ordering::HigherIsBetter).await;

	println!("Processing City mastery ranks...");
	let city_mastery = mastery::get_mastery_ranks_for_game(city::GAME_ID).await;
	serialize_to_file("Data/CityMastery.json", city_mastery, Ordering::HigherIsBetter).await;

	println!("Processing Origins mastery ranks...");
	let origins_mastery = mastery::get_mastery_ranks_for_game(origins::GAME_ID).await;
	serialize_to_file("Data/OriginsMastery.json", origins_mastery, Ordering::HigherIsBetter).await;

	println!("Processing Knight mastery ranks...");
	let knight_mastery = mastery::get_mastery_ranks_for_game(knight::GAME_ID).await;
	serialize_to_file("Data/KnightMastery.json", knight_mastery, Ordering::HigherIsBetter).await;

	println!("Processing overall mastery ranks...");
	let overall_mastery = get_overall_mastery().await;
	serialize_to_file("Data/OverallMastery.json", overall_mastery, Ordering::HigherIsBetter).await;

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

#[cfg(test)]
mod tests{
	use super::*;

	#[tokio::test]
	async fn combine_times_best_only_test(){
		let vars = HashMap::new();
		let mut all_boards: Vec<Leaderboard> = Vec::new();

		let asylum_any = src_api::get_leaderboard(asylum::GAME_ID, asylum::ANY_CAT_ID, &vars).await;
		let asylum_nms = src_api::get_leaderboard(asylum::GAME_ID, asylum::NMS_CAT_ID, &vars).await;
		let asylum_100 = src_api::get_leaderboard(asylum::GAME_ID, asylum::HUNDO_CAT_ID, &vars).await;
		let asylum_100_nms = src_api::get_leaderboard(asylum::GAME_ID, asylum::HUNDO_NMS_CAT_ID, &vars).await;
		
		assert!(asylum_any.is_some());
		assert!(asylum_nms.is_some());
		assert!(asylum_100.is_some());
		assert!(asylum_100_nms.is_some());
		all_boards.push(asylum_any.unwrap());
		all_boards.push(asylum_nms.unwrap());
		all_boards.push(asylum_100.unwrap());
		all_boards.push(asylum_100_nms.unwrap());

		let combined_times = utils::combine_times_best_only(&all_boards).await;
		assert!(combined_times.contains_key("ShikenNuggets"));
		assert!(combined_times["ShikenNuggets"] <= 3845.0);
	}
}
