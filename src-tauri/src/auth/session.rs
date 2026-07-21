use serde::{Deserialize, Serialize};
use std::sync::{LazyLock, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

const SESSION_EXPIRY_SECONDS: u64 = 3600; // 1 hour

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub user_id: String,
    pub username: String,
    pub role: String,
    pub employee_id: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
struct SessionEntry {
    session: Session,
    expires_at: u64,
}

static CURRENT_SESSION: LazyLock<Mutex<Option<SessionEntry>>> = LazyLock::new(|| Mutex::new(None));

fn now_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn set_session(session: Session) {
    let entry = SessionEntry {
        session: Session {
            created_at: now_epoch(),
            ..session
        },
        expires_at: now_epoch() + SESSION_EXPIRY_SECONDS,
    };
    if let Ok(mut s) = CURRENT_SESSION.lock() {
        *s = Some(entry);
    }
}

pub fn get_session() -> Option<Session> {
    let s = CURRENT_SESSION.lock().ok()?;
    match &*s {
        Some(entry) => {
            if now_epoch() > entry.expires_at {
                None
            } else {
                Some(entry.session.clone())
            }
        }
        None => None,
    }
}

/// Refresh session expiry on activity. Returns true if refreshed.
pub fn refresh_session() -> bool {
    if let Ok(mut s) = CURRENT_SESSION.lock() {
        if let Some(ref mut entry) = *s {
            if now_epoch() <= entry.expires_at {
                entry.expires_at = now_epoch() + SESSION_EXPIRY_SECONDS;
                return true;
            }
        }
    }
    false
}

pub fn clear_session() {
    if let Ok(mut s) = CURRENT_SESSION.lock() {
        *s = None;
    }
}

pub fn require_session() -> Result<Session, String> {
    get_session().ok_or_else(|| "Session expired or not authenticated".to_string())
}

// ponytail: management always bypasses role checks — documented in guards.rs role matrix
pub fn require_role(role: &str) -> Result<Session, String> {
    let session = require_session()?;
    if session.role != role && session.role != "management" {
        return Err("Insufficient permissions".to_string());
    }
    Ok(session)
}
