use std::{collections::HashMap};

use crate::{speedrun_api::{src_api, types::leaderboard::Leaderboard}, utils};

#[derive(Default)]
pub struct RawMasteryData{
	pub num_players: i64,
	pub mastery_percents: HashMap<String, f64>
}

pub fn calculate_mastery(wr: f64, average: f64, run_time: f64) -> f64{
	if run_time <= wr{
		return 1.0;
	}else if run_time >= average{
		return 0.0;
	}

	return (average - run_time) / (average - wr);
}

pub async fn get_raw_mastery_ranks(leaderboard: &Leaderboard) -> RawMasteryData{
	let mut raw_mastery: RawMasteryData = Default::default();
	raw_mastery.num_players = leaderboard.runs.len() as i64;

	let fastest_time = utils::get_fastest_time(leaderboard);
	if fastest_time.is_none(){
		println!("Leaderboard's fastest time was invalid");
		return raw_mastery;
	}
	let fastest_time = fastest_time.unwrap();

	let average_time = utils::get_adjusted_average_time(leaderboard);
	if average_time.is_none(){
		println!("Leaderboard's average time was invalid");
		return raw_mastery;
	}
	let average_time = average_time.unwrap();

	for run in &leaderboard.runs{
		let mastery = calculate_mastery(fastest_time, average_time, run.run.times.primary_t);
		if mastery <= 0.0{
			continue;
		}

		let runner_name = utils::get_player_name(&run.run.players[0]).await;
		if runner_name.is_none(){
			println!("Run {} had no valid player name", run.run.id);
			continue;
		}
		
		raw_mastery.mastery_percents.insert(runner_name.unwrap(), mastery);
	}

	return raw_mastery;
}

pub async fn get_mastery_ranks_for_game(game_id: &str) -> HashMap<String, f64>{
	let mut raw_mastery_datas: Vec<RawMasteryData> = Vec::new();
	let mut total_players: i64 = 0;
	
	let boards = src_api::get_all_fullgame_leaderboards(game_id).await;
	for board in boards{
		let raw_data = get_raw_mastery_ranks(&board).await;
		total_players += raw_data.num_players;
		raw_mastery_datas.push(raw_data);
	}

	let mut overall_mastery = HashMap::new();

	for raw_data in raw_mastery_datas{
		let percent_relevance = (raw_data.num_players as f64) / (total_players as f64);
		
		for entry in raw_data.mastery_percents{
			if overall_mastery.contains_key(&entry.0){
				let new_mastery = overall_mastery[&entry.0] + (entry.1 * percent_relevance);
				overall_mastery.insert(entry.0, new_mastery);
			}else{
				overall_mastery.insert(entry.0, entry.1 * percent_relevance);
			}
		}
	}

	return overall_mastery;
}
