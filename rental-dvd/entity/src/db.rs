use std::time::Duration;

use derive_more::Deref;
use figment::{
    providers::Env,
    value::{Dict, Map},
    Figment, Metadata, Profile, Provider,
};
use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::info;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DBConfig {
    pub url: String,
    pub min_connections: Option<u32>,
    pub max_connections: usize,
    pub connect_timeout: u64,
    pub idle_timeout: Option<u64>,
}

impl DBConfig {
    pub fn figment() -> Figment {
        Figment::from(DBConfig::default())
            .merge(Env::prefixed("aaa").split("_"))
            .select(Profile::from_env_or("aaaaaa", Profile::const_new("debug")))
    }
}

impl Default for DBConfig {
    fn default() -> Self {
        Self {
            url: dotenvy::var("DATABASE_URL").expect("DATABASE_URL not found.."),
            min_connections: Some(1),
            max_connections: 10,
            connect_timeout: 5,
            idle_timeout: Some(5),
        }
    }
}

impl Provider for DBConfig {
    fn metadata(&self) -> Metadata {
        Metadata::named("Rental DVD Database Config")
    }
    fn data(&self) -> Result<Map<Profile, Dict>, figment::Error> {
        figment::providers::Serialized::defaults(DBConfig::default()).data()
    }
    fn profile(&self) -> Option<Profile> {
        None
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("FigmentErr {0}")]
    Figment(#[from] figment::Error),
    #[error("SeaOrmDbErr {0}")]
    SeaOrmDb(#[from] DbErr),
    #[error("MaxConnectionsOverflowErr: {0}")]
    MaxConnectionsOverflow(#[from] std::num::TryFromIntError),
}

#[derive(Clone, Debug, Deref)]
pub struct DB {
    conn: DatabaseConnection,
}

impl DB {
    pub async fn connect(config: &DBConfig) -> Result<Self, Error> {
        let mut options: ConnectOptions = config.url.clone().into();

        options
            .max_connections(config.max_connections.try_into()?)
            .min_connections(config.min_connections.unwrap_or_default())
            .connect_timeout(Duration::from_secs(config.connect_timeout));

        info!("Connecting to database: {:?}", config);
        if let Some(idle_timeout) = config.idle_timeout {
            options.idle_timeout(Duration::from_secs(idle_timeout));
        }
        let conn = Database::connect(options).await?;

        Ok(Self { conn })
    }
    // pub async fn run_migrations(&self) -> std::result::Result<(), Error> {
    //     migration::Migrator::up(self.conn(), None).await?;
    //     Ok(())
    // }
    pub fn conn(&self) -> &DatabaseConnection {
        &self.conn
    }
}
