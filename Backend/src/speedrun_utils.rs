use std::{collections::HashMap, future, io::BufReader, path::Path, sync::Arc};

use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use speedrun_api::{api::{levels::LevelId, runs::{RunStatus, Runs, RunsSorting}, Direction, PagedEndpointExt}, error::SpeedrunApiError, types::{self, Player}, SpeedrunApiBuilder};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct RunData{
	id: String,
	run_date: Option<String>,
	submitted_date: Option<String>,
	game_id: String,
	category_id: String,
	level_id: Option<String>,
	player_id: String,
	is_user: bool,
	verified: bool,
	platform: String,
	duration_ms: i64,
	variables: HashMap<String, String>
}

pub fn read_run_data_from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<RunData>> {
	if !path.as_ref().exists(){
		return Ok(Vec::new());
	}

	let file = File::open(path)?;
	let reader = BufReader::new(file);
	let entries: Vec<RunData> = serde_json::from_reader(reader)?;
	Ok(entries)
}

fn level_id_to_string<'a>(level_id: Option<LevelId<'a>>) -> Option<String>{
	return level_id.map(|id| id.to_string());
}

fn get_primary_player_id<'a>(players: Vec<types::Player<'a>>) -> String {
	return match players.get(0) {
		Some(types::Player::User { id, uri: _ }) => id.to_string()
		Some(types::Player::Guest { name, uri: _ }) => name.clone()
		None => "".to_string(),
	};
}

fn primary_player_is_user<'a>(players: Vec<types::Player<'a>>) -> bool {
	return matches!(players.get(0), Some(types::Player::User { id: _, uri: _ }));
}

fn run_is_verified<'a>(run: types::Run<'_>) -> bool{
	return matches!(run.status, types::Status::Verified { examiner: _, verify_date: _ });
}

pub async fn get_runs_for_game(game_id: &str) -> Result<Vec<RunData>, SpeedrunApiError>{
	println!("Getting runs for game {}...", game_id);
	let client = SpeedrunApiBuilder::new().build_async()?;

	let endpoint = Runs::builder()
        .status(RunStatus::Verified)
        .orderby(RunsSorting::VerifyDate)
        .direction(Direction::Desc)
		.game(game_id)
        .build()
        .unwrap();

	let mut runs: Vec<RunData> = Vec::new();

    endpoint.stream(&client)
        .take(10)
        .try_for_each_concurrent(5, |run: types::Run|{
			//run.category
			//run.game
			//run.level
			//run.players
			//run.status
			//run.system
			//run.times
			//run.values
			let run_data = RunData
			{
				id: (run.id).to_string(),
				run_date: run.date,
				submitted_date: run.submitted,
				game_id: run.game.to_string(),
				category_id: run.category.to_string(),
				level_id: level_id_to_string(run.level),
				player_id: get_primary_player_id(run.players),
				is_user: primary_player_is_user(run.players),
				verified: run_is_verified(run),
				platform: "".to_string(), // TODO
				duration_ms: 0, // TODO
				variables: HashMap::new()
			};
			
			runs.push(run_data);
            future::ready(Ok(()))
        })
        .await.unwrap();

	Ok(runs)
}
