use uuid::Uuid;
use sqlx::Row;
use crate::db::get_pool;
use crate::auth::guards;
use crate::utils::audit::log_audit;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct PrescriptionRow {
    pub id: String,
    pub appointment_id: Option<String>,
    pub patient_id: String,
    pub doctor_id: String,
    pub prescription_date: String,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct PrescriptionItemRow {
    pub id: String,
    pub prescription_id: String,
    pub medication_id: String,
    pub dosage: String,
    pub frequency: String,
    pub duration_days: i64,
    pub instructions: Option<String>,
    pub dispensed: bool,
    pub dispensed_at: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct CreatePrescriptionRequest {
    pub appointment_id: Option<String>,
    pub patient_id: String,
    pub notes: Option<String>,
    pub items: Vec<CreatePrescriptionItemRequest>,
}

#[derive(serde::Deserialize)]
pub struct CreatePrescriptionItemRequest {
    pub medication_id: String,
    pub dosage: String,
    pub frequency: String,
    pub duration_days: i64,
    pub instructions: Option<String>,
}

#[tauri::command]
pub async fn create_prescription(request: CreatePrescriptionRequest) -> Result<PrescriptionRow, String> {
    let session = guards::doctor_only()?;

    if request.patient_id.trim().is_empty() {
        return Err("Patient ID is required".to_string());
    }
    if request.items.is_empty() {
        return Err("At least one medication must be prescribed".to_string());
    }
    for item in &request.items {
        if item.dosage.trim().is_empty() {
            return Err("Dosage is required for all items".to_string());
        }
        if item.frequency.trim().is_empty() {
            return Err("Frequency is required for all items".to_string());
        }
        if item.duration_days <= 0 || item.duration_days > 365 {
            return Err("Duration must be between 1 and 365 days".to_string());
        }
    }

    let pool = get_pool();
    let id = Uuid::new_v4().to_string();
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();

    let mut tx = pool.begin().await.map_err(|_| "Failed to start transaction".to_string())?;

    sqlx::query(
        "INSERT INTO prescriptions (id, appointment_id, patient_id, doctor_id, prescription_date, notes) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&request.appointment_id)
    .bind(&request.patient_id)
    .bind(&session.employee_id.as_deref().unwrap_or(""))
    .bind(&today)
    .bind(&request.notes)
    .execute(&mut *tx)
    .await
    .map_err(|_| "Failed to create prescription".to_string())?;

    for item in &request.items {
        let item_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO prescription_items (id, prescription_id, medication_id, dosage, frequency, duration_days, instructions) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&item_id)
        .bind(&id)
        .bind(&item.medication_id)
        .bind(&item.dosage)
        .bind(&item.frequency)
        .bind(item.duration_days)
        .bind(&item.instructions)
        .execute(&mut *tx)
        .await
        .map_err(|_| "Failed to add prescription item".to_string())?;
    }

    tx.commit().await.map_err(|_| "Failed to commit prescription".to_string())?;

    log_audit(&session, "create", "prescription", Some(&id), Some(&format!("patient={}", request.patient_id))).await;

    let row = sqlx::query("SELECT * FROM prescriptions WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(|_| "Failed to retrieve prescription".to_string())?;

    Ok(PrescriptionRow {
        id: row.get("id"),
        appointment_id: row.get("appointment_id"),
        patient_id: row.get("patient_id"),
        doctor_id: row.get("doctor_id"),
        prescription_date: row.get("prescription_date"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
    })
}

#[tauri::command]
pub async fn get_prescriptions_by_patient(patient_id: String) -> Result<Vec<PrescriptionRow>, String> {
    guards::authenticated()?;
    let pool = get_pool();
    let rows = sqlx::query_as::<_, PrescriptionRow>(
        "SELECT id, appointment_id, patient_id, doctor_id, prescription_date, notes, created_at FROM prescriptions WHERE patient_id = ? ORDER BY created_at DESC"
    )
    .bind(&patient_id)
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve prescriptions".to_string())?;
    Ok(rows)
}

#[tauri::command]
pub async fn dispense_prescription_item(item_id: String) -> Result<(), String> {
    let session = guards::authenticated()?;
    let pool = get_pool();

    let mut tx = pool.begin().await.map_err(|_| "Failed to start transaction".to_string())?;

    // Get medication_id for this prescription item
    let medication_id: String = sqlx::query_scalar(
        "SELECT medication_id FROM prescription_items WHERE id = ?"
    )
    .bind(&item_id)
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| "Failed to look up prescription item".to_string())?
    .ok_or("Prescription item not found".to_string())?;

    // Decrement from oldest available batch
    let affected = sqlx::query(
        "UPDATE inventory SET quantity = quantity - 1 WHERE id = (
            SELECT id FROM inventory WHERE medication_id = ? AND quantity > 0 AND expiry_date >= date('now')
            ORDER BY expiry_date ASC LIMIT 1
        )"
    )
    .bind(&medication_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| "Failed to update inventory".to_string())?
    .rows_affected();

    if affected == 0 {
        return Err("No stock available for this medication".to_string());
    }

    sqlx::query("UPDATE prescription_items SET dispensed = 1, dispensed_at = datetime('now') WHERE id = ?")
        .bind(&item_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| "Failed to dispense item".to_string())?;

    tx.commit().await.map_err(|_| "Failed to commit dispense".to_string())?;

    log_audit(&session, "dispense", "prescription_item", Some(&item_id), None).await;
    Ok(())
}
