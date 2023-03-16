use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub is_bot: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub id: i32,
    pub name: String,
    pub owner: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMemberPermissions {
    pub can_write: bool,
    pub can_add_members: bool,
    pub can_kick_members: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMember {
    pub id: i32,
    pub user: User,
    pub chat: Chat,
    pub permissions: ChatMemberPermissions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub id: i32,
    pub sender: User,
    pub chat: Chat,
    pub content: String,
    pub files: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum EventType {
    MemberAdded(ChatMember),
    MemberKicked { user: User, chat: Chat },
    NewMessage(ChatMessage),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Response<T> {
    pub ok: bool,
    pub error: Option<ApiError>,
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUserRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTokenRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTokenResponseData {
    pub token: String,
}
