#![allow(dead_code)]

use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RunVideo{
	pub uri: String
}

#[derive(Deserialize)]
pub struct RunVideos{
	pub links: Vec<RunVideo>
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunStatusType{
	Verified,
	Pending,
	Rejected
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RunStatus{
	status: RunStatusType,
	examiner: Option<String>,
	verify_date: Option<DateTime<Utc>>
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RunPlayerType{
	User,
	Guest
}

#[derive(Deserialize)]
pub struct RunPlayer{
	pub rel: RunPlayerType,
	pub id: Option<String>,
	pub name: Option<String>,
	pub uri: String
}

// TODO - Parse times into ISO 8601 duration
#[derive(Deserialize)]
pub struct RunTimes{
	primary: String,
	primary_t: i64,
	realtime: Option<String>,
	realtime_t: Option<i64>,
	realtime_noloads: Option<String>,
	realtime_noloads_t: Option<i64>,
	ingame: Option<String>,
	ingame_t: Option<i64>
}

#[derive(Deserialize)]
pub struct RunSystem{
	platform: Option<String>,
	emulated: bool,
	region: Option<String>
}

#[derive(Deserialize)]
pub struct RunSplits{
	rel: String,
	uri: String
}

#[derive(Deserialize)]
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