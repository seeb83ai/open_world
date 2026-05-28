use axum::{
    async_trait,
    extract::FromRequestParts,
    http::request::Parts,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, AppResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user UUID
    pub exp: i64,    // expiration time
}

pub struct JwtAuth {
    secret: String,
}

impl JwtAuth {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn encode(&self, user_id: Uuid, expiration_hours: u64) -> AppResult<String> {
        let now = chrono::Utc::now().timestamp();
        let exp = now + (expiration_hours as i64 * 3600);

        let claims = Claims {
            sub: user_id.to_string(),
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|_| AppError::InternalError("Failed to encode JWT".to_string()))
    }

    pub fn decode(&self, token: &str) -> AppResult<Claims> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|_| AppError::Unauthorized)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // TODO: Extract and validate JWT from Authorization header
        Err(AppError::Unauthorized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_auth_creation() {
        let secret = "test-secret".to_string();
        let auth = JwtAuth::new(secret.clone());
        assert_eq!(auth.secret, secret);
    }

    #[test]
    fn test_jwt_encode_and_decode() {
        let secret = "test-secret-key".to_string();
        let auth = JwtAuth::new(secret);
        let user_id = Uuid::new_v4();

        // Encode token
        let token = auth.encode(user_id, 24).expect("Failed to encode");
        assert!(!token.is_empty());

        // Decode token
        let claims = auth.decode(&token).expect("Failed to decode");
        assert_eq!(claims.sub, user_id.to_string());
    }

    #[test]
    fn test_jwt_invalid_token() {
        let secret = "test-secret-key".to_string();
        let auth = JwtAuth::new(secret);

        let result = auth.decode("invalid-token");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Unauthorized));
    }

    #[test]
    fn test_jwt_wrong_secret() {
        let secret1 = "secret1".to_string();
        let secret2 = "secret2".to_string();
        let auth1 = JwtAuth::new(secret1);
        let auth2 = JwtAuth::new(secret2);
        let user_id = Uuid::new_v4();

        // Encode with secret1
        let token = auth1.encode(user_id, 24).expect("Failed to encode");

        // Try to decode with secret2
        let result = auth2.decode(&token);
        assert!(result.is_err());
    }

    #[test]
    fn test_claims_structure() {
        let user_id = Uuid::new_v4();
        let exp = chrono::Utc::now().timestamp() + 3600;

        let claims = Claims {
            sub: user_id.to_string(),
            exp,
        };

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.exp, exp);
    }
}
