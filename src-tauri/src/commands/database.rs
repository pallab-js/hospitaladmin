use crate::auth::guards;
use crate::db::get_db_path;
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

    // Return the canonical path to prevent symlink-based attacks
    Ok(canonical)
}

#[tauri::command]
pub async fn export_database(path: String) -> Result<String, String> {
    guards::admin_only()?;

    let dest = validate_export_path(&path)?;

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|_| "Failed to create export directory".to_string())?;
    }

    // Use safe file copy instead of VACUUM INTO to avoid SQL injection risks.
    // First, ensure WAL is checkpointed so the copy includes all committed data.
    let db_path = get_db_path();
    sqlx::query("PRAGMA wal_checkpoint(TRUNCATE)")
        .execute(crate::db::get_pool())
        .await
        .map_err(|_| "Failed to checkpoint WAL".to_string())?;

    std::fs::copy(db_path, &dest).map_err(|e| format!("Failed to copy database: {}", e))?;

    // Also copy the WAL and SHM files if they exist (for a consistent snapshot)
    let wal_path = PathBuf::from(format!("{}-wal", db_path.display()));
    let shm_path = PathBuf::from(format!("{}-shm", db_path.display()));
    let _ = std::fs::copy(&wal_path, PathBuf::from(format!("{}-wal", dest.display())));
    let _ = std::fs::copy(&shm_path, PathBuf::from(format!("{}-shm", dest.display())));

    Ok(format!("Database exported to {}", dest.display()))
}
