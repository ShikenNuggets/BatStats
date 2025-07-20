use crate::speedrun_api::types::core;

use serde::Deserialize;

#[derive(Deserialize)]
pub enum TimingType{
	Ingame,
	Realtime,
	Realtime_noloads,
}

#[derive(Deserialize)]
pub struct LeaderboardRun{
	place: i32,
	run: String
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
	values: Vec<String>, // TODO - Variable values
	runs: Vec<String>,
	pub links: Vec<core::SrcLink>
}
