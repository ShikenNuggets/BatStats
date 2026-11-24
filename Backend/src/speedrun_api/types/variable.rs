#![allow(dead_code)]

use std::collections::HashMap;

use serde::Deserialize;

use crate::speedrun_api::types::traits;

#[derive(Clone, Deserialize)]
pub struct VariableScope {
    #[serde(rename = "type")]
    scope_type: String,
}

#[derive(Clone, Deserialize)]
pub struct VariableValueFlags {
    pub miscellaneous: Option<bool>,
}

#[derive(Clone, Deserialize)]
pub struct VariableValue {
    pub label: String,
    pub rules: Option<String>,
    pub flags: Option<VariableValueFlags>,
}

#[derive(Clone, Deserialize)]
pub struct VariableValues {
    pub _note: Option<String>,
    pub choices: Option<HashMap<String, String>>,
    pub values: HashMap<String, VariableValue>,
    pub default: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Variable {
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

impl traits::Cacheable for Variable {
    fn key(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Deserialize)]
pub struct VariableResponse {
    pub data: Variable,
}

#[derive(Deserialize)]
pub struct VariablesResponse {
    pub data: Vec<Variable>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_optional_note_deserializes() {
        let null_note: VariableValues = serde_json::from_value(serde_json::json!({
            "_note": null,
            "choices": null,
            "values": {},
            "default": null
        }))
        .expect("null _note should deserialize");
        assert_eq!(null_note._note, None);

        let omitted_note: VariableValues = serde_json::from_value(serde_json::json!({
            "values": {},
            "default": null
        }))
        .expect("omitted _note should deserialize");
        assert_eq!(omitted_note._note, None);
    }
}
