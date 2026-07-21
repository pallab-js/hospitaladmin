use uuid::Uuid;
use sqlx::Row;
use crate::db::get_pool;
use crate::auth::guards;
use crate::utils::audit::log_audit;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct AdmissionWithDetails {
    pub id: String,
    pub patient_id: String,
    pub patient_name: String,
    pub patient_uid: String,
    pub bed_id: String,
    pub bed_number: String,
    pub ward_name: String,
    pub doctor_id: String,
    pub doctor_name: String,
    pub admission_date: String,
    pub admission_time: String,
    pub discharge_date: Option<String>,
    pub discharge_time: Option<String>,
    pub admission_type: String,
    pub diagnosis: Option<String>,
    pub treatment_notes: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(serde::Deserialize)]
pub struct CreateAdmissionRequest {
    pub patient_id: String,
    pub bed_id: String,
    pub admission_type: Option<String>,
    pub diagnosis: Option<String>,
    pub treatment_notes: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct DischargeRequest {
    pub admission_id: String,
    pub discharge_notes: Option<String>,
}

#[tauri::command]
pub async fn create_admission(request: CreateAdmissionRequest) -> Result<AdmissionWithDetails, String> {
    let session = guards::doctor_only()?;
    let pool = get_pool();
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now();
    let today = now.format("%Y-%m-%d").to_string();
    let time = now.format("%H:%M").to_string();
    let admission_type = request.admission_type.unwrap_or_else(|| "planned".to_string());

    let bed_status: String = sqlx::query_scalar("SELECT status FROM beds WHERE id = ?")
        .bind(&request.bed_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| "Failed to check bed status".to_string())?
        .unwrap_or_default();

    if bed_status != "available" {
        return Err("Bed is not available".to_string());
    }

    sqlx::query(
        "INSERT INTO admissions (id, patient_id, bed_id, doctor_id, admission_date, admission_time, admission_type, diagnosis, treatment_notes) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&request.patient_id)
    .bind(&request.bed_id)
    .bind(&session.employee_id.as_deref().unwrap_or(""))
    .bind(&today)
    .bind(&time)
    .bind(&admission_type)
    .bind(&request.diagnosis)
    .bind(&request.treatment_notes)
    .execute(pool)
    .await
    .map_err(|_| "Failed to create admission".to_string())?;

    sqlx::query("UPDATE beds SET status = 'occupied' WHERE id = ?")
        .bind(&request.bed_id)
        .execute(pool)
        .await
        .map_err(|_| "Failed to update bed status".to_string())?;

    log_audit(&session, "create", "admission", Some(&id), Some(&format!("patient={} bed={}", request.patient_id, request.bed_id))).await;

    get_admission_by_id(&id).await
}

#[tauri::command]
pub async fn discharge_patient(request: DischargeRequest) -> Result<(), String> {
    let session = guards::doctor_only()?;
    let pool = get_pool();
    let now = chrono::Utc::now();
    let today = now.format("%Y-%m-%d").to_string();
    let time = now.format("%H:%M").to_string();

    let bed_id: String = sqlx::query_scalar("SELECT bed_id FROM admissions WHERE id = ?")
        .bind(&request.admission_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| "Failed to retrieve admission".to_string())?
        .ok_or("Admission not found".to_string())?;

    sqlx::query(
        "UPDATE admissions SET status = 'discharged', discharge_date = ?, discharge_time = ?, treatment_notes = COALESCE(?, treatment_notes), updated_at = datetime('now') WHERE id = ?"
    )
    .bind(&today)
    .bind(&time)
    .bind(&request.discharge_notes)
    .bind(&request.admission_id)
    .execute(pool)
    .await
    .map_err(|_| "Failed to discharge patient".to_string())?;

    sqlx::query("UPDATE beds SET status = 'available' WHERE id = ?")
        .bind(&bed_id)
        .execute(pool)
        .await
        .map_err(|_| "Failed to update bed status".to_string())?;

    log_audit(&session, "discharge", "admission", Some(&request.admission_id), None).await;
    Ok(())
}

#[tauri::command]
pub async fn get_active_admissions() -> Result<Vec<AdmissionWithDetails>, String> {
    guards::authenticated()?;
    let pool = get_pool();
    let rows = sqlx::query(
        r#"SELECT a.*, 
            p.first_name || ' ' || p.last_name as patient_name,
            p.patient_uid,
            b.bed_number,
            w.name as ward_name,
            s.first_name || ' ' || s.last_name as doctor_name
        FROM admissions a
        JOIN patients p ON a.patient_id = p.id
        JOIN beds b ON a.bed_id = b.id
        JOIN wards w ON b.ward_id = w.id
        JOIN staff s ON a.doctor_id = s.id
        WHERE a.status = 'active'
        ORDER BY a.admission_date DESC"#
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve admissions".to_string())?;

    Ok(rows.iter().map(|r| AdmissionWithDetails {
        id: r.get("id"),
        patient_id: r.get("patient_id"),
        patient_name: r.get("patient_name"),
        patient_uid: r.get("patient_uid"),
        bed_id: r.get("bed_id"),
        bed_number: r.get("bed_number"),
        ward_name: r.get("ward_name"),
        doctor_id: r.get("doctor_id"),
        doctor_name: r.get("doctor_name"),
        admission_date: r.get("admission_date"),
        admission_time: r.get("admission_time"),
        discharge_date: r.get("discharge_date"),
        discharge_time: r.get("discharge_time"),
        admission_type: r.get("admission_type"),
        diagnosis: r.get("diagnosis"),
        treatment_notes: r.get("treatment_notes"),
        status: r.get("status"),
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }).collect())
}

async fn get_admission_by_id(id: &str) -> Result<AdmissionWithDetails, String> {
    let pool = get_pool();
    let row = sqlx::query(
        r#"SELECT a.*, 
            p.first_name || ' ' || p.last_name as patient_name,
            p.patient_uid,
            b.bed_number,
            w.name as ward_name,
            s.first_name || ' ' || s.last_name as doctor_name
        FROM admissions a
        JOIN patients p ON a.patient_id = p.id
        JOIN beds b ON a.bed_id = b.id
        JOIN wards w ON b.ward_id = w.id
        JOIN staff s ON a.doctor_id = s.id
        WHERE a.id = ?"#
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|_| "Failed to retrieve admission".to_string())?;

    match row {
        Some(r) => Ok(AdmissionWithDetails {
            id: r.get("id"),
            patient_id: r.get("patient_id"),
            patient_name: r.get("patient_name"),
            patient_uid: r.get("patient_uid"),
            bed_id: r.get("bed_id"),
            bed_number: r.get("bed_number"),
            ward_name: r.get("ward_name"),
            doctor_id: r.get("doctor_id"),
            doctor_name: r.get("doctor_name"),
            admission_date: r.get("admission_date"),
            admission_time: r.get("admission_time"),
            discharge_date: r.get("discharge_date"),
            discharge_time: r.get("discharge_time"),
            admission_type: r.get("admission_type"),
            diagnosis: r.get("diagnosis"),
            treatment_notes: r.get("treatment_notes"),
            status: r.get("status"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }),
        None => Err("Admission not found".to_string()),
    }
}
