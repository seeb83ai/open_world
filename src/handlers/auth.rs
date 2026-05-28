use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use serde_json::json;

use crate::{
    error::{AppError, AppResult},
    models::{RegisterRequest, LoginRequest, AuthResponse},
};

pub async fn register(
    State(_pool): State<SqlitePool>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<(StatusCode, Json<serde_json::Value>)> {
    // TODO: Implement user registration
    tracing::info!("Register request for: {}", req.email);

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "status": "ok",
            "message": "User registered successfully"
        })),
    ))
}

pub async fn login(
    State(_pool): State<SqlitePool>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<serde_json::Value>> {
    // TODO: Implement user login
    tracing::info!("Login request for: {}", req.email);

    Ok(Json(json!({
        "status": "ok",
        "data": {
            "token": "placeholder_token",
            "user": {
                "id": "placeholder_id",
                "email": req.email,
                "name": "placeholder_name"
            }
        }
    })))
}
