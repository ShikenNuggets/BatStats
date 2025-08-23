#![allow(dead_code)]

use crate::speedrun_api::types::core;
use crate::speedrun_api::types::traits;

use serde::Deserialize;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CategoryType{
	PerGame,
	PerLevel
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CategoryPlayersType{
	Exactly,
	UpTo,
}

#[derive(Clone, Deserialize)]
pub struct CategoryPlayers{
	#[serde(rename = "type")]
	pub players_type: CategoryPlayersType,

	pub value: i32
}

#[derive(Clone, Deserialize)]
pub struct Category{
	pub id: String,
	pub name: String,
	pub weblink: String,

	#[serde(rename = "type")]
	pub category_type: CategoryType,

	pub rules: String,
	pub players: CategoryPlayers,
	pub miscellaneous: bool,
	pub links: Vec<core::SrcLink>
}

impl traits::Cacheable for Category{
	fn key(&self) -> String{
		return self.id.to_string();
	}
}

#[derive(Deserialize)]
pub struct CategoryResponse{
	pub data: Vec<Category>
}
