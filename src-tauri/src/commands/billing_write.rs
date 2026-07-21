use uuid::Uuid;
use sqlx::Row;
use crate::db::get_pool;
use crate::auth::guards;
use crate::utils::audit::log_audit;

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct InvoiceWithPatient {
    pub id: String,
    pub invoice_number: String,
    pub patient_id: String,
    pub patient_name: String,
    pub patient_uid: String,
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

#[derive(serde::Deserialize)]
pub struct CreateInvoiceRequest {
    pub patient_id: String,
    pub admission_id: Option<String>,
    pub items: Vec<CreateInvoiceItemRequest>,
    pub tax_rate: Option<f64>,
    pub discount: Option<f64>,
    pub notes: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct CreateInvoiceItemRequest {
    pub description: String,
    pub category: String,
    pub quantity: Option<i64>,
    pub unit_price: f64,
    pub reference_id: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct RecordPaymentRequest {
    pub invoice_id: String,
    pub amount: f64,
    pub payment_method: String,
    pub reference_number: Option<String>,
}

async fn generate_invoice_number() -> Result<String, String> {
    let pool = get_pool();
    let max_num: Option<String> = sqlx::query_scalar("SELECT MAX(invoice_number) FROM invoices")
        .fetch_optional(pool)
        .await
        .map_err(|_| "Failed to generate invoice number".to_string())?;

    let next = match max_num {
        Some(num) => {
            let n: i64 = num.strip_prefix("INV-")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            n + 1
        }
        None => 1,
    };
    Ok(format!("INV-{:06}", next))
}

#[tauri::command]
pub async fn create_invoice(request: CreateInvoiceRequest) -> Result<InvoiceWithPatient, String> {
    let session = guards::authenticated()?;
    let pool = get_pool();
    let id = Uuid::new_v4().to_string();
    let invoice_number = generate_invoice_number().await?;
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let subtotal: f64 = request.items.iter().map(|i| i.unit_price * (i.quantity.unwrap_or(1) as f64)).sum();
    let tax_rate = request.tax_rate.unwrap_or(0.1);
    let tax = subtotal * tax_rate;
    let discount = request.discount.unwrap_or(0.0);
    let total = subtotal + tax - discount;

    sqlx::query(
        "INSERT INTO invoices (id, invoice_number, patient_id, admission_id, invoice_date, subtotal, tax, discount, total, notes) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&invoice_number)
    .bind(&request.patient_id)
    .bind(&request.admission_id)
    .bind(&today)
    .bind(subtotal)
    .bind(tax)
    .bind(discount)
    .bind(total)
    .bind(&request.notes)
    .execute(pool)
    .await
    .map_err(|_| "Failed to create invoice".to_string())?;

    for item in &request.items {
        let item_id = Uuid::new_v4().to_string();
        let item_total = item.unit_price * (item.quantity.unwrap_or(1) as f64);
        sqlx::query(
            "INSERT INTO invoice_items (id, invoice_id, description, category, quantity, unit_price, total, reference_id) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&item_id)
        .bind(&id)
        .bind(&item.description)
        .bind(&item.category)
        .bind(item.quantity.unwrap_or(1))
        .bind(item.unit_price)
        .bind(item_total)
        .bind(&item.reference_id)
        .execute(pool)
        .await
        .map_err(|_| "Failed to add invoice item".to_string())?;
    }

    log_audit(&session, "create", "invoice", Some(&id), Some(&format!("patient={} total={}", request.patient_id, total))).await;

    get_invoice_with_patient(&id).await
}

#[tauri::command]
pub async fn record_payment(request: RecordPaymentRequest) -> Result<(), String> {
    let session = guards::authenticated()?;
    let pool = get_pool();
    let id = Uuid::new_v4().to_string();
    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let valid_methods = ["cash", "card", "insurance", "upi", "other"];
    if !valid_methods.contains(&request.payment_method.as_str()) {
        return Err("Invalid payment method".to_string());
    }
    if request.amount <= 0.0 {
        return Err("Payment amount must be positive".to_string());
    }

    sqlx::query(
        "INSERT INTO payments (id, invoice_id, amount, payment_method, payment_date, reference_number, received_by) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&request.invoice_id)
    .bind(request.amount)
    .bind(&request.payment_method)
    .bind(&today)
    .bind(&request.reference_number)
    .bind(&session.user_id)
    .execute(pool)
    .await
    .map_err(|_| "Failed to record payment".to_string())?;

    let total_paid: f64 = sqlx::query_scalar("SELECT COALESCE(SUM(amount), 0) FROM payments WHERE invoice_id = ?")
        .bind(&request.invoice_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0.0);

    let invoice_total: f64 = sqlx::query_scalar("SELECT total FROM invoices WHERE id = ?")
        .bind(&request.invoice_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0.0);

    let new_status = if total_paid >= invoice_total {
        "paid"
    } else if total_paid > 0.0 {
        "partial"
    } else {
        "pending"
    };

    sqlx::query("UPDATE invoices SET status = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(new_status)
        .bind(&request.invoice_id)
        .execute(pool)
        .await
        .map_err(|_| "Failed to update invoice status".to_string())?;

    log_audit(&session, "record_payment", "payment", Some(&id), Some(&format!("invoice={} amount={}", request.invoice_id, request.amount))).await;
    Ok(())
}

async fn get_invoice_with_patient(id: &str) -> Result<InvoiceWithPatient, String> {
    let pool = get_pool();
    let row = sqlx::query(
        r#"SELECT i.*, p.first_name || ' ' || p.last_name as patient_name, p.patient_uid
        FROM invoices i
        JOIN patients p ON i.patient_id = p.id
        WHERE i.id = ?"#
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|_| "Failed to retrieve invoice".to_string())?;

    match row {
        Some(r) => Ok(InvoiceWithPatient {
            id: r.get("id"),
            invoice_number: r.get("invoice_number"),
            patient_id: r.get("patient_id"),
            patient_name: r.get("patient_name"),
            patient_uid: r.get("patient_uid"),
            admission_id: r.get("admission_id"),
            invoice_date: r.get("invoice_date"),
            subtotal: r.get("subtotal"),
            tax: r.get("tax"),
            discount: r.get("discount"),
            total: r.get("total"),
            status: r.get("status"),
            notes: r.get("notes"),
            created_at: r.get("created_at"),
            updated_at: r.get("updated_at"),
        }),
        None => Err("Invoice not found".to_string()),
    }
}
