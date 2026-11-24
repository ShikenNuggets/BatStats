use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;

const GIST_ID: &str = "3adaa36be92dfb82f43b951b91387c1a";
const GIST_FILE: &str = "BatStats.json";
const USER_AGENT: &str = "rust-git-uploader";

#[derive(Serialize)]
struct GistFile {
    content: String,
}

#[derive(Serialize)]
struct GistUpdateRequest {
    files: HashMap<String, GistFile>,
}

#[derive(Deserialize)]
struct GistResponse {
    id: String,
    // files: HashMap<String, serde_json::Value>, // currently unused
}

async fn error_from_response(response: Response, context: &str) -> Box<dyn std::error::Error> {
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    let snippet: String = body.chars().take(500).collect();
    if snippet.is_empty() {
        format!("{}: {}", context, status).into()
    } else {
        format!("{}: {} - {}", context, status, snippet).into()
    }
}

pub async fn validate_github_token(github_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let gist_url = format!("https://api.github.com/gists/{}", GIST_ID);
    validate_github_token_at_url(github_token, &gist_url).await
}

async fn validate_github_token_at_url(
    github_token: &str,
    gist_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if github_token.trim().is_empty() {
        return Err("GITHUB_TOKEN is empty".into());
    }

    let client = Client::new();
    let response = client
        .get(gist_url)
        .header("User-Agent", USER_AGENT)
        .bearer_auth(github_token)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(error_from_response(response, "Failed to validate GitHub token").await);
    }

    if let Some(scopes_header) = response.headers().get("x-oauth-scopes")
        && let Ok(scopes) = scopes_header.to_str()
    {
        let has_gist_scope = scopes.split(',').any(|s| s.trim() == "gist");
        if !has_gist_scope {
            return Err("GITHUB_TOKEN does not have the required 'gist' scope".into());
        }
    }

    let gist: GistResponse = response.json().await?;
    if gist.id == GIST_ID {
        println!("GitHub token validated successfully.");
        Ok(())
    } else {
        Err("Gist ID mismatch".into())
    }
}

pub async fn upload_gist(
    file_path: &str,
    github_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let gist_url = format!("https://api.github.com/gists/{}", GIST_ID);
    upload_gist_at_url(file_path, github_token, &gist_url).await
}

async fn upload_gist_at_url(
    file_path: &str,
    github_token: &str,
    gist_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if github_token.trim().is_empty() {
        return Err("GITHUB_TOKEN is empty, cannot upload the gist".into());
    }

    let new_content = fs::read_to_string(file_path).await?;

    let mut files = HashMap::new();
    files.insert(
        GIST_FILE.to_string(),
        GistFile {
            content: new_content,
        },
    );
    let payload = GistUpdateRequest { files };

    let client = Client::new();
    let response = client
        .patch(gist_url)
        .header("User-Agent", USER_AGENT)
        .bearer_auth(github_token)
        .json(&payload)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(error_from_response(response, "Failed to update gist").await);
    }

    println!("Gist updated successfully.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    async fn spawn_mock_http_server(status_line: &str, headers: &str, body: &str) -> String {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind mock server");
        let addr = listener.local_addr().expect("local addr");
        let response = format!(
            "HTTP/1.1 {status_line}\r\n{headers}Content-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );

        tokio::spawn(async move {
            if let Ok((mut stream, _)) = listener.accept().await {
                let mut buf = [0u8; 4096];
                let _ = stream.read(&mut buf).await;
                let _ = stream.write_all(response.as_bytes()).await;
            }
        });

        format!("http://{addr}/gists/{GIST_ID}")
    }

    #[tokio::test]
    async fn test_empty_tokens_fail() {
        assert!(validate_github_token("").await.is_err());
        assert!(validate_github_token("   ").await.is_err());
        assert!(upload_gist("BatStats.json", "").await.is_err());
    }

    #[tokio::test]
    async fn test_validate_token_error_and_missing_scope() {
        let err_url = spawn_mock_http_server(
            "401 Unauthorized",
            "Content-Type: application/json\r\n",
            r#"{"message":"Bad credentials"}"#,
        )
        .await;
        let err = validate_github_token_at_url("bad_tok", &err_url)
            .await
            .unwrap_err();
        assert!(err.to_string().contains("401"));
        assert!(err.to_string().contains("Bad credentials"));

        let missing_scope_url = spawn_mock_http_server(
            "200 OK",
            "Content-Type: application/json\r\nX-OAuth-Scopes: repo\r\n",
            &format!(r#"{{"id":"{GIST_ID}"}}"#),
        )
        .await;
        let scope_err = validate_github_token_at_url("no_gist_scope", &missing_scope_url)
            .await
            .unwrap_err();
        assert!(scope_err.to_string().contains("gist"));

        let ok_url = spawn_mock_http_server(
            "200 OK",
            "Content-Type: application/json\r\nX-OAuth-Scopes: repo, gist\r\n",
            &format!(r#"{{"id":"{GIST_ID}"}}"#),
        )
        .await;
        assert!(
            validate_github_token_at_url("valid_tok", &ok_url)
                .await
                .is_ok()
        );
    }

    #[tokio::test]
    async fn test_upload_gist_propagates_non_2xx_and_missing_file() {
        assert!(
            upload_gist("/nonexistent/path/BatStats.json", "tok")
                .await
                .is_err()
        );

        let temp_path =
            std::env::temp_dir().join(format!("batstats_upload_test_{}.json", std::process::id()));
        tokio::fs::write(&temp_path, "{}")
            .await
            .expect("write temp file");
        let temp_str = temp_path.to_str().expect("utf8 path");

        let err_url = spawn_mock_http_server(
            "422 Unprocessable Entity",
            "Content-Type: application/json\r\n",
            r#"{"message":"Invalid gist update"}"#,
        )
        .await;
        let err = upload_gist_at_url(temp_str, "tok", &err_url)
            .await
            .unwrap_err();
        assert!(err.to_string().contains("422"));
        assert!(err.to_string().contains("Invalid gist update"));

        let ok_url = spawn_mock_http_server(
            "200 OK",
            "Content-Type: application/json\r\n",
            &format!(r#"{{"id":"{GIST_ID}"}}"#),
        )
        .await;
        assert!(upload_gist_at_url(temp_str, "tok", &ok_url).await.is_ok());

        let _ = tokio::fs::remove_file(&temp_path).await;
    }
}
