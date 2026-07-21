<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { auth, isAuthenticated } from "$lib/stores/auth.js";
  import { sidebar } from "$lib/stores/sidebar.js";
  import { getCurrentUser } from "$lib/lib/api.js";
  import Sidebar from "$lib/components/layout/Sidebar.svelte";
  import Header from "$lib/components/layout/Header.svelte";
  import Toast from "$lib/components/ui/toast/index.svelte";
  import "../app.css";

  let { children } = $props();
  let loading = $state(true);

  const publicRoutes = ["/login"];
  const isPublicRoute = $derived(publicRoutes.includes($page.url.pathname));

  onMount(async () => {
    try {
      const user = await getCurrentUser();
      if (user) {
        auth.login(user);
      }
    } catch (e) {
      console.log("Not authenticated or running in browser mode");
    } finally {
      loading = false;
    }
  });

  $effect(() => {
    if (!loading && !$isAuthenticated && !isPublicRoute) {
      goto("/login");
    }
  });
</script>

<Toast />

{#if loading}
  <div class="flex h-screen items-center justify-center bg-background">
    <div class="flex flex-col items-center gap-4">
      <div class="h-10 w-10 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
      <p class="text-sm text-muted-foreground">Loading HMS...</p>
    </div>
  </div>
{:else if isPublicRoute}
  {@render children()}
{:else if $isAuthenticated}
  <div class="min-h-screen">
    <Sidebar />
    <div class="transition-all duration-300 {$sidebar.isOpen ? 'pl-64' : 'pl-20'}">
      <Header />
      <main class="p-6">
        {@render children()}
      </main>
    </div>
  </div>
{:else}
  <div class="flex h-screen items-center justify-center bg-background">
    <div class="flex flex-col items-center gap-4">
      <div class="h-10 w-10 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
      <p class="text-sm text-muted-foreground">Redirecting to login...</p>
    </div>
  </div>
{/if}
