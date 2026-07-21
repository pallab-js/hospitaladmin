use super::session::{require_session, require_role, Session};

pub fn authenticated() -> Result<Session, String> {
    require_session()
}

pub fn doctor_only() -> Result<Session, String> {
    let session = require_session()?;
    if session.role != "doctor" && session.role != "management" {
        return Err("Doctor access required".to_string());
    }
    Ok(session)
}

pub fn management_only() -> Result<Session, String> {
    require_role("management")
}

pub fn staff_or_above() -> Result<Session, String> {
    let session = require_session()?;
    if session.role == "doctor" {
        return Err("Staff access required".to_string());
    }
    Ok(session)
}
