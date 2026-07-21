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

<div class={cn("relative inline-block", className)} {...restProps}>
  {@render children()}
</div>
