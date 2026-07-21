use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LabTest {
    pub id: String,
    pub name: String,
    pub code: String,
    pub category: String,
    pub normal_range: Option<String>,
    pub unit: Option<String>,
    pub price: f64,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LabOrder {
    pub id: String,
    pub appointment_id: Option<String>,
    pub patient_id: String,
    pub doctor_id: String,
    pub order_date: String,
    pub priority: String,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LabOrderItem {
    pub id: String,
    pub lab_order_id: String,
    pub test_id: String,
    pub result_value: Option<String>,
    pub result_notes: Option<String>,
    pub is_abnormal: Option<bool>,
    pub completed_at: Option<String>,
    pub completed_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LabOrderWithDetails {
    pub order: LabOrder,
    pub patient_name: String,
    pub patient_uid: String,
    pub doctor_name: String,
    pub items: Vec<LabOrderItemWithTest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LabOrderItemWithTest {
    pub item: LabOrderItem,
    pub test_name: String,
    pub test_code: String,
    pub normal_range: Option<String>,
    pub unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLabOrderRequest {
    pub patient_id: String,
    pub doctor_id: String,
    pub appointment_id: Option<String>,
    pub priority: Option<String>,
    pub notes: Option<String>,
    pub test_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLabResultRequest {
    pub item_id: String,
    pub result_value: String,
    pub result_notes: Option<String>,
    pub is_abnormal: bool,
}
