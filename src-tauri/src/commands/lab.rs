use crate::auth::guards;
use crate::db;
use crate::models::lab::LabTest;
use sqlx::Row;

#[derive(serde::Serialize)]
pub struct LabOrderRow {
    pub id: String,
    pub appointment_id: Option<String>,
    pub patient_id: String,
    pub patient_name: String,
    pub patient_uid: String,
    pub doctor_name: String,
    pub order_date: String,
    pub priority: String,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[tauri::command]
pub async fn get_lab_tests() -> Result<Vec<LabTest>, String> {
    guards::authenticated()?;
    let pool = db::get_pool();
    sqlx::query_as::<_, LabTest>(
        "SELECT id, name, code, category, normal_range, unit, price, is_active, created_at FROM lab_tests WHERE is_active = 1 ORDER BY name"
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve lab tests".to_string())
}

#[tauri::command]
pub async fn get_lab_orders() -> Result<Vec<LabOrderRow>, String> {
    guards::authenticated()?;
    let pool = db::get_pool();
    let rows = sqlx::query(
        r#"SELECT lo.id, lo.appointment_id, lo.patient_id,
            p.first_name || ' ' || p.last_name as patient_name,
            p.patient_uid,
            s.first_name || ' ' || s.last_name as doctor_name,
            lo.order_date, lo.priority, lo.status, lo.notes, lo.created_at, lo.updated_at
        FROM lab_orders lo
        JOIN patients p ON lo.patient_id = p.id
        JOIN staff s ON lo.doctor_id = s.id
        ORDER BY lo.created_at DESC
        LIMIT 20"#,
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve lab orders".to_string())?;

    Ok(rows
        .iter()
        .map(|r| LabOrderRow {
            id: r.get("id"),
            appointment_id: r.get("appointment_id"),
            patient_id: r.get("patient_id"),
            patient_name: r.get("patient_name"),
            patient_uid: r.get("patient_uid"),
            doctor_name: r.get("doctor_name"),
            order_date: r.get("order_date"),
            priority: r.get("priority"),
            status: r.get("status"),
            notes: r.get("notes"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        })
        .collect())
}
