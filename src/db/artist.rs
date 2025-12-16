use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, sqlite::SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Artist {
    pub id: i64,
    pub name: String,
    pub genre: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateArtist {
    pub name: String,
    pub genre: String,
}

#[derive(Debug, Clone)]
pub struct ArtistRepository {
    pool: SqlitePool,
}

impl ArtistRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, artist: CreateArtist) -> Result<Artist, Error> {
        let created_at = Utc::now().to_rfc3339();

        let result =
            sqlx::query("INSERT INTO artists (name, genre, created_at) VALUES (?, ?, ?)")
                .bind(&artist.name)
                .bind(&artist.genre)
                .bind(&created_at)
                .execute(&self.pool)
                .await?;

        let id = result.last_insert_rowid();

        Ok(Artist {
            id,
            name: artist.name,
            genre: artist.genre,
            created_at,
        })
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Artist, Error> {
        sqlx::query_as::<_, Artist>(
            "SELECT id, name, genre, created_at FROM artists WHERE id = ?",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_all(&self) -> Result<Vec<Artist>, Error> {
        sqlx::query_as::<_, Artist>(
            "SELECT id, name, genre, created_at FROM artists ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn update(&self, id: i64, artist: CreateArtist) -> Result<Artist, Error> {
        sqlx::query("UPDATE artists SET name = ?, genre = ? WHERE id = ?")
            .bind(&artist.name)
            .bind(&artist.genre)
            .bind(id)
            .execute(&self.pool)
            .await?;

        self.get_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> Result<(), Error> {
        sqlx::query("DELETE FROM artists WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

pub async fn create_table(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS artists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            genre TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> ArtistRepository {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        create_table(&pool).await.unwrap();
        ArtistRepository::new(pool)
    }

    #[tokio::test]
    async fn test_create_artist() {
        let repo = setup_test_db().await;
        let create_artist = CreateArtist {
            name: "Test Artist".to_string(),
            genre: "Rock".to_string(),
        };

        let artist = repo.create(create_artist.clone()).await.unwrap();

        assert_eq!(artist.name, "Test Artist");
        assert_eq!(artist.genre, "Rock");
        assert!(artist.id > 0);
    }

    #[tokio::test]
    async fn test_get_artist_by_id() {
        let repo = setup_test_db().await;
        let create_artist = CreateArtist {
            name: "Test Artist".to_string(),
            genre: "Rock".to_string(),
        };

        let created = repo.create(create_artist).await.unwrap();
        let fetched = repo.get_by_id(created.id).await.unwrap();

        assert_eq!(created, fetched);
    }

    #[tokio::test]
    async fn test_get_all_artists() {
        let repo = setup_test_db().await;

        repo.create(CreateArtist {
            name: "Artist 1".to_string(),
            genre: "Rock".to_string(),
        })
        .await
        .unwrap();

        repo.create(CreateArtist {
            name: "Artist 2".to_string(),
            genre: "Jazz".to_string(),
        })
        .await
        .unwrap();

        let artists = repo.get_all().await.unwrap();
        assert_eq!(artists.len(), 2);
    }

    #[tokio::test]
    async fn test_update_artist() {
        let repo = setup_test_db().await;
        let create_artist = CreateArtist {
            name: "Original Artist".to_string(),
            genre: "Rock".to_string(),
        };

        let created = repo.create(create_artist).await.unwrap();

        let update_artist = CreateArtist {
            name: "Updated Artist".to_string(),
            genre: "Pop".to_string(),
        };

        let updated = repo.update(created.id, update_artist).await.unwrap();

        assert_eq!(updated.id, created.id);
        assert_eq!(updated.name, "Updated Artist");
        assert_eq!(updated.genre, "Pop");
    }

    #[tokio::test]
    async fn test_delete_artist() {
        let repo = setup_test_db().await;
        let create_artist = CreateArtist {
            name: "To Delete".to_string(),
            genre: "Rock".to_string(),
        };

        let created = repo.create(create_artist).await.unwrap();
        repo.delete(created.id).await.unwrap();

        let result = repo.get_by_id(created.id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_nonexistent_artist() {
        let repo = setup_test_db().await;
        let result = repo.get_by_id(999).await;
        assert!(result.is_err());
    }
}
