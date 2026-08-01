use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
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

static SESSIONS: LazyLock<DashMap<String, SessionEntry>> = LazyLock::new(DashMap::new);

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
    SESSIONS.insert(entry.session.user_id.clone(), entry);
}

pub fn get_session() -> Option<Session> {
    // Purge expired sessions on read (cheap sweep)
    let now = now_epoch();
    SESSIONS.retain(|_, entry| now <= entry.expires_at);

    // For the current session model we return the first active session.
    // In a multi-user desktop app, this should be keyed by a window/context ID.
    // For now we iterate — DashMap makes this lock-free.
    SESSIONS.iter().find_map(|entry| {
        if now <= entry.value().expires_at {
            Some(entry.value().session.clone())
        } else {
            None
        }
    })
}

/// Refresh session expiry on activity. Returns true if refreshed.
pub fn refresh_session() -> bool {
    let now = now_epoch();
    let mut refreshed = false;
    for mut entry in SESSIONS.iter_mut() {
        if now <= entry.value().expires_at {
            entry.value_mut().expires_at = now + SESSION_EXPIRY_SECONDS;
            refreshed = true;
        }
    }
    refreshed
}

pub fn clear_user_session(user_id: &str) {
    SESSIONS.remove(user_id);
}

pub fn require_session() -> Result<Session, String> {
    get_session().ok_or_else(|| "Session expired or not authenticated".to_string())
}

// ponytail: admin always bypasses role checks — documented in guards.rs role matrix
pub fn require_role(role: &str) -> Result<Session, String> {
    let session = require_session()?;
    if session.role != role && session.role != "admin" {
        return Err("Insufficient permissions".to_string());
    }
    Ok(session)
}
