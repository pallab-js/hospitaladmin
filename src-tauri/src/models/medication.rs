use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prescription {
    pub id: String,
    pub appointment_id: Option<String>,
    pub patient_id: String,
    pub doctor_id: String,
    pub prescription_date: String,
    pub notes: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrescriptionItem {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryItem {
    pub id: String,
    pub medication_id: String,
    pub batch_number: String,
    pub quantity: i64,
    pub expiry_date: String,
    pub purchase_price: Option<f64>,
    pub supplier: Option<String>,
    pub received_date: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MedicationWithStock {
    pub medication: Medication,
    pub total_stock: i64,
    pub expiry_alert: bool,
}
