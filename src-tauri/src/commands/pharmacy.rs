use crate::db;
use crate::auth::guards;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct MedicationRow {
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

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct StockRow {
    pub medication_id: String,
    pub quantity: i64,
    pub low_threshold: i64,
    pub expiry_date: String,
}

#[tauri::command]
pub async fn get_medications() -> Result<Vec<MedicationRow>, String> {
    guards::authenticated()?;
    let pool = db::get_pool();
    let rows = sqlx::query_as::<_, MedicationRow>(
        "SELECT id, name, generic_name, category, dosage_form, strength, manufacturer, unit_price, is_active, created_at FROM medications WHERE is_active = 1 ORDER BY name"
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve medications".to_string())?;
    Ok(rows)
}

#[tauri::command]
pub async fn get_medication_stock() -> Result<Vec<StockRow>, String> {
    guards::authenticated()?;
    let pool = db::get_pool();
    let rows = sqlx::query_as::<_, StockRow>(
        "SELECT medication_id, quantity, low_threshold, expiry_date FROM inventory ORDER BY expiry_date"
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve medication stock".to_string())?;
    Ok(rows)
}
