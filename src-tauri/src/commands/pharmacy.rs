use crate::auth::guards;
use crate::db;
use crate::models::medication::Medication;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct StockRow {
    pub medication_id: String,
    pub quantity: i64,
    pub low_threshold: i64,
    pub expiry_date: String,
}

#[tauri::command]
pub async fn get_medications() -> Result<Vec<Medication>, String> {
    guards::authenticated()?;
    let pool = db::get_pool();
    sqlx::query_as::<_, Medication>(
        "SELECT id, name, generic_name, category, dosage_form, strength, manufacturer, unit_price, is_active, created_at FROM medications WHERE is_active = 1 ORDER BY name"
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve medications".to_string())
}

#[tauri::command]
pub async fn get_medication_stock() -> Result<Vec<StockRow>, String> {
    guards::authenticated()?;
    let pool = db::get_pool();
    sqlx::query_as::<_, StockRow>(
        "SELECT medication_id, quantity, low_threshold, expiry_date FROM inventory ORDER BY expiry_date"
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve medication stock".to_string())
}
