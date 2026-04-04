//! Authentication handlers

use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{auth, error::ApiError};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: String,
    pub user_name: String,
    pub expires_in: u64,
}

/// Login endpoint - generates JWT token
///
/// In production, this should validate credentials against a database
/// For now, accepts any username/password for development
pub async fn login(
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    // TODO: Validate credentials against database
    // For now, accept any non-empty credentials for development
    if req.username.is_empty() || req.password.is_empty() {
        return Err(ApiError::Unauthorized("Invalid credentials".to_string()));
    }

    // Generate JWT token
    let config = auth::AuthConfig::from_env();
    let token = auth::generate_token(&req.username, &req.username, &config.jwt_secret)
        .map_err(|e| ApiError::Internal(format!("Failed to generate token: {}", e)))?;

    Ok(Json(LoginResponse {
        token,
        user_id: req.username.clone(),
        user_name: req.username,
        expires_in: 86400, // 24 hours
    }))
}

/// Get current user info (requires authentication)
pub async fn me(
    auth_user: auth::AuthUser,
) -> Result<Json<Value>, ApiError> {
    Ok(Json(serde_json::json!({
        "id": auth_user.id,
        "name": auth_user.name,
        "auth_type": match auth_user.auth_type {
            auth::AuthType::Jwt => "jwt",
            auth::AuthType::ApiKey => "api_key",
        }
    })))
}
