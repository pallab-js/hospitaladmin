import { invoke } from "@tauri-apps/api/core";
import type {
  User,
  Patient,
  Appointment,
  AppointmentWithDetails,
  WardOccupancy,
  BedWithWard,
  Medication,
  LabOrder,
  LabTest,
  Invoice,
  Staff,
  DashboardStats,
  RevenueData,
  DepartmentLoad,
  MonthlyTrend,
} from "$lib/lib/types";

let isTauri = false;
try {
  if (typeof window !== "undefined" && window.__TAURI_INTERNALS__) {
    isTauri = true;
  }
} catch {
  isTauri = false;
}

// Demo data for browser mode
const demoUser: User = {
  id: "demo-001",
  username: "admin",
  role: "management",
  employee_id: null,
  full_name: "Admin User",
};

const demoPatients: Patient[] = [
  {
    id: "1", patient_uid: "HMS-00001", first_name: "Rajesh", last_name: "Kumar",
    date_of_birth: "1985-03-15", gender: "male", blood_group: "B+",
    phone: "9876543210", email: null, address: "123 Main St",
    emergency_contact_name: "Sunita Kumar", emergency_contact_phone: "9876543215",
    insurance_provider: "Star Health", insurance_id: "SH12345",
    allergies: "Penicillin", medical_history: null,
    is_active: true, created_at: "2024-01-15T10:00:00Z", updated_at: "2024-01-15T10:00:00Z",
  },
  {
    id: "2", patient_uid: "HMS-00002", first_name: "Priya", last_name: "Sharma",
    date_of_birth: "1990-07-22", gender: "female", blood_group: "O+",
    phone: "9876543211", email: "priya@email.com", address: "456 Oak Ave",
    emergency_contact_name: "Rahul Sharma", emergency_contact_phone: "9876543216",
    insurance_provider: null, insurance_id: null,
    allergies: null, medical_history: "Diabetes Type 2",
    is_active: true, created_at: "2024-01-16T10:00:00Z", updated_at: "2024-01-16T10:00:00Z",
  },
  {
    id: "3", patient_uid: "HMS-00003", first_name: "Amit", last_name: "Patel",
    date_of_birth: "1978-11-08", gender: "male", blood_group: "A+",
    phone: "9876543212", email: null, address: "789 Pine Rd",
    emergency_contact_name: "Neha Patel", emergency_contact_phone: "9876543217",
    insurance_provider: "ICICI Lombard", insurance_id: "IL67890",
    allergies: null, medical_history: null,
    is_active: true, created_at: "2024-01-17T10:00:00Z", updated_at: "2024-01-17T10:00:00Z",
  },
  {
    id: "4", patient_uid: "HMS-00004", first_name: "Sneha", last_name: "Reddy",
    date_of_birth: "1995-01-30", gender: "female", blood_group: "AB+",
    phone: "9876543213", email: null, address: "321 Elm St",
    emergency_contact_name: "Kiran Reddy", emergency_contact_phone: "9876543218",
    insurance_provider: null, insurance_id: null,
    allergies: "Sulfa drugs", medical_history: null,
    is_active: true, created_at: "2024-01-18T10:00:00Z", updated_at: "2024-01-18T10:00:00Z",
  },
  {
    id: "5", patient_uid: "HMS-00005", first_name: "Vikram", last_name: "Singh",
    date_of_birth: "1982-06-18", gender: "male", blood_group: "O-",
    phone: "9876543214", email: null, address: "654 Maple Dr",
    emergency_contact_name: "Pooja Singh", emergency_contact_phone: "9876543219",
    insurance_provider: "Bajaj Allianz", insurance_id: "BA11111",
    allergies: null, medical_history: "Hypertension",
    is_active: true, created_at: "2024-01-19T10:00:00Z", updated_at: "2024-01-19T10:00:00Z",
  },
];

const demoDashboardStats: DashboardStats = {
  total_patients_today: 42,
  total_appointments_today: 28,
  bed_occupancy_rate: 67.5,
  revenue_today: 12500,
  pending_lab_orders: 8,
  active_admissions: 15,
  staff_on_duty: 12,
  patients_registered_this_month: 85,
};

const demoRevenueData: RevenueData[] = [
  { date: "2024-01-09", amount: 8500 },
  { date: "2024-01-10", amount: 11200 },
  { date: "2024-01-11", amount: 9800 },
  { date: "2024-01-12", amount: 13400 },
  { date: "2024-01-13", amount: 7600 },
  { date: "2024-01-14", amount: 10200 },
  { date: "2024-01-15", amount: 12500 },
];

const demoDepartmentLoad: DepartmentLoad[] = [
  { department_name: "General Medicine", appointment_count: 12, patient_count: 10 },
  { department_name: "Cardiology", appointment_count: 8, patient_count: 7 },
  { department_name: "Orthopedics", appointment_count: 5, patient_count: 5 },
  { department_name: "Pediatrics", appointment_count: 3, patient_count: 3 },
];

const demoWardOccupancy: WardOccupancy[] = [
  { ward_id: "w1", ward_name: "General Ward", total_beds: 10, occupied: 7, available: 3, reserved: 0, occupancy_rate: 70 },
  { ward_id: "w2", ward_name: "ICU", total_beds: 6, occupied: 5, available: 1, reserved: 0, occupancy_rate: 83 },
  { ward_id: "w3", ward_name: "Semi-Private", total_beds: 8, occupied: 4, available: 3, reserved: 1, occupancy_rate: 50 },
  { ward_id: "w4", ward_name: "Private", total_beds: 4, occupied: 2, available: 2, reserved: 0, occupancy_rate: 50 },
  { ward_id: "w5", ward_name: "Emergency", total_beds: 6, occupied: 3, available: 3, reserved: 0, occupancy_rate: 50 },
];

const demoMedications: Medication[] = [
  { id: "m1", name: "Paracetamol 500mg", generic_name: "Paracetamol", category: "Analgesic", dosage_form: "Tablet", strength: "500mg", manufacturer: "Cipla", unit_price: 2.5, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "m2", name: "Amoxicillin 250mg", generic_name: "Amoxicillin", category: "Antibiotic", dosage_form: "Capsule", strength: "250mg", manufacturer: "Sun Pharma", unit_price: 8.0, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "m3", name: "Metformin 500mg", generic_name: "Metformin HCl", category: "Antidiabetic", dosage_form: "Tablet", strength: "500mg", manufacturer: "Dr. Reddy's", unit_price: 5.0, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "m4", name: "Omeprazole 20mg", generic_name: "Omeprazole", category: "Antacid", dosage_form: "Capsule", strength: "20mg", manufacturer: "AstraZeneca", unit_price: 12.0, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "m5", name: "Cetirizine 10mg", generic_name: "Cetirizine HCl", category: "Antihistamine", dosage_form: "Tablet", strength: "10mg", manufacturer: "Cipla", unit_price: 3.0, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "m6", name: "Atorvastatin 10mg", generic_name: "Atorvastatin Calcium", category: "Statin", dosage_form: "Tablet", strength: "10mg", manufacturer: "Pfizer", unit_price: 15.0, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "m7", name: "Amlodipine 5mg", generic_name: "Amlodipine Besylate", category: "Antihypertensive", dosage_form: "Tablet", strength: "5mg", manufacturer: "USV", unit_price: 6.0, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "m8", name: "Pantoprazole 40mg", generic_name: "Pantoprazole", category: "Antacid", dosage_form: "Tablet", strength: "40mg", manufacturer: "Alkem", unit_price: 10.0, is_active: true, created_at: "2024-01-01T00:00:00Z" },
];

const demoMedicationStock: { medication_id: string; quantity: number; low_threshold: number; expiry_date: string }[] = [
  { medication_id: "m1", quantity: 250, low_threshold: 50, expiry_date: "2025-06-01" },
  { medication_id: "m2", quantity: 45, low_threshold: 50, expiry_date: "2025-03-15" },
  { medication_id: "m3", quantity: 180, low_threshold: 40, expiry_date: "2025-09-01" },
  { medication_id: "m4", quantity: 8, low_threshold: 30, expiry_date: "2025-02-01" },
  { medication_id: "m5", quantity: 120, low_threshold: 30, expiry_date: "2025-12-01" },
  { medication_id: "m6", quantity: 95, low_threshold: 25, expiry_date: "2025-07-01" },
  { medication_id: "m7", quantity: 60, low_threshold: 30, expiry_date: "2025-04-01" },
  { medication_id: "m8", quantity: 22, low_threshold: 25, expiry_date: "2025-01-20" },
];

const demoLabTests: LabTest[] = [
  { id: "lt1", name: "Complete Blood Count", code: "CBC", category: "Hematology", normal_range: "WBC: 4000-11000, RBC: 4.5-5.5", unit: "cells/µL", price: 300, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "lt2", name: "Blood Sugar Fasting", code: "BSF", category: "Biochemistry", normal_range: "70-100", unit: "mg/dL", price: 150, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "lt3", name: "Lipid Profile", code: "LP", category: "Biochemistry", normal_range: "Total Cholesterol <200", unit: "mg/dL", price: 500, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "lt4", name: "Thyroid Profile", code: "TFT", category: "Endocrinology", normal_range: "TSH: 0.4-4.0", unit: "mIU/L", price: 800, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "lt5", name: "Liver Function Test", code: "LFT", category: "Biochemistry", normal_range: "SGOT: 5-40, SGPT: 7-56", unit: "U/L", price: 600, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "lt6", name: "Urinalysis", code: "UA", category: "Urinalysis", normal_range: "Clear, pH 4.5-8.0", unit: "", price: 200, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "lt7", name: "X-Ray Chest", code: "XRC", category: "Radiology", normal_range: "No active lesion", unit: "", price: 1000, is_active: true, created_at: "2024-01-01T00:00:00Z" },
  { id: "lt8", name: "ECG", code: "ECG", category: "Cardiology", normal_range: "Normal sinus rhythm", unit: "", price: 400, is_active: true, created_at: "2024-01-01T00:00:00Z" },
];

const demoLabOrders: LabOrder[] = [
  { id: "lo1", appointment_id: null, patient_id: "1", patient_name: "Rajesh Kumar", patient_uid: "HMS-00001", doctor_name: "Dr. Sarah Wilson", order_date: "2024-01-15", priority: "routine", status: "ordered", notes: null, created_at: "2024-01-15T10:30:00Z", updated_at: "2024-01-15T10:30:00Z" },
  { id: "lo2", appointment_id: null, patient_id: "2", patient_name: "Priya Sharma", patient_uid: "HMS-00002", doctor_name: "Dr. James Chen", order_date: "2024-01-15", priority: "urgent", status: "in_progress", notes: "Fasting required", created_at: "2024-01-15T11:15:00Z", updated_at: "2024-01-15T11:15:00Z" },
  { id: "lo3", appointment_id: null, patient_id: "3", patient_name: "Amit Patel", patient_uid: "HMS-00003", doctor_name: "Dr. Sarah Wilson", order_date: "2024-01-15", priority: "routine", status: "completed", notes: null, created_at: "2024-01-15T09:00:00Z", updated_at: "2024-01-15T14:00:00Z" },
  { id: "lo4", appointment_id: null, patient_id: "4", patient_name: "Sneha Reddy", patient_uid: "HMS-00004", doctor_name: "Dr. Emily Brown", order_date: "2024-01-15", priority: "routine", status: "ordered", notes: null, created_at: "2024-01-15T12:00:00Z", updated_at: "2024-01-15T12:00:00Z" },
  { id: "lo5", appointment_id: null, patient_id: "5", patient_name: "Vikram Singh", patient_uid: "HMS-00005", doctor_name: "Dr. James Chen", order_date: "2024-01-15", priority: "stat", status: "in_progress", notes: "Immediate", created_at: "2024-01-15T13:30:00Z", updated_at: "2024-01-15T13:30:00Z" },
];

const demoInvoices: Invoice[] = [
  { id: "inv1", invoice_number: "INV-000001", patient_id: "1", admission_id: null, invoice_date: "2024-01-15", subtotal: 1250, tax: 125, discount: 0, total: 1375, status: "paid", notes: null, created_at: "2024-01-15T10:00:00Z", updated_at: "2024-01-15T10:00:00Z" },
  { id: "inv2", invoice_number: "INV-000002", patient_id: "2", admission_id: null, invoice_date: "2024-01-15", subtotal: 890, tax: 89, discount: 0, total: 979, status: "pending", notes: null, created_at: "2024-01-15T11:00:00Z", updated_at: "2024-01-15T11:00:00Z" },
  { id: "inv3", invoice_number: "INV-000003", patient_id: "3", admission_id: null, invoice_date: "2024-01-14", subtotal: 2100, tax: 210, discount: 100, total: 2210, status: "partial", notes: null, created_at: "2024-01-14T10:00:00Z", updated_at: "2024-01-14T10:00:00Z" },
  { id: "inv4", invoice_number: "INV-000004", patient_id: "4", admission_id: null, invoice_date: "2024-01-14", subtotal: 450, tax: 45, discount: 0, total: 495, status: "paid", notes: null, created_at: "2024-01-14T14:00:00Z", updated_at: "2024-01-14T14:00:00Z" },
  { id: "inv5", invoice_number: "INV-000005", patient_id: "5", admission_id: null, invoice_date: "2024-01-13", subtotal: 750, tax: 75, discount: 0, total: 825, status: "overdue", notes: null, created_at: "2024-01-13T10:00:00Z", updated_at: "2024-01-13T10:00:00Z" },
];

const demoStaff: Staff[] = [
  { id: "s1", first_name: "Sarah", last_name: "Johnson", role: "doctor", department_id: "dept-001", email: "sarah@hms.com", phone: "9876540001", qualification: "MBBS, MD", specialization: "Cardiology", is_active: true, hire_date: "2020-06-01", created_at: "2020-06-01T00:00:00Z", updated_at: "2020-06-01T00:00:00Z" },
  { id: "s2", first_name: "Michael", last_name: "Chen", role: "doctor", department_id: "dept-002", email: "michael@hms.com", phone: "9876540002", qualification: "MBBS, DM", specialization: "General Medicine", is_active: true, hire_date: "2019-03-15", created_at: "2019-03-15T00:00:00Z", updated_at: "2019-03-15T00:00:00Z" },
  { id: "s3", first_name: "Emily", last_name: "Davis", role: "nurse", department_id: "dept-003", email: "emily@hms.com", phone: "9876540003", qualification: "B.Sc Nursing", specialization: "ICU", is_active: true, hire_date: "2021-08-01", created_at: "2021-08-01T00:00:00Z", updated_at: "2021-08-01T00:00:00Z" },
  { id: "s4", first_name: "Robert", last_name: "Wilson", role: "receptionist", department_id: null, email: "robert@hms.com", phone: "9876540004", qualification: "B.A", specialization: null, is_active: true, hire_date: "2022-01-10", created_at: "2022-01-10T00:00:00Z", updated_at: "2022-01-10T00:00:00Z" },
  { id: "s5", first_name: "Lisa", last_name: "Anderson", role: "pharmacist", department_id: "dept-005", email: "lisa@hms.com", phone: "9876540005", qualification: "B.Pharm", specialization: null, is_active: true, hire_date: "2021-04-01", created_at: "2021-04-01T00:00:00Z", updated_at: "2021-04-01T00:00:00Z" },
  { id: "s6", first_name: "James", last_name: "Brown", role: "lab_tech", department_id: "dept-006", email: "james@hms.com", phone: "9876540006", qualification: "B.Sc MLT", specialization: null, is_active: false, hire_date: "2020-09-01", created_at: "2020-09-01T00:00:00Z", updated_at: "2020-09-01T00:00:00Z" },
];

// Auth commands
export async function login(username: string, password: string) {
  if (!isTauri) {
    const demoUsers: Record<string, { password: string; user: User }> = {
      admin: { password: "admin123", user: { id: "u1", username: "admin", role: "management", employee_id: null, full_name: "Admin User" } },
      doctor: { password: "doctor123", user: { id: "u2", username: "doctor", role: "doctor", employee_id: "doc-001", full_name: "Dr. Sarah Johnson" } },
      staff: { password: "staff123", user: { id: "u3", username: "staff", role: "staff", employee_id: "s4", full_name: "Robert Wilson" } },
    };
    const match = demoUsers[username];
    if (match && match.password === password) {
      return { success: true, message: "Login successful", user: match.user };
    }
    return { success: false, message: "Invalid credentials. Try admin/admin123", user: null };
  }
  return invoke<{ success: boolean; message: string; user: User | null }>(
    "login",
    { request: { username, password } }
  );
}

export async function logout() {
  if (!isTauri) return;
  return invoke<void>("logout");
}

export async function getCurrentUser() {
  if (!isTauri) return null;
  return invoke<User | null>("get_current_user");
}

// Patient commands
export async function createPatient(patient: {
  first_name: string;
  last_name: string;
  date_of_birth: string;
  gender: string;
  blood_group?: string;
  phone?: string;
  email?: string;
  address?: string;
  emergency_contact_name?: string;
  emergency_contact_phone?: string;
  insurance_provider?: string;
  insurance_id?: string;
  allergies?: string;
  medical_history?: string;
}) {
  if (!isTauri) {
    const newPatient: Patient = {
      id: crypto.randomUUID(),
      patient_uid: `HMS-${String(demoPatients.length + 1).padStart(5, "0")}`,
      ...patient,
      phone: patient.phone || null,
      blood_group: patient.blood_group || null,
      email: patient.email || null,
      address: patient.address || null,
      emergency_contact_name: patient.emergency_contact_name || null,
      emergency_contact_phone: patient.emergency_contact_phone || null,
      insurance_provider: patient.insurance_provider || null,
      insurance_id: patient.insurance_id || null,
      allergies: patient.allergies || null,
      medical_history: patient.medical_history || null,
      is_active: true,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    };
    demoPatients.push(newPatient);
    return newPatient;
  }
  return invoke<Patient>("create_patient", { request: patient });
}

export async function getPatients(page?: number, limit?: number) {
  if (!isTauri) return demoPatients;
  return invoke<Patient[]>("get_patients", { page, limit });
}

export async function getPatientById(id: string) {
  if (!isTauri) {
    return demoPatients.find((p) => p.id === id) || demoPatients[0];
  }
  return invoke<Patient>("get_patient_by_id", { id });
}

export async function searchPatients(params: {
  query?: string;
  gender?: string;
  blood_group?: string;
  page?: number;
  limit?: number;
}) {
  if (!isTauri) {
    if (!params.query) return demoPatients;
    const q = params.query.toLowerCase();
    return demoPatients.filter(
      (p) =>
        p.first_name.toLowerCase().includes(q) ||
        p.last_name.toLowerCase().includes(q) ||
        p.patient_uid.toLowerCase().includes(q) ||
        (p.phone && p.phone.includes(q))
    );
  }
  return invoke<Patient[]>("search_patients", { params });
}

// Appointment commands
export async function createAppointment(appointment: {
  patient_id: string;
  doctor_id: string;
  department_id?: string;
  appointment_date: string;
  appointment_time: string;
  duration_minutes?: number;
  visit_type?: string;
  reason?: string;
}) {
  if (!isTauri) {
    return {
      id: crypto.randomUUID(),
      ...appointment,
      department_id: appointment.department_id || null,
      duration_minutes: appointment.duration_minutes || 15,
      status: "scheduled",
      visit_type: appointment.visit_type || "consultation",
      reason: appointment.reason || null,
      notes: null,
      created_at: new Date().toISOString(),
      updated_at: new Date().toISOString(),
    } as Appointment;
  }
  return invoke<Appointment>("create_appointment", { request: appointment });
}

export async function getAppointments(page?: number, limit?: number) {
  if (!isTauri) {
    return [
      { appointment: { id: "a1", patient_id: "1", doctor_id: "doc-001", department_id: "dept-001", appointment_date: "2024-01-15", appointment_time: "09:00", duration_minutes: 15, status: "completed", visit_type: "consultation", reason: "Chest pain", notes: null, created_at: "2024-01-15T09:00:00Z", updated_at: "2024-01-15T09:00:00Z" }, patient_name: "Rajesh Kumar", patient_uid: "HMS-00001", doctor_name: "Dr. Sarah Wilson", department_name: "Cardiology" },
      { appointment: { id: "a2", patient_id: "2", doctor_id: "doc-002", department_id: "dept-002", appointment_date: "2024-01-15", appointment_time: "10:30", duration_minutes: 15, status: "scheduled", visit_type: "follow_up", reason: "Diabetes review", notes: null, created_at: "2024-01-15T10:30:00Z", updated_at: "2024-01-15T10:30:00Z" }, patient_name: "Priya Sharma", patient_uid: "HMS-00002", doctor_name: "Dr. James Chen", department_name: "General Medicine" },
      { appointment: { id: "a3", patient_id: "3", doctor_id: "doc-001", department_id: "dept-001", appointment_date: "2024-01-15", appointment_time: "11:00", duration_minutes: 15, status: "scheduled", visit_type: "consultation", reason: "Annual checkup", notes: null, created_at: "2024-01-15T11:00:00Z", updated_at: "2024-01-15T11:00:00Z" }, patient_name: "Amit Patel", patient_uid: "HMS-00003", doctor_name: "Dr. Sarah Wilson", department_name: "Cardiology" },
      { appointment: { id: "a4", patient_id: "4", doctor_id: "doc-003", department_id: "dept-003", appointment_date: "2024-01-15", appointment_time: "14:00", duration_minutes: 15, status: "cancelled", visit_type: "routine_checkup", reason: "Routine check", notes: null, created_at: "2024-01-15T14:00:00Z", updated_at: "2024-01-15T14:00:00Z" }, patient_name: "Sneha Reddy", patient_uid: "HMS-00004", doctor_name: "Dr. Emily Brown", department_name: "Pediatrics" },
      { appointment: { id: "a5", patient_id: "5", doctor_id: "doc-002", department_id: "dept-002", appointment_date: "2024-01-15", appointment_time: "15:30", duration_minutes: 15, status: "no_show", visit_type: "consultation", reason: "Hypertension follow-up", notes: null, created_at: "2024-01-15T15:30:00Z", updated_at: "2024-01-15T15:30:00Z" }, patient_name: "Vikram Singh", patient_uid: "HMS-00005", doctor_name: "Dr. James Chen", department_name: "General Medicine" },
    ] as AppointmentWithDetails[];
  }
  return invoke<AppointmentWithDetails[]>("get_appointments", { page, limit });
}

export async function getAppointmentsByDate(date: string) {
  if (!isTauri) return [] as AppointmentWithDetails[];
  return invoke<AppointmentWithDetails[]>("get_appointments_by_date", { date });
}

export async function updateAppointmentStatus(id: string, status: string) {
  if (!isTauri) {
    return { id, status } as Appointment;
  }
  return invoke<Appointment>("update_appointment_status", { id, status });
}

// Staff commands
export async function getDoctors() {
  if (!isTauri) {
    return [
      { id: "doc-001", full_name: "Dr. Sarah Wilson", department_id: "dept-001" },
      { id: "doc-002", full_name: "Dr. James Chen", department_id: "dept-002" },
      { id: "doc-003", full_name: "Dr. Emily Brown", department_id: "dept-003" },
    ];
  }
  return invoke<{ id: string; full_name: string; department_id: string | null }[]>("get_doctors");
}

export async function getStaffList() {
  if (!isTauri) return demoStaff;
  return invoke<Staff[]>("get_staff_list");
}

// Ward commands
export async function getWardOccupancy() {
  if (!isTauri) return demoWardOccupancy;
  return invoke<WardOccupancy[]>("get_ward_occupancy");
}

// Pharmacy commands
export async function getMedications() {
  if (!isTauri) return demoMedications;
  return invoke<Medication[]>("get_medications");
}

export async function getMedicationStock() {
  if (!isTauri) return demoMedicationStock;
  return invoke<{ medication_id: string; quantity: number; low_threshold: number; expiry_date: string }[]>("get_medication_stock");
}

// Lab commands
export async function getLabTests() {
  if (!isTauri) return demoLabTests;
  return invoke<LabTest[]>("get_lab_tests");
}

export async function getLabOrders() {
  if (!isTauri) return demoLabOrders;
  return invoke<LabOrder[]>("get_lab_orders");
}

// Billing commands
export async function getInvoices() {
  if (!isTauri) return demoInvoices;
  return invoke<Invoice[]>("get_invoices");
}

// Dashboard commands
export async function getDashboardStats() {
  if (!isTauri) return demoDashboardStats;
  return invoke<DashboardStats>("get_dashboard_stats");
}

export async function getRevenueChart(days?: number) {
  if (!isTauri) return demoRevenueData;
  return invoke<RevenueData[]>("get_revenue_chart", { days });
}

export async function getDepartmentLoad() {
  if (!isTauri) return demoDepartmentLoad;
  return invoke<DepartmentLoad[]>("get_department_load");
}

export async function getMonthlyTrends(months?: number) {
  if (!isTauri) {
    return [
      { month: "2024-01", patients: 85, revenue: 320000, admissions: 22 },
    ] as MonthlyTrend[];
  }
  return invoke<MonthlyTrend[]>("get_monthly_trends", { months });
}
