import { z } from "zod";

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
  visit_type: z.enum(["consultation", "follow_up", "emergency", "walk_in"]).optional(),
  reason: z.string().optional(),
});

export type AppointmentFormData = z.infer<typeof appointmentSchema>;
