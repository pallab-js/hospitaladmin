use crate::auth::guards;
use crate::db::get_pool;
use std::path::PathBuf;

fn validate_export_path(path: &str) -> Result<PathBuf, String> {
    let dest = PathBuf::from(path);

    // Reject relative paths, path traversal, and dangerous characters
    if path.contains("..") {
        return Err("Path must not contain '..'".into());
    }
    if path.contains('\'') || path.contains('"') || path.contains('\0') {
        return Err("Path contains invalid characters".into());
    }
    if dest.is_relative() {
        return Err("Path must be absolute".into());
    }

    // Restrict to user's home or temp directory
    let home = dirs::home_dir().ok_or("Cannot determine home directory")?;
    let temp = std::env::temp_dir();
    let canonical = dest
        .canonicalize()
        .map_err(|e| format!("Invalid path: {}", e))?;

    let under_home = canonical.starts_with(&home);
    let under_temp = canonical.starts_with(&temp);
    if !under_home && !under_temp {
        return Err("Export path must be under home directory or /tmp".into());
    }

    Ok(dest)
}

#[tauri::command]
pub async fn export_database(path: String) -> Result<String, String> {
    guards::admin_only()?;

    let dest = validate_export_path(&path)?;

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|_| "Failed to create export directory".to_string())?;
    }

    // VACUUM INTO doesn't support parameter binding — path is pre-validated above
    let safe_path = dest.display().to_string().replace('\'', "''");
    sqlx::query(&format!("VACUUM INTO '{}'", safe_path))
        .execute(get_pool())
        .await
        .map_err(|_| "Failed to export database".to_string())?;

    Ok(format!("Database exported to {}", dest.display()))
}
