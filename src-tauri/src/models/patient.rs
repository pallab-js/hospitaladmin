use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Patient {
    pub id: String,
    pub patient_uid: String,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub blood_group: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub insurance_provider: Option<String>,
    pub insurance_id: Option<String>,
    pub allergies: Option<String>,
    pub medical_history: Option<String>,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePatientRequest {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub gender: String,
    pub blood_group: Option<String>,
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

impl CreatePatientRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.first_name.trim().is_empty() {
            return Err("First name is required".to_string());
        }
        if self.first_name.len() > 100 {
            return Err("First name must be 100 characters or less".to_string());
        }
        if self.last_name.trim().is_empty() {
            return Err("Last name is required".to_string());
        }
        if self.last_name.len() > 100 {
            return Err("Last name must be 100 characters or less".to_string());
        }
        if self.date_of_birth.trim().is_empty() {
            return Err("Date of birth is required".to_string());
        }
        if chrono::NaiveDate::parse_from_str(&self.date_of_birth, "%Y-%m-%d").is_err() {
            return Err("Date of birth must be in YYYY-MM-DD format".to_string());
        }
        let valid_genders = ["male", "female", "other"];
        if !valid_genders.contains(&self.gender.as_str()) {
            return Err("Gender must be male, female, or other".to_string());
        }
        if let Some(ref email) = self.email {
            if !email.is_empty() && !email.contains('@') {
                return Err("Invalid email format".to_string());
            }
        }
        if let Some(ref phone) = self.phone {
            if phone.len() > 20 {
                return Err("Phone number must be 20 characters or less".to_string());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PatientSearchParams {
    pub query: Option<String>,
    pub gender: Option<String>,
    pub blood_group: Option<String>,
    pub is_active: Option<bool>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}
