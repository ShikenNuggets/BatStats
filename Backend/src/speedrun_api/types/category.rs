#![allow(dead_code)]

use crate::speedrun_api::types::core;
use crate::speedrun_api::types::traits;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CategoryType {
    PerGame,
    PerLevel,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CategoryPlayersType {
    Exactly,
    UpTo,
}

#[derive(Clone, Deserialize)]
pub struct CategoryPlayers {
    #[serde(rename = "type")]
    pub players_type: CategoryPlayersType,

    pub value: i32,
}

#[derive(Clone, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub weblink: String,

    #[serde(rename = "type")]
    pub category_type: CategoryType,

    pub rules: Option<String>,
    pub players: CategoryPlayers,
    pub miscellaneous: bool,
    pub links: Vec<core::SrcLink>,
}

impl traits::Cacheable for Category {
    fn key(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Deserialize)]
pub struct CategoryResponse {
    pub data: Category,
}

#[derive(Deserialize)]
pub struct CategoriesResponse {
    pub data: Vec<Category>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_null_rules_deserializes() {
        let cat: Category = serde_json::from_value(serde_json::json!({
            "id": "cat1",
            "name": "Any%",
            "weblink": "https://www.speedrun.com/arkhamcity#Any",
            "type": "per-game",
            "rules": null,
            "players": { "type": "exactly", "value": 1 },
            "miscellaneous": false,
            "links": []
        }))
        .expect("category with null rules should deserialize");
        assert_eq!(cat.rules, None);
    }
}
