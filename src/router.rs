use crate::send_request;
use crate::ORG_REPO_TOKEN_MAPPING;

#[get("/<org_name>/registration-token")]
pub async fn get_registration_token(org_name: String) -> String {
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
pub async fn get_remove_token(org_name: String) -> String {
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
pub async fn get_repo_registration_token(
    github_repo_owner: String,
    github_repo_name: String,
) -> String {
    let github_api_url = format!(
        "https://api.github.com/repos/{}/{}/actions/runners/registration-token",
        github_repo_owner, github_repo_name
    );

    let token = ORG_REPO_TOKEN_MAPPING
        .get(&github_repo_owner)
        .map(|s| s.as_str())
        .unwrap_or_else(|| "");

    send_request(&github_api_url, token)
}

#[get("/repo/<github_repo_owner>/<github_repo_name>/remove-token")]
pub async fn get_repo_remove_token(github_repo_owner: String, github_repo_name: String) -> String {
    let github_api_url = format!(
        "https://api.github.com/repos/{}/{}/actions/runners/remove-token",
        github_repo_owner, github_repo_name
    );

    let token = ORG_REPO_TOKEN_MAPPING
        .get(&github_repo_owner)
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
