use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub registered_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Area {
    pub id: Uuid,
    pub short_description: String,
    pub full_description: Option<String>,
    pub state: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub quality: i32,
    pub durability: i32,
    pub capacity_modifier: i32,
    pub area_id: Option<Uuid>,
    pub owner_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from_area_id: Uuid,
    pub to_area_id: Uuid,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventLog {
    pub id: i64,
    pub user_id: Uuid,
    pub area_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub action_description: String,
    pub outcome: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub status: String,
    pub data: AuthData,
}

#[derive(Debug, Serialize)]
pub struct AuthData {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
}
