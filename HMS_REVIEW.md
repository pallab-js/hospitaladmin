# Critical Code Review: `hospitaladmin` (HMS)

**Stack:** Tauri 2 (Rust) + SvelteKit 5 (TS) + SQLite (WAL) + bcrypt
**Scope reviewed:** `src-tauri/src/**` (auth, db, commands, models, utils), `src-tauri/Cargo.toml`, `tauri.conf.json`, `migrations/001_initial.sql`, `db/seed.rs`, `package.json`, `.github/workflows/ci.yml`.

This is a first-pass review of the public tree. Severity is my best guess from the code alone — please verify against your real usage and threat model.

---

## TL;DR

The skeleton is solid: role guards, parameterized SQL, bcrypt, CSP, WAL, FK on, audit log table — all good defaults for an HMS. The big issues are around **session/state model**, **multi-tab / multi-window support**, **migrations**, **seed/doctor-account coupling**, **input validation gaps**, **concurrent-write races**, and **CI**. None of these are showstoppers, but several are correctness bugs, not just polish.

I rated them **🔴 critical**, **🟠 high**, **🟡 medium**, **🔵 low**.

---

## 🔴 1. Sessions are stored in a process-local `LazyLock<Mutex<…>>`

`src-tauri/src/auth/session.rs`

```rust
static CURRENT_SESSION: LazyLock<Mutex<Option<SessionEntry>>> = LazyLock::new(...);
```

Problems:
- **Only one session at a time per app instance.** Open a second Tauri window (or refresh) and the doctor’s session is overwritten/lost. Tauri 2 supports multi-window; the auth model does not.
- **No persistence.** Kill the process, lose the session. Combined with the 1-hour expiry, every cold start is a forced re-login. Fine for a kiosk, terrible for a working hospital workstation.
- **No CSRF/origin binding.** Tauri IPC is locked down, so the risk is small, but the session is identified only by being-in-this-process.
- **Mutex around a single `Option`** is fine perf-wise, but `unwrap()` on lock will panic under contention; at least use `let _ = s.lock().ok();`.

**Fixes (in order of impact):**

1. **Move session into SQLite.** Add a `sessions` table (id TEXT PK, user_id, role, employee_id, created_at, expires_at, last_seen_at) and key it by an opaque token stored in the frontend (cookie/localStorage). On every command, look up the token and check `expires_at > now`. This also lets you have multiple concurrent users (e.g. reception + management) on different windows.
2. Until then, at minimum: replace `unwrap()` with graceful `Err`, add a `refresh_session()` on activity, and log session creation/refresh/expiry in `audit_log`.
3. Bind the session token to the window/webview label so a stolen token can’t be replayed from a different window.

---

## 🔴 2. No login rate limiting / lockout

`src-tauri/src/auth/commands.rs::login`

```rust
let valid = bcrypt::verify(&request.password, &password_hash)...
```

bcrypt at cost 12 is the right shape, but you have **no failed-attempt tracking**. A doctor account with a 6-char password can be brute-forced offline-ish (if the DB file leaks) and a 4-char password can be brute-forced online in hours, since the only cost is one bcrypt verify per try.

**Fixes:**
- Add a `failed_logins` table (`username`, `ip_or_label`, `attempted_at`) or a column `users.failed_attempts`, `users.locked_until`.
- After 5 failures in 15 min: lock the account for 15 min; after 20: lock for 24h; require management role to unlock.
- Always respond in constant time (run a dummy `bcrypt::verify` on the not-found branch — see fix #3).
- Log every failed attempt to `audit_log` with `action='login_failed'`.

---

## 🔴 3. Username enumeration via timing & response

`commands.rs`:
```rust
match row {
    Some(row) => { /* bcrypt verify */ }
    None => Ok(LoginResponse { success: false, ... }),
}
```

`bcrypt::verify` takes ~200ms; the `None` branch returns immediately. An attacker can probe valid usernames in seconds.

**Fix:** on the `None` branch, run `bcrypt::verify(&request.password, &"$2b$12$....valid_looking_hash....")` against a precomputed dummy hash, then return the same `LoginResponse`. That equalizes the timing path.

Also: the message is already generic ("Invalid credentials") — good. Keep it that way; don't add "User does not exist".

---

## 🔴 4. Migrations are not really migrations

`src-tauri/src/db/mod.rs`:

```rust
sqlx::query(include_str!("../../migrations/001_initial.sql"))
    .execute(pool).await?;
```

Two real problems:

1. **Only one file is ever run.** Anything you add as `002_*.sql` will be ignored. There is no `schema_migrations` table, no ordering, no `IF NOT EXISTS` tracking per-file.
2. **Re-running on existing DBs.** Your init SQL uses `CREATE TABLE IF NOT EXISTS`, so it’s idempotent — but if you ever need to `ALTER TABLE` you have no mechanism. This will bite you the moment you ship 0.2.0.

**Fix:** introduce a `schema_migrations(version TEXT PRIMARY KEY, applied_at TEXT)` table; on startup, list files in `migrations/`, skip ones already applied, run the rest in order inside a transaction. Use `sqlx::migrate!` if you can move to it (compile-time check), or hand-roll the loader. Either way, separate `CREATE TABLE` (001) from future `ALTER`s (002+).

---

## 🟠 5. `generate_patient_uid` and `generate_invoice_number` race

`src-tauri/src/commands/patients.rs` and `billing_write.rs` both do:

```rust
let max_uid: Option<String> = sqlx::query_scalar("SELECT MAX(patient_uid) FROM patients")...
let next = uid.strip_prefix("HMS-").and_then(|s| s.parse().ok()).unwrap_or(0) + 1;
Ok(format!("HMS-{:05}", next))
```

Classic **read-then-write race**: two patients registered in the same millisecond both see `MAX = HMS-00042` and both try to insert `HMS-00043`. The second hits the `UNIQUE` constraint and crashes the command with a generic "Failed to create patient record".

**Fixes (pick one):**
- **Best:** dedicated `counters` table with `UPDATE counters SET n = n+1 RETURNING n`, inside the same transaction as the insert.
- **Acceptable:** let SQLite assign via `INSERT … ON CONFLICT(patient_uid) DO UPDATE SET patient_uid = patient_uid` loop, or retry on UNIQUE violation.
- **Easiest short-term:** wrap insert + check in a transaction, and on UNIQUE error, retry the max-and-insert a few times.

Same problem applies to `invoice_number` in `billing_write.rs::generate_invoice_number`.

---

## 🟠 6. `seed.rs` derives usernames from staff first/last names

`src-tauri/src/db/seed.rs`:

```rust
let username = format!("{}{}", first_name.to_lowercase(), last_name.to_lowercase().chars().next().unwrap_or('x'));
```

Problems:
- **Username collisions are silently ignored.** "Sarah Johnson" and "Sara Johnson" both become `sarahj`. The second `INSERT` fails the `UNIQUE` constraint and the `?` propagates, aborting the whole seed → the app boots without a doctor user, then login fails with no obvious error in the UI.
- **Username "doctor" is hard-coded in README but not seeded.** README claims `doctor / doctor123` works, but `seed.rs` only creates per-doctor accounts (e.g. `sarahj`) and one admin. The single `doctor` account mentioned in the README will not exist after seeding. The README is wrong, or the seed is incomplete.
- **Deterministic username = predictable login target.** A small hospital with 50 staff gives an attacker 50 usernames to try.

**Fixes:**
- After generating each username, check for collision; on collision, append a numeric suffix or use a stable `slugify(first+last) + short uuid`.
- Either seed a single canonical `doctor` user as README claims, or correct the README to say "log in as `sarahj / doctor123`".
- Document that auto-generated usernames are only for dev; in prod, require explicit username creation through management UI.

---

## 🟠 7. `get_ward_occupancy` is not in `lib.rs` but `ward_occupancy` is read by the UI

`src-tauri/src/lib.rs` registers `commands::beds::get_ward_occupancy`, but I didn’t see an explicit ward/bed write command (create ward, add bed, assign bed to admission). `admissions::create_admission` is registered, but I don’t see a corresponding "assign bed" command. **If you intend beds to be auto-assigned, that's fine — but please verify `admissions.rs` actually updates `beds.status = 'occupied'` in a transaction**, and add the missing command(s) to `lib.rs` if not.

(Same review note for: `commands::pharmacy::get_medications` and `get_medication_stock` exist but I see no write command for adjusting stock when `dispense_prescription_item` runs. The prescription dispense should decrement stock atomically in the same transaction; verify this and if missing, add it.)

---

## 🟠 8. `dispatch` and `staff` directory naming is inconsistent

`src-tauri/src/commands/` has both `beds.rs` and `staff.rs` (good) but the lab functionality is split across `lab.rs` (read), `lab_orders.rs` (write), and the file might not exist with that exact name. Same with `billing.rs` (read) and `billing_write.rs` (write). The split is a fair code-organization choice, but **document the convention** (e.g. a top-of-file comment in `commands/mod.rs` saying "read commands in `X.rs`, write commands in `X_write.rs`"). Without it, contributors won’t know where to add new commands.

Also: `commands::mod.rs` should `pub mod` each submodule. If it’s relying on `mod commands;` from `lib.rs` + 2021 edition + file-per-module auto-discovery, fine — but make it explicit, otherwise Rust 2021+ requires `pub mod` declarations in the parent.

---

## 🟠 9. `require_role` semantics are misleading

`src-tauri/src/auth/session.rs`:

```rust
pub fn require_role(role: &str) -> Result<Session, String> {
    let session = require_session()?;
    if session.role != role && session.role != "management" {
        return Err("Insufficient permissions".to_string());
    }
    Ok(session)
}
```

`require_role("staff")` would let management through, but `require_role("management")` would **not** let staff through (and that’s the only call site — `guards::management_only`). So the logic is *only* used for the management case where it behaves correctly. But:

- The function name suggests "must equal this role"; it actually means "must equal this role OR be management". Rename to `require_role_or_management` or split into `require_role_eq`, `require_management`, `require_any_of(&[...])`.
- Same pattern in `guards::doctor_only` (doctor OR management). Same in `staff_or_above` (everyone but doctor). These overlaps mean **management can do everything**, and doctor is the only one ever blocked. That may be intentional, but it’s a privilege-escalation footgun if a "doctor" account ever needs to do something "staff-only" — they can’t.

Document the role matrix explicitly: a small table in `guards.rs` like

| Capability         | doctor | staff | management |
|--------------------|:------:|:-----:|:----------:|
| read own appts     |   ✓    |   ✗   |     ✓      |
| record payment     |   ✗    |   ✓   |     ✓      |
| ...

Otherwise you’ll keep reinventing the wheel.

---

## 🟠 10. Tauri capabilities file is missing

`tauri.conf.json` is set up, but I don’t see a `src-tauri/capabilities/default.json` (or whatever you’re using for Tauri 2). Without it, the frontend can’t call *any* IPC commands in production builds. In dev mode it often "just works" with permissive defaults, which is why this slips through.

**Verify:** `src-tauri/capabilities/` exists with at least a `default.json` that lists your commands (or `*` for now). If it doesn’t, your `pnpm tauri build` will produce an app that boots but does nothing.

---

## 🟡 11. CSP allows `connect-src http://localhost:*`

```json
"connect-src ipc: http://ipc.localhost http://localhost:*; ..."
```

`http://localhost:*` is fine in dev but ships in the bundled app. Tighten to `ipc: http://ipc.localhost` only, and only add `http://localhost:*` under a `dev` profile or via a separate dev capability. Otherwise a packaged HMS could be tricked into fetching arbitrary localhost endpoints (SSRF-style if a devtool is running).

---

## 🟡 12. No transactions around multi-statement writes

Examples:
- `create_invoice` in `billing_write.rs`: inserts `invoices` row, then loops to insert each `invoice_items` row. If item 3 fails, the invoice header is orphaned. Wrap in a `pool.begin().await?` transaction.
- `record_payment` in same file: inserts a `payments` row, then runs a follow-up `UPDATE invoices SET status = …` (I saw it truncated, but please verify). The status update must be in the same transaction.
- `create_admission` / `discharge_patient`: should update `beds.status` and `admissions.status` atomically.
- `dispense_prescription_item`: should decrement `medications.stock_quantity` and mark the item dispensed in one transaction, with a `CHECK (stock_quantity >= 0)` constraint to prevent negative stock.

For an HMS these are not theoretical: a partially-created invoice is a billing error, not a crash.

---

## 🟡 13. `audit::log_audit` swallows errors

```rust
let _ = sqlx::query("INSERT INTO audit_log ...").execute(get_pool()).await;
```

If the audit insert fails, **you silently lose the audit row**. For an HMS in any regulated environment, that’s a compliance problem. At minimum, `eprintln!` / `tracing::error!` the failure and propagate it to the caller so the user can retry. Better: have a single dead-letter mechanism (write to a JSON file on disk) and surface a "audit failed" toast.

Also: the audit log has no index. With any volume, `SELECT … FROM audit_log WHERE user_id = ? AND created_at BETWEEN ?` will full-scan. Add `CREATE INDEX idx_audit_user_time ON audit_log(user_id, created_at)`.

---

## 🟡 14. Input validation lives in models but isn’t always called

You have `request.validate()?` in `create_patient` (good). I didn’t see it called in:
- `create_appointment`
- `create_lab_order`
- `create_prescription`
- `create_admission`
- `create_invoice` (a malicious caller can submit `unit_price = -1e9` and create a credit invoice)

Either call `.validate()?` in **every** write command, or move validation into a single decorator / trait so it can’t be forgotten. A lint or `#[tauri::command]` wrapper that rejects empty payloads would help.

Specifically, check:
- Numeric ranges (prices, quantities ≥ 0; `quantity <= 1_000_000`)
- Date sanity (`appointment_date` not in 1970 or 2099)
- Enum validation (`gender`, `blood_group`, `payment_method`, `status`) — currently these go straight to SQL with a `CHECK` constraint as your only line of defense. Returning a useful 4xx to the user beats a DB error.
- Length limits on free text (e.g. `notes` capped at 10_000 chars to prevent DB bloat).

---

## 🟡 15. SQL injection surface in `dashboard.rs::get_revenue_chart`

```rust
sqlx::query(
    r#"SELECT payment_date as date, SUM(amount) as amount
       FROM payments
       WHERE payment_date >= date('now', '-' || ? || ' days')
       GROUP BY payment_date
       ORDER BY payment_date ASC"#
)
```

The `?` is bound, so this is safe. ✅ But the same pattern elsewhere might not be. Worth a quick grep: `grep -rn "format!\|push_str" src-tauri/src/commands | grep -i "query\|sql"` to make sure no one is building SQL via `format!` somewhere I didn’t see.

(I already saw `patients::search_patients` builds the query via `String::push_str` with bound values — that’s correct, but it’s the kind of pattern that becomes a footgun in a future PR.)

---

## 🟡 16. CI: only `cargo check`, not `cargo test` / `cargo clippy`

`.github/workflows/ci.yml`:
```yaml
- run: cargo check --manifest-path src-tauri/Cargo.toml
```

- No `cargo test` even though the project structure invites unit tests (and there are no `#[cfg(test)]` modules either — please add some, especially for `guards`, `audit`, and the ID generators).
- No `cargo clippy -- -D warnings`. Clippy will catch the `unwrap()` on Mutex, the `let _ =` swallowing in `audit`, and several other things in this review.
- No `cargo fmt --check`.
- The `pnpm check` only runs `svelte-check`; it does not typecheck the `src/lib` code that calls Tauri IPC, and it doesn’t lint. Add `pnpm lint` (eslint/prettier) and ideally a Vitest pass for stores.
- `pnpm/action-setup@v4` pins to whatever the repo says; you depend on `pnpm@10.33.4` (from `packageManager`). Make sure CI honors `packageManager` or pin explicitly.
- No build artifact (i.e. `pnpm tauri build` on a tag) — you’ll want a release workflow soon.

---

## 🟡 17. `chart.js` + `svelte-chartjs` is heavy for a desktop app

You have `chart.js ^4.5.1` + `svelte-chartjs ^4.0.1` for the revenue/load dashboards. That’s ~200KB gzipped just for charts. For a Tauri desktop app with SQLite locally, you can do this with raw SVG and get faster paint, no canvas, and crisper HiDPI rendering. Or use `layerchart` (Svelte-native, tree-shakable). Not a bug, just a bundle-hygiene note.

---

## 🟡 18. `superforms ^0.0.1`?

```json
"superforms": "^0.0.1"
```

That version doesn’t look real — Superforms is at 2.x. Either you mean a different package, or this will resolve to garbage. Remove it or fix the name.

---

## 🟡 19. No backup / export story for the SQLite file

For a hospital app this is non-negotiable. The DB lives in `app.path().app_data_dir()/hms.db`. There’s no command to:
- Export a `.db` snapshot
- Export individual tables to CSV (especially `audit_log` for compliance)
- Restore from a snapshot
- Vacuum/reindex on a schedule

Add at least a `tauri::command::export_database(path: String)` behind `management_only` that copies `hms.db` to a user-chosen path, and a corresponding restore.

---

## 🟡 20. README claims `pnpm dev` runs without Tauri — verify

README:
> Start dev server (browser mode - no Tauri)
> pnpm dev

But the Tauri commands use `tauri-plugin-shell`, IPC, `invoke()` calls, etc. If your SvelteKit dev server can’t reach the Rust backend, **the entire app will be broken in browser mode**. Either:
- Mock the Tauri API in dev with a Vite plugin, or
- Add a `MOCK_TAURI=1` env that swaps `invoke()` for in-memory fakes, or
- Remove the "browser mode" claim from the README and require `pnpm tauri dev`.

Otherwise the demo credentials table in the README is misleading — new contributors will run `pnpm dev`, get errors, and bounce.

---

## 🔵 21. `unwrap_or(0)` on financial aggregates

`dashboard.rs`:
```rust
let revenue_today: f64 = ... .fetch_one(pool).await.unwrap_or(0.0);
```

On a transient DB error you silently return 0 revenue. The dashboard will display "₹0 today" and nobody will notice the underlying failure. Use `.map_err(...)?` or at least `tracing::warn!` so this shows up in the log.

---

## 🔵 22. `chrono::Utc::now()` for business dates

You format dates with `chrono::Utc::now()` everywhere, then store as TEXT in SQLite. Two issues:
- A user in IST (your default timezone) at 23:30 local will see an appointment "scheduled for tomorrow" while UTC says it’s already tomorrow. Use `chrono::Local::now()` for the date, and store time zones explicitly if you ever need them.
- TEXT dates mean you can’t easily do date math. Consider ISO-8601 (`%Y-%m-%dT%H:%M:%S%z`) and document it.

---

## 🔵 23. `is_active: i64 == 1` everywhere

`Patient` deserialization has `is_active: r.get::<i64, _>("is_active") == 1,` repeated in three places. Use `FromRow` derive, or define `From<i64> for bool` once. Same for the dozens of similar `Patient { id: r.get("id"), ... }` constructors — `sqlx::FromRow` would shrink the codebase 30%.

---

## 🔵 24. `commands::dashboard::get_dashboard_stats` is god-function

224 lines, two parallel `if/else` blocks for doctor vs non-doctor with 8 query branches. Refactor:
- Extract `fn patient_count_today(pool, doctor_id: Option<&str>) -> i64`
- Extract `fn bed_stats(pool) -> (i64, i64)`
- Extract `fn monthly_patients(pool, since: &str) -> i64`
- Compose them in a `match` on `session.role`.

Each becomes testable in isolation.

---

## 🔵 25. The "all but doctor" guard is weird

```rust
pub fn staff_or_above() -> Result<Session, String> {
    let session = require_session()?;
    if session.role == "doctor" {
        return Err("Staff access required".to_string());
    }
    Ok(session)
}
```

So `staff_or_above` actually means "everyone except doctors". A `pharmacist` staff account is fine, a `nurse` is fine, `management` is fine. The name suggests a hierarchy (`staff < doctor < management`) but the implementation is a flat exclusion. Pick a clearer name: `non_doctor_only` or `staff_and_management_only`.

Also: I didn’t see this function called anywhere in the registered commands. Dead code? If so, delete it; if not, document the call sites.

---

## 🔵 26. Frontend-side: verify route guards

I didn’t read the Svelte files, but the SvelteKit `+layout.server.ts` (or `+layout.ts`) needs to enforce that:
- Unauthenticated users get redirected to `/login`
- Authenticated users hitting `/login` get redirected to their role-specific dashboard
- A doctor hitting `/billing/admin` is denied at the route level (defense in depth — the backend guard is the source of truth)

If any of this is missing, the "auth guards on all commands" claim is undermined by a UI that exposes admin nav to doctors.

---

## Priority order to fix

1. **🔴 #4 Migrations** — will compound; do it now.
2. **🔴 #2 #3 Login rate limit + timing** — one afternoon, big win.
3. **🟠 #5 Invoice/patient UID race** — production data corruption waiting to happen.
4. **🟠 #6 Seed username collisions** — users will hit this in 5 minutes.
5. **🟠 #10 Tauri capabilities file** — confirm it exists.
6. **🟠 #12 Transactions on multi-write commands** — correctness.
7. **🟡 #14 Validation everywhere** — security hygiene.
8. **🟡 #16 CI: tests + clippy + fmt** — quality flywheel.
9. **🟡 #19 DB backup/export** — operational necessity.
10. Everything else as time allows.

---

## What’s good (worth keeping)

- Parameterized SQL everywhere I looked.
- `bcrypt` cost 12.
- WAL mode + foreign keys on.
- Audit log table exists with the right columns.
- CSP on the webview.
- Role-aware backend guards.
- Atomic ID generation via UUID v4.
- Single source of truth for the DB pool.
- Clear `commands/`, `models/`, `auth/`, `db/`, `utils/` separation.
- Sensible use of `tauri-plugin-shell` (and only that one).
- `manager` + `staff` + `doctor` seed data is realistic and useful for testing.

---

*Generated from a static review of the public main branch. Nothing was executed. Run `cargo clippy -- -D warnings` and `pnpm tauri dev` after addressing items #4, #5, #6, #10 first.*
