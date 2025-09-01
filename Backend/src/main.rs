mod speedrun_utils;
mod speedrun_api;

const ASYLUM_GAME_ID: &str = "4pd0p06e";
const CITY_GAME_ID: &str = "x3692ldl";
const ORIGINS_GAME_ID: &str = "4pdvp4dw";
const KNIGHT_GAME_ID: &str = "4d7p4rd7";
const MULTI_GAME_ID: &str = "nd2eyoed";
const CATEXT_GAME_ID: &str = "m1mnnv3d";

// -------------------------------------------------- //
// ------------------ Asylum ------------------------ //
// -------------------------------------------------- //
const ASYLUM_ANY_CAT_ID: &str = "9zdn672q";
const ASYLUM_NMS_CAT_ID: &str = "wkpyvjvk";
const ASYLUM_100_CAT_ID: &str = "zjdzyxkv";
const ASYLUM_100_NMS_CAT_ID: &str = "9kvj7mjk";

//const ASYLUM_ANY_DIFFICULTY_VAR_ID: &str = "wl32xvn1";
//	const ASYLUM_ANY_EASY_VAL_ID: &str = "klr3kyol";
//	const ASYLUM_ANY_HARD_VAL_ID: &str = "21dk83pl";

//const ASYLUM_NMS_DIFFICULTY_VAR_ID: &str = "68k2q382";
//	const ASYLUM_NMS_EASY_VAL_ID: &str = "mln8vzol";
//	const ASYLUM_NMS_HARD_VAL_ID: &str = "810vn9ol";

//const ASYLUM_100_DIFFICULTY_VAR_ID: &str = "p852z78g";
//	const ASYLUM_100_EASY_VAL_ID: &str = "zqoxe25q";
//	const ASYLUM_100_HARD_VAL_ID: &str = "0139k4r1";

//const ASYLUM_100_NMS_DIFFICULTY_VAR_ID: &str = "dlodgd8o";
//	const ASYLUM_100_NMS_EASY_VAL_ID: &str = "jq6vkdj1";
//	const ASYLUM_100_NMS_HARD_VAL_ID: &str = "5lm2j5mq";

// -------------------------------------------------- //
// -------------------- City ------------------------ //
// -------------------------------------------------- //
const CITY_ANY_CAT_ID: &str = "5dwjjogk";
const CITY_ANY_WCAT_CAT_ID: &str = "p7kjxg23";
const CITY_GLITCHLESS_CAT_ID: &str = "wdmw6ne2";
const CITY_GLITCHLESS_WCAT_CAT_ID: &str = "z273g9od";
const CITY_100_CAT_ID: &str = "4xk9qx20";

const CITY_ANY_DIFFICULTY_VAR_ID: &str = "jlze37l2";
	const CITY_ANY_EASY_VAL_ID: &str = "gq753nr1";
	const CITY_ANY_NORMAL_VAL_ID: &str = "rqvm995q";
	const CITY_ANY_HARD_VAL_ID: &str = "21gn0jol";

const CITY_ANY_WCAT_DIFFICULTY_VAR_ID: &str = "789p93nw";
	const CITY_ANY_WCAT_EASY_VAL_ID: &str = "z192e88q";
	const CITY_ANY_WCAT_NORMAL_VAL_ID: &str = "0132ppkq";
	const CITY_ANY_WCAT_HARD_VAL_ID: &str = "p12vjx4q";

const CITY_GLITCHLESS_DIFFICULTY_VAR_ID: &str = "ylpv6x6l";
	const CITY_GLITCHLESS_EASY_VAL_ID: &str = "zqoyxpx1";
	const CITY_GLITCHLESS_NORMAL_VAL_ID: &str = "013v98xl";
	const CITY_GLITCHLESS_HARD_VAL_ID: &str = "rqv4vp6q";

const CITY_GLITCHLESS_WCAT_DIFFICULTY_VAR_ID: &str = "p8564xng";
	const CITY_GLITCHLESS_WCAT_EASY_VAL_ID: &str = "5len8mpl";
	const CITY_GLITCHLESS_WCAT_NORMAL_VAL_ID: &str = "rqvm9g7q";
	const CITY_GLITCHLESS_WCAT_HARD_VAL_ID: &str = "0q548grl";

// -------------------------------------------------- //
// ------------------ Origins ----------------------- //
// -------------------------------------------------- //
const ORIGINS_ANY_CAT_ID: &str = "w5dwog2g";
const ORIGINS_GLITCHLESS_CAT_ID: &str = "rklez0q2";
const ORIGINS_100_CAT_ID: &str = "7wk69ek1";

const ORIGINS_ANY_DIFFICULTY_VAR_ID: &str = "38dq40n0";
	const ORIGINS_ANY_EASY_VAL_ID: &str = "p12vj4dq";
	const ORIGINS_ANY_NORMAL_VAL_ID: &str = "0q54ry7l";
	const ORIGINS_ANY_HARD_VAL_ID: &str = "81pyk6v1";

const ORIGINS_GLITCHLESS_DIFFICULTY_VAR_ID: &str = "gnx7vxlv";
	const ORIGINS_GLITCHLESS_EASY_VAL_ID: &str = "81w70ovq";
	const ORIGINS_GLITCHLESS_NORMAL_VAL_ID: &str = "zqojvox1";
	const ORIGINS_GLITCHLESS_HARD_VAL_ID: &str = "013x67x1";

// -------------------------------------------------- //
// ------------------- Knight ----------------------- //
// -------------------------------------------------- //
const KNIGHT_ANY_CAT_ID: &str = "7kjg8gk3";
const KNIGHT_GLITCHLESS_CAT_ID: &str = "7kjz9p3d";
const KNIGHT_FALL_CAT_ID: &str = "02q8rg72";

const KNIGHT_ANY_DIFFICULTY_VAR_ID: &str = "5ly35gn4";
	const KNIGHT_ANY_EASY_VAL_ID: &str = "21gn0o6l";
	const KNIGHT_ANY_NORMAL_VAL_ID: &str = "z19x0pj1";
	const KNIGHT_ANY_HARD_VAL_ID: &str = "jqzn2y2q";

const KNIGHT_GLITCHLESS_DIFFICULTY_VAR_ID: &str = "38d69xl0";
	const KNIGHT_GLITCHLESS_EASY_VAL_ID: &str = "5lenpwpl";
	const KNIGHT_GLITCHLESS_NORMAL_VAL_ID: &str = "4lxgmjjq";
	const KNIGHT_GLITCHLESS_HARD_VAL_ID: &str = "0q54d0rl";

use axum::{
	http::{HeaderValue, Method}, routing::get, Json, Router
};

use rand::Rng;
use serde::{Serialize};
use tower_http::cors::CorsLayer;
use tokio::net::TcpListener;

use std::{collections::{HashMap, HashSet}};


use speedrun_api::src_api;

use crate::speedrun_api::{types::{leaderboard::{Leaderboard}, run::{RunPlayer, RunPlayerType}, variable::Variable}};

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

async fn get_leaderboard_for_subcategory(game_id: &str, category_id: &str, var_id: &str, val_id: &str) -> Option<Leaderboard>{
	let mut vars: HashMap<String, String> = HashMap::new();
	vars.insert(var_id.to_string(), val_id.to_string());
	return src_api::get_leaderboard(game_id, category_id, &vars).await;
}

async fn combine_times_best_only(leaderboards: &Vec<Leaderboard>) -> HashMap<String, f64>{
	let mut combined_times: HashMap<String, f64> = HashMap::new();

	for leaderboard in leaderboards{
		let mut lb_temp: Vec<Leaderboard> = Vec::new();
		lb_temp.push(leaderboard.clone());
		let times = get_total_runner_times(&lb_temp, false).await;
		for time in times{
			if !combined_times.contains_key(&time.0) || combined_times[&time.0] > time.1{
				combined_times.insert(time.0, time.1);
			}
		}
	}
	
	return combined_times;
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

async fn get_best_asylum_any_percent_times() -> HashMap<String, f64>{
	let vars = HashMap::new();
	let asylum_any = src_api::get_leaderboard(ASYLUM_GAME_ID, ASYLUM_ANY_CAT_ID, &vars).await;
	let asylum_nms = src_api::get_leaderboard(ASYLUM_GAME_ID, ASYLUM_NMS_CAT_ID, &vars).await;
	let asylum_100 = src_api::get_leaderboard(ASYLUM_GAME_ID, ASYLUM_100_CAT_ID, &vars).await;
	let asylum_100_nms = src_api::get_leaderboard(ASYLUM_GAME_ID, ASYLUM_100_NMS_CAT_ID, &vars).await;

	if asylum_any.is_none() || asylum_nms.is_none() || asylum_100.is_none() || asylum_100_nms.is_none(){
		println!("Failed to get all Any% boards for Asylum");
		return HashMap::new();
	}

	let mut all_boards: Vec<Leaderboard> = Vec::new();
	all_boards.push(asylum_any.unwrap());
	all_boards.push(asylum_nms.unwrap());
	all_boards.push(asylum_100.unwrap());
	all_boards.push(asylum_100_nms.unwrap());

	return combine_times_best_only(&all_boards).await;
}

async fn get_best_city_any_percent_times() -> HashMap<String, f64>{
	let any_easy = get_leaderboard_for_subcategory(CITY_GAME_ID, CITY_ANY_CAT_ID, CITY_ANY_DIFFICULTY_VAR_ID, CITY_ANY_EASY_VAL_ID).await;
	let any_normal = get_leaderboard_for_subcategory(CITY_GAME_ID, CITY_ANY_CAT_ID, CITY_ANY_DIFFICULTY_VAR_ID, CITY_ANY_NORMAL_VAL_ID).await;
	let any_hard = get_leaderboard_for_subcategory(CITY_GAME_ID, CITY_ANY_CAT_ID, CITY_ANY_DIFFICULTY_VAR_ID, CITY_ANY_HARD_VAL_ID).await;

	let any_wcat_easy = get_leaderboard_for_subcategory(CITY_GAME_ID, CITY_ANY_WCAT_CAT_ID, CITY_ANY_WCAT_DIFFICULTY_VAR_ID, CITY_ANY_WCAT_EASY_VAL_ID).await;
	let any_wcat_normal = get_leaderboard_for_subcategory(CITY_GAME_ID, CITY_ANY_WCAT_CAT_ID, CITY_ANY_WCAT_DIFFICULTY_VAR_ID, CITY_ANY_WCAT_NORMAL_VAL_ID).await;
	let any_wcat_hard = get_leaderboard_for_subcategory(CITY_GAME_ID, CITY_ANY_WCAT_CAT_ID, CITY_ANY_WCAT_DIFFICULTY_VAR_ID, CITY_ANY_WCAT_HARD_VAL_ID).await;

	let glitchless_easy = get_leaderboard_for_subcategory(CITY_GAME_ID, CITY_GLITCHLESS_CAT_ID, CITY_GLITCHLESS_DIFFICULTY_VAR_ID, CITY_GLITCHLESS_EASY_VAL_ID).await;
	let glitchless_normal = get_leaderboard_for_subcategory(CITY_GAME_ID, CITY_GLITCHLESS_CAT_ID, CITY_GLITCHLESS_DIFFICULTY_VAR_ID, CITY_GLITCHLESS_NORMAL_VAL_ID).await;
	let glitchless_hard = get_leaderboard_for_subcategory(CITY_GAME_ID, CITY_GLITCHLESS_CAT_ID, CITY_GLITCHLESS_DIFFICULTY_VAR_ID, CITY_GLITCHLESS_HARD_VAL_ID).await;

	let glitchless_wcat_easy = get_leaderboard_for_subcategory(CITY_GAME_ID, CITY_GLITCHLESS_WCAT_CAT_ID, CITY_GLITCHLESS_WCAT_DIFFICULTY_VAR_ID, CITY_GLITCHLESS_WCAT_EASY_VAL_ID).await;
	let glitchless_wcat_normal = get_leaderboard_for_subcategory(CITY_GAME_ID, CITY_GLITCHLESS_WCAT_CAT_ID, CITY_GLITCHLESS_WCAT_DIFFICULTY_VAR_ID, CITY_GLITCHLESS_WCAT_NORMAL_VAL_ID).await;
	let glitchless_wcat_hard = get_leaderboard_for_subcategory(CITY_GAME_ID, CITY_GLITCHLESS_WCAT_CAT_ID, CITY_GLITCHLESS_WCAT_DIFFICULTY_VAR_ID, CITY_GLITCHLESS_WCAT_HARD_VAL_ID).await;

	let vars = HashMap::new();
	let hundo = src_api::get_leaderboard(CITY_GAME_ID, CITY_100_CAT_ID, &vars).await;

	if any_easy.is_none() || any_normal.is_none() || any_hard.is_none()
		|| any_wcat_easy.is_none() || any_wcat_normal.is_none() || any_wcat_hard.is_none()
		|| glitchless_easy.is_none() || glitchless_normal.is_none() || glitchless_hard.is_none()
		|| glitchless_wcat_easy.is_none() || glitchless_wcat_normal.is_none() || glitchless_wcat_hard.is_none()
		|| hundo.is_none()
	{
		println!("Failed to get all Any% boards for City");
		return HashMap::new();
	}

	let mut all_boards: Vec<Leaderboard> = Vec::new();
	all_boards.push(any_easy.unwrap());
	all_boards.push(any_normal.unwrap());
	all_boards.push(any_hard.unwrap());

	all_boards.push(any_wcat_easy.unwrap());
	all_boards.push(any_wcat_normal.unwrap());
	all_boards.push(any_wcat_hard.unwrap());

	all_boards.push(glitchless_easy.unwrap());
	all_boards.push(glitchless_normal.unwrap());
	all_boards.push(glitchless_hard.unwrap());

	all_boards.push(glitchless_wcat_easy.unwrap());
	all_boards.push(glitchless_wcat_normal.unwrap());
	all_boards.push(glitchless_wcat_hard.unwrap());

	all_boards.push(hundo.unwrap());

	return combine_times_best_only(&all_boards).await;
}

async fn get_best_origins_any_percent_times() -> HashMap<String, f64>{
	let any_easy = get_leaderboard_for_subcategory(ORIGINS_GAME_ID, ORIGINS_ANY_CAT_ID, ORIGINS_ANY_DIFFICULTY_VAR_ID, ORIGINS_ANY_EASY_VAL_ID).await;
	let any_normal = get_leaderboard_for_subcategory(ORIGINS_GAME_ID, ORIGINS_ANY_CAT_ID, ORIGINS_ANY_DIFFICULTY_VAR_ID, ORIGINS_ANY_NORMAL_VAL_ID).await;
	let any_hard = get_leaderboard_for_subcategory(ORIGINS_GAME_ID, ORIGINS_ANY_CAT_ID, ORIGINS_ANY_DIFFICULTY_VAR_ID, ORIGINS_ANY_HARD_VAL_ID).await;

	let glitchless_easy = get_leaderboard_for_subcategory(ORIGINS_GAME_ID, ORIGINS_GLITCHLESS_CAT_ID, ORIGINS_GLITCHLESS_DIFFICULTY_VAR_ID, ORIGINS_GLITCHLESS_EASY_VAL_ID).await;
	let glitchless_normal = get_leaderboard_for_subcategory(ORIGINS_GAME_ID, ORIGINS_GLITCHLESS_CAT_ID, ORIGINS_GLITCHLESS_DIFFICULTY_VAR_ID, ORIGINS_GLITCHLESS_NORMAL_VAL_ID).await;
	let glitchless_hard = get_leaderboard_for_subcategory(ORIGINS_GAME_ID, ORIGINS_GLITCHLESS_CAT_ID, ORIGINS_GLITCHLESS_DIFFICULTY_VAR_ID, ORIGINS_GLITCHLESS_HARD_VAL_ID).await;

	let vars = HashMap::new();
	let hundo = src_api::get_leaderboard(ORIGINS_GAME_ID, ORIGINS_100_CAT_ID, &vars).await;

	if any_easy.is_none() || any_normal.is_none() || any_hard.is_none()
		|| glitchless_easy.is_none() || glitchless_normal.is_none() || glitchless_hard.is_none()
		|| hundo.is_none()
	{
		println!("Failed to get all Any% boards for Origins");
		return HashMap::new();
	}

	let mut all_boards: Vec<Leaderboard> = Vec::new();
	all_boards.push(any_easy.unwrap());
	all_boards.push(any_normal.unwrap());
	all_boards.push(any_hard.unwrap());

	all_boards.push(glitchless_easy.unwrap());
	all_boards.push(glitchless_normal.unwrap());
	all_boards.push(glitchless_hard.unwrap());

	all_boards.push(hundo.unwrap());

	return combine_times_best_only(&all_boards).await;
}

async fn get_best_knight_any_percent_times() -> HashMap<String, f64>{
	let any_easy = get_leaderboard_for_subcategory(KNIGHT_GAME_ID, KNIGHT_ANY_CAT_ID, KNIGHT_ANY_DIFFICULTY_VAR_ID, KNIGHT_ANY_EASY_VAL_ID).await;
	let any_normal = get_leaderboard_for_subcategory(KNIGHT_GAME_ID, KNIGHT_ANY_CAT_ID, KNIGHT_ANY_DIFFICULTY_VAR_ID, KNIGHT_ANY_NORMAL_VAL_ID).await;
	let any_hard = get_leaderboard_for_subcategory(KNIGHT_GAME_ID, KNIGHT_ANY_CAT_ID, KNIGHT_ANY_DIFFICULTY_VAR_ID, KNIGHT_ANY_HARD_VAL_ID).await;

	let glitchless_easy = get_leaderboard_for_subcategory(KNIGHT_GAME_ID, KNIGHT_GLITCHLESS_CAT_ID, KNIGHT_GLITCHLESS_DIFFICULTY_VAR_ID, KNIGHT_GLITCHLESS_EASY_VAL_ID).await;
	let glitchless_normal = get_leaderboard_for_subcategory(KNIGHT_GAME_ID, KNIGHT_GLITCHLESS_CAT_ID, KNIGHT_GLITCHLESS_DIFFICULTY_VAR_ID, KNIGHT_GLITCHLESS_NORMAL_VAL_ID).await;
	let glitchless_hard = get_leaderboard_for_subcategory(KNIGHT_GAME_ID, KNIGHT_GLITCHLESS_CAT_ID, KNIGHT_GLITCHLESS_DIFFICULTY_VAR_ID, KNIGHT_GLITCHLESS_HARD_VAL_ID).await;

	let vars = HashMap::new();
	let knightfall = src_api::get_leaderboard(KNIGHT_GAME_ID, KNIGHT_FALL_CAT_ID, &vars).await;

	if any_easy.is_none() || any_normal.is_none() || any_hard.is_none()
		|| glitchless_easy.is_none() || glitchless_normal.is_none() || glitchless_hard.is_none()
		|| knightfall.is_none()
	{
		println!("Failed to get all Any% boards for Knight");
		return HashMap::new();
	}

	let mut all_boards: Vec<Leaderboard> = Vec::new();
	all_boards.push(any_easy.unwrap());
	all_boards.push(any_normal.unwrap());
	all_boards.push(any_hard.unwrap());

	all_boards.push(glitchless_easy.unwrap());
	all_boards.push(glitchless_normal.unwrap());
	all_boards.push(glitchless_hard.unwrap());

	all_boards.push(knightfall.unwrap());

	return combine_times_best_only(&all_boards).await;
}

async fn get_all_any_percent_times() -> HashMap<String, f64>{
	let asylum_any_times = get_best_asylum_any_percent_times().await;
	let city_any_times = get_best_city_any_percent_times().await;
	let origins_any_times = get_best_origins_any_percent_times().await;
	let knight_any_times = get_best_knight_any_percent_times().await;

	if asylum_any_times.contains_key("ShikenNuggets"){
		println!("Asylum = {}", asylum_any_times["ShikenNuggets"]);
	}

	if city_any_times.contains_key("ShikenNuggets"){
		println!("City = {}", city_any_times["ShikenNuggets"]);
	}

	if origins_any_times.contains_key("ShikenNuggets"){
		println!("Origins = {}", origins_any_times["ShikenNuggets"]);
	}

	if knight_any_times.contains_key("ShikenNuggets"){
		println!("Knight = {}", knight_any_times["ShikenNuggets"]);
	}

	return combine_times(&asylum_any_times, &city_any_times, &origins_any_times, &knight_any_times).await;
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

	println!("--------------------------------------------------");
	println!("Processing world records...");
	let wrs = get_world_records(&all_main_boards).await;
	println!("World Records: {:?}", wrs);

	println!("--------------------------------------------------");
	println!("Processing fastest runners...");
	let runner_times = get_total_runner_times(&all_main_boards, true).await;
	println!("All Runner Times: {:?}", runner_times);

	println!("--------------------------------------------------");
	println!("Processing highest ranking runners...");
	let runner_ranks = get_all_runner_ranks(&all_main_boards).await;
	println!("All Runner Ranks: {:?}", runner_ranks);

	println!("--------------------------------------------------");
	let any_times = get_all_any_percent_times().await;
	println!("Any% Times: {:?}", any_times);

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

		let asylum_any = src_api::get_leaderboard(ASYLUM_GAME_ID, ASYLUM_ANY_CAT_ID, &vars).await;
		let asylum_nms = src_api::get_leaderboard(ASYLUM_GAME_ID, ASYLUM_NMS_CAT_ID, &vars).await;
		let asylum_100 = src_api::get_leaderboard(ASYLUM_GAME_ID, ASYLUM_100_CAT_ID, &vars).await;
		let asylum_100_nms = src_api::get_leaderboard(ASYLUM_GAME_ID, ASYLUM_100_NMS_CAT_ID, &vars).await;
		
		assert!(asylum_any.is_some());
		assert!(asylum_nms.is_some());
		assert!(asylum_100.is_some());
		assert!(asylum_100_nms.is_some());
		all_boards.push(asylum_any.unwrap());
		all_boards.push(asylum_nms.unwrap());
		all_boards.push(asylum_100.unwrap());
		all_boards.push(asylum_100_nms.unwrap());

		let combined_times = combine_times_best_only(&all_boards).await;
		assert!(combined_times.contains_key("ShikenNuggets"));
		assert!(combined_times["ShikenNuggets"] <= 3845.0);
	}
}
