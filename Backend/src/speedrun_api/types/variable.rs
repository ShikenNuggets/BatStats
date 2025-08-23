#![allow(dead_code)]

use std::collections::HashMap;

use serde::Deserialize;

use crate::speedrun_api::types::traits;

#[derive(Clone, Deserialize)]
pub struct VariableScope{
	#[serde(rename = "type")]
	scope_type: String
}

#[derive(Clone, Deserialize)]
pub struct VariableValueFlags{
	pub miscellaneous: Option<bool>
}

#[derive(Clone, Deserialize)]
pub struct VariableValue{
	pub label: String,
	pub rules: Option<String>,
	pub flags: Option<VariableValueFlags>,
}

#[derive(Clone, Deserialize)]
pub struct VariableValues{
	pub _note: String,
	pub choices: Option<HashMap<String, String>>,
	pub values: HashMap<String, VariableValue>,
	pub default: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Variable{
	pub id: String,
	pub name: String,
	pub category: Option<String>,
	pub scope: VariableScope,
	pub mandatory: bool,
	pub user_defined: bool,
	pub obsoletes: bool,
	pub values: VariableValues,
	pub is_subcategory: bool,
	// TODO - links
}

impl traits::Cacheable for Variable{
	fn key(&self) -> String {
		return self.id.to_string();
	}
}

#[derive(Deserialize)]
pub struct VariableResponse{
	pub data: Variable,
}

#[derive(Deserialize)]
pub struct VariablesResponse{
	pub data: Vec<Variable>,
}
