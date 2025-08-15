use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RunVideo{
	uri: String
}

#[derive(Deserialize)]
pub struct RunVideos{
	links: Vec<RunVideo>
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum RunStatusType{
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
enum RunPlayerType{
	User,
	Guest
}

#[derive(Deserialize)]
pub struct RunPlayers{
	rel: RunPlayerType,
	id: Option<String>,
	name: Option<String>,
	uri: String
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
	weblink: String,
	game: String,
	level: Option<String>,
	category: String,
	comment: Option<String>,
	status: RunStatus,
	players: Vec<RunPlayers>,
	date: Option<NaiveDate>,
	submitted: Option<DateTime<Utc>>,
	times: RunTimes,
	system: RunSystem,
	splits: Option<RunSplits>,
	values: HashMap<String, String>
}