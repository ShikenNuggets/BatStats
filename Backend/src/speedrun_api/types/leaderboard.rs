use std::collections::HashMap;

use crate::speedrun_api::types::core;
use crate::speedrun_api::types::run;

use serde::Deserialize;

#[derive(Deserialize)]
pub enum TimingType{
	#[serde(rename = "ingame")]
	Ingame,

	#[serde(rename = "realtime")]
	RealTime,

	#[serde(rename = "realtime_noloads")]
	RealTimeNoLoads,
}

#[derive(Deserialize)]
pub struct LeaderboardRun{
	pub place: i32,
	pub run: run::Run
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Leaderboard{
	weblink: String,
	game: String,
	category: String,
	level: Option<String>,
	platform: Option<String>,
	region: Option<String>,
	emulators: Option<String>,
	video_only: bool,
	timing: TimingType,
	values: Vec<HashMap<String, String>>,
	pub runs: Vec<LeaderboardRun>,
	links: Vec<core::SrcLink>
}

#[derive(Deserialize)]
pub struct LeaderboardResponse{
	pub data: Leaderboard
}
