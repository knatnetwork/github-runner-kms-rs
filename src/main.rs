#[macro_use]
extern crate rocket;
use reqwest::header::{HeaderMap, HeaderValue};
use std::collections::HashMap;

use lazy_static::lazy_static;
mod response_handler;
mod router;

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
    let auth_value = HeaderValue::from_str(&format!("token {}", token)).unwrap();
    headers.insert("Authorization", auth_value);
    headers.insert("User-Agent", "GitHub Runner KMS".parse().unwrap());

    let response = HTTP_CLIENT.post(api_url).headers(headers).send();

    match response {
        Ok(response) => handle_response(response),
        Err(err) => {
            eprintln!("HTTP request failed with error: {:?}", err);
            String::from("Error")
        }
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount(
            "/",
            routes![
                router::get_registration_token,
                router::get_remove_token,
                router::get_repo_registration_token,
                router::get_repo_remove_token
            ],
        )
        .launch()
        .await?;
    Ok(())
}
