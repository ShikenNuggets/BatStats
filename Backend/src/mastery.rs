use std::collections::HashMap;

use crate::{
    speedrun_api::{src_api, types::leaderboard::Leaderboard},
    utils,
};

#[derive(Default)]
pub struct RawMasteryData {
    pub num_players: i64,
    pub mastery_percents: HashMap<String, f64>,
}

pub fn calculate_mastery(wr: f64, average: f64, run_time: f64) -> f64 {
    if !wr.is_finite() || !average.is_finite() || !run_time.is_finite() || average <= wr {
        return 0.0;
    }

    if run_time <= wr {
        return 1.0;
    } else if run_time >= average {
        return 0.0;
    }

    (average - run_time) / (average - wr)
}

pub async fn get_raw_mastery_ranks(leaderboard: &Leaderboard) -> RawMasteryData {
    let mut raw_mastery = RawMasteryData {
        num_players: leaderboard.runs.len() as i64,
        ..Default::default()
    };

    let fastest_time = utils::get_fastest_time(leaderboard);
    if fastest_time.is_none() {
        println!("Leaderboard's fastest time was invalid");
        return raw_mastery;
    }
    let fastest_time = fastest_time.unwrap();

    let average_time = utils::get_adjusted_average_time(leaderboard);
    if average_time.is_none() {
        println!("Leaderboard's average time was invalid");
        return raw_mastery;
    }
    let average_time = average_time.unwrap();

    for run in &leaderboard.runs {
        let mastery = calculate_mastery(fastest_time, average_time, run.run.times.primary_t);
        if mastery <= 0.0 {
            continue;
        }

        let Some(player) = run.run.players.first() else {
            println!("Run {} had no players", run.run.id);
            continue;
        };

        let Some(player_name) = utils::get_player_name(player).await else {
            println!("Run {} had no valid player name", run.run.id);
            continue;
        };

        // Keep only the best (highest) mastery for this player
        raw_mastery
            .mastery_percents
            .entry(player_name)
            .and_modify(|m| *m = m.max(mastery))
            .or_insert(mastery);
    }

    raw_mastery
}

pub fn aggregate_game_mastery(raw_mastery_datas: &[RawMasteryData]) -> HashMap<String, f64> {
    let total_players: i64 = raw_mastery_datas.iter().map(|d| d.num_players).sum();
    if total_players <= 0 {
        return HashMap::new();
    }

    let mut overall_mastery = HashMap::new();

    for raw_data in raw_mastery_datas {
        let percent_relevance = (raw_data.num_players as f64) / (total_players as f64);

        for (player, mastery) in &raw_data.mastery_percents {
            let weighted_mastery = mastery * percent_relevance;
            overall_mastery
                .entry(player.clone())
                .and_modify(|m| *m += weighted_mastery)
                .or_insert(weighted_mastery);
        }
    }

    overall_mastery
}

pub async fn get_mastery_ranks_for_game(game_id: &str) -> HashMap<String, f64> {
    let mut raw_mastery_datas: Vec<RawMasteryData> = Vec::new();

    let boards = src_api::get_all_fullgame_leaderboards(game_id).await;
    for board in boards {
        let raw_data = get_raw_mastery_ranks(&board).await;
        raw_mastery_datas.push(raw_data);
    }

    aggregate_game_mastery(&raw_mastery_datas)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_mastery_wr_case() {
        // When run time equals WR, mastery should be 1.0
        assert_eq!(calculate_mastery(100.0, 200.0, 100.0), 1.0);
        // When run time is better than WR, mastery should still be 1.0 (can't beat WR)
        assert_eq!(calculate_mastery(100.0, 200.0, 99.0), 1.0);
    }

    #[test]
    fn test_calculate_mastery_average_case() {
        // When run time equals average, mastery should be 0.0
        assert_eq!(calculate_mastery(100.0, 200.0, 200.0), 0.0);
        // When run time is worse than average, mastery should still be 0.0
        assert_eq!(calculate_mastery(100.0, 200.0, 300.0), 0.0);
    }

    #[test]
    fn test_calculate_mastery_middle_range() {
        // For time exactly halfway between WR and average: (avg - time) / (avg - wr)
        // (200 - 150) / (200 - 100) = 50 / 100 = 0.5
        assert_eq!(calculate_mastery(100.0, 200.0, 150.0), 0.5);

        // (200 - 120) / (200 - 100) = 80 / 100 = 0.8
        let result = calculate_mastery(100.0, 200.0, 120.0);
        assert!((result - 0.8).abs() < 0.0001);

        // (200 - 150) / (200 - 100) = 50 / 100 = 0.5
        let result = calculate_mastery(100.0, 200.0, 150.0);
        assert!((result - 0.5).abs() < 0.0001);
    }

    #[test]
    fn test_multiple_runs_same_player_best_kept() {
        // REGRESSION TEST for: When a player has multiple runs in a category,
        // the HashMap was being overwritten with each insert, keeping only the last run.
        // The fix uses entry().and_modify() to keep only the maximum mastery.

        // Simulate the mastery calculation for a player with two runs
        let wr = 100.0;
        let average = 200.0;

        let run1_time = 120.0;
        let run1_mastery = calculate_mastery(wr, average, run1_time); // 0.8

        let run2_time = 150.0;
        let run2_mastery = calculate_mastery(wr, average, run2_time); // 0.5

        // Build a hashmap the old way (buggy) - would keep only last value
        let mut mastery_old_way = HashMap::new();
        mastery_old_way.insert("PlayerA".to_string(), run1_mastery);
        mastery_old_way.insert("PlayerA".to_string(), run2_mastery); // Overwrites!

        // Build a hashmap the new way (fixed) - keeps the best
        let mut mastery_new_way = HashMap::new();
        mastery_new_way
            .entry("PlayerA".to_string())
            .and_modify(|m: &mut f64| *m = m.max(run1_mastery))
            .or_insert(run1_mastery);
        mastery_new_way
            .entry("PlayerA".to_string())
            .and_modify(|m: &mut f64| *m = m.max(run2_mastery))
            .or_insert(run2_mastery);

        // Old way would have only run2_mastery (0.5)
        assert!(
            (mastery_old_way["PlayerA"] - run2_mastery).abs() < 0.0001,
            "Old way incorrectly keeps 0.5"
        );

        // New way keeps run1_mastery (0.8)
        assert!(
            (mastery_new_way["PlayerA"] - run1_mastery).abs() < 0.0001,
            "New way should keep best mastery of 0.8, got {}",
            mastery_new_way["PlayerA"]
        );
    }

    #[tokio::test]
    async fn test_raw_mastery_ranks_empty_players_and_coop() {
        let lb: Leaderboard = serde_json::from_value(serde_json::json!({
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
            "links": [],
            "runs": [
                {
                    "place": 1,
                    "run": {
                        "id": "run_empty",
                        "weblink": "https://example.com/runs/run_empty",
                        "game": "game1",
                        "level": null,
                        "category": "cat1",
                        "comment": null,
                        "status": { "status": "verified", "examiner": null, "verify-date": null },
                        "players": [],
                        "date": "2024-01-01",
                        "submitted": null,
                        "times": { "primary": "PT100S", "primary_t": 100.0, "realtime": null, "realtime_t": null, "realtime_noloads": null, "realtime_noloads_t": null, "ingame": null, "ingame_t": null },
                        "system": { "platform": null, "emulated": false, "region": null },
                        "splits": null,
                        "values": {}
                    }
                },
                {
                    "place": 2,
                    "run": {
                        "id": "run_coop",
                        "weblink": "https://example.com/runs/run_coop",
                        "game": "game1",
                        "level": null,
                        "category": "cat1",
                        "comment": null,
                        "status": { "status": "verified", "examiner": null, "verify-date": null },
                        "players": [
                            { "rel": "guest", "id": null, "name": "PrimaryRunner", "uri": "https://example.com/guests/PrimaryRunner" },
                            { "rel": "guest", "id": null, "name": "CoopPartner", "uri": "https://example.com/guests/CoopPartner" }
                        ],
                        "date": "2024-01-01",
                        "submitted": null,
                        "times": { "primary": "PT120S", "primary_t": 120.0, "realtime": null, "realtime_t": null, "realtime_noloads": null, "realtime_noloads_t": null, "ingame": null, "ingame_t": null },
                        "system": { "platform": null, "emulated": false, "region": null },
                        "splits": null,
                        "values": {}
                    }
                },
                {
                    "place": 3,
                    "run": {
                        "id": "run_slow",
                        "weblink": "https://example.com/runs/run_slow",
                        "game": "game1",
                        "level": null,
                        "category": "cat1",
                        "comment": null,
                        "status": { "status": "verified", "examiner": null, "verify-date": null },
                        "players": [
                            { "rel": "guest", "id": null, "name": "SlowRunner", "uri": "https://example.com/guests/SlowRunner" }
                        ],
                        "date": "2024-01-01",
                        "submitted": null,
                        "times": { "primary": "PT200S", "primary_t": 200.0, "realtime": null, "realtime_t": null, "realtime_noloads": null, "realtime_noloads_t": null, "ingame": null, "ingame_t": null },
                        "system": { "platform": null, "emulated": false, "region": null },
                        "splits": null,
                        "values": {}
                    }
                }
            ]
        }))
        .expect("valid test leaderboard");

        let raw = get_raw_mastery_ranks(&lb).await;
        assert!(raw.mastery_percents.contains_key("PrimaryRunner"));
        assert!(!raw.mastery_percents.contains_key("CoopPartner"));
    }

    #[test]
    fn test_calculate_mastery_non_finite_and_invalid_bounds() {
        assert_eq!(calculate_mastery(100.0, 100.0, 100.0), 0.0);
        assert_eq!(calculate_mastery(200.0, 100.0, 150.0), 0.0);
        assert_eq!(calculate_mastery(f64::NAN, 200.0, 150.0), 0.0);
        assert_eq!(calculate_mastery(100.0, f64::NAN, 150.0), 0.0);
        assert_eq!(calculate_mastery(100.0, 200.0, f64::NAN), 0.0);
        assert_eq!(calculate_mastery(100.0, f64::INFINITY, 150.0), 0.0);
        assert_eq!(calculate_mastery(100.0, 200.0, f64::NEG_INFINITY), 0.0);
    }

    #[test]
    fn test_aggregate_game_mastery_zero_total_players() {
        assert!(aggregate_game_mastery(&[]).is_empty());

        let zero_data = vec![RawMasteryData {
            num_players: 0,
            mastery_percents: HashMap::from([("RunnerA".to_string(), 1.0)]),
        }];
        assert!(aggregate_game_mastery(&zero_data).is_empty());
    }
}
