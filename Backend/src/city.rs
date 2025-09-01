use std::collections::HashMap;

use crate::{speedrun_api::{src_api, types::leaderboard::Leaderboard}, utils};

pub const GAME_ID: &str = "x3692ldl";

pub const ANY_CAT_ID: &str = "5dwjjogk";
pub const ANY_WCAT_CAT_ID: &str = "p7kjxg23";
pub const GLITCHLESS_CAT_ID: &str = "wdmw6ne2";
pub const GLITCHLESS_WCAT_CAT_ID: &str = "z273g9od";
pub const HUNDO_CAT_ID: &str = "4xk9qx20";

pub const ANY_DIFFICULTY_VAR_ID: &str = "jlze37l2";
	pub const ANY_EASY_VAL_ID: &str = "gq753nr1";
	pub const ANY_NORMAL_VAL_ID: &str = "rqvm995q";
	pub const ANY_HARD_VAL_ID: &str = "21gn0jol";

pub const ANY_WCAT_DIFFICULTY_VAR_ID: &str = "789p93nw";
	pub const ANY_WCAT_EASY_VAL_ID: &str = "z192e88q";
	pub const ANY_WCAT_NORMAL_VAL_ID: &str = "0132ppkq";
	pub const ANY_WCAT_HARD_VAL_ID: &str = "p12vjx4q";

pub const GLITCHLESS_DIFFICULTY_VAR_ID: &str = "ylpv6x6l";
	pub const GLITCHLESS_EASY_VAL_ID: &str = "zqoyxpx1";
	pub const GLITCHLESS_NORMAL_VAL_ID: &str = "013v98xl";
	pub const GLITCHLESS_HARD_VAL_ID: &str = "rqv4vp6q";

pub const GLITCHLESS_WCAT_DIFFICULTY_VAR_ID: &str = "p8564xng";
	pub const GLITCHLESS_WCAT_EASY_VAL_ID: &str = "5len8mpl";
	pub const GLITCHLESS_WCAT_NORMAL_VAL_ID: &str = "rqvm9g7q";
	pub const GLITCHLESS_WCAT_HARD_VAL_ID: &str = "0q548grl";

pub async fn get_best_any_percent_times() -> HashMap<String, f64>{
	let any_easy = utils::get_leaderboard_for_subcategory(GAME_ID, ANY_CAT_ID, ANY_DIFFICULTY_VAR_ID, ANY_EASY_VAL_ID).await;
	let any_normal = utils::get_leaderboard_for_subcategory(GAME_ID, ANY_CAT_ID, ANY_DIFFICULTY_VAR_ID, ANY_NORMAL_VAL_ID).await;
	let any_hard = utils::get_leaderboard_for_subcategory(GAME_ID, ANY_CAT_ID, ANY_DIFFICULTY_VAR_ID, ANY_HARD_VAL_ID).await;

	let any_wcat_easy = utils::get_leaderboard_for_subcategory(GAME_ID, ANY_WCAT_CAT_ID, ANY_WCAT_DIFFICULTY_VAR_ID, ANY_WCAT_EASY_VAL_ID).await;
	let any_wcat_normal = utils::get_leaderboard_for_subcategory(GAME_ID, ANY_WCAT_CAT_ID, ANY_WCAT_DIFFICULTY_VAR_ID, ANY_WCAT_NORMAL_VAL_ID).await;
	let any_wcat_hard = utils::get_leaderboard_for_subcategory(GAME_ID, ANY_WCAT_CAT_ID, ANY_WCAT_DIFFICULTY_VAR_ID, ANY_WCAT_HARD_VAL_ID).await;

	let glitchless_easy = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_EASY_VAL_ID).await;
	let glitchless_normal = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_NORMAL_VAL_ID).await;
	let glitchless_hard = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_HARD_VAL_ID).await;

	let glitchless_wcat_easy = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_WCAT_CAT_ID, GLITCHLESS_WCAT_DIFFICULTY_VAR_ID, GLITCHLESS_WCAT_EASY_VAL_ID).await;
	let glitchless_wcat_normal = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_WCAT_CAT_ID, GLITCHLESS_WCAT_DIFFICULTY_VAR_ID, GLITCHLESS_WCAT_NORMAL_VAL_ID).await;
	let glitchless_wcat_hard = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_WCAT_CAT_ID, GLITCHLESS_WCAT_DIFFICULTY_VAR_ID, GLITCHLESS_WCAT_HARD_VAL_ID).await;

	let vars = HashMap::new();
	let hundo = src_api::get_leaderboard(GAME_ID, HUNDO_CAT_ID, &vars).await;

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

	return utils::combine_times_best_only(&all_boards).await;
}

pub async fn get_best_glitchless_times() -> HashMap<String, f64>{
	let glitchless_easy = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_EASY_VAL_ID).await;
	let glitchless_normal = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_NORMAL_VAL_ID).await;
	let glitchless_hard = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_CAT_ID, GLITCHLESS_DIFFICULTY_VAR_ID, GLITCHLESS_HARD_VAL_ID).await;

	let glitchless_wcat_easy = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_WCAT_CAT_ID, GLITCHLESS_WCAT_DIFFICULTY_VAR_ID, GLITCHLESS_WCAT_EASY_VAL_ID).await;
	let glitchless_wcat_normal = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_WCAT_CAT_ID, GLITCHLESS_WCAT_DIFFICULTY_VAR_ID, GLITCHLESS_WCAT_NORMAL_VAL_ID).await;
	let glitchless_wcat_hard = utils::get_leaderboard_for_subcategory(GAME_ID, GLITCHLESS_WCAT_CAT_ID, GLITCHLESS_WCAT_DIFFICULTY_VAR_ID, GLITCHLESS_WCAT_HARD_VAL_ID).await;

	if glitchless_easy.is_none() || glitchless_normal.is_none() || glitchless_hard.is_none()
		|| glitchless_wcat_easy.is_none() || glitchless_wcat_normal.is_none() || glitchless_wcat_hard.is_none()
	{
		println!("Failed to get all Glitchless boards for City");
		return HashMap::new();
	}

	let mut all_boards: Vec<Leaderboard> = Vec::new();
	all_boards.push(glitchless_easy.unwrap());
	all_boards.push(glitchless_normal.unwrap());
	all_boards.push(glitchless_hard.unwrap());

	all_boards.push(glitchless_wcat_easy.unwrap());
	all_boards.push(glitchless_wcat_normal.unwrap());
	all_boards.push(glitchless_wcat_hard.unwrap());

	return utils::combine_times_best_only(&all_boards).await;
}

pub async fn get_best_hundo_times() -> HashMap<String, f64>{
	let vars = HashMap::new();
	let hundo = src_api::get_leaderboard(GAME_ID, HUNDO_CAT_ID, &vars).await;

	if hundo.is_none()
	{
		println!("Failed to get all 100% boards for City");
		return HashMap::new();
	}

	let mut all_boards: Vec<Leaderboard> = Vec::new();
	all_boards.push(hundo.unwrap());
	return utils::combine_times_best_only(&all_boards).await;
}
