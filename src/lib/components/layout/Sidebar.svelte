<script lang="ts">
  import { page } from "$app/stores";
  import { auth } from "$lib/stores/auth.js";
  import { sidebar } from "$lib/stores/sidebar.js";
  import { cn } from "$lib/utils/index.js";
  import {
    LayoutDashboard,
    Users,
    Calendar,
    Building2,
    TestTube,
    Pill,
    Receipt,
    UserCog,
    BarChart3,
    Settings,
    LogOut,
    Menu,
    X,
  } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import { logout } from "$lib/lib/api.js";

  const allNavItems = [
    { title: "Dashboard", href: "/dashboard", icon: LayoutDashboard },
    { title: "Patients", href: "/patients", icon: Users },
    { title: "Appointments", href: "/appointments", icon: Calendar },
    { title: "Wards & Beds", href: "/wards", icon: Building2 },
    { title: "Lab & Diagnostics", href: "/lab", icon: TestTube },
    { title: "Pharmacy", href: "/pharmacy", icon: Pill },
    { title: "Billing", href: "/billing", icon: Receipt },
    { title: "Staff", href: "/staff", icon: UserCog, roles: ["management" as const] },
    { title: "Reports", href: "/reports", icon: BarChart3 },
    { title: "Settings", href: "/settings", icon: Settings },
  ];

  const navItems = $derived(
    allNavItems.filter(
      (item) => !item.roles || ($auth?.role && item.roles.includes($auth.role as any))
    )
  );

  async function handleLogout() {
    await logout();
    auth.logout();
    goto("/login");
  }
</script>

<!-- Desktop Sidebar -->
<aside
  class={cn(
    "fixed left-0 top-0 z-40 h-screen bg-sidebar text-sidebar-foreground transition-all duration-300",
    $sidebar.isOpen ? "w-64" : "w-20"
  )}
>
  <div class="flex h-full flex-col">
    <!-- Logo -->
    <div class="flex h-16 items-center justify-between border-b border-sidebar-border px-4">
      {#if $sidebar.isOpen}
        <div class="flex items-center gap-2">
          <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-sidebar-primary">
            <span class="text-sm font-bold text-sidebar-primary-foreground">H</span>
          </div>
          <span class="text-lg font-semibold">HMS</span>
        </div>
      {:else}
        <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-sidebar-primary mx-auto">
          <span class="text-sm font-bold text-sidebar-primary-foreground">H</span>
        </div>
      {/if}
      <button
        class="hidden lg:flex h-8 w-8 items-center justify-center rounded-md hover:bg-sidebar-accent"
        onclick={() => sidebar.toggle()}
      >
        <Menu class="h-4 w-4" />
      </button>
    </div>

    <!-- Navigation -->
    <nav class="flex-1 space-y-1 p-3">
      {#each navItems as item}
        {@const isActive = $page.url.pathname.startsWith(item.href)}
        <a
          href={item.href}
          class={cn(
            "flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors",
            isActive
              ? "bg-sidebar-primary text-sidebar-primary-foreground"
              : "text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
            !$sidebar.isOpen && "justify-center"
          )}
        >
          <item.icon class="h-5 w-5 shrink-0" />
          {#if $sidebar.isOpen}
            <span>{item.title}</span>
          {/if}
        </a>
      {/each}
    </nav>

    <!-- User Section -->
    <div class="border-t border-sidebar-border p-3">
      <button
        class={cn(
          "flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium text-sidebar-foreground hover:bg-sidebar-accent transition-colors",
          !$sidebar.isOpen && "justify-center"
        )}
        onclick={handleLogout}
      >
        <LogOut class="h-5 w-5 shrink-0" />
        {#if $sidebar.isOpen}
          <span>Logout</span>
        {/if}
      </button>
    </div>
  </div>
</aside>

<!-- Mobile Sidebar -->
{#if $sidebar.isMobileOpen}
  <div class="fixed inset-0 z-50 lg:hidden">
    <div class="fixed inset-0 bg-black/50" onclick={() => sidebar.closeMobile()} onkeydown={() => sidebar.closeMobile()} role="button" tabindex="-1"></div>
    <aside class="fixed left-0 top-0 h-full w-64 bg-sidebar text-sidebar-foreground">
      <div class="flex h-full flex-col">
        <div class="flex h-16 items-center justify-between border-b border-sidebar-border px-4">
          <div class="flex items-center gap-2">
            <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-sidebar-primary">
              <span class="text-sm font-bold text-sidebar-primary-foreground">H</span>
            </div>
            <span class="text-lg font-semibold">HMS</span>
          </div>
          <button
            class="h-8 w-8 flex items-center justify-center rounded-md hover:bg-sidebar-accent"
            onclick={() => sidebar.closeMobile()}
          >
            <X class="h-4 w-4" />
          </button>
        </div>

        <nav class="flex-1 space-y-1 p-3">
          {#each navItems as item}
            {@const isActive = $page.url.pathname.startsWith(item.href)}
            <a
              href={item.href}
              class={cn(
                "flex items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium transition-colors",
                isActive
                  ? "bg-sidebar-primary text-sidebar-primary-foreground"
                  : "text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
              )}
              onclick={() => sidebar.closeMobile()}
            >
              <item.icon class="h-5 w-5 shrink-0" />
              <span>{item.title}</span>
            </a>
          {/each}
        </nav>

        <div class="border-t border-sidebar-border p-3">
          <button
            class="flex w-full items-center gap-3 rounded-lg px-3 py-2.5 text-sm font-medium text-sidebar-foreground hover:bg-sidebar-accent transition-colors"
            onclick={handleLogout}
          >
            <LogOut class="h-5 w-5 shrink-0" />
            <span>Logout</span>
          </button>
        </div>
      </div>
    </aside>
  </div>
{/if}
