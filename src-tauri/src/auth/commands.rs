use super::session::{clear_session, get_session, set_session, Session};
use crate::db::get_pool;
use serde::{Deserialize, Serialize};
use sqlx::Row;

const MAX_FAILED_ATTEMPTS: i64 = 5;
const LOCKOUT_MINUTES: i64 = 15;
const HARD_LOCKOUT_MINUTES: i64 = 1440; // 24 hours after 20 failures
const HARD_LOCKOUT_THRESHOLD: i64 = 20;

// ponytail: precomputed dummy hash for timing-equalized not-found branch
const DUMMY_HASH: &str = "$2b$12$YqGKfDwKlVnxqHMFfJzKmOBvN0wzX0Z8U0KzQrZgXfXnKqHvQkZmO";

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub message: String,
    pub user: Option<UserInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub role: String,
    pub employee_id: Option<String>,
    pub full_name: Option<String>,
}

#[tauri::command]
pub async fn login(request: LoginRequest) -> Result<LoginResponse, String> {
    let pool = get_pool();

    let row = sqlx::query(
        "SELECT id, username, password_hash, role, employee_id, failed_attempts, locked_until FROM users WHERE username = ? AND is_active = 1"
    )
    .bind(&request.username)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    match row {
        Some(row) => {
            let locked_until: Option<String> = row.get("locked_until");
            if let Some(ref lock_time) = locked_until {
                if let Ok(lock_dt) =
                    chrono::NaiveDateTime::parse_from_str(lock_time, "%Y-%m-%d %H:%M:%S")
                {
                    if chrono::Local::now().naive_local() < lock_dt {
                        return Ok(LoginResponse {
                            success: false,
                            message: "Account is locked. Try again later.".to_string(),
                            user: None,
                        });
                    }
                }
            }

            let password_hash: String = row.get("password_hash");
            let valid = bcrypt::verify(&request.password, &password_hash)
                .map_err(|e| format!("Verification error: {}", e))?;

            if valid {
                let user_id: String = row.get("id");
                let username: String = row.get("username");
                let role: String = row.get("role");
                let employee_id: Option<String> = row.get("employee_id");

                // Reset failed attempts on success
                sqlx::query(
                    "UPDATE users SET failed_attempts = 0, locked_until = NULL WHERE id = ?",
                )
                .bind(&user_id)
                .execute(pool)
                .await
                .ok();

                let full_name = if let Some(ref emp_id) = employee_id {
                    sqlx::query("SELECT first_name, last_name FROM staff WHERE id = ?")
                        .bind(emp_id)
                        .fetch_optional(pool)
                        .await
                        .ok()
                        .flatten()
                        .map(|r| {
                            let first_name: String = r.get("first_name");
                            let last_name: String = r.get("last_name");
                            format!("{} {}", first_name, last_name)
                        })
                } else {
                    None
                };

                sqlx::query("UPDATE users SET last_login_at = datetime('now') WHERE id = ?")
                    .bind(&user_id)
                    .execute(pool)
                    .await
                    .ok();

                let session = Session {
                    user_id: user_id.clone(),
                    username: username.clone(),
                    role: role.clone(),
                    employee_id: employee_id.clone(),
                    created_at: 0,
                };
                set_session(session);

                Ok(LoginResponse {
                    success: true,
                    message: "Login successful".to_string(),
                    user: Some(UserInfo {
                        id: user_id,
                        username,
                        role,
                        employee_id,
                        full_name,
                    }),
                })
            } else {
                let failed_attempts: i64 = row.get("failed_attempts");
                let new_count = failed_attempts + 1;

                let lock_until = if new_count >= HARD_LOCKOUT_THRESHOLD {
                    Some(
                        (chrono::Local::now() + chrono::Duration::minutes(HARD_LOCKOUT_MINUTES))
                            .format("%Y-%m-%d %H:%M:%S")
                            .to_string(),
                    )
                } else if new_count >= MAX_FAILED_ATTEMPTS {
                    Some(
                        (chrono::Local::now() + chrono::Duration::minutes(LOCKOUT_MINUTES))
                            .format("%Y-%m-%d %H:%M:%S")
                            .to_string(),
                    )
                } else {
                    None
                };

                sqlx::query("UPDATE users SET failed_attempts = ?, locked_until = COALESCE(?, locked_until) WHERE username = ?")
                    .bind(new_count)
                    .bind(&lock_until)
                    .bind(&request.username)
                    .execute(pool)
                    .await
                    .ok();

                // Log failed attempt
                if let Some(ref user_id_val) = row.get::<Option<String>, _>("id") {
                    crate::utils::audit::log_audit(
                        &Session {
                            user_id: user_id_val.clone(),
                            username: request.username.clone(),
                            role: String::new(),
                            employee_id: None,
                            created_at: 0,
                        },
                        "login_failed",
                        "user",
                        Some(user_id_val),
                        Some(&format!("attempts={}", new_count)),
                    )
                    .await;
                }

                Ok(LoginResponse {
                    success: false,
                    message: "Invalid credentials".to_string(),
                    user: None,
                })
            }
        }
        None => {
            // Timing equalization: run dummy bcrypt on not-found branch
            let _ = bcrypt::verify("dummy", DUMMY_HASH);

            Ok(LoginResponse {
                success: false,
                message: "Invalid credentials".to_string(),
                user: None,
            })
        }
    }
}

#[tauri::command]
pub async fn logout() -> Result<(), String> {
    clear_session();
    Ok(())
}

#[tauri::command]
pub async fn get_current_user() -> Result<Option<UserInfo>, String> {
    match get_session() {
        Some(session) => {
            let pool = get_pool();
            let full_name = if let Some(ref emp_id) = session.employee_id {
                sqlx::query("SELECT first_name, last_name FROM staff WHERE id = ?")
                    .bind(emp_id)
                    .fetch_optional(pool)
                    .await
                    .ok()
                    .flatten()
                    .map(|r| {
                        let first_name: String = r.get("first_name");
                        let last_name: String = r.get("last_name");
                        format!("{} {}", first_name, last_name)
                    })
            } else {
                None
            };

            Ok(Some(UserInfo {
                id: session.user_id,
                username: session.username,
                role: session.role,
                employee_id: session.employee_id,
                full_name,
            }))
        }
        None => Ok(None),
    }
}
