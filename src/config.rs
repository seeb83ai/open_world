use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: u64,
    pub log_level: String,
    pub rate_limit_ip: u32,
    pub rate_limit_auth: u32,
    pub initial_capacity: i32,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://./open_world.db".to_string()),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "dev-secret-change-in-production".to_string()),
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(24),
            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "INFO".to_string()),
            rate_limit_ip: env::var("RATE_LIMIT_IP")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100),
            rate_limit_auth: env::var("RATE_LIMIT_AUTH")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(500),
            initial_capacity: env::var("INITIAL_CAPACITY")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let config = Config {
            database_url: "sqlite://./open_world.db".to_string(),
            jwt_secret: "secret".to_string(),
            jwt_expiration_hours: 24,
            log_level: "INFO".to_string(),
            rate_limit_ip: 100,
            rate_limit_auth: 500,
            initial_capacity: 2,
        };

        assert_eq!(config.database_url, "sqlite://./open_world.db");
        assert_eq!(config.jwt_expiration_hours, 24);
        assert_eq!(config.log_level, "INFO");
        assert_eq!(config.rate_limit_ip, 100);
        assert_eq!(config.rate_limit_auth, 500);
        assert_eq!(config.initial_capacity, 2);
    }

    #[test]
    fn test_config_custom_values() {
        let config = Config {
            database_url: "sqlite://./custom.db".to_string(),
            jwt_secret: "custom-secret".to_string(),
            jwt_expiration_hours: 48,
            log_level: "DEBUG".to_string(),
            rate_limit_ip: 200,
            rate_limit_auth: 1000,
            initial_capacity: 5,
        };

        assert_eq!(config.database_url, "sqlite://./custom.db");
        assert_eq!(config.jwt_expiration_hours, 48);
        assert_eq!(config.log_level, "DEBUG");
        assert_eq!(config.rate_limit_ip, 200);
        assert_eq!(config.rate_limit_auth, 1000);
        assert_eq!(config.initial_capacity, 5);
    }
}
