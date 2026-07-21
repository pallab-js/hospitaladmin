import { writable } from "svelte/store";

function createSidebarStore() {
  const { subscribe, set, update } = writable({
    isOpen: true,
    isMobileOpen: false,
  });

  return {
    subscribe,
    toggle: () => update((s) => ({ ...s, isOpen: !s.isOpen })),
    open: () => update((s) => ({ ...s, isOpen: true })),
    close: () => update((s) => ({ ...s, isOpen: false })),
    openMobile: () => update((s) => ({ ...s, isMobileOpen: true })),
    closeMobile: () => update((s) => ({ ...s, isMobileOpen: false })),
  };
}

export const sidebar = createSidebarStore();
