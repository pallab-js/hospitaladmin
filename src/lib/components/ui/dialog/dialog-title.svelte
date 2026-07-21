<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";
  import { cn } from "$lib/utils/index.js";

  let {
    class: className = "",
    children,
    ...restProps
  }: HTMLAttributes<HTMLHeadingElement> & { children: Snippet } = $props();
</script>

<h2
  class={cn("text-lg font-semibold leading-none tracking-tight", className)}
  {...restProps}
>
  {@render children()}
</h2>
