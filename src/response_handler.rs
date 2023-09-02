use reqwest::blocking::Response;
use serde::Deserialize;

#[derive(Deserialize)]
struct GithubRegistrationTokenResponse {
    token: String,
}

pub fn handle_response(response: Response) -> String {
    if response.status().is_success() {
        match response.json::<GithubRegistrationTokenResponse>() {
            Ok(token_response) => token_response.token,
            Err(err) => {
                eprintln!("Failed to parse JSON response: {:?}", err);
                "Error".to_string()
            }
        }
    } else {
        eprintln!("HTTP request failed with status: {:?}", response.status());
        eprintln!("HTTP request failed with body: {:?}", response.text());
        "Error".to_string()
    }
}
