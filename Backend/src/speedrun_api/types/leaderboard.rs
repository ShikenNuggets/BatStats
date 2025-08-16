#![allow(dead_code)]

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

#[derive(Deserialize)]
pub struct LeaderboardResponse{
	pub data: Leaderboard
}
