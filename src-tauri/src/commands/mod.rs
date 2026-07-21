// Command modules: read commands in `<name>.rs`, write commands in `<name>_write.rs`.
// Shared model types live in `models/`; command-local structs stay in the command file.
pub mod patients;
pub mod appointments;
pub mod doctors;
pub mod beds;
pub mod pharmacy;
pub mod lab;
pub mod lab_orders;
pub mod billing;
pub mod billing_write;
pub mod staff;
pub mod reports;
pub mod dashboard;
pub mod prescriptions;
pub mod admissions;
pub mod database;
