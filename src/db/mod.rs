mod record;
mod artist;

pub use record::{CreateRecord, RecordRepository};
pub use artist::{CreateArtist, ArtistRepository};

use sqlx::{Error, sqlite::SqlitePool};

#[derive(Debug, Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, Error> {
        // Extract the file path from the database URL and ensure directory exists
        if let Some(db_path) = database_url.strip_prefix("sqlite:") {
            let path_without_query = db_path.split('?').next().unwrap_or(db_path);
            if let Some(parent) = std::path::Path::new(path_without_query).parent() {
                if !parent.as_os_str().is_empty() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        Error::Io(std::io::Error::other(format!(
                            "Failed to create database directory: {}",
                            e
                        )))
                    })?;
                }
            }
        }

        // Connect with create_if_missing option
        let connection_url = if database_url.contains('?') {
            format!("{}&create_if_missing=true", database_url)
        } else {
            format!("{}?mode=rwc", database_url)
        };

        let pool = SqlitePool::connect(&connection_url).await?;

        // Create tables
        record::create_table(&pool).await?;
        artist::create_table(&pool).await?;

        Ok(Database { pool })
    }

    pub fn record_repo(&self) -> RecordRepository {
        RecordRepository::new(self.pool.clone())
    }

    pub fn artist_repo(&self) -> ArtistRepository {
        ArtistRepository::new(self.pool.clone())
    }
}
