use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Staff {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub department_id: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub qualification: Option<String>,
    pub specialization: Option<String>,
    pub is_active: bool,
    pub hire_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateStaffRequest {
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub department_id: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub qualification: Option<String>,
    pub specialization: Option<String>,
    pub hire_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Department {
    pub id: String,
    pub name: String,
    pub code: String,
    pub head_doctor_id: Option<String>,
    pub is_active: bool,
    pub created_at: String,
}
