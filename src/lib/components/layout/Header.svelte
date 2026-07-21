<script lang="ts">
  import { page } from "$app/stores";
  import { auth, userName, userRole } from "$lib/stores/auth.js";
  import { sidebar } from "$lib/stores/sidebar.js";
  import { notifications } from "$lib/stores/notifications.js";
  import { Search, Bell, Menu } from "@lucide/svelte";
  import Avatar from "$lib/components/ui/avatar/index.svelte";
  import AvatarFallback from "$lib/components/ui/avatar/avatar-fallback.svelte";
  import Button from "$lib/components/ui/button/index.svelte";

  const breadcrumbs = $derived(
    $page.url.pathname
      .split("/")
      .filter(Boolean)
      .map((segment, index, arr) => ({
        label: segment.charAt(0).toUpperCase() + segment.slice(1).replace(/-/g, " "),
        href: "/" + arr.slice(0, index + 1).join("/"),
        isLast: index === arr.length - 1,
      }))
  );

  const unreadCount = $derived($notifications.length);

  function getInitials(name: string): string {
    return name
      .split(" ")
      .map((n) => n[0])
      .join("")
      .toUpperCase()
      .slice(0, 2);
  }

  let searchQuery = $state("");
  let showSearch = $state(false);

  function handleSearch() {
    if (searchQuery.trim()) {
      notifications.add({ type: "info", title: "Search", message: `Searching for "${searchQuery}"...` });
      searchQuery = "";
      showSearch = false;
    }
  }
</script>

<header class="sticky top-0 z-30 flex h-16 items-center gap-4 border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60 px-6">
  <!-- Mobile menu button -->
  <Button variant="ghost" size="icon" class="lg:hidden" onclick={() => sidebar.openMobile()}>
    <Menu class="h-5 w-5" />
  </Button>

  <!-- Breadcrumbs -->
  <nav class="flex items-center gap-1 text-sm">
    <a href="/dashboard" class="text-muted-foreground hover:text-foreground">Home</a>
    {#each breadcrumbs as crumb}
      <span class="text-muted-foreground">/</span>
      {#if crumb.isLast}
        <span class="font-medium">{crumb.label}</span>
      {:else}
        <a href={crumb.href} class="text-muted-foreground hover:text-foreground">{crumb.label}</a>
      {/if}
    {/each}
  </nav>

  <!-- Right side -->
  <div class="ml-auto flex items-center gap-4">
    <!-- Search -->
    {#if showSearch}
      <form onsubmit={(e) => { e.preventDefault(); handleSearch(); }} class="flex items-center gap-2">
        <input
          type="text"
          placeholder="Search patients, appointments..."
          bind:value={searchQuery}
          class="h-9 w-64 rounded-md border bg-background px-3 text-sm outline-none focus:ring-2 focus:ring-ring"
        />
        <Button type="submit" size="sm">Go</Button>
        <Button type="button" variant="ghost" size="sm" onclick={() => { showSearch = false; searchQuery = ""; }}>Cancel</Button>
      </form>
    {:else}
      <Button variant="ghost" size="icon" onclick={() => (showSearch = true)}>
        <Search class="h-5 w-5" />
      </Button>
    {/if}

    <!-- Notifications -->
    <Button variant="ghost" size="icon" class="relative" onclick={() => notifications.add({ type: "info", title: "Notifications", message: "No new notifications" })}>
      <Bell class="h-5 w-5" />
      {#if unreadCount > 0}
        <span class="absolute -top-1 -right-1 h-4 w-4 rounded-full bg-destructive text-[10px] font-bold text-destructive-foreground flex items-center justify-center">{unreadCount > 9 ? "9+" : unreadCount}</span>
      {/if}
    </Button>

    <!-- User menu -->
    <div class="flex items-center gap-3">
      <Avatar>
        <AvatarFallback class="bg-primary text-primary-foreground text-sm">
          {getInitials($userName)}
        </AvatarFallback>
      </Avatar>
      <div class="hidden md:block">
        <p class="text-sm font-medium">{$userName}</p>
        <p class="text-xs text-muted-foreground capitalize">{$userRole}</p>
      </div>
    </div>
  </div>
</header>
