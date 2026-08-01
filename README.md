# HMS - Hospital Management System

A modern, desktop-first hospital management system built with **Tauri 2**, **SvelteKit 5**, **Rust**, and **SQLite**.

> **IMPORTANT DISCLAIMER:** This software is provided for administrative and organizational purposes only. It is NOT a medical device and should NOT be used for clinical decision-making, diagnosis, or treatment. All medical decisions must be made by qualified healthcare professionals. Use of this software does not replace professional medical judgment.

## Features

- **Dashboard** - Role-based dashboards (Management, Doctor, Staff) with real-time KPIs, revenue charts, and department load
- **Patient Management** - Register, search, and manage patient records with full medical history
- **Appointments** - Schedule, reschedule, and track patient appointments with doctor availability
- **Wards & Beds** - Monitor bed occupancy across wards (ICU, General, Private, Emergency)
- **Lab & Diagnostics** - Order lab tests, record results, and flag abnormal values
- **Pharmacy** - Manage medication inventory, stock levels, and expiry tracking
- **Billing** - Generate invoices, record payments, and track payment status
- **Prescriptions** - Create prescriptions, dispense medications, and maintain records
- **Admissions** - Admit and discharge patients with bed assignment tracking
- **Staff Management** - View staff directory with roles and department assignments
- **Audit Logging** - Track all write operations for compliance
- **Database Backup** - Export database snapshot for backup (management only)

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Frontend | SvelteKit 5, Tailwind CSS 4, bits-ui |
| Backend | Rust, Tauri 2 |
| Database | SQLite (WAL mode) |
| Auth | bcrypt password hashing, session-based |
| Icons | Lucide Svelte |

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (latest stable)
- [pnpm](https://pnpm.io/) (recommended)

### Development

```bash
# Install dependencies
pnpm install

# Start Tauri dev (desktop app - required)
pnpm tauri dev
```

> **Note:** `pnpm dev` starts a Vite dev server for frontend hot-reload only. The app requires the Tauri/Rust backend for auth, database, and all commands — it will not function standalone in a browser.

### Build

```bash
# Build for production
pnpm tauri build
```

### First Launch

On first launch, the application seeds initial data including:
- Departments and ward/bed structure
- Sample staff records
- Medications and lab tests

**Admin credentials are generated randomly on first run and logged to the application's temp directory.** Check the console output on first launch for the admin username and password. Change these immediately after first login.

> **Note:** User accounts can only be created by an administrator through the application. There is no self-registration.

## Project Structure

```
hms/
├── src/                    # SvelteKit frontend
│   ├── lib/
│   │   ├── components/     # UI components
│   │   ├── stores/         # Svelte stores
│   │   └── lib/            # API & types
│   └── routes/             # Page routes
├── src-tauri/              # Rust backend
│   ├── migrations/         # SQL migration files
│   ├── capabilities/       # Tauri IPC permissions
│   └── src/
│       ├── auth/           # Authentication
│       ├── commands/       # Tauri command handlers
│       ├── models/         # Data models
│       ├── db/             # Database & migrations
│       └── utils/          # ID generators, audit, validation
└── static/                 # Static assets
```

## Security

- Content Security Policy (CSP) enabled
- Role-based auth guards on all commands (admin/doctor/nurse/receptionist/pharmacist/lab_tech/billing_staff)
- Login rate limiting (5 failed attempts → 15min lock, 20 → 24h lock)
- Constant-time login responses (bcrypt dummy verify on invalid users)
- bcrypt password hashing (cost factor 12)
- Session expiry (1 hour) with activity refresh
- Transactional multi-statement writes (no partial data)
- Atomic ID generation (no race conditions on patient UID / invoice number)
- Audit logging for all write operations
- Input validation on backend and frontend
- Multi-file migration system with version tracking

For security vulnerability reporting, see [SECURITY.md](SECURITY.md).

## Data Handling

This application handles sensitive patient health information (PHI). Ensure compliance with:
- Local hospital data protection regulations
- Patient privacy laws applicable in your jurisdiction
- Your organization's data governance policies

### Deployment Best Practices

1. **Change default credentials** immediately after first run
2. **Enable full-disk encryption** on machines running HMS
3. **Regular database backups** using the export feature
4. **Restrict physical access** to machines running HMS
5. **Keep the application updated** with latest security patches
6. **Monitor audit logs** for suspicious activity
7. **Do not share user accounts** between staff members

## License

[MIT](LICENSE)
