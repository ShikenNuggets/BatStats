use std::collections::HashMap;

use crate::{speedrun_api::{src_api, types::leaderboard::Leaderboard}, utils};

pub const GAME_ID: &str = "4pdvp4dw";

pub const ANY_CAT_ID: &str = "w5dwog2g";
pub const GLITCHLESS_CAT_ID: &str = "rklez0q2";
pub const HUNDO_CAT_ID: &str = "7wk69ek1";

pub const ANY_DIFFICULTY_VAR_ID: &str = "38dq40n0";
	pub const ANY_EASY_VAL_ID: &str = "p12vj4dq";
	pub const ANY_NORMAL_VAL_ID: &str = "0q54ry7l";
	pub const ANY_HARD_VAL_ID: &str = "81pyk6v1";

pub const GLITCHLESS_DIFFICULTY_VAR_ID: &str = "gnx7vxlv";
	pub const GLITCHLESS_EASY_VAL_ID: &str = "81w70ovq";
	pub const GLITCHLESS_NORMAL_VAL_ID: &str = "zqojvox1";
	pub const GLITCHLESS_HARD_VAL_ID: &str = "013x67x1";

pub async fn get_best_any_percent_times() -> HashMap<String, f64>{
	let any_easy = utils::get_leaderboard_for_subcategory(GAME_ID, ANY_CAT_ID, ANY_DIFFICULTY_VAR_ID, ANY_EASY_VAL_ID).await;
	let any_normal = utils::get_leaderboard_for_subcategory(GAME_ID, ANY_CAT_ID, ANY_DIFFICULTY_VAR_ID, ANY_NORMAL_VAL_ID).await;
	let any_hard = utils::get_leaderboard_for_subcategory(GAME_ID, ANY_CAT_ID, ANY_DIFFICULTY_VAR_ID, ANY_HARD_VAL_ID).await;

	let glitchless_easy = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_EASY_VAL_ID).await;
	let glitchless_normal = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_NORMAL_VAL_ID).await;
	let glitchless_hard = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_HARD_VAL_ID).await;

	let vars = HashMap::new();
	let hundo = src_api::get_leaderboard(GAME_ID, HUNDO_CAT_ID, &vars).await;

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

	return utils::combine_times_best_only(&all_boards).await;
}

pub async fn get_best_glitchless_times() -> HashMap<String, f64>{
	let glitchless_easy = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_EASY_VAL_ID).await;
	let glitchless_normal = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_NORMAL_VAL_ID).await;
	let glitchless_hard = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_HARD_VAL_ID).await;

	if glitchless_easy.is_none() || glitchless_normal.is_none() || glitchless_hard.is_none()
	{
		println!("Failed to get all Glitchless boards for Origins");
		return HashMap::new();
	}

	let mut all_boards: Vec<Leaderboard> = Vec::new();
	all_boards.push(glitchless_easy.unwrap());
	all_boards.push(glitchless_normal.unwrap());
	all_boards.push(glitchless_hard.unwrap());
	return utils::combine_times_best_only(&all_boards).await;
}

pub async fn get_best_hundo_times() -> HashMap<String, f64>{
	let vars = HashMap::new();
	let hundo = src_api::get_leaderboard(GAME_ID, HUNDO_CAT_ID, &vars).await;

	if hundo.is_none()
	{
		println!("Failed to get all 100% boards for Origins");
		return HashMap::new();
	}

	let mut all_boards: Vec<Leaderboard> = Vec::new();
	all_boards.push(hundo.unwrap());
	return utils::combine_times_best_only(&all_boards).await;
}
