use std::{env, io};

use oauth2::{basic::BasicClient, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl, TokenResponse};

use oauth2::reqwest::{async_http_client};

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

async fn get_user_code() -> String {
    println!("Enter the authorization code:");
    let mut code = String::new();
    io::stdin().read_line(&mut code).expect("Failed to read line");
    code.trim().to_string()
}

pub async fn setup_drive_upload() -> String{
	let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID is not set");
	let client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET is not set");

	let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/auth".to_string()).unwrap();
	let token_url = TokenUrl::new("https://accounts.google.com/o/oauth2/token".to_string()).unwrap();

	let client = BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new("http://localhost:8080".to_string()).unwrap());

	let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("https://www.googleapis.com/auth/drive.file".to_string())) // Add each scope separately
        .url();


    println!("Go to the following URL to authorize the app: \n{}", auth_url);

    // The user needs to visit the URL, get the code, and enter it
    let code = get_user_code().await;  // You need to implement this
	let auth_code = AuthorizationCode::new(code);

    // Exchange the authorization code for an access token
     let token_response = client
        .exchange_code(auth_code)
        .request_async(async_http_client)
        .await
        .expect("Token exchange failed");

    println!("Access Token: {}", token_response.access_token().secret());
	return token_response.access_token().secret().to_string();
}

pub async fn upload_file_to_drive(access_token: &str, local_path: &str, file_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let metadata = serde_json::json!({
        "name": file_name,
    });

    let file_content = std::fs::read(local_path)?;

    let boundary = "foo_bar_baz";

    let multipart_body = format!(
        "--{boundary}\r\n\
        Content-Type: application/json; charset=UTF-8\r\n\r\n\
        {metadata}\r\n\
        --{boundary}\r\n\
        Content-Type: application/octet-stream\r\n\r\n",
        boundary = boundary,
        metadata = metadata.to_string()
    );

    let mut body = multipart_body.into_bytes();
    body.extend_from_slice(&file_content);
    body.extend_from_slice(format!("\r\n--{}--", boundary).as_bytes());

    let response = client
        .post("https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart")
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
        .header(CONTENT_TYPE, format!("multipart/related; boundary={}", boundary))
        .body(body)
        .send().await.unwrap();

    if response.status().is_success() {
        println!("File uploaded successfully!");
        Ok(())
    } else {
		let status = response.status();
        let text = response.text().await.unwrap();
        Err(format!("Failed to upload file: {}\n{}", status, text).into())
    }
}
