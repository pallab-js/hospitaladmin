use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DashboardStats {
    pub total_patients_today: i64,
    pub total_appointments_today: i64,
    pub bed_occupancy_rate: f64,
    pub revenue_today: f64,
    pub pending_lab_orders: i64,
    pub active_admissions: i64,
    pub staff_on_duty: i64,
    pub patients_registered_this_month: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevenueData {
    pub date: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentLoad {
    pub department_name: String,
    pub appointment_count: i64,
    pub patient_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonthlyTrend {
    pub month: String,
    pub patients: i64,
    pub revenue: f64,
    pub admissions: i64,
}
