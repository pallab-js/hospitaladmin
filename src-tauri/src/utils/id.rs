pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn generate_patient_uid(count: i64) -> String {
    format!("HMS-{:05}", count + 1)
}

pub fn generate_invoice_number(count: i64) -> String {
    format!("INV-{:06}", count + 1)
}
