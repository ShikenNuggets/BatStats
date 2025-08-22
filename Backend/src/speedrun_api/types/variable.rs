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
	pub miscellaneous: Option<bool>
}

#[derive(Deserialize)]
pub struct VariableValue{
	pub label: String,
	pub rules: Option<String>,
	pub flags: Option<VariableValueFlags>,
}

#[derive(Deserialize)]
pub struct VariableValues{
	pub _note: String,
	pub choices: Option<HashMap<String, String>>,
	pub values: HashMap<String, VariableValue>,
	pub default: Option<String>,
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
	pub values: VariableValues,
	pub is_subcategory: bool,
	// TODO - links
}

#[derive(Deserialize)]
pub struct VariableResponse{
	pub data: Variable,
}

#[derive(Deserialize)]
pub struct VariablesResponse{
	pub data: Vec<Variable>,
}
