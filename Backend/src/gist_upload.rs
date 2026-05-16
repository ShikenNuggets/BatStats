use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::env;

const GIST_ID: &str = "3adaa36be92dfb82f43b951b91387c1a";
const GIST_FILE: &str = "BatStats.json";
const USER_AGENT: &str = "rust-git-uploader";

#[derive(Serialize)]
struct GistFile{
	content: String
}

#[derive(Serialize)]
struct GistUpdateRequest{
	files: HashMap<String, GistFile>
}

#[derive(Deserialize)]
struct GistResponse{
	id: String,
	// files: HashMap<String, serde_json::Value>, // currently unused
}

pub async fn validate_github_token() -> Result<(), Box<dyn std::error::Error>> {
	let github_token = env::var("GITHUB_TOKEN")
		.expect("GITHUB_TOKEN environment variable is needed");

	if github_token.is_empty(){
		return Err("GITHUB_TOKEN is empty".into());
	}

	let client = Client::new();
	let gist_url = format!("https://api.github.com/gists/{}", GIST_ID);

	let response = client
		.get(&gist_url)
		.header("User-Agent", USER_AGENT)
		.bearer_auth(&github_token)
		.send().await?;

	if !response.status().is_success(){
		return Err(format!("Failed to validate GitHub token: {}", response.status()).into());
	}

	let gist: GistResponse = response.json().await?;
	if gist.id == GIST_ID {
		println!("GitHub token validated successfully.");
		Ok(())
	} else {
		Err("Gist ID mismatch".into())
	}
}

pub async fn upload_gist(file_path: &str) -> Result<(), Box<dyn std::error::Error>>{
	let github_token = env::var("GITHUB_TOKEN")
		.expect("GITHUB_TOKEN environment variable is needed for gist upload");

	if github_token.is_empty(){
		println!("GITHUB_TOKEN is empty, cannot upload the gist!");
		return Err(("Empty GITHUB_TOKEN").into());
	}

	let new_content = fs::read_to_string(file_path)?;

	let mut files = HashMap::new();
	files.insert(GIST_FILE.to_string(), GistFile{ content: new_content });
	let payload = GistUpdateRequest{ files };

	let client = Client::new();
	let gist_url = format!("https://api.github.com/gists/{}", GIST_ID);

	let response = client
		.patch(&gist_url)
		.header("User-Agent", USER_AGENT)
		.bearer_auth(&github_token)
		.json(&payload)
		.send().await?;

	if !response.status().is_success(){
		println!("Failed to update gist: {}", response.status());
	}else{
		println!("Gist updated successfully.");
	}

	Ok(())
}
