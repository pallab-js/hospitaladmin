# HMS - Hospital Management System

A modern, desktop-first hospital management system built with **Tauri 2**, **SvelteKit 5**, **Rust**, and **SQLite**.

> **IMPORTANT DISCLAIMER:** This software is provided for administrative and organizational purposes only. It is NOT a medical device and should NOT be used for clinical decision-making, diagnosis, or treatment. All medical decisions must be made by qualified healthcare professionals. Use of this software does not replace professional medical judgment.

## Features

- **Role-Based Dashboard** - Admin, Doctor, and Staff dashboards with real-time KPIs, revenue charts, department load, and ward occupancy
- **Patient Management** - Register, search, edit, and manage patient records with full medical history
- **Appointments** - Schedule, reschedule, and track patient appointments with doctor availability
- **Wards & Beds** - Monitor bed occupancy across wards (ICU, General, Private, Emergency) with real-time status
- **Lab & Diagnostics** - Order lab tests, record results, flag abnormal values, and track completion
- **Pharmacy** - Manage medication inventory, stock levels, expiry tracking, and dispense prescriptions
- **Billing** - Generate invoices, record payments (cash/card/insurance/UPI), and track payment status
- **Prescriptions** - Create prescriptions with medication items and dispense with inventory deduction
- **Admissions** - Admit and discharge patients with atomic bed assignment (no race conditions)
- **Staff Management** - View staff directory with roles, departments, and specializations
- **Audit Logging** - Track all write operations for compliance and accountability
- **Database Backup** - Export database snapshot for backup (admin only, path-restricted)

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Desktop Shell | Tauri 2 |
| Frontend | SvelteKit 5 (runes), Tailwind CSS 4, shadcn-svelte |
| Backend | Rust (Tauri IPC commands) |
| Database | SQLite (WAL mode) via sqlx |
| Auth | bcrypt (cost 12), in-memory sessions, progressive lockout |
| Validation | Zod (frontend), Rust model methods (backend) |
| Icons | Lucide Svelte |
| CI | GitHub Actions (Node 22, Rust stable) |

## Getting Started

### Prerequisites

- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) (latest stable)
- [pnpm](https://pnpm.io/) 10+

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
- 10 departments with codes
- 10 staff members (5 doctors, 2 nurses, 1 receptionist, 1 pharmacist, 1 lab tech)
- Admin and doctor user accounts
- 5 wards with 34 beds
- 10 medications with inventory
- 10 lab tests
- 5 sample patients

**Admin credentials are generated randomly on first run and printed to the terminal.** Look for the `[seed] === INITIAL CREDENTIALS` block in the console output. Change these immediately after first login.

> **Note:** User accounts can only be created by an administrator through the application. There is no self-registration.

## Project Structure

```
hms/
├── src/                        # SvelteKit 5 frontend
│   ├── lib/
│   │   ├── components/         # UI components (shadcn-svelte, layout, domain)
│   │   ├── stores/             # Svelte stores (auth, sidebar)
│   │   ├── lib/                # API bridge, types, Zod validation
│   │   └── utils/              # cn(), formatters, status helpers
│   └── routes/                 # File-based routing
├── src-tauri/                  # Rust backend
│   ├── migrations/             # SQL migrations (001-004)
│   ├── capabilities/           # Tauri IPC permissions
│   └── src/
│       ├── auth/               # Session management, guards, commands
│       ├── commands/           # Tauri command handlers (30+)
│       ├── models/             # Data models with validation
│       ├── db/                 # SQLite pool, migrations, seeding
│       └── utils/              # ID generators, audit, password validation
└── static/                     # Static assets
```

## Security

- Content Security Policy (CSP) enabled via Tauri config
- **Role-based auth guards** on all 30+ commands (admin/doctor/nurse/receptionist/pharmacist/lab_tech/billing_staff)
- **Progressive account lockout** — 5 failures → 15min lock, 20 → 24h lock
- **Timing-equalized login** — bcrypt dummy verify on invalid users to prevent timing side-channel
- **bcrypt password hashing** (cost factor 12)
- **Session management** — 1-hour expiry with activity refresh, most-recent session resolution
- **Transactional writes** — admissions, prescriptions, invoices, payments use SQL transactions
- **Atomic bed allocation** — `UPDATE ... WHERE status = 'available'` with row-count check (no race conditions)
- **Atomic ID generation** — sequential patient UIDs and invoice numbers via counter table
- **Pagination bounds** — all list endpoints enforce `limit` 1–100, `page` ≥ 1
- **Input validation** — Zod schemas (frontend) + Rust model methods (backend)
- **Path traversal protection** — database export restricted to home/tmp with canonical path resolution
- **Audit logging** — all write operations logged with user, action, entity, and details
- **SQL injection prevention** — all queries use parameterized bindings via sqlx
- **Dark mode** — theme-aware status colors (no hardcoded light-only classes)

For security vulnerability reporting, see [SECURITY.md](SECURITY.md).

## Role Permissions

| Capability | admin | doctor | nurse | receptionist | pharmacist | lab_tech | billing_staff |
|------------|:-----:|:------:|:-----:|:------------:|:----------:|:--------:|:-------------:|
| Dashboard (all data) | ✓ | — | — | — | — | — | — |
| Dashboard (own scope) | — | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Register patients | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Edit patients | ✓ | ✓ | ✓ | — | — | — | — |
| Create appointments | ✓ | ✓ | — | ✓ | — | — | — |
| Create prescriptions | ✓ | ✓ | — | — | — | — | — |
| Create lab orders | ✓ | ✓ | — | — | — | — | — |
| Admit/discharge patients | ✓ | ✓ | ✓ | — | — | — | — |
| Update bed status | ✓ | — | ✓ | — | — | — | — |
| Update lab results | ✓ | — | — | — | — | ✓ | — |
| Update inventory | ✓ | — | — | — | ✓ | — | — |
| Create invoices | ✓ | — | — | — | — | — | ✓ |
| Record payments | ✓ | — | — | — | — | — | ✓ |
| View revenue/trends | ✓ | — | — | — | — | — | — |
| Manage staff/users | ✓ | — | — | — | — | — | — |
| Export database | ✓ | — | — | — | — | — | — |

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
