use crate::auth::guards;
use crate::db;
use crate::models::bed::WardOccupancy;

#[tauri::command]
pub async fn get_ward_occupancy() -> Result<Vec<WardOccupancy>, String> {
    guards::authenticated()?;
    let pool = db::get_pool();
    sqlx::query_as::<_, WardOccupancy>(
        r#"
        SELECT
            w.id as ward_id,
            w.name as ward_name,
            COUNT(b.id) as total_beds,
            SUM(CASE WHEN b.status = 'occupied' THEN 1 ELSE 0 END) as occupied,
            SUM(CASE WHEN b.status = 'available' THEN 1 ELSE 0 END) as available,
            SUM(CASE WHEN b.status = 'reserved' THEN 1 ELSE 0 END) as reserved,
            ROUND(CAST(SUM(CASE WHEN b.status = 'occupied' THEN 1 ELSE 0 END) AS FLOAT) / COUNT(b.id) * 100, 1) as occupancy_rate
        FROM wards w
        JOIN beds b ON b.ward_id = w.id
        WHERE w.is_active = 1
        GROUP BY w.id, w.name
        ORDER BY w.name
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|_| "Failed to retrieve ward data".to_string())
}
