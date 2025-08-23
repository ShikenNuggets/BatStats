#![allow(dead_code)]

use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

use crate::speedrun_api::types::traits;

#[derive(Deserialize)]
pub struct RunVideo{
	pub uri: String
}

#[derive(Deserialize)]
pub struct RunVideos{
	pub links: Vec<RunVideo>
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunStatusType{
	Verified,
	Pending,
	Rejected
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RunStatus{
	status: RunStatusType,
	examiner: Option<String>,
	verify_date: Option<DateTime<Utc>>
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunPlayerType{
	User,
	Guest
}

#[derive(Clone, Deserialize)]
pub struct RunPlayer{
	pub rel: RunPlayerType,
	pub id: Option<String>,
	pub name: Option<String>,
	pub uri: String
}

// TODO - Parse times into ISO 8601 duration
#[derive(Clone, Deserialize)]
pub struct RunTimes{
	pub primary: String,
	pub primary_t: f64,
	pub realtime: Option<String>,
	pub realtime_t: Option<f64>,
	pub realtime_noloads: Option<String>,
	pub realtime_noloads_t: Option<f64>,
	pub ingame: Option<String>,
	pub ingame_t: Option<f64>
}

#[derive(Clone, Deserialize)]
pub struct RunSystem{
	platform: Option<String>,
	emulated: bool,
	region: Option<String>
}

#[derive(Clone, Deserialize)]
pub struct RunSplits{
	rel: String,
	uri: String
}

#[derive(Clone, Deserialize)]
pub struct Run{
	pub id: String,
	pub weblink: String,
	pub game: String,
	pub level: Option<String>,
	pub category: String,
	pub comment: Option<String>,
	pub status: RunStatus,
	pub players: Vec<RunPlayer>,
	pub date: Option<NaiveDate>,
	pub submitted: Option<DateTime<Utc>>,
	pub times: RunTimes,
	pub system: RunSystem,
	pub splits: Option<RunSplits>,
	pub values: HashMap<String, String>
}

impl traits::Cacheable for Run{
	fn key(&self) -> String {
		return self.id.to_string();
	}
}
