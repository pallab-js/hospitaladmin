use uuid::Uuid;
use sqlx::Row;
use crate::db::get_pool;
use crate::models::patient::{CreatePatientRequest, Patient, PatientSearchParams};
use crate::auth::guards;
use crate::utils::audit::log_audit;
use crate::utils::id::generate_patient_uid;

#[tauri::command]
pub async fn create_patient(request: CreatePatientRequest) -> Result<Patient, String> {
    let session = guards::authenticated()?;
    request.validate()?;
    let pool = get_pool();
    let id = Uuid::new_v4().to_string();
    let patient_uid = generate_patient_uid().await?;

    sqlx::query(
        r#"INSERT INTO patients (id, patient_uid, first_name, last_name, date_of_birth, gender, blood_group, phone, email, address, emergency_contact_name, emergency_contact_phone, insurance_provider, insurance_id, allergies, medical_history)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(&id)
    .bind(&patient_uid)
    .bind(&request.first_name)
    .bind(&request.last_name)
    .bind(&request.date_of_birth)
    .bind(&request.gender)
    .bind(&request.blood_group)
    .bind(&request.phone)
    .bind(&request.email)
    .bind(&request.address)
    .bind(&request.emergency_contact_name)
    .bind(&request.emergency_contact_phone)
    .bind(&request.insurance_provider)
    .bind(&request.insurance_id)
    .bind(&request.allergies)
    .bind(&request.medical_history)
    .execute(pool)
    .await
    .map_err(|_| "Failed to create patient record".to_string())?;

    log_audit(&session, "create", "patient", Some(&id), Some(&format!("{} {}", request.first_name, request.last_name))).await;

    get_patient_by_id(id).await
}

#[tauri::command]
pub async fn get_patients(
    page: Option<i64>,
    limit: Option<i64>,
) -> Result<Vec<Patient>, String> {
    guards::authenticated()?;
    let pool = get_pool();
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    let rows = sqlx::query(
        "SELECT * FROM patients WHERE is_active = 1 ORDER BY created_at DESC LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve patients".to_string())?;

    Ok(rows.iter().map(|r| Patient {
        id: r.get("id"),
        patient_uid: r.get("patient_uid"),
        first_name: r.get("first_name"),
        last_name: r.get("last_name"),
        date_of_birth: r.get("date_of_birth"),
        gender: r.get("gender"),
        blood_group: r.get("blood_group"),
        phone: r.get("phone"),
        email: r.get("email"),
        address: r.get("address"),
        emergency_contact_name: r.get("emergency_contact_name"),
        emergency_contact_phone: r.get("emergency_contact_phone"),
        insurance_provider: r.get("insurance_provider"),
        insurance_id: r.get("insurance_id"),
        allergies: r.get("allergies"),
        medical_history: r.get("medical_history"),
        is_active: r.get::<i64, _>("is_active") == 1,
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }).collect())
}

#[tauri::command]
pub async fn get_patient_by_id(id: String) -> Result<Patient, String> {
    guards::authenticated()?;
    let pool = get_pool();

    let row = sqlx::query("SELECT * FROM patients WHERE id = ?")
        .bind(&id)
        .fetch_optional(pool)
        .await
        .map_err(|_| "Failed to retrieve patient".to_string())?;

    match row {
        Some(r) => Ok(Patient {
            id: r.get("id"),
            patient_uid: r.get("patient_uid"),
            first_name: r.get("first_name"),
            last_name: r.get("last_name"),
            date_of_birth: r.get("date_of_birth"),
            gender: r.get("gender"),
            blood_group: r.get("blood_group"),
            phone: r.get("phone"),
            email: r.get("email"),
            address: r.get("address"),
            emergency_contact_name: r.get("emergency_contact_name"),
            emergency_contact_phone: r.get("emergency_contact_phone"),
            insurance_provider: r.get("insurance_provider"),
            insurance_id: r.get("insurance_id"),
            allergies: r.get("allergies"),
            medical_history: r.get("medical_history"),
            is_active: r.get::<i64, _>("is_active") == 1,
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }),
        None => Err("Patient not found".to_string()),
    }
}

#[tauri::command]
pub async fn search_patients(params: PatientSearchParams) -> Result<Vec<Patient>, String> {
    guards::authenticated()?;
    let pool = get_pool();
    let mut query = String::from("SELECT * FROM patients WHERE is_active = 1");
    let mut bind_values: Vec<String> = Vec::new();

    if let Some(ref q) = params.query {
        query.push_str(" AND (first_name LIKE ? OR last_name LIKE ? OR patient_uid LIKE ? OR phone LIKE ?)");
        let pattern = format!("%{}%", q);
        bind_values.extend(vec![pattern.clone(), pattern.clone(), pattern.clone(), pattern]);
    }

    if let Some(ref gender) = params.gender {
        query.push_str(" AND gender = ?");
        bind_values.push(gender.clone());
    }

    if let Some(ref blood_group) = params.blood_group {
        query.push_str(" AND blood_group = ?");
        bind_values.push(blood_group.clone());
    }

    query.push_str(" ORDER BY created_at DESC LIMIT ? OFFSET ?");

    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    let mut sql = sqlx::query(&query);
    for value in &bind_values {
        sql = sql.bind(value);
    }
    sql = sql.bind(limit).bind(offset);

    let rows = sql.fetch_all(pool)
        .await
        .map_err(|_| "Failed to search patients".to_string())?;

    Ok(rows.iter().map(|r| Patient {
        id: r.get("id"),
        patient_uid: r.get("patient_uid"),
        first_name: r.get("first_name"),
        last_name: r.get("last_name"),
        date_of_birth: r.get("date_of_birth"),
        gender: r.get("gender"),
        blood_group: r.get("blood_group"),
        phone: r.get("phone"),
        email: r.get("email"),
        address: r.get("address"),
        emergency_contact_name: r.get("emergency_contact_name"),
        emergency_contact_phone: r.get("emergency_contact_phone"),
        insurance_provider: r.get("insurance_provider"),
        insurance_id: r.get("insurance_id"),
        allergies: r.get("allergies"),
        medical_history: r.get("medical_history"),
        is_active: r.get::<i64, _>("is_active") == 1,
        created_at: r.get("created_at"),
        updated_at: r.get("updated_at"),
    }).collect())
}
