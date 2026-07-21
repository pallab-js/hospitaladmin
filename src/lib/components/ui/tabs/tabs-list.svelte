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
    "inline-flex h-10 items-center justify-center rounded-md bg-muted p-1 text-muted-foreground",
    className
  )}
  role="tablist"
  {...restProps}
>
  {@render children()}
</div>
