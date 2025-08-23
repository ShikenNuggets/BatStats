#![allow(dead_code)]

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::speedrun_api::types::traits::{self};

#[derive(Clone, Deserialize)]
pub struct GameNames{
	pub international: String,
	pub japanese: Option<String>,
	pub twitch: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GameRuleset{
	pub show_milliseconds: bool,
	pub require_verification: bool,
	pub require_video: bool,
	pub run_times: Vec<String>,
	pub default_time: String,
	pub emulators_allowed: bool,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Game{
	pub id: String,
	pub names: GameNames,
	pub abbreviation: String,
	pub weblink: String,
	pub released: i64,
	pub release_date: String, // TODO - Date
	pub ruleset: GameRuleset,
	pub romhack: bool,
	pub gametypes: Vec<String>,
	pub platforms: Vec<String>,
	pub regions: Vec<String>,
	pub genres: Vec<String>,
	pub engines: Vec<String>,
	pub developers: Vec<String>,
	pub publishers: Vec<String>,
	pub moderators: HashMap<String, String>,
	pub created: DateTime<Utc>,
	// TODO - Assets
	// TODO - Links
}

impl traits::Cacheable for Game{
	fn key(&self) -> String {
		return self.id.to_string();
	}
}

#[derive(Deserialize)]
pub struct GameResponse{
	pub data: Game
}
