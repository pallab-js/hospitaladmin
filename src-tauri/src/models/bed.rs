use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct WardOccupancy {
    pub ward_id: String,
    pub ward_name: String,
    pub total_beds: i64,
    pub occupied: i64,
    pub available: i64,
    pub reserved: i64,
    pub occupancy_rate: f64,
}
