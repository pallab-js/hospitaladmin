<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLSelectAttributes } from "svelte/elements";
  import { cn } from "$lib/utils/index.js";
  import { ChevronDown } from "@lucide/svelte";

  let {
    class: className = "",
    value = $bindable(),
    children,
    ...restProps
  }: HTMLSelectAttributes & { children: Snippet } = $props();
</script>

<div class="relative">
  <select
    class={cn(
      "flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 pr-8 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 [&>option]:bg-background [&>option]:text-foreground",
      className
    )}
    bind:value
    {...restProps}
  >
    {@render children()}
  </select>
  <ChevronDown class="absolute right-3 top-1/2 h-4 w-4 -translate-y-1/2 opacity-50 pointer-events-none" />
</div>
