use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Appointment {
    pub id: String,
    pub patient_id: String,
    pub doctor_id: String,
    pub department_id: Option<String>,
    pub appointment_date: String,
    pub appointment_time: String,
    pub duration_minutes: i64,
    pub status: String,
    pub visit_type: String,
    pub reason: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppointmentRequest {
    pub patient_id: String,
    pub doctor_id: String,
    pub department_id: Option<String>,
    pub appointment_date: String,
    pub appointment_time: String,
    pub duration_minutes: Option<i64>,
    pub visit_type: Option<String>,
    pub reason: Option<String>,
}

impl CreateAppointmentRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.patient_id.trim().is_empty() {
            return Err("Patient is required".to_string());
        }
        if self.doctor_id.trim().is_empty() {
            return Err("Doctor is required".to_string());
        }
        if self.appointment_date.trim().is_empty() {
            return Err("Appointment date is required".to_string());
        }
        if chrono::NaiveDate::parse_from_str(&self.appointment_date, "%Y-%m-%d").is_err() {
            return Err("Appointment date must be in YYYY-MM-DD format".to_string());
        }
        if self.appointment_time.trim().is_empty() {
            return Err("Appointment time is required".to_string());
        }
        if chrono::NaiveTime::parse_from_str(&self.appointment_time, "%H:%M").is_err() {
            return Err("Appointment time must be in HH:MM format".to_string());
        }
        if let Some(dur) = self.duration_minutes {
            if !(5..=480).contains(&dur) {
                return Err("Duration must be between 5 and 480 minutes".to_string());
            }
        }
        if let Some(ref vt) = self.visit_type {
            let valid_types = ["consultation", "follow_up", "emergency", "walk_in"];
            if !valid_types.contains(&vt.as_str()) {
                return Err("Invalid visit type".to_string());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentWithDetails {
    pub appointment: Appointment,
    pub patient_name: String,
    pub patient_uid: String,
    pub doctor_name: String,
    pub department_name: Option<String>,
}
