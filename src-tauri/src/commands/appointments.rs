use uuid::Uuid;
use sqlx::Row;
use crate::db::get_pool;
use crate::models::appointment::{Appointment, AppointmentWithDetails, CreateAppointmentRequest};
use crate::auth::guards;
use crate::utils::audit::log_audit;

#[tauri::command]
pub async fn create_appointment(request: CreateAppointmentRequest) -> Result<Appointment, String> {
    let session = guards::authenticated()?;
    request.validate()?;
    let pool = get_pool();
    let id = Uuid::new_v4().to_string();
    let duration = request.duration_minutes.unwrap_or(15);
    let visit_type = request.visit_type.unwrap_or_else(|| "consultation".to_string());

    sqlx::query(
        r#"INSERT INTO appointments (id, patient_id, doctor_id, department_id, appointment_date, appointment_time, duration_minutes, visit_type, reason)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
    )
    .bind(&id)
    .bind(&request.patient_id)
    .bind(&request.doctor_id)
    .bind(&request.department_id)
    .bind(&request.appointment_date)
    .bind(&request.appointment_time)
    .bind(duration)
    .bind(&visit_type)
    .bind(&request.reason)
    .execute(pool)
    .await
    .map_err(|_| "Failed to create appointment".to_string())?;

    log_audit(&session, "create", "appointment", Some(&id), Some(&format!("date={} time={}", request.appointment_date, request.appointment_time))).await;

    let row = sqlx::query("SELECT * FROM appointments WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(|_| "Failed to retrieve appointment".to_string())?;

    Ok(Appointment {
        id: row.get("id"),
        patient_id: row.get("patient_id"),
        doctor_id: row.get("doctor_id"),
        department_id: row.get("department_id"),
        appointment_date: row.get("appointment_date"),
        appointment_time: row.get("appointment_time"),
        duration_minutes: row.get("duration_minutes"),
        status: row.get("status"),
        visit_type: row.get("visit_type"),
        reason: row.get("reason"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

#[tauri::command]
pub async fn get_appointments(
    page: Option<i64>,
    limit: Option<i64>,
) -> Result<Vec<AppointmentWithDetails>, String> {
    guards::authenticated()?;
    let pool = get_pool();
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(20);
    let offset = (page - 1) * limit;

    let rows = sqlx::query(
        r#"SELECT a.*, 
            p.first_name || ' ' || p.last_name as patient_name,
            p.patient_uid,
            s.first_name || ' ' || s.last_name as doctor_name,
            d.name as department_name
        FROM appointments a
        JOIN patients p ON a.patient_id = p.id
        JOIN staff s ON a.doctor_id = s.id
        LEFT JOIN departments d ON a.department_id = d.id
        ORDER BY a.appointment_date DESC, a.appointment_time DESC
        LIMIT ? OFFSET ?"#
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve appointments".to_string())?;

    Ok(rows.iter().map(|r| AppointmentWithDetails {
        appointment: Appointment {
            id: r.get("id"),
            patient_id: r.get("patient_id"),
            doctor_id: r.get("doctor_id"),
            department_id: r.get("department_id"),
            appointment_date: r.get("appointment_date"),
            appointment_time: r.get("appointment_time"),
            duration_minutes: r.get("duration_minutes"),
            status: r.get("status"),
            visit_type: r.get("visit_type"),
            reason: r.get("reason"),
            notes: r.get("notes"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        },
        patient_name: r.get("patient_name"),
        patient_uid: r.get("patient_uid"),
        doctor_name: r.get("doctor_name"),
        department_name: r.get("department_name"),
    }).collect())
}

#[tauri::command]
pub async fn get_appointments_by_date(date: String) -> Result<Vec<AppointmentWithDetails>, String> {
    guards::authenticated()?;
    let pool = get_pool();

    let rows = sqlx::query(
        r#"SELECT a.*, 
            p.first_name || ' ' || p.last_name as patient_name,
            p.patient_uid,
            s.first_name || ' ' || s.last_name as doctor_name,
            d.name as department_name
        FROM appointments a
        JOIN patients p ON a.patient_id = p.id
        JOIN staff s ON a.doctor_id = s.id
        LEFT JOIN departments d ON a.department_id = d.id
        WHERE a.appointment_date = ?
        ORDER BY a.appointment_time ASC"#
    )
    .bind(&date)
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve appointments".to_string())?;

    Ok(rows.iter().map(|r| AppointmentWithDetails {
        appointment: Appointment {
            id: r.get("id"),
            patient_id: r.get("patient_id"),
            doctor_id: r.get("doctor_id"),
            department_id: r.get("department_id"),
            appointment_date: r.get("appointment_date"),
            appointment_time: r.get("appointment_time"),
            duration_minutes: r.get("duration_minutes"),
            status: r.get("status"),
            visit_type: r.get("visit_type"),
            reason: r.get("reason"),
            notes: r.get("notes"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        },
        patient_name: r.get("patient_name"),
        patient_uid: r.get("patient_uid"),
        doctor_name: r.get("doctor_name"),
        department_name: r.get("department_name"),
    }).collect())
}

#[tauri::command]
pub async fn update_appointment_status(
    id: String,
    status: String,
) -> Result<Appointment, String> {
    let session = guards::authenticated()?;
    let pool = get_pool();

    sqlx::query("UPDATE appointments SET status = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(&status)
        .bind(&id)
        .execute(pool)
        .await
        .map_err(|_| "Failed to update appointment".to_string())?;

    log_audit(&session, "update_status", "appointment", Some(&id), Some(&format!("status={}", status))).await;

    let row = sqlx::query("SELECT * FROM appointments WHERE id = ?")
        .bind(&id)
        .fetch_one(pool)
        .await
        .map_err(|_| "Failed to retrieve appointment".to_string())?;

    Ok(Appointment {
        id: row.get("id"),
        patient_id: row.get("patient_id"),
        doctor_id: row.get("doctor_id"),
        department_id: row.get("department_id"),
        appointment_date: row.get("appointment_date"),
        appointment_time: row.get("appointment_time"),
        duration_minutes: row.get("duration_minutes"),
        status: row.get("status"),
        visit_type: row.get("visit_type"),
        reason: row.get("reason"),
        notes: row.get("notes"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}
