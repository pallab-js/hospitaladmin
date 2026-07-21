import { writable, derived } from "svelte/store";
import type { User } from "$lib/lib/types";

function createAuthStore() {
  const { subscribe, set, update } = writable<User | null>(null);

  return {
    subscribe,
    login: (user: User) => set(user),
    logout: () => set(null),
    update: (updater: (user: User | null) => User | null) => update(updater),
  };
}

export const auth = createAuthStore();
export const isAuthenticated = derived(auth, ($auth) => $auth !== null);
export const userRole = derived(auth, ($auth) => $auth?.role ?? null);
export const userName = derived(
  auth,
  ($auth) => $auth?.full_name ?? $auth?.username ?? "User"
);
