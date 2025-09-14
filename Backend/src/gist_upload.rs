use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::env;

#[derive(Serialize)]
struct GistFile{
	content: String
}

#[derive(Serialize)]
struct GistUpdateRequest{
	files: HashMap<String, GistFile>
}

pub async fn upload_gist(file_path: &str) -> Result<(), Box<dyn std::error::Error>>{
	let github_token = env::var("GITHUB_TOKEN")
		.expect("GITHUB_TOKEN environment variable is needed for gist upload");

	let gist_id = "3adaa36be92dfb82f43b951b91387c1a";
	let gist_file = "BatStats.json";

	let new_content = fs::read_to_string(file_path)?;

	let mut files = HashMap::new();
	files.insert(gist_file.to_string(), GistFile{ content: new_content });
	let payload = GistUpdateRequest{ files };

	let client = Client::new();
	let gist_url = format!("https://api.github.com/gists/{}", gist_id);

	let response = client
		.patch(&gist_url)
		.header("User-Agent", "rust-git-uploader")
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
