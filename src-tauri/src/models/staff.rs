use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
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
