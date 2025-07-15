use std::{future, io::BufReader, path::Path};

use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use speedrun_api::{api::{runs::{RunStatus, Runs, RunsSorting}, Direction, PagedEndpointExt}, error::SpeedrunApiError, types, SpeedrunApiBuilder};
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct RunData{
	id: String,
	run_date: Option<String>,
	submitted_date: Option<String>
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

pub async fn get_runs_for_game(game_id: &str) -> Result<Vec<RunData>, SpeedrunApiError>{
	let client = SpeedrunApiBuilder::new().build_async()?;

	let endpoint = Runs::builder()
        .status(RunStatus::Verified)
        .orderby(RunsSorting::VerifyDate)
        .direction(Direction::Asc)
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
				submitted_date: run.submitted
			};
			
			runs.push(run_data);
            future::ready(Ok(()))
        })
        .await.unwrap();

	Ok(runs)
}
