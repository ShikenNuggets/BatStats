use std::collections::HashMap;

use crate::{speedrun_api::{src_api, types::leaderboard::Leaderboard}, utils};

pub const GAME_ID: &str = "4pd0p06e";

pub const ANY_CAT_ID: &str = "9zdn672q";
pub const NMS_CAT_ID: &str = "wkpyvjvk";
pub const HUNDO_CAT_ID: &str = "zjdzyxkv";
pub const HUNDO_NMS_CAT_ID: &str = "9kvj7mjk";

//pub const ANY_DIFFICULTY_VAR_ID: &str = "wl32xvn1";
//	pub const ANY_EASY_VAL_ID: &str = "klr3kyol";
//	pub const ANY_HARD_VAL_ID: &str = "21dk83pl";

//pub const NMS_DIFFICULTY_VAR_ID: &str = "68k2q382";
//	pub const NMS_EASY_VAL_ID: &str = "mln8vzol";
//	pub const NMS_HARD_VAL_ID: &str = "810vn9ol";

//pub const HUNDO_DIFFICULTY_VAR_ID: &str = "p852z78g";
//	pub const HUNDO_EASY_VAL_ID: &str = "zqoxe25q";
//	pub const HUNDO_HARD_VAL_ID: &str = "0139k4r1";

//pub const HUNDO_NMS_DIFFICULTY_VAR_ID: &str = "dlodgd8o";
//	pub const HUNDO_NMS_EASY_VAL_ID: &str = "jq6vkdj1";
//	pub const HUNDO_NMS_HARD_VAL_ID: &str = "5lm2j5mq";

pub async fn get_best_any_percent_times() -> HashMap<String, f64>{
	let vars = HashMap::new();
	let any = src_api::get_leaderboard(GAME_ID, ANY_CAT_ID, &vars).await;
	let nms = src_api::get_leaderboard(GAME_ID, NMS_CAT_ID, &vars).await;
	let hundo = src_api::get_leaderboard(GAME_ID, HUNDO_CAT_ID, &vars).await;
	let hundo_nms = src_api::get_leaderboard(GAME_ID, HUNDO_NMS_CAT_ID, &vars).await;

	if any.is_none() || nms.is_none() || hundo.is_none() || hundo_nms.is_none(){
		println!("Failed to get all Any% boards for Asylum");
		return HashMap::new();
	}

	let mut all_boards: Vec<Leaderboard> = Vec::new();
	all_boards.push(any.unwrap());
	all_boards.push(nms.unwrap());
	all_boards.push(hundo.unwrap());
	all_boards.push(hundo_nms.unwrap());

	return utils::combine_times_best_only(&all_boards).await;
}

pub async fn get_best_glitchless_times() -> HashMap<String, f64>{
	let vars = HashMap::new();
	let nms = src_api::get_leaderboard(GAME_ID, NMS_CAT_ID, &vars).await;
	let hundo_nms = src_api::get_leaderboard(GAME_ID, HUNDO_NMS_CAT_ID, &vars).await;

	if nms.is_none() || hundo_nms.is_none(){
		println!("Failed to get all Any% boards for Asylum");
		return HashMap::new();
	}

	let mut all_boards: Vec<Leaderboard> = Vec::new();
	all_boards.push(nms.unwrap());
	all_boards.push(hundo_nms.unwrap());

	return utils::combine_times_best_only(&all_boards).await;
}

pub async fn get_best_hundo_times() -> HashMap<String, f64>{
	let vars = HashMap::new();
	let hundo = src_api::get_leaderboard(GAME_ID, HUNDO_CAT_ID, &vars).await;
	let hundo_nms = src_api::get_leaderboard(GAME_ID, HUNDO_NMS_CAT_ID, &vars).await;

	if hundo.is_none() || hundo_nms.is_none(){
		println!("Failed to get all Any% boards for Asylum");
		return HashMap::new();
	}

	let mut all_boards: Vec<Leaderboard> = Vec::new();
	all_boards.push(hundo.unwrap());
	all_boards.push(hundo_nms.unwrap());

	return utils::combine_times_best_only(&all_boards).await;
}
