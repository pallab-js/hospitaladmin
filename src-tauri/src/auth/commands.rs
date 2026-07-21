use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::db::get_pool;
use super::session::{set_session, clear_session, get_session, Session};

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
        "SELECT id, username, password_hash, role, employee_id FROM users WHERE username = ? AND is_active = 1"
    )
    .bind(&request.username)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    match row {
        Some(row) => {
            let password_hash: String = row.get("password_hash");
            let valid = bcrypt::verify(&request.password, &password_hash)
                .map_err(|e| format!("Verification error: {}", e))?;

            if valid {
                let user_id: String = row.get("id");
                let username: String = row.get("username");
                let role: String = row.get("role");
                let employee_id: Option<String> = row.get("employee_id");

                // Get full name from staff table if employee_id exists
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

                // Update last login
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
                Ok(LoginResponse {
                    success: false,
                    message: "Invalid credentials".to_string(),
                    user: None,
                })
            }
        }
        None => Ok(LoginResponse {
            success: false,
            message: "Invalid credentials".to_string(),
            user: None,
        }),
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
