#![allow(dead_code)]

use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SrcLinkType {
    #[serde(rename = "self")]
    Self_,

    Game,
    Category,
    Variables,
    Records,
    Runs,
    Leaderboard,

    #[serde(other)]
    Unknown,
}

#[derive(Clone, Deserialize)]
pub struct SrcLink {
    #[serde(rename = "rel")]
    pub link_type: SrcLinkType,

    pub uri: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_src_link_unknown_rel_deserializes() {
        let derived: SrcLink = serde_json::from_str(
            r#"{"rel":"derived-games","uri":"https://www.speedrun.com/api/v1/games/x/derived-games"}"#,
        )
        .expect("unknown link rel should deserialize");
        assert_eq!(derived.link_type, SrcLinkType::Unknown);

        let self_link: SrcLink = serde_json::from_str(
            r#"{"rel":"self","uri":"https://www.speedrun.com/api/v1/games/x"}"#,
        )
        .expect("self link should deserialize");
        assert_eq!(self_link.link_type, SrcLinkType::Self_);
    }
}
