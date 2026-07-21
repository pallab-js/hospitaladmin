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
    let pool = get_pool();
    let id = Uuid::new_v4().to_string();
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

    sqlx::query(
        "INSERT INTO prescriptions (id, appointment_id, patient_id, doctor_id, prescription_date, notes) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&request.appointment_id)
    .bind(&request.patient_id)
    .bind(&session.employee_id.as_deref().unwrap_or(""))
    .bind(&today)
    .bind(&request.notes)
    .execute(pool)
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
        .execute(pool)
        .await
        .map_err(|_| "Failed to add prescription item".to_string())?;
    }

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

    sqlx::query("UPDATE prescription_items SET dispensed = 1, dispensed_at = datetime('now') WHERE id = ?")
        .bind(&item_id)
        .execute(pool)
        .await
        .map_err(|_| "Failed to dispense item".to_string())?;

    log_audit(&session, "dispense", "prescription_item", Some(&item_id), None).await;
    Ok(())
}
