mod auth;
mod commands;
mod db;
mod models;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::block_on(async move {
                db::init(&app_handle)
                    .await
                    .expect("Failed to initialize database");
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            auth::commands::login,
            auth::commands::logout,
            auth::commands::get_current_user,
            commands::patients::create_patient,
            commands::patients::get_patients,
            commands::patients::get_patient_by_id,
            commands::patients::search_patients,
            commands::appointments::create_appointment,
            commands::appointments::get_appointments,
            commands::appointments::get_appointments_by_date,
            commands::appointments::update_appointment_status,
            commands::doctors::get_doctors,
            commands::beds::get_ward_occupancy,
            commands::staff::get_staff_list,
            commands::pharmacy::get_medications,
            commands::pharmacy::get_medication_stock,
            commands::lab::get_lab_tests,
            commands::lab::get_lab_orders,
            commands::lab_orders::create_lab_order,
            commands::lab_orders::update_lab_result,
            commands::lab_orders::complete_lab_order,
            commands::billing::get_invoices,
            commands::billing_write::create_invoice,
            commands::billing_write::record_payment,
            commands::prescriptions::create_prescription,
            commands::prescriptions::get_prescriptions_by_patient,
            commands::prescriptions::dispense_prescription_item,
            commands::admissions::create_admission,
            commands::admissions::discharge_patient,
            commands::admissions::get_active_admissions,
            commands::dashboard::get_dashboard_stats,
            commands::dashboard::get_revenue_chart,
            commands::dashboard::get_department_load,
            commands::dashboard::get_monthly_trends,
            commands::database::export_database,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
