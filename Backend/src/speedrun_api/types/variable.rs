#![allow(dead_code)]

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct VariableScope{
	#[serde(rename = "type")]
	scope_type: String
}

#[derive(Deserialize)]
pub struct VariableValueFlags{
	pub miscellaneous: bool
}

#[derive(Deserialize)]
pub struct VariableValue{
	pub label: String,
	pub rules: String,
	pub flags: VariableValueFlags,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Variable{
	pub id: String,
	pub name: String,
	pub category: Option<String>,
	pub scope: VariableScope,
	pub mandatory: bool,
	pub user_defined: bool,
	pub obsoletes: bool,
	pub values: HashMap<String, VariableValue>
	// TODO - links
}

#[derive(Deserialize)]
pub struct VariableResponse{
	pub data: Variable,
}
