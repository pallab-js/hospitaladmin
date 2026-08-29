use sqlx::SqlitePool;

pub async fn seed(_pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
