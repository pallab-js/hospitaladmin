<script lang="ts">
  import { page } from "$app/stores";
  import Button from "$lib/components/ui/button/index.svelte";
  import Card from "$lib/components/ui/card/index.svelte";
  import CardContent from "$lib/components/ui/card/card-content.svelte";
  import { AlertTriangle, Home, RefreshCw } from "@lucide/svelte";

  function goHome() {
    window.location.href = "/dashboard";
  }

  function refresh() {
    window.location.reload();
  }
</script>

<div class="flex min-h-screen items-center justify-center bg-background p-4">
  <Card class="w-full max-w-md">
    <CardContent class="p-8 text-center space-y-6">
      <div class="mx-auto flex h-16 w-16 items-center justify-center rounded-full bg-destructive/10">
        <AlertTriangle class="h-8 w-8 text-destructive" />
      </div>
      <div>
        <h1 class="text-2xl font-bold">Something went wrong</h1>
        <p class="mt-2 text-muted-foreground">
          {#if $page.error?.message}
            {$page.error.message}
          {:else}
            An unexpected error occurred. Please try again.
          {/if}
        </p>
        {#if $page.status}
          <p class="mt-1 text-sm text-muted-foreground">Error code: {$page.status}</p>
        {/if}
      </div>
      <div class="flex justify-center gap-3">
        <Button variant="outline" onclick={refresh}>
          <RefreshCw class="h-4 w-4 mr-2" />
          Try Again
        </Button>
        <Button onclick={goHome}>
          <Home class="h-4 w-4 mr-2" />
          Go to Dashboard
        </Button>
      </div>
    </CardContent>
  </Card>
</div>
