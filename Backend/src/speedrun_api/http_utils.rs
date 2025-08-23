use dashmap::DashSet;
use once_cell::sync::Lazy;
use reqwest;
use url::Url;
use std::{collections::{HashMap}, error::Error};

static URL_REQUESTS: Lazy<DashSet<Url>> = Lazy::new(DashSet::new);

async fn get_http_result_internal(url: Url) -> Result<String, Box<dyn Error>>{
	if URL_REQUESTS.contains(&url){
		println!("Duplicate request \"{}\" detected!", url);
	}else{
		URL_REQUESTS.insert(url.clone());
	}

	let response = reqwest::get(url).await?;
	let response = response.error_for_status()?;
	let body = response.text().await?;
	Ok(body)
}

pub fn parse_to_url(base_url: &str, args: HashMap<String, String>) -> Result<Url, Box<dyn Error>>{
	let mut url = Url::parse(&base_url)?;
	url.query_pairs_mut().extend_pairs(args.iter());
	Ok(url)
}

pub async fn get_http_result(url: &str) -> Result<String, Box<dyn Error>>{
	let url = Url::parse(url)?;
	let result = get_http_result_internal(url).await?;
	Ok(result)
}

pub async fn get_http_result_with_args(base_url: &str, args: HashMap<String, String>) -> Result<String, Box<dyn Error>>{
	let final_url = parse_to_url(base_url, args)?;
	let result = get_http_result_internal(final_url).await?;
	Ok(result)
}
