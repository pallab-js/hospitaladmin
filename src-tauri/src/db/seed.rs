use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub async fn seed(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let user_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    if user_count > 0 {
        return Ok(());
    }

    println!("[seed] Seeding database with initial data...");

    seed_departments(pool).await?;
    seed_staff(pool).await?;
    seed_users(pool).await?;
    seed_wards_and_beds(pool).await?;
    seed_medications(pool).await?;
    seed_lab_tests(pool).await?;
    seed_sample_patients(pool).await?;

    // Initialize counters to match seeded data
    sqlx::query("UPDATE counters SET current_value = (SELECT COUNT(*) FROM patients) WHERE name = 'patient_uid'")
        .execute(pool).await?;
    sqlx::query("UPDATE counters SET current_value = (SELECT COUNT(*) FROM invoices) WHERE name = 'invoice_number'")
        .execute(pool).await?;

    println!("[seed] Database seeded successfully!");
    Ok(())
}

async fn seed_departments(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let departments = vec![
        ("General Medicine", "GM"),
        ("Cardiology", "CARD"),
        ("Orthopedics", "ORTH"),
        ("Pediatrics", "PED"),
        ("Emergency", "EMER"),
        ("Radiology", "RAD"),
        ("Pathology", "PATH"),
        ("Pharmacy", "PHARM"),
        ("ICU", "ICU"),
        ("Surgery", "SURG"),
    ];

    for (name, code) in departments {
        let id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO departments (id, name, code) VALUES (?, ?, ?)")
            .bind(&id)
            .bind(name)
            .bind(code)
            .execute(pool)
            .await?;
    }
    Ok(())
}

async fn seed_staff(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let staff_data = vec![
        (
            "Sarah",
            "Johnson",
            "doctor",
            "GM",
            "MBBS, MD",
            "General Medicine",
        ),
        (
            "Michael",
            "Chen",
            "doctor",
            "CARD",
            "MBBS, DM",
            "Cardiology",
        ),
        (
            "Emily",
            "Williams",
            "doctor",
            "ORTH",
            "MBBS, MS",
            "Orthopedics",
        ),
        ("David", "Brown", "doctor", "PED", "MBBS, MD", "Pediatrics"),
        (
            "Lisa",
            "Davis",
            "doctor",
            "EMER",
            "MBBS, Diploma in Emergency Medicine",
            "Emergency",
        ),
        ("James", "Wilson", "nurse", "ICU", "B.Sc Nursing", ""),
        ("Emma", "Taylor", "nurse", "GM", "B.Sc Nursing", ""),
        ("Robert", "Anderson", "receptionist", "GM", "B.Com", ""),
        ("Maria", "Garcia", "pharmacist", "PHARM", "B.Pharm", ""),
        (
            "John",
            "Martinez",
            "lab_tech",
            "PATH",
            "B.Sc Medical Lab Tech",
            "",
        ),
    ];

    for (first, last, role, dept, qual, spec) in staff_data {
        let id = Uuid::new_v4().to_string();
        let dept_id = sqlx::query_scalar::<_, String>("SELECT id FROM departments WHERE code = ?")
            .bind(dept)
            .fetch_optional(pool)
            .await?
            .unwrap_or_default();

        sqlx::query(
            "INSERT INTO staff (id, first_name, last_name, role, department_id, qualification, specialization, is_active) VALUES (?, ?, ?, ?, ?, ?, ?, 1)"
        )
        .bind(&id)
        .bind(first)
        .bind(last)
        .bind(role)
        .bind(&dept_id)
        .bind(qual)
        .bind(spec)
        .execute(pool)
        .await?;
    }
    Ok(())
}

async fn seed_users(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let admin_id = Uuid::new_v4().to_string();
    let password_hash = bcrypt::hash("admin123", 12)?;

    sqlx::query(
        "INSERT INTO users (id, username, password_hash, role, is_active) VALUES (?, ?, ?, ?, 1)",
    )
    .bind(&admin_id)
    .bind("admin")
    .bind(&password_hash)
    .bind("management")
    .execute(pool)
    .await?;

    // Create a canonical doctor user (matches README credentials)
    let doc_user_id = Uuid::new_v4().to_string();
    sqlx::query(
        "INSERT INTO users (id, username, password_hash, role, is_active) VALUES (?, ?, ?, 'doctor', 1)"
    )
    .bind(&doc_user_id)
    .bind("doctor")
    .bind(&password_hash)
    .execute(pool)
    .await?;

    // Create user accounts for doctors
    let doctors = sqlx::query("SELECT id, first_name, last_name FROM staff WHERE role = 'doctor'")
        .fetch_all(pool)
        .await?;

    for doc in &doctors {
        let user_id = Uuid::new_v4().to_string();
        let first_name: String = doc.get("first_name");
        let last_name: String = doc.get("last_name");
        let mut username = format!(
            "{}{}",
            first_name.to_lowercase(),
            last_name.to_lowercase().chars().next().unwrap_or('x')
        );

        // Collision handling: append numeric suffix
        let mut suffix = 1;
        while username_exists(pool, &username).await? {
            username = format!(
                "{}{}{}",
                first_name.to_lowercase(),
                last_name.to_lowercase().chars().next().unwrap_or('x'),
                suffix
            );
            suffix += 1;
        }

        let hash = bcrypt::hash("doctor123", 12)?;

        sqlx::query(
            "INSERT INTO users (id, username, password_hash, role, employee_id, is_active) VALUES (?, ?, ?, 'doctor', ?, 1)"
        )
        .bind(&user_id)
        .bind(&username)
        .bind(&hash)
        .bind(doc.get::<String, _>("id"))
        .execute(pool)
        .await?;
    }

    // Create user for staff
    let staff =
        sqlx::query("SELECT id, first_name, last_name FROM staff WHERE role != 'doctor' LIMIT 3")
            .fetch_all(pool)
            .await?;

    for s in &staff {
        let user_id = Uuid::new_v4().to_string();
        let first_name: String = s.get("first_name");
        let last_name: String = s.get("last_name");
        let mut username = format!(
            "{}{}",
            first_name.to_lowercase(),
            last_name.to_lowercase().chars().next().unwrap_or('x')
        );

        let mut suffix = 1;
        while username_exists(pool, &username).await? {
            username = format!(
                "{}{}{}",
                first_name.to_lowercase(),
                last_name.to_lowercase().chars().next().unwrap_or('x'),
                suffix
            );
            suffix += 1;
        }

        let hash = bcrypt::hash("staff123", 12)?;

        sqlx::query(
            "INSERT INTO users (id, username, password_hash, role, employee_id, is_active) VALUES (?, ?, ?, 'staff', ?, 1)"
        )
        .bind(&user_id)
        .bind(&username)
        .bind(&hash)
        .bind(s.get::<String, _>("id"))
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn username_exists(
    pool: &SqlitePool,
    username: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;
    Ok(count > 0)
}

async fn seed_wards_and_beds(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let wards = vec![
        ("General Ward", 1, "general", 10),
        ("ICU", 1, "icu", 6),
        ("Semi-Private Ward", 2, "semi_private", 8),
        ("Private Ward", 2, "private", 4),
        ("Emergency Ward", 1, "emergency", 6),
    ];

    for (name, floor, ward_type, bed_count) in wards {
        let ward_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO wards (id, name, floor, ward_type, is_active) VALUES (?, ?, ?, ?, 1)",
        )
        .bind(&ward_id)
        .bind(name)
        .bind(floor)
        .bind(ward_type)
        .execute(pool)
        .await?;

        for bed_num in 1..=bed_count {
            let bed_id = Uuid::new_v4().to_string();
            let room_num = format!("{}-{}", &name[..1].to_uppercase(), bed_num);
            sqlx::query(
                "INSERT INTO beds (id, ward_id, room_number, bed_number, bed_type, status, daily_rate) VALUES (?, ?, ?, ?, 'general', 'available', 500)"
            )
            .bind(&bed_id)
            .bind(&ward_id)
            .bind(&room_num)
            .bind(format!("{}", bed_num))
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}

async fn seed_medications(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let medications = vec![
        (
            "Paracetamol 500mg",
            "Paracetamol",
            "Analgesic",
            "Tablet",
            "500mg",
            "Cipla",
            2.0,
        ),
        (
            "Amoxicillin 250mg",
            "Amoxicillin",
            "Antibiotic",
            "Capsule",
            "250mg",
            "Sun Pharma",
            8.0,
        ),
        (
            "Metformin 500mg",
            "Metformin",
            "Antidiabetic",
            "Tablet",
            "500mg",
            "USV",
            5.0,
        ),
        (
            "Omeprazole 20mg",
            "Omeprazole",
            "Antacid",
            "Capsule",
            "20mg",
            "Dr. Reddy's",
            6.0,
        ),
        (
            "Cetirizine 10mg",
            "Cetirizine",
            "Antihistamine",
            "Tablet",
            "10mg",
            "Cipla",
            3.0,
        ),
        (
            "Amlodipine 5mg",
            "Amlodipine",
            "Antihypertensive",
            "Tablet",
            "5mg",
            "Pfizer",
            7.0,
        ),
        (
            "Azithromycin 500mg",
            "Azithromycin",
            "Antibiotic",
            "Tablet",
            "500mg",
            "Zydus",
            12.0,
        ),
        (
            "Pantoprazole 40mg",
            "Pantoprazole",
            "Antacid",
            "Tablet",
            "40mg",
            "Alkem",
            8.0,
        ),
        (
            "Diclofenac 50mg",
            "Diclofenac",
            "NSAID",
            "Tablet",
            "50mg",
            "Novartis",
            4.0,
        ),
        (
            "Salbutamol Inhaler",
            "Salbutamol",
            "Bronchodilator",
            "Inhaler",
            "100mcg",
            "Cipla",
            150.0,
        ),
    ];

    for (name, generic, category, form, strength, mfr, price) in medications {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO medications (id, name, generic_name, category, dosage_form, strength, manufacturer, unit_price, is_active) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1)"
        )
        .bind(&id)
        .bind(name)
        .bind(generic)
        .bind(category)
        .bind(form)
        .bind(strength)
        .bind(mfr)
        .bind(price)
        .execute(pool)
        .await?;

        // Add inventory
        let inv_id = Uuid::new_v4().to_string();
        let qty = rand_qty();
        sqlx::query(
            "INSERT INTO inventory (id, medication_id, batch_number, quantity, expiry_date, received_date) VALUES (?, ?, ?, ?, '2027-12-31', date('now'))"
        )
        .bind(&inv_id)
        .bind(&id)
        .bind(format!("BATCH-{}", &id[..8].to_uppercase()))
        .bind(qty)
        .execute(pool)
        .await?;
    }
    Ok(())
}

fn rand_qty() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    50 + (nanos % 200) as i64
}

async fn seed_lab_tests(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let tests = vec![
        (
            "Complete Blood Count",
            "CBC",
            "Pathology",
            "4.0-5.5 million/uL",
            "million/uL",
            150.0,
        ),
        (
            "Blood Sugar Fasting",
            "BSF",
            "Pathology",
            "70-100 mg/dL",
            "mg/dL",
            80.0,
        ),
        (
            "Blood Sugar Post Prandial",
            "BSPP",
            "Pathology",
            "80-140 mg/dL",
            "mg/dL",
            80.0,
        ),
        (
            "Lipid Profile",
            "LP",
            "Pathology",
            "Desirable <200 mg/dL",
            "mg/dL",
            300.0,
        ),
        (
            "Liver Function Test",
            "LFT",
            "Pathology",
            "AST 10-40 U/L",
            "U/L",
            400.0,
        ),
        (
            "Kidney Function Test",
            "KFT",
            "Pathology",
            "Creatinine 0.6-1.2 mg/dL",
            "mg/dL",
            350.0,
        ),
        (
            "Thyroid Profile",
            "TFT",
            "Pathology",
            "TSH 0.4-4.0 mIU/L",
            "mIU/L",
            500.0,
        ),
        ("Urine Routine", "UR", "Pathology", "Normal", "", 100.0),
        ("ECG", "ECG", "Cardiology", "Normal Sinus Rhythm", "", 200.0),
        (
            "X-Ray Chest",
            "XRC",
            "Radiology",
            "No active lesion",
            "",
            400.0,
        ),
    ];

    for (name, code, category, range, unit, price) in tests {
        let id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO lab_tests (id, name, code, category, normal_range, unit, price, is_active) VALUES (?, ?, ?, ?, ?, ?, ?, 1)"
        )
        .bind(&id)
        .bind(name)
        .bind(code)
        .bind(category)
        .bind(range)
        .bind(unit)
        .bind(price)
        .execute(pool)
        .await?;
    }
    Ok(())
}

async fn seed_sample_patients(pool: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let patients = vec![
        (
            "Rajesh",
            "Kumar",
            "1985-03-15",
            "male",
            "B+",
            "9876543210",
            "123 Main St",
        ),
        (
            "Priya",
            "Sharma",
            "1990-07-22",
            "female",
            "O+",
            "9876543211",
            "456 Oak Ave",
        ),
        (
            "Amit",
            "Patel",
            "1978-11-08",
            "male",
            "A+",
            "9876543212",
            "789 Pine Rd",
        ),
        (
            "Sneha",
            "Reddy",
            "1995-01-30",
            "female",
            "AB+",
            "9876543213",
            "321 Elm St",
        ),
        (
            "Vikram",
            "Singh",
            "1982-06-18",
            "male",
            "O-",
            "9876543214",
            "654 Maple Dr",
        ),
    ];

    for (first, last, dob, gender, bg, phone, addr) in patients {
        let id = Uuid::new_v4().to_string();
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM patients")
            .fetch_one(pool)
            .await?;
        let uid = format!("HMS-{:05}", count + 1);

        sqlx::query(
            "INSERT INTO patients (id, patient_uid, first_name, last_name, date_of_birth, gender, blood_group, phone, address, is_active) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 1)"
        )
        .bind(&id)
        .bind(&uid)
        .bind(first)
        .bind(last)
        .bind(dob)
        .bind(gender)
        .bind(bg)
        .bind(phone)
        .bind(addr)
        .execute(pool)
        .await?;
    }
    Ok(())
}
