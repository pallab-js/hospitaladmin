# HMS - Hospital Management System

A modern, desktop-first hospital management system built with **Tauri 2**, **SvelteKit 5**, **Rust**, and **SQLite**.

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

# Start dev server (browser mode - no Tauri)
pnpm dev

# Start Tauri dev (desktop app)
pnpm tauri dev
```

### Build

```bash
# Build for production
pnpm tauri build
```

## Demo Credentials

When running in browser mode (`pnpm dev`), the following demo accounts are available:

| Username | Password | Role |
|----------|----------|------|
| `admin` | `admin123` | Management |
| `doctor` | `doctor123` | Doctor |
| `staff` | `staff123` | Staff |

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
│   └── src/
│       ├── auth/           # Authentication
│       ├── commands/       # Tauri command handlers
│       ├── models/         # Data models
│       └── db/             # Database & migrations
└── static/                 # Static assets
```

## Security

- Content Security Policy (CSP) enabled
- Auth guards on all API commands
- bcrypt password hashing (cost factor 12)
- Session expiry (1 hour)
- Audit logging for all write operations
- Input validation on backend and frontend

## License

[MIT](LICENSE)
