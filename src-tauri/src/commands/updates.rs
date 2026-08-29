use crate::auth::guards;
use crate::db::get_pool;
use crate::utils::audit::log_audit;
use serde::{Deserialize, Serialize};

const MAX_NAME_LEN: usize = 100;
const MAX_EMAIL_LEN: usize = 254;
const MAX_PHONE_LEN: usize = 20;
const MAX_ADDRESS_LEN: usize = 500;
const MAX_CONTACT_NAME_LEN: usize = 200;
const MAX_TEXT_LEN: usize = 2000;
const MAX_LONG_TEXT_LEN: usize = 5000;

fn validate_optional_str(
    value: &Option<String>,
    max_len: usize,
    field_name: &str,
) -> Result<(), String> {
    if let Some(ref v) = value {
        if v.len() > max_len {
            return Err(format!(
                "{} must be {} characters or less",
                field_name, max_len
            ));
        }
    }
    Ok(())
}

fn validate_optional_email(value: &Option<String>) -> Result<(), String> {
    if let Some(ref v) = value {
        if !v.is_empty() && !v.contains('@') {
            return Err("Invalid email format".to_string());
        }
        if v.len() > MAX_EMAIL_LEN {
            return Err(format!(
                "Email must be {} characters or less",
                MAX_EMAIL_LEN
            ));
        }
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub qualification: Option<String>,
    pub specialization: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePatientRequest {
    pub id: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub insurance_provider: Option<String>,
    pub insurance_id: Option<String>,
    pub allergies: Option<String>,
    pub medical_history: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateWardBedRequest {
    pub bed_id: String,
    pub status: Option<String>,
    pub bed_type: Option<String>,
    pub daily_rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMedicationRequest {
    pub medication_id: String,
    pub name: Option<String>,
    pub unit_price: Option<f64>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInventoryRequest {
    pub inventory_id: String,
    pub quantity: Option<i64>,
    pub expiry_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLabResultRequest {
    pub order_item_id: String,
    pub result_value: String,
    pub result_notes: Option<String>,
    pub is_abnormal: bool,
}

#[tauri::command]
pub async fn update_my_profile(request: UpdateProfileRequest) -> Result<(), String> {
    let session = guards::authenticated()?;
    let pool = get_pool();

    let emp_id = session
        .employee_id
        .as_ref()
        .ok_or("No employee profile linked")?;

    // Validate input lengths
    validate_optional_str(&request.first_name, MAX_NAME_LEN, "First name")?;
    validate_optional_str(&request.last_name, MAX_NAME_LEN, "Last name")?;
    validate_optional_email(&request.email)?;
    validate_optional_str(&request.phone, MAX_PHONE_LEN, "Phone")?;

    // Only doctors can update qualification/specialization
    let is_doctor = session.role == "doctor" || session.role == "admin";

    let mut sets = Vec::new();
    if request.first_name.is_some() {
        sets.push("first_name = ?");
    }
    if request.last_name.is_some() {
        sets.push("last_name = ?");
    }
    if request.email.is_some() {
        sets.push("email = ?");
    }
    if request.phone.is_some() {
        sets.push("phone = ?");
    }
    if request.qualification.is_some() && is_doctor {
        sets.push("qualification = ?");
    }
    if request.specialization.is_some() && is_doctor {
        sets.push("specialization = ?");
    }

    if sets.is_empty() {
        return Ok(());
    }

    sets.push("updated_at = datetime('now')");
    let query = format!("UPDATE staff SET {} WHERE id = ?", sets.join(", "));

    let mut q = sqlx::query(&query);
    if let Some(ref v) = request.first_name {
        q = q.bind(v);
    }
    if let Some(ref v) = request.last_name {
        q = q.bind(v);
    }
    if let Some(ref v) = request.email {
        q = q.bind(v);
    }
    if let Some(ref v) = request.phone {
        q = q.bind(v);
    }
    if request.qualification.is_some() && is_doctor {
        if let Some(ref v) = request.qualification {
            q = q.bind(v);
        }
    }
    if request.specialization.is_some() && is_doctor {
        if let Some(ref v) = request.specialization {
            q = q.bind(v);
        }
    }
    q = q.bind(emp_id);

    q.execute(pool)
        .await
        .map_err(|_| "Failed to update profile".to_string())?;

    log_audit(&session, "update_profile", "staff", Some(emp_id), None).await;
    Ok(())
}

#[tauri::command]
pub async fn update_patient(request: UpdatePatientRequest) -> Result<(), String> {
    let session = guards::doctor_or_nurse()?;
    let pool = get_pool();

    if request.id.trim().is_empty() {
        return Err("Patient ID is required".into());
    }

    // Validate input lengths
    validate_optional_str(&request.first_name, MAX_NAME_LEN, "First name")?;
    validate_optional_str(&request.last_name, MAX_NAME_LEN, "Last name")?;
    validate_optional_str(&request.phone, MAX_PHONE_LEN, "Phone")?;
    validate_optional_email(&request.email)?;
    validate_optional_str(&request.address, MAX_ADDRESS_LEN, "Address")?;
    validate_optional_str(
        &request.emergency_contact_name,
        MAX_CONTACT_NAME_LEN,
        "Emergency contact name",
    )?;
    validate_optional_str(
        &request.emergency_contact_phone,
        MAX_PHONE_LEN,
        "Emergency contact phone",
    )?;
    validate_optional_str(&request.allergies, MAX_TEXT_LEN, "Allergies")?;
    validate_optional_str(
        &request.medical_history,
        MAX_LONG_TEXT_LEN,
        "Medical history",
    )?;

    let mut sets = Vec::new();
    if request.first_name.is_some() {
        sets.push("first_name = ?");
    }
    if request.last_name.is_some() {
        sets.push("last_name = ?");
    }
    if request.phone.is_some() {
        sets.push("phone = ?");
    }
    if request.email.is_some() {
        sets.push("email = ?");
    }
    if request.address.is_some() {
        sets.push("address = ?");
    }
    if request.emergency_contact_name.is_some() {
        sets.push("emergency_contact_name = ?");
    }
    if request.emergency_contact_phone.is_some() {
        sets.push("emergency_contact_phone = ?");
    }
    if request.insurance_provider.is_some() {
        sets.push("insurance_provider = ?");
    }
    if request.insurance_id.is_some() {
        sets.push("insurance_id = ?");
    }
    if request.allergies.is_some() {
        sets.push("allergies = ?");
    }
    if request.medical_history.is_some() {
        sets.push("medical_history = ?");
    }

    if sets.is_empty() {
        return Ok(());
    }

    sets.push("updated_at = datetime('now')");
    let query = format!("UPDATE patients SET {} WHERE id = ?", sets.join(", "));

    let mut q = sqlx::query(&query);
    if let Some(ref v) = request.first_name {
        q = q.bind(v);
    }
    if let Some(ref v) = request.last_name {
        q = q.bind(v);
    }
    if let Some(ref v) = request.phone {
        q = q.bind(v);
    }
    if let Some(ref v) = request.email {
        q = q.bind(v);
    }
    if let Some(ref v) = request.address {
        q = q.bind(v);
    }
    if let Some(ref v) = request.emergency_contact_name {
        q = q.bind(v);
    }
    if let Some(ref v) = request.emergency_contact_phone {
        q = q.bind(v);
    }
    if let Some(ref v) = request.insurance_provider {
        q = q.bind(v);
    }
    if let Some(ref v) = request.insurance_id {
        q = q.bind(v);
    }
    if let Some(ref v) = request.allergies {
        q = q.bind(v);
    }
    if let Some(ref v) = request.medical_history {
        q = q.bind(v);
    }
    q = q.bind(&request.id);

    let affected = q
        .execute(pool)
        .await
        .map_err(|_| "Failed to update patient".to_string())?
        .rows_affected();
    if affected == 0 {
        return Err("Patient not found".into());
    }

    log_audit(&session, "update", "patient", Some(&request.id), None).await;
    Ok(())
}

#[tauri::command]
pub async fn update_bed(request: UpdateWardBedRequest) -> Result<(), String> {
    let session = guards::authenticated()?;
    let pool = get_pool();

    // Only nurse, admin can update beds
    let role = session.role.as_str();
    if role != "admin" && role != "nurse" {
        return Err("Only nurses and admins can update bed information".into());
    }

    // Validate status if provided
    if let Some(ref status) = request.status {
        let valid_statuses = [
            "available",
            "occupied",
            "reserved",
            "cleaning",
            "maintenance",
        ];
        if !valid_statuses.contains(&status.as_str()) {
            return Err("Invalid bed status".into());
        }
    }

    // Validate daily_rate if provided
    if let Some(rate) = request.daily_rate {
        if rate < 0.0 {
            return Err("Daily rate cannot be negative".into());
        }
    }

    let mut sets = Vec::new();
    if request.status.is_some() {
        sets.push("status = ?");
    }
    if request.bed_type.is_some() {
        sets.push("bed_type = ?");
    }
    if request.daily_rate.is_some() {
        sets.push("daily_rate = ?");
    }

    if sets.is_empty() {
        return Ok(());
    }

    let query = format!("UPDATE beds SET {} WHERE id = ?", sets.join(", "));
    let mut q = sqlx::query(&query);
    if let Some(ref v) = request.status {
        q = q.bind(v);
    }
    if let Some(ref v) = request.bed_type {
        q = q.bind(v);
    }
    if let Some(v) = request.daily_rate {
        q = q.bind(v);
    }
    q = q.bind(&request.bed_id);

    let affected = q
        .execute(pool)
        .await
        .map_err(|_| "Failed to update bed".to_string())?
        .rows_affected();
    if affected == 0 {
        return Err("Bed not found".into());
    }

    log_audit(&session, "update", "bed", Some(&request.bed_id), None).await;
    Ok(())
}

#[tauri::command]
pub async fn update_medication(request: UpdateMedicationRequest) -> Result<(), String> {
    let session = guards::authenticated()?;
    let pool = get_pool();

    // Only pharmacist, admin can update medications
    let role = session.role.as_str();
    if role != "admin" && role != "pharmacist" {
        return Err("Only pharmacists and admins can update medications".into());
    }

    // Validate unit_price if provided
    if let Some(price) = request.unit_price {
        if price < 0.0 {
            return Err("Unit price cannot be negative".into());
        }
    }

    let mut sets = Vec::new();
    if request.name.is_some() {
        sets.push("name = ?");
    }
    if request.unit_price.is_some() {
        sets.push("unit_price = ?");
    }
    if request.is_active.is_some() {
        sets.push("is_active = ?");
    }

    if sets.is_empty() {
        return Ok(());
    }

    let query = format!("UPDATE medications SET {} WHERE id = ?", sets.join(", "));
    let mut q = sqlx::query(&query);
    if let Some(ref v) = request.name {
        q = q.bind(v);
    }
    if let Some(v) = request.unit_price {
        q = q.bind(v);
    }
    if let Some(v) = request.is_active {
        q = q.bind(v as i64);
    }
    q = q.bind(&request.medication_id);

    let affected = q
        .execute(pool)
        .await
        .map_err(|_| "Failed to update medication".to_string())?
        .rows_affected();
    if affected == 0 {
        return Err("Medication not found".into());
    }

    log_audit(
        &session,
        "update",
        "medication",
        Some(&request.medication_id),
        None,
    )
    .await;
    Ok(())
}

#[tauri::command]
pub async fn update_inventory(request: UpdateInventoryRequest) -> Result<(), String> {
    let session = guards::authenticated()?;
    let pool = get_pool();

    // Only pharmacist, admin can update inventory
    let role = session.role.as_str();
    if role != "admin" && role != "pharmacist" {
        return Err("Only pharmacists and admins can update inventory".into());
    }

    // Validate quantity if provided
    if let Some(qty) = request.quantity {
        if qty < 0 {
            return Err("Quantity cannot be negative".into());
        }
    }

    let mut sets = Vec::new();
    if request.quantity.is_some() {
        sets.push("quantity = ?");
    }
    if request.expiry_date.is_some() {
        sets.push("expiry_date = ?");
    }

    if sets.is_empty() {
        return Ok(());
    }

    let query = format!("UPDATE inventory SET {} WHERE id = ?", sets.join(", "));
    let mut q = sqlx::query(&query);
    if let Some(v) = request.quantity {
        q = q.bind(v);
    }
    if let Some(ref v) = request.expiry_date {
        q = q.bind(v);
    }
    q = q.bind(&request.inventory_id);

    let affected = q
        .execute(pool)
        .await
        .map_err(|_| "Failed to update inventory".to_string())?
        .rows_affected();
    if affected == 0 {
        return Err("Inventory item not found".into());
    }

    log_audit(
        &session,
        "update",
        "inventory",
        Some(&request.inventory_id),
        None,
    )
    .await;
    Ok(())
}

#[tauri::command]
pub async fn update_lab_result_entry(request: UpdateLabResultRequest) -> Result<(), String> {
    let session = guards::authenticated()?;
    let pool = get_pool();

    // Only lab_tech, admin can update lab results
    let role = session.role.as_str();
    if role != "admin" && role != "lab_tech" {
        return Err("Only lab technicians and admins can update lab results".into());
    }

    sqlx::query(
        "UPDATE lab_order_items SET result_value = ?, result_notes = ?, is_abnormal = ?, completed_at = datetime('now'), completed_by = ? WHERE id = ?"
    )
    .bind(&request.result_value)
    .bind(&request.result_notes)
    .bind(request.is_abnormal as i64)
    .bind(session.employee_id.as_deref().unwrap_or(""))
    .bind(&request.order_item_id)
    .execute(pool)
    .await
    .map_err(|_| "Failed to update lab result".to_string())?;

    log_audit(
        &session,
        "update_result",
        "lab_order_item",
        Some(&request.order_item_id),
        None,
    )
    .await;
    Ok(())
}
