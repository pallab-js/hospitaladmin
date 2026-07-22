use super::session::{refresh_session, require_role, require_session, Session};

// Role matrix (management bypasses all checks):
// | Capability              | doctor | staff | management |
// |-------------------------|:------:|:-----:|:----------:|
// | read own appointments   |   ✓    |   ✗   |     ✓      |
// | create appointments     |   ✓    |   ✗   |     ✓      |
// | create prescriptions    |   ✓    |   ✗   |     ✓      |
// | create lab orders       |   ✓    |   ✗   |     ✓      |
// | create admissions       |   ✓    |   ✗   |     ✓      |
// | discharge patients      |   ✓    |   ✗   |     ✓      |
// | record payment          |   ✗    |   ✓   |     ✓      |
// | create invoices         |   ✗    |   ✓   |     ✓      |
// | view billing            |   ✗    |   ✓   |     ✓      |
// | view revenue chart      |   ✗    |   ✗   |     ✓      |
// | monthly trends          |   ✗    |   ✗   |     ✓      |
// | all other reads         |   ✓    |   ✓   |     ✓      |

pub fn authenticated() -> Result<Session, String> {
    let session = require_session()?;
    refresh_session();
    Ok(session)
}

pub fn doctor_only() -> Result<Session, String> {
    let session = require_session()?;
    if session.role != "doctor" && session.role != "management" {
        return Err("Doctor access required".to_string());
    }
    refresh_session();
    Ok(session)
}

pub fn management_only() -> Result<Session, String> {
    let session = require_role("management")?;
    refresh_session();
    Ok(session)
}
