import { writable } from "svelte/store";

export interface Notification {
  id: string;
  type: "success" | "error" | "warning" | "info";
  title: string;
  message?: string;
  duration?: number;
}

function createNotificationStore() {
  const { subscribe, update } = writable<Notification[]>([]);

  return {
    subscribe,
    add: (notification: Omit<Notification, "id">) => {
      const id = Math.random().toString(36).substr(2, 9);
      const duration = notification.duration ?? 5000;

      update((n) => [...n, { ...notification, id }]);

      if (duration > 0) {
        setTimeout(() => {
          update((n) => n.filter((item) => item.id !== id));
        }, duration);
      }
    },
    remove: (id: string) => {
      update((n) => n.filter((item) => item.id !== id));
    },
    clear: () => update(() => []),
  };
}

export const notifications = createNotificationStore();
