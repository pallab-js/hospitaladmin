use crate::db;
use crate::auth::guards;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct StaffInfo {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub department_id: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub qualification: Option<String>,
    pub specialization: Option<String>,
    pub is_active: bool,
    pub hire_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[tauri::command]
pub async fn get_staff_list() -> Result<Vec<StaffInfo>, String> {
    guards::authenticated()?;
    let pool = db::get_pool();
    let rows = sqlx::query_as::<_, StaffInfo>(
        "SELECT id, first_name, last_name, role, department_id, email, phone, qualification, specialization, is_active, hire_date, created_at, updated_at FROM staff WHERE is_active = 1 ORDER BY first_name"
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve staff list".to_string())?;
    Ok(rows)
}
