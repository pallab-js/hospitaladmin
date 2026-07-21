<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";
  import { cn } from "$lib/utils/index.js";

  let {
    class: className = "",
    open = $bindable(false),
    children,
    ...restProps
  }: HTMLAttributes<HTMLDivElement> & {
    open?: boolean;
    children: Snippet;
  } = $props();
</script>

<div class={cn("relative inline-block text-left", className)} {...restProps}>
  {@render children()}
</div>
