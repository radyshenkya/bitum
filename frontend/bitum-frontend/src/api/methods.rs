use gloo_net::http::Request;

use super::*;

const API_PREFIX: &str = "http://0.0.0.0:8000/api";

fn endpoint(method: &str) -> String {
    format!("{}{}", API_PREFIX, method)
}

#[derive(Debug)]
pub struct ApiCallError {
    pub message: String,
}

pub async fn get_current_user_info() -> Result<Response<User>, ApiCallError> {
    let response: Response<User> = Request::get(&endpoint("/user"))
        .send()
        .await
        .map_err(|e| ApiCallError {
            message: e.to_string(),
        })?
        .json()
        .await
        .map_err(|e| ApiCallError {
            message: e.to_string(),
        })?;

    Ok(response)
}
