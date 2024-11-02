use std::error::Error;
use std::fs;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlConnectOptions;
use sqlx::MySqlPool;
use tokio::sync::{RwLock, RwLockReadGuard};
use tracing::error;

#[derive(Serialize, Deserialize)]
struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub database: String,
    pub host: String,
    pub port: u16
}

impl DatabaseConfig {
    fn read() -> Result<Self, Box<dyn Error>> {
        let c = fs::read_to_string("database_config.json")?;
        serde_json::from_str(c.as_str()).map_err(|e| e.into())
    }
}

#[derive(Clone)]
pub(crate) struct Database {
    pub(crate) pool: Arc<RwLock<MySqlPool>>
}

impl Database {
    pub(crate) async fn init() -> Self {
        let config = match DatabaseConfig::read() {
            Ok(c) => c,
            Err(e) => {
                error!(target: "Database", "Cannot read the database config: {e:#?}");
                panic!("Unable to load the configuration of the database");
            }
        };

        let conn = MySqlPool::connect_with(
            MySqlConnectOptions::new()
                .database(config.database.as_str())
                .username(config.username.as_str())
                .charset("utf8")
                .password(config.password.as_str())
                .port(config.port)
                .host(config.host.as_str())
        ).await;

        if let Err(e) = conn {
            error!(target: "Database", "Cannot connect the database: {e:#?}");
            panic!("Unable to connect to the database");
        }

        Self {
            pool: Arc::new(RwLock::new(conn.unwrap()))
        }
    }
    pub(crate) async fn get_pool(&self) -> RwLockReadGuard<'_, MySqlPool> {
        self.pool.read().await
    }
}