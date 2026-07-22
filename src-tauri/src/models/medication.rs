use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Medication {
    pub id: String,
    pub name: String,
    pub generic_name: Option<String>,
    pub category: String,
    pub dosage_form: String,
    pub strength: Option<String>,
    pub manufacturer: Option<String>,
    pub unit_price: f64,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Prescription {
    pub id: String,
    pub appointment_id: Option<String>,
    pub patient_id: String,
    pub doctor_id: String,
    pub prescription_date: String,
    pub notes: Option<String>,
    pub created_at: String,
}
