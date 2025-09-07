use std::{collections::HashMap};

use crate::speedrun_api::src_cache::LEADERBOARD_CACHE;
use crate::speedrun_api::{http_utils, src_api, src_cache};
use crate::speedrun_api::types::category::{Category, CategoryType};
use crate::speedrun_api::types::game::Game;
use crate::speedrun_api::types::leaderboard::Leaderboard;
use crate::speedrun_api::types::variable::{Variable, VariablesResponse};
use crate::speedrun_api::types::{self, category};

const API_BASE_URL: &str = "https://www.speedrun.com/api/v1/";

pub async fn get_game_id(game_name: &str){
	let request_str = format!("{}games", API_BASE_URL);

	let mut args: HashMap<String, String> = HashMap::new();
	args.insert("name".to_string(), game_name.to_string());

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

pub async fn get_game(game_id: &str) -> Option<Game>{
	let cached_game = src_cache::GAME_CACHE.get(game_id);
	if cached_game.is_some(){
		return cached_game;
	}

	let str = format!("{}games/{}", API_BASE_URL, game_id);
	let result = http_utils::get_http_result(&str).await;

	let body = match result{
		Ok(body) => body,
		Err(err) => {
			println!("Failed to parse JSON: {}", err);
			return None;
		}
	};

	let result: Result<types::game::GameResponse, serde_json::Error> = serde_json::from_str(&body);
	let map = match result {
		Ok(parsed) => parsed,
		Err(err) => {
			println!("Failed to parse JSON: {}", err);
			println!("{}", body);
			return None;
		}
	};

	src_cache::GAME_CACHE.insert(&map.data);
	return Some(map.data);
}

pub async fn get_category(category_id: &str) -> Option<Category>{
	let cached_category = src_cache::CATEGORY_CACHE.get(category_id);
	if cached_category.is_some(){
		return cached_category;
	}

	let str = format!("{}categories/{}", API_BASE_URL, category_id);
	let result = http_utils::get_http_result(&str).await;

	let body = match  result {
		Ok(body) => body,
		Err(err) => {
			println!("Failed to parse JSON: {}", err);
			return None;
		}
	};

	let result: Result<types::category::CategoryResponse, serde_json::Error> = serde_json::from_str(&body);
	let map = match result {
		Ok(parsed) => parsed,
		Err(err) => {
			println!("Failed to parse JSON: {}", err);
			println!("{}", body);
			return None;
		}
	};

	src_cache::CATEGORY_CACHE.insert(&map.data);
	return Some(map.data);
}

pub async fn get_all_categories_for_game(game: &str) -> Vec<category::Category>{
	let mut ret_val: Vec<category::Category> = Vec::new();

	let request_str = format!("{}games/{}/categories", API_BASE_URL, game);
	if src_cache::ALL_CATS_CACHE.contains_key(&request_str){
		return src_cache::ALL_CATS_CACHE.get(&request_str).unwrap().to_vec();
	}

	let result = http_utils::get_http_result(&request_str).await;

	let body = match result{
		Ok(body) => body,
		Err(err) => {
			println!("HTTP request returned an error: {}", err);
			return Vec::new();
		}
	};

	let result: Result<types::category::CategoriesResponse, serde_json::Error> = serde_json::from_str(&body);
	match result {
		Ok(parsed) => {
			for var in parsed.data{
				ret_val.push(var);
			}
		},
		Err(err) => {
			println!("Failed to parse category response JSON: {}", err);
		}
	}

	src_cache::ALL_CATS_CACHE.insert(request_str, ret_val.clone());
	return ret_val;

	//let map = match result {
	//	Ok(parsed) => parsed,
	//	Err(err) => {
	//		println!("Failed to parse JSON: {}", err);
	//	}
	//};

	//println!("{:?}", map);
}

pub async fn get_leaderboard(game: &str, category: &str, variables: &HashMap<String, String>) -> Option<types::leaderboard::Leaderboard>{
	let str = format!("{}leaderboards/{}/category/{}", API_BASE_URL, game, category);

	let mut args: HashMap<String, String> = HashMap::new();
	for var in variables{
		args.insert(format!("var-{}", var.0), var.1.clone());
	}

	let cache_key = types::leaderboard::get_leaderboard_cache_key(game, category, variables);
	let cached_leaderboard = LEADERBOARD_CACHE.get(&cache_key);
	if cached_leaderboard.is_some(){
		return cached_leaderboard;
	}

	let result = http_utils::get_http_result_with_args(&str, args).await;

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
			println!("{}", body);
	 		return None;
	 	}
	};

	LEADERBOARD_CACHE.insert(&map.data);
	return Some(map.data);
}

pub fn get_subcategories_for_category(category_id: &str, variables: &Vec<Variable>) -> Vec<HashMap<String, String>>{
	let mut result: Vec<HashMap<String, String>> = Vec::new();

	for var in variables{
		if !var.is_subcategory{
			continue;
		}

		if !var.category.is_some() || !var.category.as_ref().unwrap().contains(category_id){
			continue;
		}

		if var.id == "kn0k0d78"{
			continue;
		}

		for val in &var.values.values{
			let mut map = HashMap::new();
			map.insert(var.id.clone(), val.0.clone());
			result.push(map);
		}
	}

	return result;
}

pub async fn get_all_fullgame_leaderboards(game_id: &str) -> Vec<Leaderboard>{
	let mut result: Vec<Leaderboard> = Vec::new();

	let vars = get_variables_for_game(game_id).await;
	let categories = get_all_categories_for_game(game_id).await;
	for cat in categories{
		if let CategoryType::PerLevel = cat.category_type{
			continue; // Ignore level categories
		}

		let game = src_api::get_game(game_id).await;

		let subcats = get_subcategories_for_category(&cat.id, &vars);

		if subcats.is_empty(){
			let vars = HashMap::new();
			let response = get_leaderboard(game_id, &cat.id, &vars).await;
			if response.is_some(){
				result.push(response.unwrap());
			}
		}else{
			for subcat_combo in &subcats{
				let response = get_leaderboard(game_id, &cat.id, subcat_combo).await;
				if response.is_some(){
					result.push(response.unwrap());
				}
			}
		}
	}

	return result;
}

pub async fn get_user(user_id: &str) -> Option<types::user::User>{
	let cached_user = src_cache::USER_CACHE.get(user_id);
	if cached_user.is_some(){
		return cached_user;
	}

	let str = format!("{}users/{}", API_BASE_URL, user_id);
	let result = http_utils::get_http_result(&str).await;

	let body = match result{
		Ok(body) => body,
		Err(err) => {
			println!("Failed to parse JSON: {}", err);
			return None;
		}
	};

	let result: Result<types::user::UserResponse, serde_json::Error> = serde_json::from_str(&body);
	let map = match result {
		Ok(parsed) => parsed,
		Err(err) => {
			println!("Failed to parse JSON: {}", err);
			return None;
		}
	};

	src_cache::USER_CACHE.insert(&map.data);
	return Some(map.data);
}

pub async fn get_variable(variable_id: &str) -> Option<Variable>{
	let cached_var = src_cache::VARIABLE_CACHE.get(variable_id);
	if cached_var.is_some(){
		return cached_var;
	}

	let str = format!("{}variables/{}", API_BASE_URL, variable_id);
	let result = http_utils::get_http_result(&str).await;

	let body = match result{
		Ok(body) => body,
		Err(err) => {
			println!("Failed to parse JSON: {}", err);
			return None;
		}
	};

	let result: Result<types::variable::VariableResponse, serde_json::Error> = serde_json::from_str(&body);
	let map = match result {
		Ok(parsed) => parsed,
		Err(err) => {
			println!("Failed to parse JSON: {}", err);
			println!("{}", body);
			return None;
		}
	};

	src_cache::VARIABLE_CACHE.insert(&map.data);
	return Some(map.data);
}

pub async fn get_variables_for_game(game_id: &str) -> Vec<Variable>{
	let str = format!("{}games/{}/variables", API_BASE_URL, game_id);
	if src_cache::ALL_VARS_CACHE.contains_key(&str){
		return src_cache::ALL_VARS_CACHE.get(&str).unwrap().to_vec();
	}

	let result = http_utils::get_http_result(&str).await;

	let body = match result{
		Ok(body) => body,
		Err(err) => {
			println!("Failed to parse JSON for variable HTTP result: {}", err);
			return Vec::new();
		}
	};

	let result: Result<VariablesResponse, serde_json::Error> = serde_json::from_str(&body);
	let map = match result {
		Ok(parsed) => parsed,
		Err(err) => {
			println!("Failed to parse JSON for game variables: {}", err);
			println!("{}", body);
			return Vec::new();
		}
	};

	src_cache::ALL_VARS_CACHE.insert(str, map.data.clone());
	return map.data;
}
