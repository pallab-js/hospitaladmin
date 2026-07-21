use crate::db;
use crate::auth::guards;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct DoctorInfo {
    pub id: String,
    pub full_name: String,
    pub department_id: Option<String>,
}

#[tauri::command]
pub async fn get_doctors() -> Result<Vec<DoctorInfo>, String> {
    guards::authenticated()?;
    let pool = db::get_pool();
    let rows = sqlx::query_as::<_, DoctorInfo>(
        "SELECT id, COALESCE(full_name, username) as full_name, department_id FROM users WHERE role = 'doctor' AND is_active = 1"
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve doctors".to_string())?;
    Ok(rows)
}
