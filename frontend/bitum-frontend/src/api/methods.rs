use gloo_net::http::Request;
use web_sys::{Blob, File, FormData};

use super::*;

const API_PREFIX: &str = "/api";

fn endpoint(method: &str) -> String {
    format!("{}{}", API_PREFIX, method)
}

#[derive(Debug)]
pub struct ApiCallError {
    pub message: String,
}

pub async fn get_current_user_info() -> Result<Response<User>, ApiCallError> {
    let response: Response<User> = Request::get(&endpoint("/user"))
        .credentials(web_sys::RequestCredentials::Include)
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

pub async fn new_user(new_user_request: NewUserRequest) -> Result<Response<User>, ApiCallError> {
    let response: Response<User> = Request::post(&endpoint("/user"))
        .credentials(web_sys::RequestCredentials::Include)
        .json(&new_user_request)
        .map_err(|e| ApiCallError {
            message: e.to_string(),
        })?
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

pub async fn get_user_token(
    token_request: GetTokenRequest,
) -> Result<Response<GetTokenResponseData>, ApiCallError> {
    let response: Response<GetTokenResponseData> = Request::post(&endpoint("/user/token"))
        .credentials(web_sys::RequestCredentials::Include)
        .json(&token_request)
        .map_err(|e| ApiCallError {
            message: e.to_string(),
        })?
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

pub async fn new_chat(new_chat_requests: NewChatRequest) -> Result<Response<Chat>, ApiCallError> {
    let response: Response<Chat> = Request::post(&endpoint("/chat"))
        .credentials(web_sys::RequestCredentials::Include)
        .json(&new_chat_requests)
        .map_err(|e| ApiCallError {
            message: e.to_string(),
        })?
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

pub async fn get_chats() -> Result<Response<Vec<Chat>>, ApiCallError> {
    let response: Response<Vec<Chat>> = Request::get(&endpoint("/chats"))
        .credentials(web_sys::RequestCredentials::Include)
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

pub async fn get_chat(chat_id: i32) -> Result<Response<Chat>, ApiCallError> {
    let response: Response<Chat> = Request::get(&endpoint(&format!("/chat/{}", chat_id)))
        .credentials(web_sys::RequestCredentials::Include)
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

pub async fn upload_file(file: File) -> Result<Response<Vec<String>>, ApiCallError> {
    let form_data = FormData::new().unwrap();

    form_data
        .append_with_blob("file", &Blob::from(file.clone()))
        .map_err(|e| ApiCallError {
            message: format!("{:?}", e),
        })?;

    let response: Response<Vec<String>> = Request::post(&endpoint("/files/"))
        .credentials(web_sys::RequestCredentials::Include)
        .body(form_data)
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

pub async fn get_messages(
    chat_id: i32,
    limit: i32,
    offset: i32,
) -> Result<Response<Vec<ChatMessage>>, ApiCallError> {
    let response: Response<Vec<ChatMessage>> =
        Request::get(&endpoint(&format!("/chat/{}/messages", chat_id)))
            .credentials(web_sys::RequestCredentials::Include)
            .query([
                ("limit", format!("{}", limit)),
                ("offset", format!("{}", offset)),
            ])
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
