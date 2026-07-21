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

{#if open}
  <div
    class={cn("fixed inset-0 z-50 flex items-center justify-center", className)}
    {...restProps}
  >
    <button
      class="fixed inset-0 bg-black/80"
      onclick={() => (open = false)}
      onkeydown={(e) => e.key === 'Escape' && (open = false)}
      aria-label="Close dialog"
    ></button>
    {@render children()}
  </div>
{/if}
