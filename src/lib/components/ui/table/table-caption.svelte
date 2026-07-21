<script lang="ts">
  import type { Snippet } from "svelte";
  import { cn } from "$lib/utils/index.js";

  let {
    class: className = "",
    children,
  }: {
    class?: string;
    children: Snippet;
  } = $props();
</script>

<caption class={cn("mt-4 text-sm text-muted-foreground", className)}>
  {@render children()}
</caption>
