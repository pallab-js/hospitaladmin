use crate::db;
use crate::auth::guards;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct InvoiceRow {
    pub id: String,
    pub invoice_number: String,
    pub patient_id: String,
    pub admission_id: Option<String>,
    pub invoice_date: String,
    pub subtotal: f64,
    pub tax: f64,
    pub discount: f64,
    pub total: f64,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[tauri::command]
pub async fn get_invoices() -> Result<Vec<InvoiceRow>, String> {
    guards::authenticated()?;
    let pool = db::get_pool();
    let rows = sqlx::query_as::<_, InvoiceRow>(
        "SELECT id, invoice_number, patient_id, admission_id, invoice_date, subtotal, tax, discount, total, status, notes, created_at, updated_at FROM invoices ORDER BY created_at DESC LIMIT 20"
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve invoices".to_string())?;
    Ok(rows)
}
