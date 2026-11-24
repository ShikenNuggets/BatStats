use std::collections::{HashMap, HashSet};

use chrono::{Duration, NaiveDate, Utc};

use crate::speedrun_api::{
    src_api,
    types::{
        leaderboard::Leaderboard,
        run::{RunPlayer, RunPlayerType},
    },
};

pub async fn get_player_name(player: &RunPlayer) -> Option<String> {
    if let RunPlayerType::Guest = player.rel {
        return player.name.clone();
    }

    if player.id.is_none() {
        return player.name.clone();
    }

    let user = src_api::get_user(&player.id.clone().unwrap()).await;
    user.as_ref()?;

    Some(user.unwrap().names.international)
}

async fn get_all_runners(leaderboards: &Vec<Leaderboard>) -> HashSet<String> {
    let mut runners: HashSet<String> = HashSet::new();

    for leaderboard in leaderboards {
        for run in &leaderboard.runs {
            let Some(player) = run.run.players.first() else {
                continue;
            };
            let Some(player_name) = get_player_name(player).await else {
                continue;
            };
            runners.insert(player_name);
        }
    }

    runners
}

pub fn get_fastest_time(leaderboard: &Leaderboard) -> Option<f64> {
    if leaderboard.runs.is_empty() {
        return None;
    }

    Some(leaderboard.runs.first().unwrap().run.times.primary_t)
}

fn get_slowest_time(leaderboard: &Leaderboard) -> Option<f64> {
    if leaderboard.runs.is_empty() {
        return None;
    }

    Some(leaderboard.runs.last().unwrap().run.times.primary_t)
}

pub fn get_average_time(leaderboard: &Leaderboard) -> Option<f64> {
    if leaderboard.runs.is_empty() {
        return None;
    }

    let mut total_time: f64 = 0.0;
    for run in &leaderboard.runs {
        total_time += run.run.times.primary_t;
    }

    Some(total_time / (leaderboard.runs.len() as f64))
}

fn is_more_than_n_years_ago(date: &NaiveDate, years: i64) -> bool {
    let current_date = Utc::now().date_naive();
    let n_years_ago = current_date - Duration::days(365 * years);
    date < &n_years_ago
}

pub fn get_adjusted_average_time(leaderboard: &Leaderboard) -> Option<f64> {
    let real_average = get_average_time(leaderboard);
    if real_average.is_none() || leaderboard.runs.len() < 5 {
        return real_average;
    }
    let real_average = real_average.unwrap();

    let mut total_time: f64 = 0.0;
    let mut num_times: i64 = 0;

    for run in &leaderboard.runs {
        if run.run.date.is_none() {
            continue; // Ignore runs that are too old to have a date
        }

        if run.run.times.primary_t > real_average
            && is_more_than_n_years_ago(&run.run.date.unwrap(), 5)
        {
            continue; // Ignore very old runs that are above average
        }

        num_times += 1;
        total_time += run.run.times.primary_t;
    }

    if num_times == 0 {
        return Some(real_average);
    }

    let adjusted_average = total_time / (num_times as f64);

    if !adjusted_average.is_finite() || adjusted_average <= 0.0 || adjusted_average > real_average {
        return Some(real_average);
    }

    Some(adjusted_average)
}

async fn get_runner_times_map(leaderboard: &Leaderboard) -> HashMap<String, f64> {
    let mut run_times: HashMap<String, f64> = HashMap::new();

    let fastest_time = get_fastest_time(leaderboard);
    let slowest_time = get_slowest_time(leaderboard);

    if fastest_time.is_none() || slowest_time.is_none() {
        return run_times;
    }

    for run in &leaderboard.runs {
        let Some(player) = run.run.players.first() else {
            continue;
        };
        let Some(runner_name) = get_player_name(player).await else {
            continue;
        };

        if !run_times.contains_key(&runner_name)
            || run_times[&runner_name] > run.run.times.primary_t
        {
            run_times.insert(runner_name, run.run.times.primary_t);
        }
    }

    run_times
}

async fn get_total_runner_times(
    leaderboards: &Vec<Leaderboard>,
    subtract_from_wr: bool,
) -> HashMap<String, f64> {
    let mut runner_times: HashMap<String, f64> = HashMap::new();

    let all_runners = get_all_runners(leaderboards).await;
    for runner in &all_runners {
        runner_times.insert(runner.to_string(), 0.0);
    }

    for leaderboard in leaderboards {
        let api_category = src_api::get_category(&leaderboard.category).await;
        if api_category.is_none() || api_category.as_ref().unwrap().miscellaneous {
            continue;
        }

        let fastest_time = get_fastest_time(leaderboard);
        let slowest_time = get_slowest_time(leaderboard);
        let times = get_runner_times_map(leaderboard).await;

        if fastest_time.is_none() || slowest_time.is_none() || times.is_empty() {
            println!(
                "Something went wrong for leaderboard of category {}",
                leaderboard.category
            );
            continue;
        }

        let fastest_time = fastest_time.unwrap();
        let slowest_time = slowest_time.unwrap() - fastest_time;

        for runner in &all_runners {
            let mut time_to_add = slowest_time;
            if times.contains_key(runner) {
                time_to_add = times[runner];
                if subtract_from_wr {
                    time_to_add -= fastest_time;
                }
            }

            runner_times.insert(runner.to_string(), runner_times[runner] + time_to_add);
        }
    }

    runner_times
}

pub async fn get_leaderboard_for_subcategory(
    game_id: &str,
    category_id: &str,
    var_id: &str,
    val_id: &str,
) -> Option<Leaderboard> {
    let mut vars: HashMap<String, String> = HashMap::new();
    vars.insert(var_id.to_string(), val_id.to_string());
    return src_api::get_leaderboard(game_id, category_id, &vars).await;
}

pub async fn combine_times_best_only(leaderboards: &Vec<Leaderboard>) -> HashMap<String, f64> {
    let mut combined_times: HashMap<String, f64> = HashMap::new();

    for leaderboard in leaderboards {
        let lb_temp = vec![leaderboard.clone()];
        let times = get_total_runner_times(&lb_temp, false).await;
        for time in times {
            if !combined_times.contains_key(&time.0) || combined_times[&time.0] > time.1 {
                combined_times.insert(time.0, time.1);
            }
        }
    }

    combined_times
}

pub async fn get_game_name_from_leaderboard(leaderboard: &Leaderboard) -> String {
    let game = src_api::get_game(&leaderboard.game).await;
    if game.is_none() {
        return "Unknown Game".to_string();
    }

    let game_name = game.unwrap().names.international;
    if game_name.contains("Asylum") {
        return "Asylum".to_string();
    } else if game_name.contains("City") {
        return "City".to_string();
    } else if game_name.contains("Origins") {
        return "Origins".to_string();
    } else if game_name.contains("Knight") {
        return "Knight".to_string();
    }

    game_name
}

pub async fn get_category_name_from_leaderboard(leaderboard: &Leaderboard) -> String {
    let category = src_api::get_category(&leaderboard.category).await;
    if category.is_none() {
        return "Unknown Category".to_string();
    }

    category.unwrap().name
}

pub async fn get_subcategory_name_from_leaderboard(leaderboard: &Leaderboard) -> String {
    let mut subcategory_parts: Vec<String> = Vec::new();

    for (var_id, value_id) in &leaderboard.values {
        let var = src_api::get_variable(var_id).await;
        if var.is_none() {
            continue;
        }

        let var = var.unwrap();
        if !var.is_subcategory {
            continue;
        }

        if let Some(value) = var.values.values.get(value_id) {
            subcategory_parts.push(value.label.clone());
        }
    }

    if subcategory_parts.is_empty() {
        return "".to_string();
    }

    subcategory_parts.join(", ")
}

pub async fn get_full_category_name(leaderboard: &Leaderboard) -> String {
    let game_name = get_game_name_from_leaderboard(leaderboard).await;
    let category_name = get_category_name_from_leaderboard(leaderboard).await;
    let subcategory_name = get_subcategory_name_from_leaderboard(leaderboard).await;

    if subcategory_name.is_empty() {
        return format!("{} - {}", game_name, category_name);
    }

    format!("{} - {} ({})", game_name, category_name, subcategory_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_leaderboard(runs: Vec<(i32, f64, Vec<&str>)>) -> Leaderboard {
        let runs_json: Vec<serde_json::Value> = runs
            .into_iter()
            .enumerate()
            .map(|(idx, (place, time, players))| {
                let players_json: Vec<serde_json::Value> = players
                    .into_iter()
                    .map(|name| {
                        serde_json::json!({
                            "rel": "guest",
                            "id": null,
                            "name": name,
                            "uri": format!("https://example.com/guests/{name}")
                        })
                    })
                    .collect();

                serde_json::json!({
                    "place": place,
                    "run": {
                        "id": format!("run_{idx}"),
                        "weblink": format!("https://example.com/runs/run_{idx}"),
                        "game": "game1",
                        "level": null,
                        "category": "cat1",
                        "comment": null,
                        "status": {
                            "status": "verified",
                            "examiner": null,
                            "verify-date": null
                        },
                        "players": players_json,
                        "date": "2024-01-01",
                        "submitted": null,
                        "times": {
                            "primary": "PT100S",
                            "primary_t": time,
                            "realtime": null,
                            "realtime_t": null,
                            "realtime_noloads": null,
                            "realtime_noloads_t": null,
                            "ingame": null,
                            "ingame_t": null
                        },
                        "system": {
                            "platform": null,
                            "emulated": false,
                            "region": null
                        },
                        "splits": null,
                        "values": {}
                    }
                })
            })
            .collect();

        serde_json::from_value(serde_json::json!({
            "weblink": "https://example.com/lb",
            "game": "game1",
            "category": "cat1",
            "level": null,
            "platform": null,
            "region": null,
            "emulators": null,
            "video-only": false,
            "timing": "realtime",
            "values": {},
            "runs": runs_json,
            "links": []
        }))
        .expect("valid test leaderboard")
    }

    #[tokio::test]
    async fn test_empty_players_and_coop_attribution() {
        let lb = make_leaderboard(vec![
            (1, 100.0, vec![]),
            (2, 120.0, vec!["PrimaryRunner", "CoopPartner"]),
        ]);

        let runners = get_all_runners(&vec![lb.clone()]).await;
        assert_eq!(runners, HashSet::from(["PrimaryRunner".to_string()]));

        let times = get_runner_times_map(&lb).await;
        assert_eq!(times.get("PrimaryRunner"), Some(&120.0));
        assert!(!times.contains_key("CoopPartner"));
    }

    #[test]
    fn test_get_adjusted_average_time_all_dateless_runs() {
        let mut lb = make_leaderboard(vec![
            (1, 100.0, vec!["R1"]),
            (2, 120.0, vec!["R2"]),
            (3, 140.0, vec!["R3"]),
            (4, 160.0, vec!["R4"]),
            (5, 180.0, vec!["R5"]),
        ]);
        for run in &mut lb.runs {
            run.run.date = None;
        }

        let avg = get_adjusted_average_time(&lb);
        assert_eq!(avg, Some(140.0));
    }
}
