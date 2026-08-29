-- Add routine_checkup to visit_type CHECK constraint
-- SQLite doesn't support ALTER TABLE for CHECK constraints, so we recreate the table

CREATE TABLE IF NOT EXISTS appointments_new (
    id TEXT PRIMARY KEY,
    patient_id TEXT NOT NULL REFERENCES patients(id),
    doctor_id TEXT NOT NULL REFERENCES staff(id),
    department_id TEXT REFERENCES departments(id),
    appointment_date TEXT NOT NULL,
    appointment_time TEXT NOT NULL,
    duration_minutes INTEGER NOT NULL DEFAULT 15,
    status TEXT NOT NULL DEFAULT 'scheduled' CHECK (status IN ('scheduled','confirmed','in_progress','completed','cancelled','no_show')),
    visit_type TEXT NOT NULL DEFAULT 'consultation' CHECK (visit_type IN ('consultation','follow_up','emergency','walk_in','routine_checkup')),
    reason TEXT,
    notes TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

INSERT INTO appointments_new SELECT * FROM appointments;

DROP TABLE appointments;

ALTER TABLE appointments_new RENAME TO appointments;

CREATE INDEX IF NOT EXISTS idx_appt_date ON appointments(appointment_date);
CREATE INDEX IF NOT EXISTS idx_appt_doctor_date ON appointments(doctor_id, appointment_date);
CREATE INDEX IF NOT EXISTS idx_appt_patient ON appointments(patient_id);

-- Add missing indexes for performance
CREATE INDEX IF NOT EXISTS idx_payments_date ON payments(payment_date);
CREATE INDEX IF NOT EXISTS idx_lab_orders_doctor ON lab_orders(doctor_id);
