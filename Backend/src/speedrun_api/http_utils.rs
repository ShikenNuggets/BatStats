use dashmap::DashSet;
use once_cell::sync::Lazy;
use reqwest::{self, Client, StatusCode, header::HeaderMap};
use std::{collections::HashMap, error::Error, time::Duration};
use url::Url;

const USER_AGENT: &str = "BatStats/1.0";
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_RETRIES: u32 = 5;
const MAX_RETRY_DELAY: Duration = Duration::from_secs(60);
const POLITE_DELAY: Duration = Duration::from_millis(101);

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| {
    Client::builder()
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .user_agent(USER_AGENT)
        .build()
        .expect("failed to build shared HTTP client")
});

static URL_REQUESTS: Lazy<DashSet<Url>> = Lazy::new(DashSet::new);

pub fn http_client() -> &'static Client {
    &HTTP_CLIENT
}

#[derive(Clone, Copy)]
struct RetryPolicy {
    polite_delay: Duration,
    max_retries: u32,
    max_retry_delay: Duration,
}

const DEFAULT_RETRY_POLICY: RetryPolicy = RetryPolicy {
    polite_delay: POLITE_DELAY,
    max_retries: MAX_RETRIES,
    max_retry_delay: MAX_RETRY_DELAY,
};

fn parse_retry_after(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get(reqwest::header::RETRY_AFTER)?.to_str().ok()?;
    let secs = value.trim().parse::<u64>().ok()?;
    Some(Duration::from_secs(secs))
}

fn is_retryable_status(status: StatusCode) -> bool {
    status.as_u16() == 420 || status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error()
}

fn calculate_backoff(
    attempt: u32,
    status: Option<StatusCode>,
    retry_after: Option<Duration>,
    max_delay: Duration,
) -> Duration {
    let raw = match status {
        Some(s) if s.as_u16() == 420 || s == StatusCode::TOO_MANY_REQUESTS => {
            if let Some(delay) = retry_after {
                delay
            } else {
                Duration::from_secs(5 + (5 * u64::from(attempt)))
            }
        }
        _ => Duration::from_secs(2u64.saturating_pow(attempt)),
    };

    raw.min(max_delay)
}

async fn fetch_url_with_policy(url: Url, policy: RetryPolicy) -> Result<String, Box<dyn Error>> {
    if !URL_REQUESTS.insert(url.clone()) {
        println!("Duplicate request \"{}\" detected!", url);
    }

    let client = http_client();
    let mut attempt: u32 = 0;

    loop {
        if !policy.polite_delay.is_zero() {
            tokio::time::sleep(policy.polite_delay).await;
        }

        match client.get(url.clone()).send().await {
            Ok(resp) => {
                let status = resp.status();
                if is_retryable_status(status) {
                    if attempt >= policy.max_retries {
                        return Err(format!(
                            "Request to {} failed after {} retries (last status: {})",
                            url, policy.max_retries, status
                        )
                        .into());
                    }

                    let retry_after = parse_retry_after(resp.headers());
                    let delay = calculate_backoff(
                        attempt,
                        Some(status),
                        retry_after,
                        policy.max_retry_delay,
                    );
                    println!(
                        "Received HTTP {}. Retrying in {}ms (attempt {}/{})...",
                        status.as_u16(),
                        delay.as_millis(),
                        attempt + 1,
                        policy.max_retries
                    );
                    attempt += 1;
                    tokio::time::sleep(delay).await;
                    continue;
                }

                let resp = resp.error_for_status()?;
                match resp.text().await {
                    Ok(body) => return Ok(body),
                    Err(err) if attempt < policy.max_retries => {
                        let delay = calculate_backoff(attempt, None, None, policy.max_retry_delay);
                        attempt += 1;
                        println!(
                            "Error reading response body from {}: {}. Retrying in {}ms...",
                            url,
                            err,
                            delay.as_millis()
                        );
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    Err(err) => return Err(err.into()),
                }
            }
            Err(err) => {
                if attempt >= policy.max_retries {
                    return Err(format!(
                        "HTTP request to {} failed after {} retries: {}",
                        url, policy.max_retries, err
                    )
                    .into());
                }

                let delay = calculate_backoff(attempt, None, None, policy.max_retry_delay);
                println!(
                    "HTTP transport error for {}: {}. Retrying in {}ms (attempt {}/{})...",
                    url,
                    err,
                    delay.as_millis(),
                    attempt + 1,
                    policy.max_retries
                );
                attempt += 1;
                tokio::time::sleep(delay).await;
            }
        }
    }
}

async fn get_http_result_internal(url: Url) -> Result<String, Box<dyn Error>> {
    fetch_url_with_policy(url, DEFAULT_RETRY_POLICY).await
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

    #[test]
    fn test_parse_retry_after() {
        let mut headers = HeaderMap::new();
        assert_eq!(parse_retry_after(&headers), None);

        headers.insert(reqwest::header::RETRY_AFTER, "15".parse().unwrap());
        assert_eq!(parse_retry_after(&headers), Some(Duration::from_secs(15)));

        headers.insert(reqwest::header::RETRY_AFTER, " 7 ".parse().unwrap());
        assert_eq!(parse_retry_after(&headers), Some(Duration::from_secs(7)));

        headers.insert(
            reqwest::header::RETRY_AFTER,
            "not-a-number".parse().unwrap(),
        );
        assert_eq!(parse_retry_after(&headers), None);
    }

    #[test]
    fn test_calculate_backoff_rate_limit_and_transient() {
        let max = Duration::from_secs(60);
        let status_420 = StatusCode::from_u16(420).unwrap();

        assert_eq!(
            calculate_backoff(0, Some(status_420), None, max),
            Duration::from_secs(5)
        );
        assert_eq!(
            calculate_backoff(2, Some(status_420), None, max),
            Duration::from_secs(15)
        );
        assert_eq!(
            calculate_backoff(
                0,
                Some(StatusCode::TOO_MANY_REQUESTS),
                Some(Duration::from_secs(12)),
                max
            ),
            Duration::from_secs(12)
        );
        assert_eq!(
            calculate_backoff(
                0,
                Some(StatusCode::TOO_MANY_REQUESTS),
                Some(Duration::from_secs(300)),
                max
            ),
            Duration::from_secs(60)
        );
        assert_eq!(
            calculate_backoff(0, Some(StatusCode::SERVICE_UNAVAILABLE), None, max),
            Duration::from_secs(1)
        );
        assert_eq!(
            calculate_backoff(3, None, None, max),
            Duration::from_secs(8)
        );
    }

    #[tokio::test]
    async fn test_retry_cap_terminates_and_transient_retries_succeed() {
        use std::sync::{
            Arc,
            atomic::{AtomicUsize, Ordering},
        };
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        use tokio::net::TcpListener;

        async fn spawn_sequence_server(
            responses: Vec<&'static str>,
        ) -> (Url, Arc<AtomicUsize>, tokio::task::JoinHandle<()>) {
            let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
            let addr = listener.local_addr().expect("addr");
            let hits = Arc::new(AtomicUsize::new(0));
            let hits_clone = hits.clone();

            let handle = tokio::spawn(async move {
                for resp in responses {
                    if let Ok((mut stream, _)) = listener.accept().await {
                        hits_clone.fetch_add(1, Ordering::SeqCst);
                        let mut buf = [0u8; 4096];
                        let _ = stream.read(&mut buf).await;
                        let _ = stream.write_all(resp.as_bytes()).await;
                    }
                }
            });

            (
                Url::parse(&format!("http://{addr}/test")).expect("valid url"),
                hits,
                handle,
            )
        }

        let fast_policy = RetryPolicy {
            polite_delay: Duration::ZERO,
            max_retries: 5,
            max_retry_delay: Duration::from_millis(1),
        };

        // Always 420: should make 1 initial attempt + 5 retries = 6 requests, then return Err
        let resp_420 =
            "HTTP/1.1 420 Rate Limited\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let (url_420, hits_420, h_420) = spawn_sequence_server(vec![resp_420; 6]).await;
        let err = fetch_url_with_policy(url_420, fast_policy)
            .await
            .expect_err("persistent 420 must fail at retry cap");
        h_420.await.expect("server done");
        assert_eq!(hits_420.load(Ordering::SeqCst), 6);
        assert!(err.to_string().contains("5 retries"));

        // Transient 429 + 503 then 200: should retry and succeed on attempt 3
        let resp_429 = "HTTP/1.1 429 Too Many Requests\r\nRetry-After: 1\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let resp_503 =
            "HTTP/1.1 503 Service Unavailable\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let resp_200 = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok";
        let (url_ok, hits_ok, h_ok) =
            spawn_sequence_server(vec![resp_429, resp_503, resp_200]).await;
        let body = fetch_url_with_policy(url_ok, fast_policy)
            .await
            .expect("should recover on 200");
        h_ok.await.expect("server done");
        assert_eq!(hits_ok.load(Ordering::SeqCst), 3);
        assert_eq!(body, "ok");

        // Non-retryable 404: fails on first attempt without retrying
        let resp_404 = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        let (url_404, hits_404, h_404) = spawn_sequence_server(vec![resp_404]).await;
        assert!(fetch_url_with_policy(url_404, fast_policy).await.is_err());
        h_404.await.expect("server done");
        assert_eq!(hits_404.load(Ordering::SeqCst), 1);
    }
}
