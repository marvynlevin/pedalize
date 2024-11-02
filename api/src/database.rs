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
    pub port: u16,
}

impl DatabaseConfig {
    fn read() -> Result<Self, Box<dyn Error>> {
        let c = fs::read_to_string("database_config.json")?;
        serde_json::from_str(&c).map_err(|e| e.into())
    }
}

#[derive(Clone)]
pub(crate) struct Database {
    pub(crate) pool: Arc<RwLock<MySqlPool>>,
}

impl Database {
    pub(crate) async fn init() -> Self {
        // Lire la configuration de la base de données
        let config = DatabaseConfig::read().unwrap_or_else(|e| {
            error!(target: "Database", "Cannot read the database config: {e:#?}");
            panic!("Unable to load the configuration of the database");
        });

        // Initialiser la connexion à la base de données
        let pool = MySqlPool::connect_with(
            MySqlConnectOptions::new()
                .database(&config.database)
                .username(&config.username)
                .password(&config.password)
                .host(&config.host)
                .port(config.port)
                .charset("utf8"),
        ).await.unwrap_or_else(|e| {
            error!(target: "Database", "Cannot connect to the database: {e:#?}");
            panic!("Unable to connect to the database");
        });

        Self {
            pool: Arc::new(RwLock::new(pool)),
        }
    }

    pub(crate) async fn get_pool(&self) -> RwLockReadGuard<'_, MySqlPool> {
        self.pool.read().await
    }
}
