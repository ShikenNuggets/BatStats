#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::speedrun_api::types::traits;

#[derive(Clone, Deserialize)]
pub struct UserNames {
    pub international: String,
    pub japanese: Option<String>,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct User {
    pub id: String,
    pub names: UserNames,
    pub pronouns: Option<String>,
    pub weblink: String,
    // TODO - name-styles
    pub role: String,
    pub signup: Option<DateTime<Utc>>,
    // TODO - location
    // TODO - Twitch
    // TODO - Hitbox
    // TODO - YouTube
    // TODO - Twitter
    // TODO - SpeedrunsLive
    // TODO - Assets
    // TODO - Links
}

impl traits::Cacheable for User {
    fn key(&self) -> String {
        self.id.to_string()
    }
}

#[derive(Deserialize)]
pub struct UserResponse {
    pub data: User,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_null_signup_deserializes() {
        let user: User = serde_json::from_value(serde_json::json!({
            "id": "user1",
            "names": {
                "international": "LegacyRunner",
                "japanese": null
            },
            "pronouns": null,
            "weblink": "https://www.speedrun.com/users/LegacyRunner",
            "role": "user",
            "signup": null
        }))
        .expect("user with null signup should deserialize");
        assert_eq!(user.signup, None);
    }
}
