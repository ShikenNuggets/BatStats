#![allow(dead_code)]

use std::collections::HashMap;

use crate::speedrun_api::types::core;
use crate::speedrun_api::types::run;
use crate::speedrun_api::types::traits;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub enum TimingType{
	#[serde(rename = "ingame")]
	Ingame,

	#[serde(rename = "realtime")]
	RealTime,

	#[serde(rename = "realtime_noloads")]
	RealTimeNoLoads,
}

#[derive(Clone, Deserialize)]
pub struct LeaderboardRun{
	pub place: i32,
	pub run: run::Run
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Leaderboard{
	pub weblink: String,
	pub game: String,
	pub category: String,
	pub level: Option<String>,
	pub platform: Option<String>,
	pub region: Option<String>,
	pub emulators: Option<String>,
	pub video_only: bool,
	pub timing: TimingType,
	pub values: HashMap<String, String>,
	pub runs: Vec<LeaderboardRun>,
	pub links: Vec<core::SrcLink>
}

pub fn get_leaderboard_cache_key(game: &str, category: &str, variables: &HashMap<String, String>) -> String{
	let mut key: String = Default::default();
	key.reserve(game.len() + category.len() + 2 + (variables.len() * 16));
	key.push_str(&game);
	key.push_str("_");
	key.push_str(&category);

	for pair in variables{
		key.reserve(pair.0.len() + pair.1.len() + 2);
		key.push_str("_");
		key.push_str(&pair.0);
		key.push_str("_");
		key.push_str(&pair.1);
	}

	return key;
}

impl traits::Cacheable for Leaderboard{
	fn key(&self) -> String {
		return get_leaderboard_cache_key(&self.game, &self.category, &self.values);
	}
}

#[derive(Deserialize)]
pub struct LeaderboardResponse{
	pub data: Leaderboard
}
