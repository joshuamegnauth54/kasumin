use std::str::FromStr;

use sqlx::sqlite::{SqliteAutoVacuum, SqliteConnectOptions, SqliteJournalMode, SqlitePool};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn connect(url: &str) -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::from_str(url)?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .thread_name(|id| format!("nico_sqlite-{id}"))
            .optimize_on_close(true, None)
            .analysis_limit(Some(512))
            .auto_vacuum(SqliteAutoVacuum::Full)
            .extension("spellfix");
        let pool = SqlitePool::connect_with(options).await?;

        Ok(Self { pool })
    }
}
