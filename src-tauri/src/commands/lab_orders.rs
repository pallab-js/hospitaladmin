use uuid::Uuid;
use crate::db::get_pool;
use crate::auth::guards;
use crate::utils::audit::log_audit;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct LabOrderWithDetails {
    pub id: String,
    pub appointment_id: Option<String>,
    pub patient_id: String,
    pub patient_name: String,
    pub patient_uid: String,
    pub doctor_id: String,
    pub doctor_name: String,
    pub order_date: String,
    pub priority: String,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(serde::Deserialize)]
pub struct CreateLabOrderRequest {
    pub patient_id: String,
    pub appointment_id: Option<String>,
    pub test_ids: Vec<String>,
    pub priority: Option<String>,
    pub notes: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct UpdateLabResultRequest {
    pub order_item_id: String,
    pub result_value: String,
    pub result_notes: Option<String>,
    pub is_abnormal: bool,
}

#[tauri::command]
pub async fn create_lab_order(request: CreateLabOrderRequest) -> Result<String, String> {
    let session = guards::doctor_only()?;

    if request.patient_id.trim().is_empty() {
        return Err("Patient ID is required".to_string());
    }
    if request.test_ids.is_empty() {
        return Err("At least one test must be selected".to_string());
    }

    let pool = get_pool();
    let id = Uuid::new_v4().to_string();
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let priority = request.priority.unwrap_or_else(|| "normal".to_string());

    let mut tx = pool.begin().await.map_err(|_| "Failed to start transaction".to_string())?;

    sqlx::query(
        "INSERT INTO lab_orders (id, appointment_id, patient_id, doctor_id, order_date, priority, notes) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&request.appointment_id)
    .bind(&request.patient_id)
    .bind(&session.employee_id.as_deref().unwrap_or(""))
    .bind(&today)
    .bind(&priority)
    .bind(&request.notes)
    .execute(&mut *tx)
    .await
    .map_err(|_| "Failed to create lab order".to_string())?;

    for test_id in &request.test_ids {
        let item_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO lab_order_items (id, lab_order_id, test_id) VALUES (?, ?, ?)"
        )
        .bind(&item_id)
        .bind(&id)
        .bind(test_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| "Failed to add lab order item".to_string())?;
    }

    tx.commit().await.map_err(|_| "Failed to commit lab order".to_string())?;

    log_audit(&session, "create", "lab_order", Some(&id), Some(&format!("patient={}", request.patient_id))).await;
    Ok(id)
}

#[tauri::command]
pub async fn update_lab_result(request: UpdateLabResultRequest) -> Result<(), String> {
    let session = guards::authenticated()?;
    let pool = get_pool();

    sqlx::query(
        "UPDATE lab_order_items SET result_value = ?, result_notes = ?, is_abnormal = ?, completed_at = datetime('now'), completed_by = ? WHERE id = ?"
    )
    .bind(&request.result_value)
    .bind(&request.result_notes)
    .bind(request.is_abnormal as i64)
    .bind(&session.employee_id.as_deref().unwrap_or(""))
    .bind(&request.order_item_id)
    .execute(pool)
    .await
    .map_err(|_| "Failed to update lab result".to_string())?;

    log_audit(&session, "update_result", "lab_order_item", Some(&request.order_item_id), None).await;
    Ok(())
}

#[tauri::command]
pub async fn complete_lab_order(order_id: String) -> Result<(), String> {
    let session = guards::authenticated()?;
    let pool = get_pool();

    sqlx::query("UPDATE lab_orders SET status = 'completed', updated_at = datetime('now') WHERE id = ?")
        .bind(&order_id)
        .execute(pool)
        .await
        .map_err(|_| "Failed to complete lab order".to_string())?;

    log_audit(&session, "complete", "lab_order", Some(&order_id), None).await;
    Ok(())
}
