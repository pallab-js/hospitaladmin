-- Security: account lockout columns
ALTER TABLE users ADD COLUMN failed_attempts INTEGER NOT NULL DEFAULT 0;
ALTER TABLE users ADD COLUMN locked_until TEXT;

-- Atomic counters for sequential ID generation (fixes race condition)
CREATE TABLE IF NOT EXISTS counters (
    name TEXT PRIMARY KEY,
    current_value INTEGER NOT NULL DEFAULT 0
);

INSERT OR IGNORE INTO counters (name, current_value) VALUES ('patient_uid', 0);
INSERT OR IGNORE INTO counters (name, current_value) VALUES ('invoice_number', 0);

-- Audit log performance index
CREATE INDEX IF NOT EXISTS idx_audit_user_time ON audit_log(user_id, created_at);
