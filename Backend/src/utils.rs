use std::collections::{HashMap, HashSet};

use chrono::{Duration, NaiveDate, Utc};

use crate::speedrun_api::{src_api, types::{leaderboard::Leaderboard, run::{RunPlayer, RunPlayerType}}};

pub async fn get_player_name(player : &RunPlayer) -> Option<String>{
	if let RunPlayerType::Guest = player.rel{
		return player.name.clone();
	}

	if player.id.is_none(){
		return player.name.clone();
	}

	let user = src_api::get_user(&player.id.clone().unwrap()).await;
	if user.is_none(){
		return None;
	}

	return Some(user.unwrap().names.international);
}

async fn get_all_runners(leaderboards: &Vec<Leaderboard>) -> HashSet<String>{
	let mut runners: HashSet<String> = HashSet::new();

	for leaderboard in leaderboards{
		for run in &leaderboard.runs{
			for player in &run.run.players{
				let player_name = get_player_name(&player).await;
				if player_name.is_none(){
					continue;
				}
				runners.insert(player_name.unwrap());
			}
		}
	}

	return runners;
}

pub fn get_fastest_time(leaderboard: &Leaderboard) -> Option<f64>{
	if leaderboard.runs.is_empty(){
		return None;
	}

	return Some(leaderboard.runs.first().unwrap().run.times.primary_t);
}

fn get_slowest_time(leaderboard: &Leaderboard) -> Option<f64>{
	if leaderboard.runs.is_empty(){
		return None;
	}

	return Some(leaderboard.runs.last().unwrap().run.times.primary_t);
}

pub fn get_average_time(leaderboard: &Leaderboard) -> Option<f64>{
	if leaderboard.runs.is_empty(){
		return None;
	}

	let mut total_time: f64 = 0.0;
	for run in &leaderboard.runs{
		total_time += run.run.times.primary_t;
	}

	return Some(total_time / (leaderboard.runs.len() as f64));
}

fn is_more_than_n_years_ago(date: &NaiveDate, years: i64) -> bool{
	let current_date = Utc::now().date_naive();
	let n_years_ago = current_date - Duration::days(365 * years);
	return date < &n_years_ago;
}

pub fn get_adjusted_average_time(leaderboard: &Leaderboard) -> Option<f64>{
	let real_average = get_average_time(leaderboard);
	if real_average.is_none() || leaderboard.runs.len() < 5{
		return real_average;
	}
	let real_average = real_average.unwrap();

	let mut total_time: f64 = 0.0;
	let mut num_times: i64 = 0;

	for run in &leaderboard.runs{
		if run.run.date.is_none(){
			continue; // Ignore runs that are too old to have a date
		}

		if run.run.times.primary_t > real_average && is_more_than_n_years_ago(&run.run.date.unwrap(), 5){
			continue; // Ignore very old runs that are above average
		}

		num_times += 1;
		total_time += run.run.times.primary_t;
	}

	let adjusted_average = total_time / (num_times as f64);

	if adjusted_average <= 0.0 || adjusted_average > real_average || num_times == 0{
		return Some(real_average);
	}

	return Some(adjusted_average);
}

async fn get_runner_times_map(leaderboard: &Leaderboard) -> HashMap<String, f64>{
	let mut run_times: HashMap<String, f64> = HashMap::new();

	let fastest_time = get_fastest_time(&leaderboard);
	let slowest_time = get_slowest_time(&leaderboard);

	if fastest_time.is_none() || slowest_time.is_none(){
		return run_times;
	}
	
	for run in &leaderboard.runs{
		let runner_name = get_player_name(run.run.players.first().unwrap()).await;
		if runner_name.is_none(){
			continue;
		}

		run_times.insert(runner_name.unwrap(), run.run.times.primary_t);
	}

	return run_times;
}

async fn get_total_runner_times(leaderboards: &Vec<Leaderboard>, subtract_from_wr: bool) -> HashMap<String, f64>{
	let mut runner_times: HashMap<String, f64> = HashMap::new();

	let all_runners = get_all_runners(leaderboards).await;
	for runner in &all_runners{
		runner_times.insert(runner.to_string(), 0.0);
	}

	for leaderboard in leaderboards{
		let api_category = src_api::get_category(&leaderboard.category).await;
		if api_category.is_none() || api_category.as_ref().unwrap().miscellaneous{
			continue;
		}

		let fastest_time = get_fastest_time(&leaderboard);
		let slowest_time = get_slowest_time(&leaderboard);
		let times = get_runner_times_map(leaderboard).await;

		if fastest_time.is_none() || slowest_time.is_none() || times.is_empty(){
			println!("Something went wrong for leaderboard of category {}", leaderboard.category);
			continue;
		}

		let fastest_time = fastest_time.unwrap();
		let slowest_time = slowest_time.unwrap() - fastest_time;

		for runner in &all_runners{
			let mut time_to_add = slowest_time;
			if times.contains_key(runner){
				time_to_add = times[runner];
				if subtract_from_wr{
					time_to_add -= fastest_time;
				}
			}

			runner_times.insert(runner.to_string(), runner_times[runner] + time_to_add);
		}
	}

	return runner_times;
}

pub async fn get_leaderboard_for_subcategory(game_id: &str, category_id: &str, var_id: &str, val_id: &str) -> Option<Leaderboard>{
	let mut vars: HashMap<String, String> = HashMap::new();
	vars.insert(var_id.to_string(), val_id.to_string());
	return src_api::get_leaderboard(game_id, category_id, &vars).await;
}

pub async fn combine_times_best_only(leaderboards: &Vec<Leaderboard>) -> HashMap<String, f64>{
	let mut combined_times: HashMap<String, f64> = HashMap::new();

	for leaderboard in leaderboards{
		let mut lb_temp: Vec<Leaderboard> = Vec::new();
		lb_temp.push(leaderboard.clone());
		let times = get_total_runner_times(&lb_temp, false).await;
		for time in times{
			if !combined_times.contains_key(&time.0) || combined_times[&time.0] > time.1{
				combined_times.insert(time.0, time.1);
			}
		}
	}
	
	return combined_times;
}
