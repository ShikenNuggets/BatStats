use dashmap::DashSet;
use once_cell::sync::Lazy;
use reqwest::{self, StatusCode};
use std::{collections::HashMap, error::Error, time::Duration};
use url::Url;

static URL_REQUESTS: Lazy<DashSet<Url>> = Lazy::new(DashSet::new);

async fn get_http_result_internal(url: Url) -> Result<String, Box<dyn Error>> {
    if URL_REQUESTS.contains(&url) {
        println!("Duplicate request \"{}\" detected!", url);
    } else {
        URL_REQUESTS.insert(url.clone());
    }

    let mut retries = 0;
    loop {
        tokio::time::sleep(Duration::from_millis(101)).await;
        let response = reqwest::get(url.clone()).await;
        match response {
            Ok(resp) => {
                if resp.status() == StatusCode::from_u16(420).unwrap() {
                    //if retries >= 5{
                    //	return Err("Too many retries due to rate limiting (HTTP 420)".into());
                    //}

                    let retry_duration = std::cmp::min(5 + (5 * retries), 60);
                    println!("Received 420. Retrying in {} second(s)...", retry_duration);
                    retries += 1;
                    tokio::time::sleep(Duration::from_secs(retry_duration)).await;
                    continue;
                }

                let response = resp.error_for_status()?;
                let body = response.text().await?;
                return Ok(body);
            }
            Err(e) => return Err(e.into()),
        }
    }
}

pub fn parse_to_url(base_url: &str, args: &HashMap<String, String>) -> Result<Url, Box<dyn Error>> {
    let mut url = Url::parse(base_url)?;
    let mut sorted_args: Vec<(&String, &String)> = args.iter().collect();
    sorted_args.sort_by(|a, b| a.0.cmp(b.0).then_with(|| a.1.cmp(b.1)));
    url.query_pairs_mut().extend_pairs(sorted_args);
    Ok(url)
}

pub async fn get_http_result(url: &str) -> Result<String, Box<dyn Error>> {
    let url = Url::parse(url)?;
    let result = get_http_result_internal(url).await?;
    Ok(result)
}

pub async fn get_http_result_with_args(
    base_url: &str,
    args: HashMap<String, String>,
) -> Result<String, Box<dyn Error>> {
    let final_url = parse_to_url(base_url, &args)?;
    let result = get_http_result_internal(final_url).await?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_to_url_sorts_query_parameters_deterministically() {
        let args_a = HashMap::from([
            ("var-zeta".to_string(), "2".to_string()),
            ("var-alpha".to_string(), "1".to_string()),
            ("embed".to_string(), "players".to_string()),
        ]);
        let args_b = HashMap::from([
            ("embed".to_string(), "players".to_string()),
            ("var-alpha".to_string(), "1".to_string()),
            ("var-zeta".to_string(), "2".to_string()),
        ]);

        let expected = "https://www.speedrun.com/api/v1/leaderboards/g/category/c?embed=players&var-alpha=1&var-zeta=2";
        for _ in 0..20 {
            let url_a = parse_to_url(
                "https://www.speedrun.com/api/v1/leaderboards/g/category/c",
                &args_a,
            )
            .unwrap();
            let url_b = parse_to_url(
                "https://www.speedrun.com/api/v1/leaderboards/g/category/c",
                &args_b,
            )
            .unwrap();
            assert_eq!(url_a.as_str(), expected);
            assert_eq!(url_b.as_str(), expected);
        }
    }
}
