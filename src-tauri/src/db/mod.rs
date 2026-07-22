pub mod seed;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use std::sync::OnceLock;
use tauri::AppHandle;
use tauri::Manager;

static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

pub async fn init(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let app_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data dir");
    std::fs::create_dir_all(&app_dir)?;

    let db_path = app_dir.join("hms.db");
    let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

    let options = SqliteConnectOptions::from_str(&db_url)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    sqlx::query("PRAGMA journal_mode=WAL")
        .execute(&pool)
        .await?;
    sqlx::query("PRAGMA foreign_keys=ON").execute(&pool).await?;

    if let Err(e) = run_migrations(&pool).await {
        eprintln!(
            "[db] Migration failed: {}. Deleting stale database and retrying...",
            e
        );
        drop(pool);
        std::fs::remove_file(&db_path)?;
        // Re-init with fresh DB
        let options2 = SqliteConnectOptions::from_str(&db_url)?
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .foreign_keys(true);
        let pool2 = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options2)
            .await?;
        sqlx::query("PRAGMA journal_mode=WAL")
            .execute(&pool2)
            .await?;
        sqlx::query("PRAGMA foreign_keys=ON")
            .execute(&pool2)
            .await?;
        run_migrations(&pool2).await?;
        seed::seed(&pool2).await?;
        DB_POOL
            .set(pool2)
            .expect("Database pool already initialized");
    } else {
        seed::seed(&pool).await?;
        DB_POOL
            .set(pool)
            .expect("Database pool already initialized");
    }

    Ok(())
}

pub fn get_pool() -> &'static SqlitePool {
    DB_POOL.get().expect("Database pool not initialized")
}

async fn run_migrations(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version TEXT PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        )",
    )
    .execute(pool)
    .await?;

    let migrations_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    if !migrations_dir.exists() {
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(&migrations_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "sql")
                .unwrap_or(false)
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    let applied: Vec<String> = sqlx::query_scalar("SELECT version FROM schema_migrations")
        .fetch_all(pool)
        .await?;

    for entry in entries {
        let path = entry.path();
        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        if applied.contains(&filename) {
            continue;
        }

        let sql = std::fs::read_to_string(&path)?;
        let mut tx = pool.begin().await?;
        sqlx::query(&sql).execute(&mut *tx).await?;
        sqlx::query("INSERT INTO schema_migrations (version) VALUES (?)")
            .bind(&filename)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        println!("[migration] applied {}", filename);
    }

    Ok(())
}
