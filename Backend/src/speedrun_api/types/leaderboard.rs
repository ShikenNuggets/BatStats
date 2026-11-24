#![allow(dead_code)]

use std::collections::HashMap;

use crate::speedrun_api::types::core;
use crate::speedrun_api::types::run;
use crate::speedrun_api::types::traits;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub enum TimingType {
    #[serde(rename = "ingame")]
    Ingame,

    #[serde(rename = "realtime")]
    RealTime,

    #[serde(rename = "realtime_noloads")]
    RealTimeNoLoads,
}

#[derive(Clone, Deserialize)]
pub struct LeaderboardRun {
    pub place: i32,
    pub run: run::Run,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Leaderboard {
    pub weblink: String,
    pub game: String,
    pub category: String,
    pub level: Option<String>,
    pub platform: Option<String>,
    pub region: Option<String>,
    pub emulators: Option<bool>,
    pub video_only: bool,
    pub timing: Option<TimingType>,
    pub values: HashMap<String, String>,
    pub runs: Vec<LeaderboardRun>,
    pub links: Vec<core::SrcLink>,
}

pub fn get_leaderboard_cache_key(
    game: &str,
    category: &str,
    variables: &HashMap<String, String>,
) -> String {
    let mut sorted_vars: Vec<(&String, &String)> = variables.iter().collect();
    sorted_vars.sort_by(|a, b| a.0.cmp(b.0).then_with(|| a.1.cmp(b.1)));

    let mut key = String::with_capacity(game.len() + category.len() + 2 + (variables.len() * 16));
    key.push_str(game);
    key.push('_');
    key.push_str(category);

    for (var_id, val_id) in sorted_vars {
        key.reserve(var_id.len() + val_id.len() + 2);
        key.push('_');
        key.push_str(var_id);
        key.push('_');
        key.push_str(val_id);
    }

    key
}

impl traits::Cacheable for Leaderboard {
    fn key(&self) -> String {
        get_leaderboard_cache_key(&self.game, &self.category, &self.values)
    }
}

#[derive(Deserialize)]
pub struct LeaderboardResponse {
    pub data: Leaderboard,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_leaderboard_cache_key_is_order_independent() {
        let vars_ab = HashMap::from([
            ("var_z".to_string(), "val_2".to_string()),
            ("var_a".to_string(), "val_1".to_string()),
            ("var_m".to_string(), "val_3".to_string()),
        ]);
        let vars_ba = HashMap::from([
            ("var_m".to_string(), "val_3".to_string()),
            ("var_z".to_string(), "val_2".to_string()),
            ("var_a".to_string(), "val_1".to_string()),
        ]);

        let expected = "game1_cat1_var_a_val_1_var_m_val_3_var_z_val_2";
        for _ in 0..20 {
            assert_eq!(
                get_leaderboard_cache_key("game1", "cat1", &vars_ab),
                expected
            );
            assert_eq!(
                get_leaderboard_cache_key("game1", "cat1", &vars_ba),
                expected
            );
        }
    }

    #[test]
    fn test_leaderboard_nullable_timing_and_bool_emulators() {
        let lb_bool: Leaderboard = serde_json::from_value(serde_json::json!({
            "weblink": "https://example.com/lb",
            "game": "g1",
            "category": "c1",
            "level": null,
            "platform": null,
            "region": null,
            "emulators": true,
            "video-only": false,
            "timing": null,
            "values": {},
            "runs": [],
            "links": [{ "rel": "derived-games", "uri": "https://example.com/derived" }]
        }))
        .expect("leaderboard with bool emulators and null timing should deserialize");
        assert_eq!(lb_bool.emulators, Some(true));
        assert!(lb_bool.timing.is_none());

        let lb_null: Leaderboard = serde_json::from_value(serde_json::json!({
            "weblink": "https://example.com/lb",
            "game": "g1",
            "category": "c1",
            "level": null,
            "platform": null,
            "region": null,
            "emulators": null,
            "video-only": false,
            "timing": null,
            "values": {},
            "runs": [],
            "links": []
        }))
        .expect("leaderboard with null emulators should deserialize");
        assert_eq!(lb_null.emulators, None);
    }
}
