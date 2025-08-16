use std::{collections::HashMap, hash::Hash, result};

use crate::speedrun_api::http_utils;
use crate::speedrun_api::types::category::CategoryType;
use crate::speedrun_api::types::leaderboard::LeaderboardResponse;
use crate::speedrun_api::types::{self, category};

const API_BASE_URL: &str = "https://www.speedrun.com/api/v1/";

pub async fn get_game_id(game_name: &str){
	let request_str = format!("{}games", API_BASE_URL);

	let mut args: HashMap<&str, &str> = HashMap::new();
	args.insert("name", game_name);

	let result = http_utils::get_http_result_with_args(&request_str, args).await;
	let body = match result{
		Ok(body) => body,
		Err(err) => {
			println!("HTPP request returned an error: {}", err);
			return;
		}
	};

	println!("{}", body);
}

pub async fn get_all_categories_for_game(game: &str) -> Vec<category::Category>{
	let mut ret_val: Vec<category::Category> = Vec::new();

	let request_str = format!("{}games/{}/categories", API_BASE_URL, game);
	let result = http_utils::get_http_result(&request_str).await;

	let body = match result{
		Ok(body) => body,
		Err(err) => {
			println!("HTTP request returned an error: {}", err);
			return Vec::new();
		}
	};

	let result: Result<types::category::CategoryResponse, serde_json::Error> = serde_json::from_str(&body);
	match result {
		Ok(parsed) => {
			for var in parsed.data{
				println!("{} - {}", var.id, var.name);
				ret_val.push(var);
			}
		},
		Err(err) => {
			println!("Failed to parse category response JSON: {}", err);
		}
	}

	return ret_val;

	//let map = match result {
	//	Ok(parsed) => parsed,
	//	Err(err) => {
	//		println!("Failed to parse JSON: {}", err);
	//	}
	//};

	//println!("{:?}", map);
}

pub async fn get_leaderboard(game: &str, category: &str) -> Option<LeaderboardResponse>{
	let str = format!("{}leaderboards/{}/category/{}", API_BASE_URL, game, category);
	let result = http_utils::get_http_result_with_args(&str, HashMap::new()).await;

	let body = match result {
		Ok(body) => body,
		Err(err) => {
			println!("HTTP request returned an error: {}", err);
			return None;
		}
	};
	
	let result: Result<types::leaderboard::LeaderboardResponse, serde_json::Error> = serde_json::from_str(&body);
	let map = match result {
		Ok(parsed) => parsed,
	 	Err(err) => {
	 		println!("Failed to parse JSON: {}", err);
	 		return None;
	 	}
	};

	return Some(map);
}
