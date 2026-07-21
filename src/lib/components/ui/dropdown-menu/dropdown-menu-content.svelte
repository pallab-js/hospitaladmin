<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";
  import { cn } from "$lib/utils/index.js";

  let {
    class: className = "",
    children,
    ...restProps
  }: HTMLAttributes<HTMLDivElement> & { children: Snippet } = $props();
</script>

<div
  class={cn(
    "absolute right-0 z-50 mt-2 min-w-[8rem] overflow-hidden rounded-md border bg-popover p-1 text-popover-foreground shadow-md",
    className
  )}
  {...restProps}
>
  {@render children()}
</div>
