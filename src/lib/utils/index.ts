import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import type { Patient } from "$lib/lib/types.js";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatDate(date: string | Date): string {
  const d = new Date(date);
  return d.toLocaleDateString("en-IN", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

export function formatDateTime(date: string | Date): string {
  const d = new Date(date);
  return d.toLocaleDateString("en-IN", {
    year: "numeric",
    month: "short",
    day: "numeric",
    hour: "2-digit",
    minute: "2-digit",
  });
}

export function formatCurrency(amount: number): string {
  return new Intl.NumberFormat("en-IN", {
    style: "currency",
    currency: "INR",
    maximumFractionDigits: 0,
  }).format(amount);
}

export function formatTime(time: string): string {
  const [hours, minutes] = time.split(":");
  const h = parseInt(hours);
  const ampm = h >= 12 ? "PM" : "AM";
  const displayHour = h % 12 || 12;
  return `${displayHour}:${minutes} ${ampm}`;
}

export function getStatusColor(status: string): string {
  const colors: Record<string, string> = {
    scheduled: "bg-blue-500/15 text-blue-700 dark:text-blue-300",
    confirmed: "bg-green-500/15 text-green-700 dark:text-green-300",
    in_progress: "bg-yellow-500/15 text-yellow-700 dark:text-yellow-300",
    completed: "bg-green-500/15 text-green-700 dark:text-green-300",
    cancelled: "bg-red-500/15 text-red-700 dark:text-red-300",
    no_show: "bg-gray-500/15 text-gray-700 dark:text-gray-300",
    available: "bg-green-500/15 text-green-700 dark:text-green-300",
    occupied: "bg-red-500/15 text-red-700 dark:text-red-300",
    reserved: "bg-yellow-500/15 text-yellow-700 dark:text-yellow-300",
    cleaning: "bg-blue-500/15 text-blue-700 dark:text-blue-300",
    maintenance: "bg-gray-500/15 text-gray-700 dark:text-gray-300",
    active: "bg-green-500/15 text-green-700 dark:text-green-300",
    discharged: "bg-gray-500/15 text-gray-700 dark:text-gray-300",
    pending: "bg-yellow-500/15 text-yellow-700 dark:text-yellow-300",
    paid: "bg-green-500/15 text-green-700 dark:text-green-300",
    partial: "bg-orange-500/15 text-orange-700 dark:text-orange-300",
    overdue: "bg-red-500/15 text-red-700 dark:text-red-300",
    ordered: "bg-blue-500/15 text-blue-700 dark:text-blue-300",
  };
  return colors[status] || "bg-gray-500/15 text-gray-700 dark:text-gray-300";
}

export function getGenderBadge(gender: string): string {
  const colors: Record<string, string> = {
    male: "bg-blue-500/15 text-blue-700 dark:text-blue-300",
    female: "bg-pink-500/15 text-pink-700 dark:text-pink-300",
    other: "bg-purple-500/15 text-purple-700 dark:text-purple-300",
  };
  return colors[gender] || "bg-gray-500/15 text-gray-700 dark:text-gray-300";
}

export function getPriorityColor(priority: string): string {
  const colors: Record<string, string> = {
    stat: "bg-red-500/15 text-red-700 dark:text-red-300",
    urgent: "bg-orange-500/15 text-orange-700 dark:text-orange-300",
    routine: "bg-blue-500/15 text-blue-700 dark:text-blue-300",
  };
  return colors[priority] || "bg-gray-500/15 text-gray-700 dark:text-gray-300";
}

export function getPatientName(patients: Patient[], id: string): string {
  const p = patients.find((p) => p.id === id);
  return p ? `${p.first_name} ${p.last_name}` : "Unknown";
}

export function buildPatientMap(patients: Patient[]): Map<string, Patient> {
  return new Map(patients.map((p) => [p.id, p]));
}

export function getInitials(firstOrFull: string, last?: string): string {
  if (last) return `${firstOrFull[0]}${last[0]}`.toUpperCase();
  return firstOrFull
    .split(" ")
    .map((n) => n[0])
    .join("")
    .toUpperCase()
    .slice(0, 2);
}

export function getStatusLabel(status: string): string {
  const labels: Record<string, string> = {
    scheduled: "Scheduled",
    confirmed: "Confirmed",
    in_progress: "In Progress",
    completed: "Completed",
    cancelled: "Cancelled",
    no_show: "No Show",
    ordered: "Ordered",
    paid: "Paid",
    pending: "Pending",
    partial: "Partial",
    overdue: "Overdue",
    active: "Active",
    discharged: "Discharged",
    available: "Available",
    occupied: "Occupied",
    reserved: "Reserved",
    cleaning: "Cleaning",
    maintenance: "Maintenance",
    transferred: "Transferred",
    routine: "Routine",
    urgent: "Urgent",
    stat: "STAT",
    routine_checkup: "Routine Checkup",
    walk_in: "Walk-in",
    follow_up: "Follow-up",
    consultation: "Consultation",
  };
  return labels[status] || status.replace(/_/g, " ");
}

export function debounce<T extends (...args: unknown[]) => unknown>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: ReturnType<typeof setTimeout>;
  return (...args: Parameters<T>) => {
    clearTimeout(timeout);
    timeout = setTimeout(() => func(...args), wait);
  };
}
