# Hospital Management System (HMS)

Enterprise-grade hospital management desktop application for a small hospital (<50 beds) operating on a private LAN.

## Tech Stack

- **Frontend**: SvelteKit 2, Svelte 5 (runes), TypeScript 5, TailwindCSS 4
- **Desktop Shell**: Tauri v2 (Rust backend)
- **UI Components**: shadcn-svelte (Svelte 5 native)
- **Charts**: Chart.js + svelte-chartjs
- **Icons**: @lucide/svelte
- **Database**: SQLite (embedded, WAL mode) via sqlx (Rust)
- **Auth**: bcrypt + session token (local)
- **Forms**: Superforms + Zod

## Architecture

```
Tauri v2 Desktop Shell
├── SvelteKit Frontend (adapter-static → WebView)
│   ├── Routes (file-based)
│   ├── Components (ui/, domain/, layout/, charts/)
│   ├── Stores (auth, sidebar, notifications)
│   └── API wrapper (invoke to Rust)
└── Rust Backend
    ├── Commands (Tauri IPC handlers)
    ├── Models (serde structs)
    ├── DB (sqlx + SQLite)
    └── Auth (bcrypt + session)
```

## User Roles

| Role | Access Level |
|------|-------------|
| **doctor** | Patient records, appointments, prescriptions, lab orders, own dashboard |
| **management** | Full access to all modules, analytics, staff management, reports |
| **staff** | Patient registration, appointments, bed allocation, pharmacy dispensing, lab data entry |

## Modules

1. **Patient Registration & Records** - CRUD patients, search, medical history
2. **Appointment Scheduling** - Calendar, booking, status tracking
3. **Doctor Dashboard** - Today's patients, schedule, quick actions
4. **Billing & Payments** - Invoices, payments, revenue tracking
5. **Pharmacy / Inventory** - Medications, stock, dispensing, expiry alerts
6. **Lab & Diagnostics** - Test orders, results, abnormal flagging
7. **Bed / Ward Management** - Occupancy visualization, admit/discharge
8. **Staff Management** - Directory, shifts, attendance, leaves
9. **Dashboard & Analytics** - KPIs, charts per role
10. **Reporting & Export** - PDF/CSV reports, date range filters

## Design System

- **Primary**: Medical Teal (#0f766e)
- **Font**: Inter
- **Components**: shadcn-svelte with custom hospital theme
- **Dark mode**: Supported via mode-watcher
- **Spacing**: 4px base, TailwindCSS utilities

## Key Conventions

- Patient UID: `HMS-XXXXX`
- Invoice number: `INV-XXXXXX`
- All IDs: UUID v4
- Database: SQLite with WAL mode
- Auth: Session-based (in-memory, single-user desktop)
