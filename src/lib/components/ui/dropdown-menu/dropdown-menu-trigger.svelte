<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";
  import { cn } from "$lib/utils/index.js";

  let {
    class: className = "",
    children,
    ...restProps
  }: HTMLButtonAttributes & { children: Snippet } = $props();
</script>

<button
  class={cn("inline-flex items-center justify-center", className)}
  {...restProps}
>
  {@render children()}
</button>
