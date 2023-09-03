#[macro_use]
extern crate rocket;
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;

use lazy_static::lazy_static;
mod response_handler;

use response_handler::handle_response;

lazy_static! {
    static ref ORG_REPO_TOKEN_MAPPING: HashMap<String, String> = get_org_repo_token_mapping();
    static ref HTTP_CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();
}

fn get_org_repo_token_mapping() -> HashMap<String, String> {
    let mut org_repo_token_mapping = HashMap::new();

    let env_vars = std::env::vars();

    for (key, value) in env_vars {
        if key.starts_with("PAT_") {
            let org_name: String = key.trim_start_matches("PAT_").to_string();
            org_repo_token_mapping.insert(org_name, value);
        }
    }
    org_repo_token_mapping
}

fn send_request(api_url: &str, token: &str) -> String {
    let mut headers = HeaderMap::new();
    let auth_value = HeaderValue::from_str(&format!("Bearer {}", token)).unwrap();
    headers.insert("Authorization", auth_value);
    headers.insert("User-Agent", "GitHub Runner KMS".parse().unwrap());

    let response = HTTP_CLIENT.post(api_url).headers(headers).send();

    match response {
        Ok(response) => handle_response(response),
        Err(err) => {
            eprintln!("HTTP request failed with error: {:?}", err);
            "Error".to_string()
        }
    }
}

#[get("/<org_name>/registration-token")]
async fn get_registration_token(org_name: String) -> String {
    let github_api_url = format!(
        "https://api.github.com/orgs/{}/actions/runners/registration-token",
        org_name
    );

    let token = ORG_REPO_TOKEN_MAPPING
        .get(&org_name)
        .map(|s| s.as_str())
        .unwrap_or_else(|| "");
    if token.is_empty() {
        return format!("No token found for org: {}", org_name);
    }

    send_request(&github_api_url, token)
}

#[get("/<org_name>/remove-token")]
async fn get_remove_token(org_name: String) -> String {
    let github_api_url = format!(
        "https://api.github.com/orgs/{}/actions/runners/remove-token",
        org_name
    );

    let token = ORG_REPO_TOKEN_MAPPING
        .get(&org_name)
        .map(|s| s.as_str())
        .unwrap_or_else(|| "");
    if token.is_empty() {
        return format!("No token found for org: {}", org_name);
    }

    send_request(&github_api_url, token)
}

#[get("/repo/<github_repo_owner>/<github_repo_name>/registration-token")]
async fn get_repo_registration_token(
    github_repo_owner: String,
    github_repo_name: String,
) -> String {
    let github_api_url = format!(
        "https://api.github.com/repos/{}/{}/actions/runners/registration-token",
        github_repo_owner, github_repo_name
    );

    let full_github_repo_name = format!("{}/{}", github_repo_owner, github_repo_name);

    let token = ORG_REPO_TOKEN_MAPPING
        .get(&full_github_repo_name)
        .map(|s| s.as_str())
        .unwrap_or_else(|| "");

    send_request(&github_api_url, token)
}

#[get("/repo/<github_repo_owner>/<github_repo_name>/remove-token")]
async fn get_repo_remove_token(github_repo_owner: String, github_repo_name: String) -> String {
    let github_api_url = format!(
        "https://api.github.com/repos/{}/{}/actions/runners/remove-token",
        github_repo_owner, github_repo_name
    );

    let full_github_repo_name = format!("{}/{}", github_repo_owner, github_repo_name);

    let token = ORG_REPO_TOKEN_MAPPING
        .get(&full_github_repo_name)
        .map(|s| s.as_str())
        .unwrap_or_else(|| "");

    if token.is_empty() {
        return format!(
            "No token found for repo: {}/{}",
            github_repo_owner, github_repo_name
        );
    }

    send_request(&github_api_url, token)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount(
            "/",
            routes![
                get_registration_token,
                get_remove_token,
                get_repo_registration_token,
                get_repo_remove_token
            ],
        )
        .launch()
        .await?;
    Ok(())
}
