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
  }: HTMLAttributes<HTMLButtonElement> & {
    value: string;
    children: Snippet;
  } = $props();

  const ctx = getTabsContext();
  const isActive = $derived(ctx.value === value);
</script>

<button
  type="button"
  role="tab"
  aria-selected={isActive}
  data-state={isActive ? "active" : "inactive"}
  class={cn(
    "inline-flex items-center justify-center whitespace-nowrap rounded-sm px-3 py-1.5 text-sm font-medium ring-offset-background transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
    "data-[state=active]:bg-background data-[state=active]:text-foreground data-[state=active]:shadow-sm",
    className
  )}
  onclick={() => (ctx.value = value)}
  {...restProps}
>
  {@render children()}
</button>
