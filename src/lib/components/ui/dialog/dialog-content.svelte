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
    "relative z-50 grid w-full max-w-lg gap-4 border bg-background p-6 shadow-lg sm:rounded-lg",
    className
  )}
  {...restProps}
>
  {@render children()}
</div>
