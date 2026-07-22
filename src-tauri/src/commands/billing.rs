use crate::auth::guards;
use crate::db;
use crate::models::billing::Invoice;

#[tauri::command]
pub async fn get_invoices() -> Result<Vec<Invoice>, String> {
    guards::authenticated()?;
    let pool = db::get_pool();
    sqlx::query_as::<_, Invoice>(
        "SELECT id, invoice_number, patient_id, admission_id, invoice_date, subtotal, tax, discount, total, status, notes, created_at, updated_at FROM invoices ORDER BY created_at DESC LIMIT 20"
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve invoices".to_string())
}
