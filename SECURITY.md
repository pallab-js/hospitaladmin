# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability in HMS, please report it responsibly:

**Do NOT open a public GitHub issue for security vulnerabilities.**

Instead, please email: [INSERT YOUR SECURITY EMAIL]

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact assessment
- Any suggested fix (optional)

You should receive an acknowledgment within 48 hours. We will work with you to understand and address the issue before any public disclosure.

## Scope

This security policy applies to:
- The HMS desktop application (Tauri/Rust backend)
- The SvelteKit frontend bundled with the application
- SQLite database storage
- Authentication and authorization mechanisms

## Out of Scope

- Third-party dependencies (report upstream)
- Physical security of hospital machines
- Network security of the hospital's private network

## Security Measures

### Authentication
- Passwords hashed with bcrypt (cost factor 12)
- Account lockout after failed attempts (5 attempts → 15min, 20 → 24h)
- Constant-time login responses to prevent timing attacks
- Session expiry (1 hour) with activity refresh

### Authorization
- Role-based access control (RBAC) enforced on all backend commands
- Roles: admin, doctor, nurse, receptionist, pharmacist, lab_tech, billing_staff
- Admin bypasses all role checks

### Data Protection
- All SQL queries use parameterized statements (no SQL injection)
- Content Security Policy (CSP) enabled on the webview
- Audit logging for all write operations
- Patient data stored in local SQLite database only

### Input Validation
- Backend validation on all write operations
- Frontend validation via Zod schemas
- Password complexity requirements enforced

## Best Practices for Deployment

1. **Change default credentials** immediately after first run
2. **Enable full-disk encryption** on machines running HMS
3. **Regular database backups** using the export feature
4. **Restrict physical access** to machines running HMS
5. **Keep the application updated** with latest security patches
6. **Monitor audit logs** for suspicious activity
7. **Do not share user accounts** between staff members

## Data Handling

This application handles sensitive patient health information (PHI). Ensure compliance with:
- Local hospital data protection regulations
- Patient privacy laws applicable in your jurisdiction
- Your organization's data governance policies

## Disclaimer

This software is provided for administrative and organizational purposes only. It is NOT a medical device and should NOT be used for clinical decision-making, diagnosis, or treatment. All medical decisions must be made by qualified healthcare professionals.
