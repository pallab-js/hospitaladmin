<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";

  onMount(() => {
    goto("/login");
  });
</script>

<div class="flex h-screen items-center justify-center">
  <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent"></div>
</div>
