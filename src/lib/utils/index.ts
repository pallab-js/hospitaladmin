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
    scheduled: "bg-blue-100 text-blue-800",
    confirmed: "bg-green-100 text-green-800",
    in_progress: "bg-yellow-100 text-yellow-800",
    completed: "bg-green-100 text-green-800",
    cancelled: "bg-red-100 text-red-800",
    no_show: "bg-gray-100 text-gray-800",
    available: "bg-green-100 text-green-800",
    occupied: "bg-red-100 text-red-800",
    reserved: "bg-yellow-100 text-yellow-800",
    cleaning: "bg-blue-100 text-blue-800",
    maintenance: "bg-gray-100 text-gray-800",
    active: "bg-green-100 text-green-800",
    discharged: "bg-gray-100 text-gray-800",
    pending: "bg-yellow-100 text-yellow-800",
    paid: "bg-green-100 text-green-800",
    partial: "bg-orange-100 text-orange-800",
    overdue: "bg-red-100 text-red-800",
    ordered: "bg-blue-100 text-blue-800",
  };
  return colors[status] || "bg-gray-100 text-gray-800";
}

export function getGenderBadge(gender: string): string {
  const colors: Record<string, string> = {
    male: "bg-blue-100 text-blue-800",
    female: "bg-pink-100 text-pink-800",
    other: "bg-purple-100 text-purple-800",
  };
  return colors[gender] || "bg-gray-100 text-gray-800";
}

export function getPriorityColor(priority: string): string {
  const colors: Record<string, string> = {
    stat: "bg-red-100 text-red-800",
    urgent: "bg-orange-100 text-orange-800",
    routine: "bg-blue-100 text-blue-800",
  };
  return colors[priority] || "bg-gray-100 text-gray-800";
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
