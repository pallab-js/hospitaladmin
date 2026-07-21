-- =============================================
-- AUTH & USERS
-- =============================================
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('doctor','management','staff')),
    employee_id TEXT UNIQUE,
    is_active INTEGER NOT NULL DEFAULT 1,
    last_login_at TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- =============================================
-- DEPARTMENTS
-- =============================================
CREATE TABLE IF NOT EXISTS departments (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    head_doctor_id TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- =============================================
-- STAFF / EMPLOYEES
-- =============================================
CREATE TABLE IF NOT EXISTS staff (
    id TEXT PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    role TEXT NOT NULL CHECK (role IN ('doctor','nurse','receptionist','pharmacist','lab_tech','admin','other')),
    department_id TEXT REFERENCES departments(id),
    email TEXT,
    phone TEXT,
    qualification TEXT,
    specialization TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    hire_date TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- =============================================
-- SHIFTS
-- =============================================
CREATE TABLE IF NOT EXISTS shifts (
    id TEXT PRIMARY KEY,
    staff_id TEXT NOT NULL REFERENCES staff(id),
    shift_date TEXT NOT NULL,
    start_time TEXT NOT NULL,
    end_time TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'scheduled' CHECK (status IN ('scheduled','completed','absent')),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- =============================================
-- ATTENDANCE
-- =============================================
CREATE TABLE IF NOT EXISTS attendance (
    id TEXT PRIMARY KEY,
    staff_id TEXT NOT NULL REFERENCES staff(id),
    check_in TEXT NOT NULL,
    check_out TEXT,
    date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'present' CHECK (status IN ('present','absent','leave','half_day')),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- =============================================
-- LEAVES
-- =============================================
CREATE TABLE IF NOT EXISTS leaves (
    id TEXT PRIMARY KEY,
    staff_id TEXT NOT NULL REFERENCES staff(id),
    leave_type TEXT NOT NULL CHECK (leave_type IN ('sick','casual','earned','other')),
    from_date TEXT NOT NULL,
    to_date TEXT NOT NULL,
    reason TEXT,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending','approved','rejected')),
    approved_by TEXT REFERENCES users(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- =============================================
-- PATIENTS
-- =============================================
CREATE TABLE IF NOT EXISTS patients (
    id TEXT PRIMARY KEY,
    patient_uid TEXT NOT NULL UNIQUE,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    date_of_birth TEXT NOT NULL,
    gender TEXT NOT NULL CHECK (gender IN ('male','female','other')),
    blood_group TEXT,
    phone TEXT,
    email TEXT,
    address TEXT,
    emergency_contact_name TEXT,
    emergency_contact_phone TEXT,
    insurance_provider TEXT,
    insurance_id TEXT,
    allergies TEXT,
    medical_history TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_patients_name ON patients(first_name, last_name);
CREATE INDEX IF NOT EXISTS idx_patients_uid ON patients(patient_uid);
CREATE INDEX IF NOT EXISTS idx_patients_phone ON patients(phone);

-- =============================================
-- APPOINTMENTS
-- =============================================
CREATE TABLE IF NOT EXISTS appointments (
    id TEXT PRIMARY KEY,
    patient_id TEXT NOT NULL REFERENCES patients(id),
    doctor_id TEXT NOT NULL REFERENCES staff(id),
    department_id TEXT REFERENCES departments(id),
    appointment_date TEXT NOT NULL,
    appointment_time TEXT NOT NULL,
    duration_minutes INTEGER NOT NULL DEFAULT 15,
    status TEXT NOT NULL DEFAULT 'scheduled' CHECK (status IN ('scheduled','confirmed','in_progress','completed','cancelled','no_show')),
    visit_type TEXT NOT NULL DEFAULT 'consultation' CHECK (visit_type IN ('consultation','follow_up','emergency','walk_in')),
    reason TEXT,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_appt_date ON appointments(appointment_date);
CREATE INDEX IF NOT EXISTS idx_appt_doctor_date ON appointments(doctor_id, appointment_date);
CREATE INDEX IF NOT EXISTS idx_appt_patient ON appointments(patient_id);

-- =============================================
-- WARDS
-- =============================================
CREATE TABLE IF NOT EXISTS wards (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    floor INTEGER NOT NULL DEFAULT 1,
    ward_type TEXT NOT NULL CHECK (ward_type IN ('general','icu','semi_private','private','emergency')),
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- =============================================
-- BEDS
-- =============================================
CREATE TABLE IF NOT EXISTS beds (
    id TEXT PRIMARY KEY,
    ward_id TEXT NOT NULL REFERENCES wards(id),
    room_number TEXT NOT NULL,
    bed_number TEXT NOT NULL,
    bed_type TEXT NOT NULL DEFAULT 'general' CHECK (bed_type IN ('general','icu','premium')),
    status TEXT NOT NULL DEFAULT 'available' CHECK (status IN ('available','occupied','reserved','cleaning','maintenance')),
    daily_rate REAL NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_beds_status ON beds(status);

-- =============================================
-- ADMISSIONS
-- =============================================
CREATE TABLE IF NOT EXISTS admissions (
    id TEXT PRIMARY KEY,
    patient_id TEXT NOT NULL REFERENCES patients(id),
    bed_id TEXT NOT NULL REFERENCES beds(id),
    doctor_id TEXT NOT NULL REFERENCES staff(id),
    admission_date TEXT NOT NULL,
    admission_time TEXT NOT NULL,
    discharge_date TEXT,
    discharge_time TEXT,
    admission_type TEXT NOT NULL DEFAULT 'planned' CHECK (admission_type IN ('emergency','planned','transfer')),
    diagnosis TEXT,
    treatment_notes TEXT,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active','discharged','transferred')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_admissions_patient ON admissions(patient_id);
CREATE INDEX IF NOT EXISTS idx_admissions_status ON admissions(status);

-- =============================================
-- MEDICATIONS
-- =============================================
CREATE TABLE IF NOT EXISTS medications (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    generic_name TEXT,
    category TEXT NOT NULL,
    dosage_form TEXT NOT NULL,
    strength TEXT,
    manufacturer TEXT,
    unit_price REAL NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- =============================================
-- PRESCRIPTIONS
-- =============================================
CREATE TABLE IF NOT EXISTS prescriptions (
    id TEXT PRIMARY KEY,
    appointment_id TEXT REFERENCES appointments(id),
    patient_id TEXT NOT NULL REFERENCES patients(id),
    doctor_id TEXT NOT NULL REFERENCES staff(id),
    prescription_date TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS prescription_items (
    id TEXT PRIMARY KEY,
    prescription_id TEXT NOT NULL REFERENCES prescriptions(id),
    medication_id TEXT NOT NULL REFERENCES medications(id),
    dosage TEXT NOT NULL,
    frequency TEXT NOT NULL,
    duration_days INTEGER NOT NULL,
    instructions TEXT,
    dispensed INTEGER NOT NULL DEFAULT 0,
    dispensed_at TEXT
);

-- =============================================
-- INVENTORY
-- =============================================
CREATE TABLE IF NOT EXISTS inventory (
    id TEXT PRIMARY KEY,
    medication_id TEXT NOT NULL REFERENCES medications(id),
    batch_number TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 0,
    expiry_date TEXT NOT NULL,
    purchase_price REAL,
    supplier TEXT,
    received_date TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_inventory_med ON inventory(medication_id);
CREATE INDEX IF NOT EXISTS idx_inventory_expiry ON inventory(expiry_date);

-- =============================================
-- LAB TESTS
-- =============================================
CREATE TABLE IF NOT EXISTS lab_tests (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL,
    normal_range TEXT,
    unit TEXT,
    price REAL NOT NULL DEFAULT 0,
    is_active INTEGER NOT NULL DEFAULT 1,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- =============================================
-- LAB ORDERS
-- =============================================
CREATE TABLE IF NOT EXISTS lab_orders (
    id TEXT PRIMARY KEY,
    appointment_id TEXT REFERENCES appointments(id),
    patient_id TEXT NOT NULL REFERENCES patients(id),
    doctor_id TEXT NOT NULL REFERENCES staff(id),
    order_date TEXT NOT NULL,
    priority TEXT NOT NULL DEFAULT 'normal' CHECK (priority IN ('normal','urgent','stat')),
    status TEXT NOT NULL DEFAULT 'ordered' CHECK (status IN ('ordered','in_progress','completed','cancelled')),
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS lab_order_items (
    id TEXT PRIMARY KEY,
    lab_order_id TEXT NOT NULL REFERENCES lab_orders(id),
    test_id TEXT NOT NULL REFERENCES lab_tests(id),
    result_value TEXT,
    result_notes TEXT,
    is_abnormal INTEGER,
    completed_at TEXT,
    completed_by TEXT REFERENCES staff(id)
);

CREATE INDEX IF NOT EXISTS idx_lab_orders_patient ON lab_orders(patient_id);
CREATE INDEX IF NOT EXISTS idx_lab_orders_status ON lab_orders(status);

-- =============================================
-- INVOICES
-- =============================================
CREATE TABLE IF NOT EXISTS invoices (
    id TEXT PRIMARY KEY,
    invoice_number TEXT NOT NULL UNIQUE,
    patient_id TEXT NOT NULL REFERENCES patients(id),
    admission_id TEXT REFERENCES admissions(id),
    invoice_date TEXT NOT NULL,
    subtotal REAL NOT NULL DEFAULT 0,
    tax REAL NOT NULL DEFAULT 0,
    discount REAL NOT NULL DEFAULT 0,
    total REAL NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'pending' CHECK (status IN ('pending','partial','paid','cancelled')),
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS invoice_items (
    id TEXT PRIMARY KEY,
    invoice_id TEXT NOT NULL REFERENCES invoices(id),
    description TEXT NOT NULL,
    category TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit_price REAL NOT NULL,
    total REAL NOT NULL,
    reference_id TEXT
);

-- =============================================
-- PAYMENTS
-- =============================================
CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    invoice_id TEXT NOT NULL REFERENCES invoices(id),
    amount REAL NOT NULL,
    payment_method TEXT NOT NULL CHECK (payment_method IN ('cash','card','insurance','upi','other')),
    payment_date TEXT NOT NULL,
    reference_number TEXT,
    received_by TEXT REFERENCES users(id),
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_invoices_patient ON invoices(patient_id);
CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);
CREATE INDEX IF NOT EXISTS idx_invoices_date ON invoices(invoice_date);

-- =============================================
-- AUDIT LOG
-- =============================================
CREATE TABLE IF NOT EXISTS audit_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id TEXT REFERENCES users(id),
    action TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT,
    details TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
