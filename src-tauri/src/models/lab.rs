use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
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
