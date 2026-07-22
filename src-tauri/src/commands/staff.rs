use crate::auth::guards;
use crate::db;
use crate::models::staff::Staff;

#[tauri::command]
pub async fn get_staff_list() -> Result<Vec<Staff>, String> {
    guards::authenticated()?;
    let pool = db::get_pool();
    sqlx::query_as::<_, Staff>(
        "SELECT id, first_name, last_name, role, department_id, email, phone, qualification, specialization, is_active, hire_date, created_at, updated_at FROM staff WHERE is_active = 1 ORDER BY first_name"
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve staff list".to_string())
}
