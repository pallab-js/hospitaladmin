use crate::db::get_pool;

pub async fn generate_patient_uid() -> Result<String, String> {
    let pool = get_pool();
    let next: i64 = sqlx::query_scalar(
        "UPDATE counters SET current_value = current_value + 1 WHERE name = 'patient_uid' RETURNING current_value"
    )
    .fetch_one(pool)
    .await
    .map_err(|_| "Failed to generate patient UID".to_string())?;
    Ok(format!("HMS-{:05}", next))
}

pub async fn generate_invoice_number() -> Result<String, String> {
    let pool = get_pool();
    let next: i64 = sqlx::query_scalar(
        "UPDATE counters SET current_value = current_value + 1 WHERE name = 'invoice_number' RETURNING current_value"
    )
    .fetch_one(pool)
    .await
    .map_err(|_| "Failed to generate invoice number".to_string())?;
    Ok(format!("INV-{:06}", next))
}
