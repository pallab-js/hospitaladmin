import { z } from "zod";

export const loginSchema = z.object({
  username: z.string().min(1, "Username is required"),
  password: z.string().min(1, "Password is required"),
});

export type LoginFormData = z.infer<typeof loginSchema>;

export const registerSchema = z.object({
  username: z.string().min(3, "Username must be at least 3 characters").max(50, "Username must be 50 characters or less"),
  password: z
    .string()
    .min(8, "Password must be at least 8 characters")
    .regex(/[a-z]/, "Password must contain a lowercase letter")
    .regex(/[A-Z]/, "Password must contain an uppercase letter")
    .regex(/[0-9]/, "Password must contain a digit")
    .regex(/[^a-zA-Z0-9]/, "Password must contain a special character"),
  first_name: z.string().min(1, "First name is required").max(100, "First name must be 100 characters or less"),
  last_name: z.string().min(1, "Last name is required").max(100, "Last name must be 100 characters or less"),
  role: z.enum(["admin", "doctor", "nurse", "receptionist", "pharmacist", "lab_tech", "billing_staff"], {
    message: "Please select a valid role",
  }),
  email: z.string().email("Invalid email format").optional().or(z.literal("")),
  phone: z.string().max(20, "Phone must be 20 characters or less").optional().or(z.literal("")),
  qualification: z.string().optional().or(z.literal("")),
  specialization: z.string().optional().or(z.literal("")),
});

export type RegisterFormData = z.infer<typeof registerSchema>;

export const patientSchema = z.object({
  first_name: z.string().min(1, "First name is required").max(100, "First name must be 100 characters or less"),
  last_name: z.string().min(1, "Last name is required").max(100, "Last name must be 100 characters or less"),
  date_of_birth: z.string().regex(/^\d{4}-\d{2}-\d{2}$/, "Date must be in YYYY-MM-DD format"),
  gender: z.enum(["male", "female", "other"], { message: "Gender must be male, female, or other" }),
  blood_group: z.string().optional(),
  phone: z.string().max(20, "Phone must be 20 characters or less").optional().or(z.literal("")),
  email: z.string().email("Invalid email format").optional().or(z.literal("")),
  address: z.string().optional(),
  emergency_contact_name: z.string().optional(),
  emergency_contact_phone: z.string().optional(),
  insurance_provider: z.string().optional(),
  insurance_id: z.string().optional(),
  allergies: z.string().optional(),
  medical_history: z.string().optional(),
});

export type PatientFormData = z.infer<typeof patientSchema>;

export const appointmentSchema = z.object({
  patient_id: z.string().min(1, "Patient is required"),
  doctor_id: z.string().min(1, "Doctor is required"),
  department_id: z.string().optional(),
  appointment_date: z.string().regex(/^\d{4}-\d{2}-\d{2}$/, "Date must be in YYYY-MM-DD format"),
  appointment_time: z.string().regex(/^\d{2}:\d{2}$/, "Time must be in HH:MM format"),
  duration_minutes: z.number().min(5, "Duration must be at least 5 minutes").max(480, "Duration must be at most 480 minutes").optional(),
  visit_type: z.enum(["consultation", "follow_up", "emergency", "walk_in", "routine_checkup"]).optional(),
  reason: z.string().optional(),
});

export type AppointmentFormData = z.infer<typeof appointmentSchema>;

export const profileUpdateSchema = z.object({
  first_name: z.string().min(1, "First name is required").max(100).optional(),
  last_name: z.string().min(1, "Last name is required").max(100).optional(),
  email: z.string().email("Invalid email format").optional().or(z.literal("")),
  phone: z.string().max(20).optional().or(z.literal("")),
});

export type ProfileUpdateFormData = z.infer<typeof profileUpdateSchema>;

export const passwordChangeSchema = z.object({
  current_password: z.string().min(1, "Current password is required"),
  new_password: z
    .string()
    .min(8, "Password must be at least 8 characters")
    .regex(/[a-z]/, "Must contain a lowercase letter")
    .regex(/[A-Z]/, "Must contain an uppercase letter")
    .regex(/[0-9]/, "Must contain a digit")
    .regex(/[^a-zA-Z0-9]/, "Must contain a special character"),
  confirm_password: z.string().min(1, "Please confirm your password"),
}).refine((data) => data.new_password === data.confirm_password, {
  message: "Passwords do not match",
  path: ["confirm_password"],
});

export type PasswordChangeFormData = z.infer<typeof passwordChangeSchema>;
