use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ward {
    pub id: String,
    pub name: String,
    pub floor: i64,
    pub ward_type: String,
    pub is_active: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bed {
    pub id: String,
    pub ward_id: String,
    pub room_number: String,
    pub bed_number: String,
    pub bed_type: String,
    pub status: String,
    pub daily_rate: f64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BedWithWard {
    pub bed: Bed,
    pub ward_name: String,
    pub ward_type: String,
    pub patient_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Admission {
    pub id: String,
    pub patient_id: String,
    pub bed_id: String,
    pub doctor_id: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct WardOccupancy {
    pub ward_id: String,
    pub ward_name: String,
    pub total_beds: i64,
    pub occupied: i64,
    pub available: i64,
    pub reserved: i64,
    pub occupancy_rate: f64,
}
