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
    "flex h-full w-full items-center justify-center rounded-full bg-muted",
    className
  )}
  {...restProps}
>
  {@render children()}
</div>
