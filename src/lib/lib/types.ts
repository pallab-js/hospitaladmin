export interface User {
  id: string;
  username: string;
  role: "doctor" | "management" | "staff";
  employee_id: string | null;
  full_name: string | null;
}

export interface Patient {
  id: string;
  patient_uid: string;
  first_name: string;
  last_name: string;
  date_of_birth: string;
  gender: string;
  blood_group: string | null;
  phone: string | null;
  email: string | null;
  address: string | null;
  emergency_contact_name: string | null;
  emergency_contact_phone: string | null;
  insurance_provider: string | null;
  insurance_id: string | null;
  allergies: string | null;
  medical_history: string | null;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface Staff {
  id: string;
  first_name: string;
  last_name: string;
  role: string;
  department_id: string | null;
  email: string | null;
  phone: string | null;
  qualification: string | null;
  specialization: string | null;
  is_active: boolean;
  hire_date: string | null;
  created_at: string;
  updated_at: string;
}

export interface Department {
  id: string;
  name: string;
  code: string;
  head_doctor_id: string | null;
  is_active: boolean;
  created_at: string;
}

export interface Appointment {
  id: string;
  patient_id: string;
  doctor_id: string;
  department_id: string | null;
  appointment_date: string;
  appointment_time: string;
  duration_minutes: number;
  status: string;
  visit_type: string;
  reason: string | null;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface AppointmentWithDetails {
  appointment: Appointment;
  patient_name: string;
  patient_uid: string;
  doctor_name: string;
  department_name: string | null;
}

export interface Ward {
  id: string;
  name: string;
  floor: number;
  ward_type: string;
  is_active: boolean;
  created_at: string;
}

export interface Bed {
  id: string;
  ward_id: string;
  room_number: string;
  bed_number: string;
  bed_type: string;
  status: string;
  daily_rate: number;
  created_at: string;
}

export interface BedWithWard {
  bed: Bed;
  ward_name: string;
  ward_type: string;
  patient_name: string | null;
}

export interface WardOccupancy {
  ward_id: string;
  ward_name: string;
  total_beds: number;
  occupied: number;
  available: number;
  reserved: number;
  occupancy_rate: number;
}

export interface Medication {
  id: string;
  name: string;
  generic_name: string | null;
  category: string;
  dosage_form: string;
  strength: string | null;
  manufacturer: string | null;
  unit_price: number;
  is_active: boolean;
  created_at: string;
}

export interface LabTest {
  id: string;
  name: string;
  code: string;
  category: string;
  normal_range: string | null;
  unit: string | null;
  price: number;
  is_active: boolean;
  created_at: string;
}

export interface LabOrder {
  id: string;
  appointment_id: string | null;
  patient_id: string;
  patient_name: string;
  patient_uid: string;
  doctor_name: string;
  order_date: string;
  priority: string;
  status: string;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface Invoice {
  id: string;
  invoice_number: string;
  patient_id: string;
  admission_id: string | null;
  invoice_date: string;
  subtotal: number;
  tax: number;
  discount: number;
  total: number;
  status: string;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface DashboardStats {
  total_patients_today: number;
  total_appointments_today: number;
  bed_occupancy_rate: number;
  revenue_today: number;
  pending_lab_orders: number;
  active_admissions: number;
  staff_on_duty: number;
  patients_registered_this_month: number;
}

export interface RevenueData {
  date: string;
  amount: number;
}

export interface DepartmentLoad {
  department_name: string;
  appointment_count: number;
  patient_count: number;
}

export interface MonthlyTrend {
  month: string;
  patients: number;
  revenue: number;
  admissions: number;
}

export type UserRole = "doctor" | "management" | "staff";

export interface NavItem {
  title: string;
  href: string;
  icon: string;
  roles?: UserRole[];
}
