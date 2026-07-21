use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Invoice {
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InvoiceItem {
    pub id: String,
    pub invoice_id: String,
    pub description: String,
    pub category: String,
    pub quantity: i64,
    pub unit_price: f64,
    pub total: f64,
    pub reference_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payment {
    pub id: String,
    pub invoice_id: String,
    pub amount: f64,
    pub payment_method: String,
    pub payment_date: String,
    pub reference_number: Option<String>,
    pub received_by: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvoiceWithDetails {
    pub invoice: Invoice,
    pub patient_name: String,
    pub patient_uid: String,
    pub items: Vec<InvoiceItem>,
    pub payments: Vec<Payment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceRequest {
    pub patient_id: String,
    pub admission_id: Option<String>,
    pub invoice_date: String,
    pub items: Vec<CreateInvoiceItemRequest>,
    pub tax: Option<f64>,
    pub discount: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateInvoiceItemRequest {
    pub description: String,
    pub category: String,
    pub quantity: Option<i64>,
    pub unit_price: f64,
    pub reference_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordPaymentRequest {
    pub invoice_id: String,
    pub amount: f64,
    pub payment_method: String,
    pub payment_date: String,
    pub reference_number: Option<String>,
}
