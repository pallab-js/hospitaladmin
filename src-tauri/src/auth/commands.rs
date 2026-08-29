use super::session::{clear_user_session, get_session, set_session, Session};
use crate::db::get_pool;
use crate::utils::password::validate_password;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub department_id: Option<String>,
    pub qualification: Option<String>,
    pub specialization: Option<String>,
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
    .map_err(|_| "Authentication service unavailable".to_string())?;

    match row {
        Some(row) => {
            let password_hash: String = row.get("password_hash");

            // Always run bcrypt to equalize timing across all branches
            let valid = bcrypt::verify(&request.password, &password_hash)
                .map_err(|_| "Authentication service unavailable".to_string())?;

            // Check lockout AFTER bcrypt to prevent timing side-channel
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
                .map_err(|_| "Failed to reset login attempts".to_string())?;

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
                    .map_err(|_| "Failed to update last login time".to_string())?;

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
                // Atomic increment + lockout calculation in a single query
                sqlx::query(
                    "UPDATE users SET
                        failed_attempts = failed_attempts + 1,
                        locked_until = CASE
                            WHEN failed_attempts + 1 >= ? THEN datetime('now', '+' || ? || ' minutes')
                            WHEN failed_attempts + 1 >= ? THEN datetime('now', '+' || ? || ' minutes')
                            ELSE locked_until
                        END
                    WHERE username = ?",
                )
                .bind(HARD_LOCKOUT_THRESHOLD)
                .bind(HARD_LOCKOUT_MINUTES)
                .bind(MAX_FAILED_ATTEMPTS)
                .bind(LOCKOUT_MINUTES)
                .bind(&request.username)
                .execute(pool)
                .await
                .map_err(|_| "Failed to track login attempt".to_string())?;

                // Log failed attempt
                if let Some(ref user_id_val) = row.get::<Option<String>, _>("id") {
                    let failed_attempts: i64 = row.get("failed_attempts");
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
                        Some(&format!("attempts={}", failed_attempts + 1)),
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
    if let Some(session) = get_session() {
        clear_user_session(&session.user_id);
    }
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

#[tauri::command]
pub async fn register(request: RegisterRequest) -> Result<LoginResponse, String> {
    crate::auth::guards::admin_only()?;
    let pool = get_pool();

    // Validate inputs
    if request.username.trim().is_empty() {
        return Err("Username is required".into());
    }
    if request.username.len() < 3 || request.username.len() > 50 {
        return Err("Username must be 3-50 characters".into());
    }
    if !request
        .username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err("Username must contain only letters, numbers, and underscores".into());
    }
    validate_password(&request.password)?;
    if request.first_name.trim().is_empty() {
        return Err("First name is required".into());
    }
    if request.last_name.trim().is_empty() {
        return Err("Last name is required".into());
    }

    let valid_roles = [
        "doctor",
        "nurse",
        "receptionist",
        "pharmacist",
        "lab_tech",
        "billing_staff",
    ];
    if !valid_roles.contains(&request.role.as_str()) {
        return Err("Invalid role".into());
    }

    // Check username uniqueness
    let existing: Option<String> = sqlx::query_scalar("SELECT id FROM users WHERE username = ?")
        .bind(&request.username)
        .fetch_optional(pool)
        .await
        .map_err(|_| "Registration service unavailable".to_string())?;
    if existing.is_some() {
        return Err("Username already taken".into());
    }

    let user_id = Uuid::new_v4().to_string();
    let staff_id = Uuid::new_v4().to_string();
    let password_hash = bcrypt::hash(&request.password, 12)
        .map_err(|_| "Registration service unavailable".to_string())?;

    let mut tx = pool
        .begin()
        .await
        .map_err(|_| "Registration service unavailable".to_string())?;

    // Create staff record
    sqlx::query(
        "INSERT INTO staff (id, first_name, last_name, role, department_id, email, phone, qualification, specialization, is_active)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 1)"
    )
    .bind(&staff_id)
    .bind(request.first_name.trim())
    .bind(request.last_name.trim())
    .bind(&request.role)
    .bind(&request.department_id)
    .bind(&request.email)
    .bind(&request.phone)
    .bind(&request.qualification)
    .bind(&request.specialization)
    .execute(&mut *tx)
    .await
    .map_err(|_| "Failed to create staff profile".to_string())?;

    // Create user account
    sqlx::query(
        "INSERT INTO users (id, username, password_hash, role, employee_id, is_active)
         VALUES (?, ?, ?, ?, ?, 1)",
    )
    .bind(&user_id)
    .bind(&request.username)
    .bind(&password_hash)
    .bind(&request.role)
    .bind(&staff_id)
    .execute(&mut *tx)
    .await
    .map_err(|_| "Failed to create user account".to_string())?;

    tx.commit()
        .await
        .map_err(|_| "Registration failed".to_string())?;

    let full_name = format!("{} {}", request.first_name.trim(), request.last_name.trim());

    Ok(LoginResponse {
        success: true,
        message: "Registration successful".to_string(),
        user: Some(UserInfo {
            id: user_id,
            username: request.username,
            role: request.role,
            employee_id: Some(staff_id),
            full_name: Some(full_name),
        }),
    })
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[tauri::command]
pub async fn change_password(request: ChangePasswordRequest) -> Result<(), String> {
    let session = crate::auth::guards::authenticated()?;
    let pool = get_pool();

    // Validate new password
    validate_password(&request.new_password)?;

    if request.current_password == request.new_password {
        return Err("New password must be different from current password".into());
    }

    // Fetch current password hash
    let row = sqlx::query("SELECT password_hash FROM users WHERE id = ? AND is_active = 1")
        .bind(&session.user_id)
        .fetch_optional(pool)
        .await
        .map_err(|_| "Service unavailable".to_string())?
        .ok_or("User not found".to_string())?;

    let password_hash: String = row.get("password_hash");

    // Verify current password
    let valid = bcrypt::verify(&request.current_password, &password_hash)
        .map_err(|_| "Service unavailable".to_string())?;

    if !valid {
        return Err("Current password is incorrect".into());
    }

    // Hash new password
    let new_hash =
        bcrypt::hash(&request.new_password, 12).map_err(|_| "Service unavailable".to_string())?;

    // Update password
    sqlx::query("UPDATE users SET password_hash = ?, updated_at = datetime('now') WHERE id = ?")
        .bind(&new_hash)
        .bind(&session.user_id)
        .execute(pool)
        .await
        .map_err(|_| "Failed to update password".to_string())?;

    // Invalidate all sessions for this user except current
    clear_user_session(&session.user_id);

    Ok(())
}
