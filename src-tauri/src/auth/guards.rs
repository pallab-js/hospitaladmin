use super::session::{refresh_session, require_role, require_session, Session};

// Role matrix (admin bypasses all checks):
// | Capability              | doctor | nurse | receptionist | pharmacist | lab_tech | billing_staff | admin |
// |-------------------------|:------:|:-----:|:------------:|:----------:|:--------:|:-------------:|:-----:|
// | read own appointments   |   ✓    |   ✗   |      ✓       |     ✗      |    ✗     |      ✗        |   ✓   |
// | create appointments     |   ✓    |   ✗   |      ✓       |     ✗      |    ✗     |      ✗        |   ✓   |
// | create prescriptions    |   ✓    |   ✗   |      ✗       |     ✗      |    ✗     |      ✗        |   ✓   |
// | create lab orders       |   ✓    |   ✗   |      ✗       |     ✗      |    ✗     |      ✗        |   ✓   |
// | create admissions       |   ✓    |   ✓   |      ✗       |     ✗      |    ✗     |      ✗        |   ✓   |
// | discharge patients      |   ✓    |   ✓   |      ✗       |     ✗      |    ✗     |      ✗        |   ✓   |
// | update lab results      |   ✗    |   ✗   |      ✗       |     ✗      |    ✓     |      ✗        |   ✓   |
// | update inventory        |   ✗    |   ✗   |      ✗       |     ✓      |    ✗     |      ✗        |   ✓   |
// | update bed status       |   ✗    |   ✓   |      ✗       |     ✗      |    ✗     |      ✗        |   ✓   |
// | record payment          |   ✗    |   ✗   |      ✗       |     ✗      |    ✗     |      ✓        |   ✓   |
// | create invoices         |   ✗    |   ✗   |      ✗       |     ✗      |    ✗     |      ✓        |   ✓   |
// | view billing            |   ✗    |   ✗   |      ✗       |     ✗      |    ✗     |      ✓        |   ✓   |
// | view revenue chart      |   ✗    |   ✗   |      ✗       |     ✗      |    ✗     |      ✗        |   ✓   |
// | monthly trends          |   ✗    |   ✗   |      ✗       |     ✗      |    ✗     |      ✗        |   ✓   |
// | all other reads         |   ✓    |   ✓   |      ✓       |     ✓      |    ✓     |      ✓        |   ✓   |

pub fn authenticated() -> Result<Session, String> {
    let session = require_session()?;
    refresh_session(&session.user_id);
    Ok(session)
}

pub fn doctor_only() -> Result<Session, String> {
    let session = require_session()?;
    if session.role != "doctor" && session.role != "admin" {
        return Err("Doctor access required".to_string());
    }
    refresh_session(&session.user_id);
    Ok(session)
}

pub fn admin_only() -> Result<Session, String> {
    let session = require_role("admin")?;
    refresh_session(&session.user_id);
    Ok(session)
}

pub fn lab_tech_only() -> Result<Session, String> {
    let session = require_session()?;
    if session.role != "lab_tech" && session.role != "admin" {
        return Err("Lab technician access required".to_string());
    }
    refresh_session(&session.user_id);
    Ok(session)
}

pub fn pharmacist_only() -> Result<Session, String> {
    let session = require_session()?;
    if session.role != "pharmacist" && session.role != "admin" {
        return Err("Pharmacist access required".to_string());
    }
    refresh_session(&session.user_id);
    Ok(session)
}

pub fn billing_only() -> Result<Session, String> {
    let session = require_session()?;
    if session.role != "billing_staff" && session.role != "admin" {
        return Err("Billing staff access required".to_string());
    }
    refresh_session(&session.user_id);
    Ok(session)
}

#[allow(dead_code)]
pub fn nurse_only() -> Result<Session, String> {
    let session = require_session()?;
    if session.role != "nurse" && session.role != "admin" {
        return Err("Nurse access required".to_string());
    }
    refresh_session(&session.user_id);
    Ok(session)
}

pub fn doctor_or_nurse() -> Result<Session, String> {
    let session = require_session()?;
    if session.role != "doctor" && session.role != "nurse" && session.role != "admin" {
        return Err("Doctor or nurse access required".to_string());
    }
    refresh_session(&session.user_id);
    Ok(session)
}
