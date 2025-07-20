use reqwest;
use url::Url;
use std::{collections::HashMap, error::Error};

pub async fn get_http_result(url: Url) -> Result<(), Box<dyn Error>>{
	let response = reqwest::get(url).await?;
	let body = response.text().await?;

	println!("Response Body: \n{}", body);
	Ok(())
}

pub fn parse_to_url(base_url: &str, args: HashMap<&str, &str>) -> Url{
	let mut url = Url::parse(&base_url).unwrap();
	url.query_pairs_mut().extend_pairs(args.iter());
	return url;
}

pub async fn get_http_result_with_args(base_url: &str, args: HashMap<&str, &str>) -> Result<(), Box<dyn Error>>{
	let final_url = parse_to_url(base_url, args);
	get_http_result(final_url).await?;
	Ok(())
}
