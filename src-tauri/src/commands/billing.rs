use crate::auth::guards;
use crate::db;
use crate::models::billing::Invoice;

#[tauri::command]
pub async fn get_invoices(page: Option<i64>, limit: Option<i64>) -> Result<Vec<Invoice>, String> {
    guards::billing_only()?;
    let pool = db::get_pool();
    let page = page.unwrap_or(1).max(1);
    let limit = limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;

    sqlx::query_as::<_, Invoice>(
        "SELECT id, invoice_number, patient_id, admission_id, invoice_date, subtotal, tax, discount, total, status, notes, created_at, updated_at FROM invoices ORDER BY created_at DESC LIMIT ? OFFSET ?"
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve invoices".to_string())
}
