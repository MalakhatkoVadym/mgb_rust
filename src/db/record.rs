use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, sqlite::SqlitePool};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, PartialEq)]
pub struct Record {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateRecord {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct RecordRepository {
    pool: SqlitePool,
}

impl RecordRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, record: CreateRecord) -> Result<Record, Error> {
        let created_at = Utc::now().to_rfc3339();

        let result =
            sqlx::query("INSERT INTO records (title, content, created_at) VALUES (?, ?, ?)")
                .bind(&record.title)
                .bind(&record.content)
                .bind(&created_at)
                .execute(&self.pool)
                .await?;

        let id = result.last_insert_rowid();

        Ok(Record {
            id,
            title: record.title,
            content: record.content,
            created_at,
        })
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Record, Error> {
        sqlx::query_as::<_, Record>(
            "SELECT id, title, content, created_at FROM records WHERE id = ?",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_all(&self) -> Result<Vec<Record>, Error> {
        sqlx::query_as::<_, Record>(
            "SELECT id, title, content, created_at FROM records ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn update(&self, id: i64, record: CreateRecord) -> Result<Record, Error> {
        sqlx::query("UPDATE records SET title = ?, content = ? WHERE id = ?")
            .bind(&record.title)
            .bind(&record.content)
            .bind(id)
            .execute(&self.pool)
            .await?;

        self.get_by_id(id).await
    }

    pub async fn delete(&self, id: i64) -> Result<(), Error> {
        sqlx::query("DELETE FROM records WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

pub async fn create_table(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
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

    async fn setup_test_db() -> RecordRepository {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        create_table(&pool).await.unwrap();
        RecordRepository::new(pool)
    }

    #[tokio::test]
    async fn test_create_record() {
        let repo = setup_test_db().await;
        let create_record = CreateRecord {
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
        };

        let record = repo.create(create_record.clone()).await.unwrap();

        assert_eq!(record.title, "Test Title");
        assert_eq!(record.content, "Test Content");
        assert!(record.id > 0);
    }

    #[tokio::test]
    async fn test_get_record_by_id() {
        let repo = setup_test_db().await;
        let create_record = CreateRecord {
            title: "Test Title".to_string(),
            content: "Test Content".to_string(),
        };

        let created = repo.create(create_record).await.unwrap();
        let fetched = repo.get_by_id(created.id).await.unwrap();

        assert_eq!(created, fetched);
    }

    #[tokio::test]
    async fn test_get_all_records() {
        let repo = setup_test_db().await;

        repo.create(CreateRecord {
            title: "Record 1".to_string(),
            content: "Content 1".to_string(),
        })
        .await
        .unwrap();

        repo.create(CreateRecord {
            title: "Record 2".to_string(),
            content: "Content 2".to_string(),
        })
        .await
        .unwrap();

        let records = repo.get_all().await.unwrap();
        assert_eq!(records.len(), 2);
    }

    #[tokio::test]
    async fn test_update_record() {
        let repo = setup_test_db().await;
        let create_record = CreateRecord {
            title: "Original Title".to_string(),
            content: "Original Content".to_string(),
        };

        let created = repo.create(create_record).await.unwrap();

        let update_record = CreateRecord {
            title: "Updated Title".to_string(),
            content: "Updated Content".to_string(),
        };

        let updated = repo.update(created.id, update_record).await.unwrap();

        assert_eq!(updated.id, created.id);
        assert_eq!(updated.title, "Updated Title");
        assert_eq!(updated.content, "Updated Content");
    }

    #[tokio::test]
    async fn test_delete_record() {
        let repo = setup_test_db().await;
        let create_record = CreateRecord {
            title: "To Delete".to_string(),
            content: "Will be deleted".to_string(),
        };

        let created = repo.create(create_record).await.unwrap();
        repo.delete(created.id).await.unwrap();

        let result = repo.get_by_id(created.id).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_nonexistent_record() {
        let repo = setup_test_db().await;
        let result = repo.get_by_id(999).await;
        assert!(result.is_err());
    }
}
