use crate::db::get_pool;
use crate::auth::session::Session;

pub async fn log_audit(
    session: &Session,
    action: &str,
    entity_type: &str,
    entity_id: Option<&str>,
    details: Option<&str>,
) {
    if let Err(e) = sqlx::query(
        "INSERT INTO audit_log (user_id, action, entity_type, entity_id, details) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&session.user_id)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(details)
    .execute(get_pool())
    .await
    {
        eprintln!("[audit] FAILED to write audit log: {} (action={}, entity={})", e, action, entity_type);
    }
}
