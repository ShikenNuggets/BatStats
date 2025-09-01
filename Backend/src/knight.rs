use std::collections::HashMap;

use crate::{speedrun_api::{types::leaderboard::Leaderboard}, utils};

pub const GAME_ID: &str = "4d7p4rd7";

pub const ANY_CAT_ID: &str = "7kjg8gk3";
pub const GLITCHLESS_CAT_ID: &str = "7kjz9p3d";
pub const KNIGHTFALL_CAT_ID: &str = "02q8rg72";

pub const ANY_DIFFICULTY_VAR_ID: &str = "5ly35gn4";
	pub const ANY_EASY_VAL_ID: &str = "21gn0o6l";
	pub const ANY_NORMAL_VAL_ID: &str = "z19x0pj1";
	pub const ANY_HARD_VAL_ID: &str = "jqzn2y2q";

pub const GLITCHLESS_DIFFICULTY_VAR_ID: &str = "38d69xl0";
	pub const GLITCHLESS_EASY_VAL_ID: &str = "5lenpwpl";
	pub const GLITCHLESS_NORMAL_VAL_ID: &str = "4lxgmjjq";
	pub const GLITCHLESS_HARD_VAL_ID: &str = "0q54d0rl";

pub const KNIGHTFALL_PURITY_VAR_ID: &str = "0nwxwxlq";
	pub const KNIGHTFALL_FIRST_VAL_ID: &str = "81p476e1";
	pub const KNIGHTFALL_FULL_VAL_ID: &str = "xqk9r691";

pub async fn get_best_any_percent_times() -> HashMap<String, f64>{
	let any_easy = utils::get_leaderboard_for_subcategory(GAME_ID, ANY_CAT_ID, ANY_DIFFICULTY_VAR_ID, ANY_EASY_VAL_ID).await;
	let any_normal = utils::get_leaderboard_for_subcategory(GAME_ID, ANY_CAT_ID, ANY_DIFFICULTY_VAR_ID, ANY_NORMAL_VAL_ID).await;
	let any_hard = utils::get_leaderboard_for_subcategory(GAME_ID, ANY_CAT_ID, ANY_DIFFICULTY_VAR_ID, ANY_HARD_VAL_ID).await;

	let glitchless_easy = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_EASY_VAL_ID).await;
	let glitchless_normal = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_NORMAL_VAL_ID).await;
	let glitchless_hard = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_HARD_VAL_ID).await;

	let knightfall_first = utils::get_leaderboard_for_subcategory(GAME_ID, KNIGHTFALL_CAT_ID, KNIGHTFALL_PURITY_VAR_ID, KNIGHTFALL_FIRST_VAL_ID).await;
	let knightfall_full = utils::get_leaderboard_for_subcategory(GAME_ID, KNIGHTFALL_CAT_ID, KNIGHTFALL_PURITY_VAR_ID, KNIGHTFALL_FULL_VAL_ID).await;

	if any_easy.is_none() || any_normal.is_none() || any_hard.is_none()
		|| glitchless_easy.is_none() || glitchless_normal.is_none() || glitchless_hard.is_none()
		|| knightfall_first.is_none() || knightfall_full.is_none()
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

	all_boards.push(knightfall_first.unwrap());
	all_boards.push(knightfall_full.unwrap());

	return utils::combine_times_best_only(&all_boards).await;
}

pub async fn get_best_glitchless_times() -> HashMap<String, f64>{
	let glitchless_easy = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_EASY_VAL_ID).await;
	let glitchless_normal = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_NORMAL_VAL_ID).await;
	let glitchless_hard = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_HARD_VAL_ID).await;

	if glitchless_easy.is_none() || glitchless_normal.is_none() || glitchless_hard.is_none()
	{
		println!("Failed to get all Glitchless boards for Knight");
		return HashMap::new();
	}

	let mut all_boards: Vec<Leaderboard> = Vec::new();
	all_boards.push(glitchless_easy.unwrap());
	all_boards.push(glitchless_normal.unwrap());
	all_boards.push(glitchless_hard.unwrap());
	return utils::combine_times_best_only(&all_boards).await;
}

pub async fn get_best_hundo_times() -> HashMap<String, f64>{
	let knightfall_full = utils::get_leaderboard_for_subcategory(GAME_ID, KNIGHTFALL_CAT_ID, KNIGHTFALL_PURITY_VAR_ID, KNIGHTFALL_FULL_VAL_ID).await;
	if knightfall_full.is_none()
	{
		println!("Failed to get all Any% boards for Knight");
		return HashMap::new();
	}

	let mut all_boards: Vec<Leaderboard> = Vec::new();
	all_boards.push(knightfall_full.unwrap());
	return utils::combine_times_best_only(&all_boards).await;
}
