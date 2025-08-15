#![allow(dead_code)]

use crate::speedrun_api::types::core;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CategoryType{
	PerGame,
	PerLevel
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CategoryPlayersType{
	Exactly,
	UpTo,
}

#[derive(Deserialize)]
pub struct CategoryPlayers{
	#[serde(rename = "type")]
	players_type: CategoryPlayersType,

	value: i32
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct CategoryResponse{
	pub data: Vec<Category>
}
