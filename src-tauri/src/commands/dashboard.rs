use sqlx::Row;
use crate::db::get_pool;
use crate::models::dashboard::*;
use crate::auth::guards;

#[tauri::command]
pub async fn get_dashboard_stats() -> Result<DashboardStats, String> {
    let session = guards::authenticated()?;
    let pool = get_pool();

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let month_start = chrono::Utc::now().format("%Y-%m-01").to_string();

    let (total_patients_today, total_appointments_today, pending_lab_orders, active_admissions) = if session.role == "doctor" {
        if let Some(ref emp_id) = session.employee_id {
            let p: i64 = sqlx::query_scalar(
                "SELECT COUNT(DISTINCT patient_id) FROM appointments WHERE appointment_date = ? AND doctor_id = ?"
            )
            .bind(&today)
            .bind(emp_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let a: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM appointments WHERE appointment_date = ? AND doctor_id = ?"
            )
            .bind(&today)
            .bind(emp_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let l: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM lab_orders WHERE status IN ('ordered', 'in_progress') AND doctor_id = ?"
            )
            .bind(emp_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            let ad: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM admissions WHERE status = 'active' AND doctor_id = ?"
            )
            .bind(emp_id)
            .fetch_one(pool)
            .await
            .unwrap_or(0);

            (p, a, l, ad)
        } else {
            (0, 0, 0, 0)
        }
    } else {
        let p: i64 = sqlx::query_scalar(
            "SELECT COUNT(DISTINCT patient_id) FROM appointments WHERE appointment_date = ?"
        )
        .bind(&today)
        .fetch_one(pool)
        .await
        .unwrap_or(0);

        let a: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM appointments WHERE appointment_date = ?"
        )
        .bind(&today)
        .fetch_one(pool)
        .await
        .unwrap_or(0);

        let l: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM lab_orders WHERE status IN ('ordered', 'in_progress')"
        )
        .fetch_one(pool)
        .await
        .unwrap_or(0);

        let ad: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM admissions WHERE status = 'active'"
        )
        .fetch_one(pool)
        .await
        .unwrap_or(0);

        (p, a, l, ad)
    };

    let total_beds: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM beds")
        .fetch_one(pool)
        .await
        .unwrap_or(1);

    let occupied_beds: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM beds WHERE status = 'occupied'"
    )
    .fetch_one(pool)
    .await
    .unwrap_or(0);

    let bed_occupancy_rate = (occupied_beds as f64 / total_beds as f64) * 100.0;

    let revenue_today: f64 = if session.role == "management" {
        sqlx::query_scalar(
            "SELECT COALESCE(SUM(amount), 0) FROM payments WHERE payment_date = ?"
        )
        .bind(&today)
        .fetch_one(pool)
        .await
        .unwrap_or(0.0)
    } else {
        0.0
    };

    let staff_on_duty: i64 = sqlx::query_scalar(
        "SELECT COUNT(DISTINCT staff_id) FROM attendance WHERE date = ? AND status = 'present'"
    )
    .bind(&today)
    .fetch_one(pool)
    .await
    .unwrap_or(0);

    let patients_registered_this_month: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM patients WHERE created_at >= ?"
    )
    .bind(&month_start)
    .fetch_one(pool)
    .await
    .unwrap_or(0);

    Ok(DashboardStats {
        total_patients_today,
        total_appointments_today,
        bed_occupancy_rate,
        revenue_today,
        pending_lab_orders,
        active_admissions,
        staff_on_duty,
        patients_registered_this_month,
    })
}

#[tauri::command]
pub async fn get_revenue_chart(days: Option<i64>) -> Result<Vec<RevenueData>, String> {
    guards::management_only()?;
    let pool = get_pool();
    let days = days.unwrap_or(7);

    let rows = sqlx::query(
        r#"SELECT payment_date as date, SUM(amount) as amount
        FROM payments
        WHERE payment_date >= date('now', '-' || ? || ' days')
        GROUP BY payment_date
        ORDER BY payment_date ASC"#
    )
    .bind(days)
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve revenue data".to_string())?;

    Ok(rows.iter().map(|r| RevenueData {
        date: r.get("date"),
        amount: r.get("amount"),
    }).collect())
}

#[tauri::command]
pub async fn get_department_load() -> Result<Vec<DepartmentLoad>, String> {
    guards::authenticated()?;
    let pool = get_pool();

    let today = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let rows = sqlx::query(
        r#"SELECT d.name as department_name,
            COUNT(DISTINCT a.patient_id) as appointment_count,
            COUNT(DISTINCT p.id) as patient_count
        FROM departments d
        LEFT JOIN appointments a ON d.id = a.department_id AND a.appointment_date = ?
        LEFT JOIN patients p ON a.patient_id = p.id
        WHERE d.is_active = 1
        GROUP BY d.id, d.name
        ORDER BY appointment_count DESC"#
    )
    .bind(&today)
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve department load".to_string())?;

    Ok(rows.iter().map(|r| DepartmentLoad {
        department_name: r.get("department_name"),
        appointment_count: r.get("appointment_count"),
        patient_count: r.get("patient_count"),
    }).collect())
}

#[tauri::command]
pub async fn get_monthly_trends(months: Option<i64>) -> Result<Vec<MonthlyTrend>, String> {
    guards::management_only()?;
    let pool = get_pool();
    let months = months.unwrap_or(6);

    let rows = sqlx::query(
        r#"SELECT 
            strftime('%Y-%m', created_at) as month,
            COUNT(*) as patients,
            0.0 as revenue,
            0 as admissions
        FROM patients
        WHERE created_at >= date('now', '-' || ? || ' months')
        GROUP BY strftime('%Y-%m', created_at)
        ORDER BY month ASC"#
    )
    .bind(months)
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve monthly trends".to_string())?;

    Ok(rows.iter().map(|r| MonthlyTrend {
        month: r.get("month"),
        patients: r.get("patients"),
        revenue: r.get("revenue"),
        admissions: r.get("admissions"),
    }).collect())
}
