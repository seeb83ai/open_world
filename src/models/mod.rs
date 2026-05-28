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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User {
            id: Uuid::new_v4(),
            email: "test@example.com".to_string(),
            name: "Test User".to_string(),
            password_hash: "hash".to_string(),
            registered_at: Utc::now(),
            last_login_at: None,
        };

        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.name, "Test User");
        assert!(user.last_login_at.is_none());
    }

    #[test]
    fn test_area_creation() {
        let area = Area {
            id: Uuid::new_v4(),
            short_description: "A small house".to_string(),
            full_description: Some("A quaint cottage in the woods".to_string()),
            state: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(area.short_description, "A small house");
        assert!(area.full_description.is_some());
    }

    #[test]
    fn test_item_with_quality_and_durability() {
        let item = Item {
            id: Uuid::new_v4(),
            name: "Rusty Key".to_string(),
            description: "A worn key with rust".to_string(),
            quality: 30,
            durability: 45,
            capacity_modifier: 0,
            area_id: Some(Uuid::new_v4()),
            owner_id: None,
        };

        assert_eq!(item.quality, 30);
        assert_eq!(item.durability, 45);
        assert!(item.area_id.is_some());
        assert!(item.owner_id.is_none());
    }

    #[test]
    fn test_item_in_inventory() {
        let owner_id = Uuid::new_v4();
        let item = Item {
            id: Uuid::new_v4(),
            name: "Bag".to_string(),
            description: "A canvas bag".to_string(),
            quality: 100,
            durability: 80,
            capacity_modifier: 5,
            area_id: None,
            owner_id: Some(owner_id),
        };

        assert!(item.owner_id.is_some());
        assert!(item.area_id.is_none());
        assert_eq!(item.capacity_modifier, 5);
    }

    #[test]
    fn test_connection_between_areas() {
        let from_id = Uuid::new_v4();
        let to_id = Uuid::new_v4();

        let conn = Connection {
            from_area_id: from_id,
            to_area_id: to_id,
            description: "A narrow path through the woods".to_string(),
        };

        assert_eq!(conn.from_area_id, from_id);
        assert_eq!(conn.to_area_id, to_id);
        assert!(conn.description.contains("path"));
    }

    #[test]
    fn test_register_request_validation() {
        let req = RegisterRequest {
            email: "user@example.com".to_string(),
            name: "User Name".to_string(),
            password: "secure_password".to_string(),
        };

        assert!(!req.email.is_empty());
        assert!(!req.name.is_empty());
        assert!(!req.password.is_empty());
    }

    #[test]
    fn test_event_log_creation() {
        let user_id = Uuid::new_v4();
        let area_id = Uuid::new_v4();

        let event = EventLog {
            id: 1,
            user_id,
            area_id,
            timestamp: Utc::now(),
            action_description: "collected rusty key".to_string(),
            outcome: Some(serde_json::json!({"success": true})),
        };

        assert_eq!(event.id, 1);
        assert_eq!(event.user_id, user_id);
        assert_eq!(event.area_id, area_id);
        assert!(event.outcome.is_some());
    }
}
