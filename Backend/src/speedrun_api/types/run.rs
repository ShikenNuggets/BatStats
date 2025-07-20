use std::collections::HashMap;

use chrono::{DateTime, TimeDelta, Utc};
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
pub struct RunPlayers{
	rel: String,
	id: String,
	uri: String
}

#[derive(Deserialize)]
pub struct RunTimes{
	primary: TimeDelta,
	primary_t: i64,
	realtime: Option<TimeDelta>,
	realtime_t: Option<i64>,
	realtime_noloads: Option<TimeDelta>,
	realtime_noloads_t: Option<i64>,
	ingame: Option<TimeDelta>,
	ingame_t: Option<i64>
}

#[derive(Deserialize)]
pub struct RunSystem{
	platform: Option<String>,
	emulated: bool,
	region: Option<String>
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
	players: RunPlayers,
	date: DateTime<Utc>,
	submitted: DateTime<Utc>,
	times: RunTimes,
	system: RunSystem,
	splits: Option<String>,
	values: Vec<HashMap<String, String>>
}