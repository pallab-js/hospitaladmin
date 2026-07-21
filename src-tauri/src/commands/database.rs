use crate::auth::guards;
use crate::db::get_pool;
use std::path::PathBuf;

#[tauri::command]
pub async fn export_database(path: String) -> Result<String, String> {
    guards::management_only()?;

    let dest = PathBuf::from(&path);
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // VACUUM INTO doesn't support parameter binding; sanitize path against injection
    let safe_path = dest.display().to_string().replace('\'', "''");
    sqlx::query(&format!("VACUUM INTO '{}'", safe_path))
        .execute(get_pool())
        .await
        .map_err(|e| format!("Failed to export database: {}", e))?;

    Ok(format!("Database exported to {}", dest.display()))
}
