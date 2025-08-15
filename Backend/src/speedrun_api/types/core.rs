use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SrcLinkType{
	#[serde(rename = "self")]
	Self_,

	Game,
	Category,
	Variables,
	Records,
	Runs,
	Leaderboard
}

#[derive(Deserialize)]
pub struct SrcLink{
	#[serde(rename = "rel")]
	link_type : SrcLinkType,

	uri: String
}