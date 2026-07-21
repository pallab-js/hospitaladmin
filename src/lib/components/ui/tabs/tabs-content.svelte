<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";
  import { cn } from "$lib/utils/index.js";
  import { getTabsContext } from "./tabs-context.js";

  let {
    class: className = "",
    value,
    children,
    ...restProps
  }: HTMLAttributes<HTMLDivElement> & {
    value: string;
    children: Snippet;
  } = $props();

  const ctx = getTabsContext();
  const isActive = $derived(ctx.value === value);
</script>

{#if isActive}
  <div
    role="tabpanel"
    data-state={isActive ? "active" : "inactive"}
    class={cn(
      "mt-2 ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2",
      className
    )}
    {...restProps}
  >
    {@render children()}
  </div>
{/if}
