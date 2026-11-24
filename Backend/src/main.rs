mod asylum;
mod city;
mod knight;
mod origins;

mod gist_upload;
mod mastery;
mod speedrun_api;
mod utils;

// const MULTI_GAME_ID: &str = "nd2eyoed";
// const CATEXT_GAME_ID: &str = "m1mnnv3d";

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};
use tokio::fs;

use std::{
    collections::{HashMap, HashSet},
    env,
    path::Path,
};

use speedrun_api::src_api;

use crate::speedrun_api::types::{
    leaderboard::Leaderboard,
    run::{RunPlayer, RunPlayerType},
};

async fn get_player_name(player: &RunPlayer) -> Option<String> {
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

/// Counts world records (`place == 1`) per primary runner across all provided leaderboards.
///
/// Intentionally includes `miscellaneous` categories; tied `place == 1` holders on a leaderboard
/// are all credited (deduplicated per player within a single board).
async fn get_world_records(leaderboards: &Vec<Leaderboard>) -> HashMap<String, i32> {
    let mut world_records: HashMap<String, i32> = HashMap::new();
    for lb in leaderboards {
        let mut seen_players = HashSet::new();
        for run in &lb.runs {
            if run.place != 1 {
                continue;
            }

            let Some(player) = run.run.players.first() else {
                continue;
            };
            let Some(player_name) = get_player_name(player).await else {
                continue;
            };
            if !seen_players.insert(player_name.clone()) {
                continue;
            }

            *world_records.entry(player_name).or_insert(0) += 1;
        }
    }

    world_records
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

fn get_fastest_time(leaderboard: &Leaderboard) -> Option<f64> {
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

fn get_last_place(leaderboard: &Leaderboard) -> i32 {
    if leaderboard.runs.is_empty() {
        return 0;
    }

    leaderboard.runs.last().unwrap().place
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

        let time = run.run.times.primary_t;
        run_times
            .entry(runner_name)
            .and_modify(|best| *best = best.min(time))
            .or_insert(time);
    }

    run_times
}

async fn get_runner_ranks_map(leaderboard: &Leaderboard) -> HashMap<String, i32> {
    let mut run_ranks: HashMap<String, i32> = HashMap::new();

    let last_place = get_last_place(leaderboard);
    if last_place <= 0 {
        return run_ranks;
    }

    for run in &leaderboard.runs {
        let Some(player) = run.run.players.first() else {
            continue;
        };
        let Some(runner_name) = get_player_name(player).await else {
            continue;
        };

        let rank = run.place - 1;
        run_ranks
            .entry(runner_name)
            .and_modify(|best| *best = (*best).min(rank))
            .or_insert(rank);
    }

    run_ranks
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
        if utils::should_exclude_misc_category(leaderboard).await {
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
        let slowest_time = slowest_time.unwrap();
        let missing_penalty = if subtract_from_wr {
            slowest_time - fastest_time
        } else {
            slowest_time
        };

        for runner in &all_runners {
            let time_to_add = match times.get(runner) {
                Some(&runner_time) if subtract_from_wr => runner_time - fastest_time,
                Some(&runner_time) => runner_time,
                None => missing_penalty,
            };

            *runner_times.entry(runner.clone()).or_insert(0.0) += time_to_add;
        }
    }

    runner_times
}

async fn get_all_runner_ranks(leaderboards: &Vec<Leaderboard>) -> HashMap<String, i32> {
    let mut runner_ranks: HashMap<String, i32> = HashMap::new();

    let all_runners = get_all_runners(leaderboards).await;
    for runner in &all_runners {
        runner_ranks.insert(runner.to_string(), 0);
    }

    for leaderboard in leaderboards {
        if utils::should_exclude_misc_category(leaderboard).await {
            continue;
        }

        let last_place = get_last_place(leaderboard);
        let ranks = get_runner_ranks_map(leaderboard).await;

        if last_place <= 0 || ranks.is_empty() {
            println!(
                "Something went wrong for leaderboard of category {}",
                leaderboard.category
            );
            continue;
        }

        for runner in &all_runners {
            let mut rank_to_add = last_place;
            if ranks.contains_key(runner) {
                rank_to_add = ranks[runner];
            }

            runner_ranks.insert(runner.to_string(), runner_ranks[runner] + rank_to_add);
        }
    }

    runner_ranks
}

async fn combine_times(
    asylum_times: &HashMap<String, f64>,
    city_times: &HashMap<String, f64>,
    origins_times: &HashMap<String, f64>,
    knight_times: &HashMap<String, f64>,
) -> HashMap<String, f64> {
    let mut final_times: HashMap<String, f64> = HashMap::new();

    for time in asylum_times {
        if !city_times.contains_key(time.0)
            || !origins_times.contains_key(time.0)
            || !knight_times.contains_key(time.0)
        {
            continue;
        }

        let time_val = time.1 + city_times[time.0] + origins_times[time.0] + knight_times[time.0];
        final_times.insert(time.0.to_string(), time_val);
    }

    final_times
}

async fn get_all_any_percent_times() -> HashMap<String, f64> {
    let asylum_any_times = asylum::get_best_any_percent_times().await;
    let city_any_times = city::get_best_any_percent_times().await;
    let origins_any_times = origins::get_best_any_percent_times().await;
    let knight_any_times = knight::get_best_any_percent_times().await;

    return combine_times(
        &asylum_any_times,
        &city_any_times,
        &origins_any_times,
        &knight_any_times,
    )
    .await;
}

async fn get_all_glitchless_times() -> HashMap<String, f64> {
    let asylum_glitchless_times = asylum::get_best_glitchless_times().await;
    let city_glitchless_times = city::get_best_glitchless_times().await;
    let origins_glitchless_times = origins::get_best_glitchless_times().await;
    let knight_glitchless_times = knight::get_best_glitchless_times().await;

    return combine_times(
        &asylum_glitchless_times,
        &city_glitchless_times,
        &origins_glitchless_times,
        &knight_glitchless_times,
    )
    .await;
}

async fn get_all_hundo_times() -> HashMap<String, f64> {
    let asylum_hundo_times = asylum::get_best_hundo_times().await;
    let city_hundo_times = city::get_best_hundo_times().await;
    let origins_hundo_times = origins::get_best_hundo_times().await;
    let knight_hundo_times = knight::get_best_hundo_times().await;

    return combine_times(
        &asylum_hundo_times,
        &city_hundo_times,
        &origins_hundo_times,
        &knight_hundo_times,
    )
    .await;
}

fn merge_mastery(
    mastery: &HashMap<String, f64>,
    overall_mastery: &mut HashMap<String, f64>,
    divisor: f64,
) {
    for (name, score) in mastery {
        let weighted = score / divisor;
        *overall_mastery.entry(name.clone()).or_insert(0.0) += weighted;
    }
}

fn get_overall_mastery(
    asylum_mastery: &HashMap<String, f64>,
    city_mastery: &HashMap<String, f64>,
    origins_mastery: &HashMap<String, f64>,
    knight_mastery: &HashMap<String, f64>,
) -> HashMap<String, f64> {
    let games = [
        asylum_mastery,
        city_mastery,
        origins_mastery,
        knight_mastery,
    ];
    let divisor = games.len() as f64;
    let mut overall_mastery = HashMap::new();

    for game_mastery in games {
        merge_mastery(game_mastery, &mut overall_mastery, divisor);
    }

    overall_mastery
}

#[derive(Clone, Deserialize, Serialize)]
pub struct WorldRecordInfo {
    pub player_name: String,
    pub category_name: String,
    pub date: DateTime<Utc>,
}

fn sort_oldest_world_records(world_records: &mut [WorldRecordInfo]) {
    world_records.sort_by(|a, b| {
        a.date
            .cmp(&b.date)
            .then_with(|| a.category_name.cmp(&b.category_name))
            .then_with(|| a.player_name.cmp(&b.player_name))
    });
}

/// Collects all dated `place == 1` world records across all provided leaderboards, including
/// `miscellaneous` categories and tied world record holders, sorted oldest-first.
async fn get_oldest_world_records(leaderboards: &Vec<Leaderboard>) -> Vec<WorldRecordInfo> {
    let mut world_records: Vec<WorldRecordInfo> = Vec::new();

    for lb in leaderboards {
        let wr_runs: Vec<_> = lb.runs.iter().filter(|run| run.place == 1).collect();
        if wr_runs.is_empty() {
            continue;
        }

        let category_name = utils::get_full_category_name(lb).await;
        let mut seen_players = HashSet::new();

        for wr_run in wr_runs {
            let Some(player) = wr_run.run.players.first() else {
                continue;
            };
            let Some(player_name) = get_player_name(player).await else {
                continue;
            };
            if !seen_players.insert(player_name.clone()) {
                continue;
            }

            let date = match wr_run.run.date {
                Some(naive_date) => DateTime::<Utc>::from_naive_utc_and_offset(
                    naive_date.and_hms_opt(0, 0, 0).unwrap(),
                    Utc,
                ),
                None => continue,
            };

            world_records.push(WorldRecordInfo {
                player_name,
                category_name: category_name.clone(),
                date,
            });
        }
    }

    sort_oldest_world_records(&mut world_records);
    world_records
}

enum Ordering {
    LowerIsBetter,
    HigherIsBetter,
}

fn get_sorted_values<T: Copy + Into<f64>>(
    map: HashMap<String, T>,
    order: Ordering,
) -> Vec<(String, T)> {
    let mut vec: Vec<(String, T)> = map.into_iter().collect();
    vec.sort_by(|a, b| {
        let a_val: f64 = a.1.into();
        let b_val: f64 = b.1.into();
        let cmp = match order {
            Ordering::LowerIsBetter => a_val.total_cmp(&b_val),
            Ordering::HigherIsBetter => b_val.total_cmp(&a_val),
        };
        cmp.then_with(|| a.0.cmp(&b.0))
    });

    vec
}

const TIE_EPSILON: f64 = 1e-9;

fn get_values_as_data_entries<T: Into<f64>>(data: Vec<(String, T)>) -> Vec<DataEntry> {
    let mut result = Vec::with_capacity(data.len());
    let mut prev_value: Option<f64> = None;
    let mut prev_rank: i64 = 0;

    for (index, (player, value)) in data.into_iter().enumerate() {
        let value_f64 = value.into();
        let is_tie = prev_value.is_some_and(|prev| {
            (value_f64.is_nan() && prev.is_nan())
                || (value_f64.is_finite()
                    && prev.is_finite()
                    && (value_f64 - prev).abs() < TIE_EPSILON)
                || (value_f64 == prev)
        });
        let rank = if is_tie {
            prev_rank
        } else {
            (index as i64) + 1
        };

        result.push(DataEntry {
            rank,
            player,
            value: value_f64,
        });

        prev_value = Some(value_f64);
        prev_rank = rank;
    }

    result
}

fn serialize_to_json<T: Copy + Into<f64> + Serialize>(
    map: HashMap<String, T>,
    order: Ordering,
) -> String {
    let sorted = get_sorted_values(map, order);
    let data_entries = get_values_as_data_entries(sorted);
    serde_json::to_string(&data_entries).unwrap()
}

pub fn as_json_string<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Parse the string as JSON Value
    let json_value: serde_json::Value =
        serde_json::from_str(value).map_err(serde::ser::Error::custom)?;
    json_value.serialize(serializer)
}

#[derive(Deserialize, Serialize)]
pub struct DataEntry {
    pub rank: i64,
    pub player: String,
    pub value: f64,
}

impl Default for DataEntry {
    fn default() -> Self {
        DataEntry {
            rank: -1,
            player: String::new(),
            value: 0.0,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct MetaData {
    pub date: DateTime<Utc>,
}

#[derive(Default, Deserialize, Serialize)]
pub struct OutputType {
    #[serde(serialize_with = "as_json_string")]
    pub meta: String,

    #[serde(serialize_with = "as_json_string")]
    pub world_records: String,

    #[serde(serialize_with = "as_json_string")]
    pub runner_times: String,

    #[serde(serialize_with = "as_json_string")]
    pub runner_ranks: String,

    #[serde(serialize_with = "as_json_string")]
    pub any_times: String,

    #[serde(serialize_with = "as_json_string")]
    pub glitchless_times: String,

    #[serde(serialize_with = "as_json_string")]
    pub hundo_times: String,

    #[serde(serialize_with = "as_json_string")]
    pub asylum_mastery: String,

    #[serde(serialize_with = "as_json_string")]
    pub city_mastery: String,

    #[serde(serialize_with = "as_json_string")]
    pub origins_mastery: String,

    #[serde(serialize_with = "as_json_string")]
    pub knight_mastery: String,

    #[serde(serialize_with = "as_json_string")]
    pub overall_mastery: String,

    #[serde(serialize_with = "as_json_string")]
    pub oldest_world_records: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let skip_upload = std::env::args().any(|arg| arg == "--skip-upload");

    let github_token = if !skip_upload {
        let token = env::var("GITHUB_TOKEN").map_err(|err| match err {
            env::VarError::NotPresent => {
                "You must set the GITHUB_TOKEN environment variable unless --skip-upload is set"
                    .to_string()
            }
            env::VarError::NotUnicode(_) => {
                "GITHUB_TOKEN environment variable exists, but is not valid Unicode".to_string()
            }
        })?;

        // Validate the GitHub token by trying to read the gist
        gist_upload::validate_github_token(&token)
            .await
            .map_err(|e| {
                format!(
                    "GitHub token was invalid or did not have access to the gist: {}",
                    e
                )
            })?;
        Some(token)
    } else {
        println!("Gist upload is disabled (--skip-upload flag is set)");
        None
    };

    println!("Getting initial leaderboard data...");
    let asylum_leaderboards = src_api::get_all_fullgame_leaderboards(asylum::GAME_ID).await;
    let city_leaderboards = src_api::get_all_fullgame_leaderboards(city::GAME_ID).await;
    let origins_leaderboards = src_api::get_all_fullgame_leaderboards(origins::GAME_ID).await;
    let knight_leaderboards = src_api::get_all_fullgame_leaderboards(knight::GAME_ID).await;

    let mut all_main_boards = Vec::with_capacity(
        asylum_leaderboards.len()
            + city_leaderboards.len()
            + origins_leaderboards.len()
            + knight_leaderboards.len(),
    );
    all_main_boards.extend(asylum_leaderboards);
    all_main_boards.extend(city_leaderboards);
    all_main_boards.extend(origins_leaderboards);
    all_main_boards.extend(knight_leaderboards);

    let mut output_data: OutputType = Default::default();

    println!("Processing world records...");
    let wrs = get_world_records(&all_main_boards).await;
    output_data.world_records = serialize_to_json(wrs, Ordering::HigherIsBetter);

    println!("Processing fastest runners...");
    let runner_times = get_total_runner_times(&all_main_boards, true).await;
    output_data.runner_times = serialize_to_json(runner_times, Ordering::LowerIsBetter);

    println!("Processing highest ranking runners...");
    let runner_ranks = get_all_runner_ranks(&all_main_boards).await;
    output_data.runner_ranks = serialize_to_json(runner_ranks, Ordering::LowerIsBetter);

    println!("Processing Any% times...");
    let any_times = get_all_any_percent_times().await;
    output_data.any_times = serialize_to_json(any_times, Ordering::LowerIsBetter);

    println!("Processing Glitchless times...");
    let glitchless_times = get_all_glitchless_times().await;
    output_data.glitchless_times = serialize_to_json(glitchless_times, Ordering::LowerIsBetter);

    println!("Processing 100% times...");
    let hundo_times = get_all_hundo_times().await;
    output_data.hundo_times = serialize_to_json(hundo_times, Ordering::LowerIsBetter);

    println!("Processing Asylum mastery ranks...");
    let asylum_mastery = mastery::get_mastery_ranks_for_game(asylum::GAME_ID).await;

    println!("Processing City mastery ranks...");
    let city_mastery = mastery::get_mastery_ranks_for_game(city::GAME_ID).await;

    println!("Processing Origins mastery ranks...");
    let origins_mastery = mastery::get_mastery_ranks_for_game(origins::GAME_ID).await;

    println!("Processing Knight mastery ranks...");
    let knight_mastery = mastery::get_mastery_ranks_for_game(knight::GAME_ID).await;

    println!("Processing overall mastery ranks...");
    let overall_mastery = get_overall_mastery(
        &asylum_mastery,
        &city_mastery,
        &origins_mastery,
        &knight_mastery,
    );

    output_data.asylum_mastery = serialize_to_json(asylum_mastery, Ordering::HigherIsBetter);
    output_data.city_mastery = serialize_to_json(city_mastery, Ordering::HigherIsBetter);
    output_data.origins_mastery = serialize_to_json(origins_mastery, Ordering::HigherIsBetter);
    output_data.knight_mastery = serialize_to_json(knight_mastery, Ordering::HigherIsBetter);
    output_data.overall_mastery = serialize_to_json(overall_mastery, Ordering::HigherIsBetter);

    println!("Processing oldest world records...");
    let oldest_world_records = get_oldest_world_records(&all_main_boards).await;
    output_data.oldest_world_records = serde_json::to_string(&oldest_world_records).unwrap();

    let meta_data = MetaData { date: Utc::now() };
    output_data.meta = serde_json::to_string(&meta_data).unwrap();

    let output_path = Path::new("BatStats.json");
    if let Some(dir) = output_path.parent()
        && !dir.as_os_str().is_empty()
        && !dir.exists()
    {
        fs::create_dir_all(dir).await?;
    }

    let content = serde_json::to_string(&output_data)?;
    fs::write(output_path, content).await?;

    if let Some(token) = github_token.as_deref() {
        gist_upload::upload_gist("BatStats.json", token).await?;
    } else {
        println!("Skipping gist upload (--skip-upload flag is set)");
    }

    Ok(())
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

        let ranks = get_runner_ranks_map(&lb).await;
        assert_eq!(ranks.get("PrimaryRunner"), Some(&1));
        assert!(!ranks.contains_key("CoopPartner"));
    }

    #[tokio::test]
    async fn test_multiple_runs_same_player_keeps_best_time_and_rank() {
        let lb = make_leaderboard(vec![
            (1, 100.0, vec!["RunnerA"]),
            (2, 110.0, vec!["RunnerB"]),
            (3, 150.0, vec!["RunnerA"]),
            (4, 105.0, vec!["RunnerB"]),
        ]);

        let times = get_runner_times_map(&lb).await;
        assert_eq!(times.get("RunnerA"), Some(&100.0));
        assert_eq!(times.get("RunnerB"), Some(&105.0));

        let ranks = get_runner_ranks_map(&lb).await;
        assert_eq!(ranks.get("RunnerA"), Some(&0));
        assert_eq!(ranks.get("RunnerB"), Some(&1));
    }

    #[test]
    fn test_get_sorted_values_handles_nan_without_panic() {
        let map = HashMap::from([
            ("RunnerA".to_string(), 100.0_f64),
            ("RunnerNaN".to_string(), f64::NAN),
            ("RunnerB".to_string(), 50.0_f64),
        ]);

        let asc = get_sorted_values(map.clone(), Ordering::LowerIsBetter);
        assert_eq!(asc.len(), 3);
        assert_eq!(asc[0].0, "RunnerB");
        assert_eq!(asc[1].0, "RunnerA");
        assert_eq!(asc[2].0, "RunnerNaN");

        let desc = get_sorted_values(map, Ordering::HigherIsBetter);
        assert_eq!(desc.len(), 3);
        assert_eq!(desc[0].0, "RunnerNaN");
        assert_eq!(desc[1].0, "RunnerA");
        assert_eq!(desc[2].0, "RunnerB");
    }

    #[test]
    fn test_get_sorted_values_and_data_entries_deterministic_ties() {
        let map = HashMap::from([
            ("Charlie".to_string(), 10.0),
            ("Alice".to_string(), 10.0),
            ("Bob".to_string(), 10.0),
            ("Dave".to_string(), 20.0),
        ]);

        let first = get_sorted_values(map.clone(), Ordering::LowerIsBetter);
        for _ in 0..10 {
            let repeated = get_sorted_values(map.clone(), Ordering::LowerIsBetter);
            assert_eq!(repeated, first);
        }
        assert_eq!(
            first.iter().map(|(p, _)| p.as_str()).collect::<Vec<_>>(),
            vec!["Alice", "Bob", "Charlie", "Dave"]
        );

        // Verify epsilon tie ranking handles float summation drift (0.1 + 0.2 vs 0.3)
        let entries = get_values_as_data_entries(vec![
            ("Alice".to_string(), 0.1_f64 + 0.2_f64),
            ("Bob".to_string(), 0.3_f64),
            ("Charlie".to_string(), 0.4_f64),
        ]);
        assert_eq!(entries[0].rank, 1);
        assert_eq!(entries[1].rank, 1);
        assert_eq!(entries[2].rank, 3);
    }

    #[test]
    fn test_sort_oldest_world_records_deterministic_tie_order() {
        let d1 = DateTime::<Utc>::from_naive_utc_and_offset(
            chrono::NaiveDate::from_ymd_opt(2020, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            Utc,
        );
        let d2 = DateTime::<Utc>::from_naive_utc_and_offset(
            chrono::NaiveDate::from_ymd_opt(2021, 5, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            Utc,
        );

        let mut records = vec![
            WorldRecordInfo {
                player_name: "Zed".to_string(),
                category_name: "City - Any%".to_string(),
                date: d1,
            },
            WorldRecordInfo {
                player_name: "Bob".to_string(),
                category_name: "Asylum - Any%".to_string(),
                date: d2,
            },
            WorldRecordInfo {
                player_name: "Alice".to_string(),
                category_name: "City - Any%".to_string(),
                date: d1,
            },
            WorldRecordInfo {
                player_name: "Cara".to_string(),
                category_name: "Asylum - Any%".to_string(),
                date: d1,
            },
        ];

        sort_oldest_world_records(&mut records);

        let order: Vec<(&str, &str)> = records
            .iter()
            .map(|r| (r.category_name.as_str(), r.player_name.as_str()))
            .collect();
        assert_eq!(
            order,
            vec![
                ("Asylum - Any%", "Cara"),
                ("City - Any%", "Alice"),
                ("City - Any%", "Zed"),
                ("Asylum - Any%", "Bob"),
            ]
        );
    }

    #[tokio::test]
    async fn test_get_total_runner_times_missing_runner_penalty() {
        let cat: crate::speedrun_api::types::category::Category =
            serde_json::from_value(serde_json::json!({
                "id": "cat1",
                "name": "Any%",
                "weblink": "https://example.com/cat1",
                "type": "per-game",
                "rules": "",
                "players": { "type": "exactly", "value": 1 },
                "miscellaneous": false,
                "links": []
            }))
            .expect("valid test category");
        crate::speedrun_api::src_cache::CATEGORY_CACHE.insert(&cat);

        let lb1 = make_leaderboard(vec![
            (1, 100.0, vec!["RunnerA"]),
            (2, 150.0, vec!["RunnerB"]),
        ]);
        let lb2 = make_leaderboard(vec![
            (1, 200.0, vec!["RunnerA"]),
            (2, 260.0, vec!["RunnerC"]),
        ]);
        let boards = vec![lb1, lb2];

        let raw_totals = get_total_runner_times(&boards, false).await;
        // RunnerA: 100 + 200 = 300
        // RunnerB: 150 + 260 (raw slowest on lb2) = 410
        // RunnerC: 150 (raw slowest on lb1) + 260 = 410
        assert_eq!(raw_totals.get("RunnerA"), Some(&300.0));
        assert_eq!(raw_totals.get("RunnerB"), Some(&410.0));
        assert_eq!(raw_totals.get("RunnerC"), Some(&410.0));

        let delta_totals = get_total_runner_times(&boards, true).await;
        // RunnerA: (100 - 100) + (200 - 200) = 0
        // RunnerB: (150 - 100) + (260 - 200) = 110
        // RunnerC: (150 - 100) + (260 - 200) = 110
        assert_eq!(delta_totals.get("RunnerA"), Some(&0.0));
        assert_eq!(delta_totals.get("RunnerB"), Some(&110.0));
        assert_eq!(delta_totals.get("RunnerC"), Some(&110.0));
    }

    #[tokio::test]
    async fn test_tied_world_records_and_misc_policy() {
        let game: crate::speedrun_api::types::game::Game =
            serde_json::from_value(serde_json::json!({
                "id": "game_be10",
                "names": {
                    "international": "Batman: Arkham City",
                    "japanese": null,
                    "twitch": null
                },
                "abbreviation": "bac",
                "weblink": "https://example.com/game_be10",
                "released": 2011,
                "release-date": "2011-10-18",
                "ruleset": {
                    "show-milliseconds": true,
                    "require-verification": true,
                    "require-video": false,
                    "run-times": ["realtime"],
                    "default-time": "realtime",
                    "emulators-allowed": false
                },
                "romhack": false,
                "gametypes": [],
                "platforms": [],
                "regions": [],
                "genres": [],
                "engines": [],
                "developers": [],
                "publishers": [],
                "moderators": {},
                "created": null
            }))
            .expect("valid test game");
        crate::speedrun_api::src_cache::GAME_CACHE.insert(&game);

        let main_cat: crate::speedrun_api::types::category::Category =
            serde_json::from_value(serde_json::json!({
                "id": "cat_main_be10",
                "name": "Any%",
                "weblink": "https://example.com/cat_main_be10",
                "type": "per-game",
                "rules": "",
                "players": { "type": "exactly", "value": 1 },
                "miscellaneous": false,
                "links": []
            }))
            .expect("valid test category");
        crate::speedrun_api::src_cache::CATEGORY_CACHE.insert(&main_cat);

        let misc_cat: crate::speedrun_api::types::category::Category =
            serde_json::from_value(serde_json::json!({
                "id": "cat_misc_be10",
                "name": "NG+ Glitchless",
                "weblink": "https://example.com/cat_misc_be10",
                "type": "per-game",
                "rules": "",
                "players": { "type": "exactly", "value": 1 },
                "miscellaneous": true,
                "links": []
            }))
            .expect("valid test misc category");
        crate::speedrun_api::src_cache::CATEGORY_CACHE.insert(&misc_cat);

        let mut lb_main = make_leaderboard(vec![
            (1, 100.0, vec!["RunnerB"]),
            (1, 100.0, vec!["RunnerA"]),
            (3, 150.0, vec!["RunnerC"]),
        ]);
        lb_main.game = "game_be10".to_string();
        lb_main.category = "cat_main_be10".to_string();

        let mut lb_misc = make_leaderboard(vec![
            (1, 200.0, vec!["RunnerA"]),
            (1, 200.0, vec!["RunnerA"]), // Same runner twice on one board shouldn't double-count
        ]);
        lb_misc.game = "game_be10".to_string();
        lb_misc.category = "cat_misc_be10".to_string();

        let boards = vec![lb_main, lb_misc];

        // WRs include misc boards and credit both tied place == 1 runners
        let wrs = get_world_records(&boards).await;
        assert_eq!(wrs.get("RunnerA"), Some(&2));
        assert_eq!(wrs.get("RunnerB"), Some(&1));
        assert_eq!(wrs.get("RunnerC"), None);

        // Oldest WRs include all tied place == 1 holders across main + misc boards
        let oldest = get_oldest_world_records(&boards).await;
        assert_eq!(oldest.len(), 3);
        let pairs: Vec<(&str, &str)> = oldest
            .iter()
            .map(|r| (r.category_name.as_str(), r.player_name.as_str()))
            .collect();
        assert_eq!(
            pairs,
            vec![
                ("City - Any%", "RunnerA"),
                ("City - Any%", "RunnerB"),
                ("City - NG+ Glitchless", "RunnerA"),
            ]
        );

        // Penalty aggregates exclude misc categories consistently
        let totals = get_total_runner_times(&boards, true).await;
        assert_eq!(totals.get("RunnerA"), Some(&0.0));
        assert_eq!(totals.get("RunnerB"), Some(&0.0));
        assert_eq!(totals.get("RunnerC"), Some(&50.0));

        let ranks = get_all_runner_ranks(&boards).await;
        assert_eq!(ranks.get("RunnerA"), Some(&0));
        assert_eq!(ranks.get("RunnerB"), Some(&0));
        assert_eq!(ranks.get("RunnerC"), Some(&2));
    }

    #[test]
    fn test_get_overall_mastery_weights_games_equally() {
        let asylum = HashMap::from([("Alice".to_string(), 1.0), ("Bob".to_string(), 0.8)]);
        let city = HashMap::from([("Alice".to_string(), 0.6), ("Cara".to_string(), 1.0)]);
        let origins = HashMap::from([("Alice".to_string(), 0.4), ("Bob".to_string(), 0.4)]);
        let knight = HashMap::from([("Alice".to_string(), 0.8)]);

        let overall = get_overall_mastery(&asylum, &city, &origins, &knight);

        assert!((overall["Alice"] - 0.7).abs() < 1e-12); // (1.0 + 0.6 + 0.4 + 0.8) / 4 = 0.7
        assert!((overall["Bob"] - 0.3).abs() < 1e-12); // (0.8 + 0.4) / 4 = 0.3
        assert!((overall["Cara"] - 0.25).abs() < 1e-12); // 1.0 / 4 = 0.25
        assert_eq!(overall.len(), 3);
    }
}
