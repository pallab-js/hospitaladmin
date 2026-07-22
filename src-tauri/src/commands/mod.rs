// Command modules: read commands in `<name>.rs`, write commands in `<name>_write.rs`.
// Shared model types live in `models/`; command-local structs stay in the command file.
pub mod admissions;
pub mod appointments;
pub mod beds;
pub mod billing;
pub mod billing_write;
pub mod dashboard;
pub mod database;
pub mod doctors;
pub mod lab;
pub mod lab_orders;
pub mod patients;
pub mod pharmacy;
pub mod prescriptions;
pub mod reports;
pub mod staff;
