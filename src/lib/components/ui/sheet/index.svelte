<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";
  import { cn } from "$lib/utils/index.js";

  let {
    class: className = "",
    open = $bindable(false),
    side = "left",
    children,
    ...restProps
  }: HTMLAttributes<HTMLDivElement> & {
    open?: boolean;
    side?: "left" | "right";
    children: Snippet;
  } = $props();
</script>

{#if open}
  <div class="fixed inset-0 z-50">
    <button
      class="fixed inset-0 bg-black/50"
      onclick={() => (open = false)}
      onkeydown={(e) => e.key === "Escape" && (open = false)}
      aria-label="Close sheet"
    ></button>
    <div
      class={cn(
        "fixed inset-y-0 z-50 h-full w-72 bg-background p-6 shadow-lg transition-transform",
        side === "left" ? "left-0" : "right-0",
        className
      )}
      {...restProps}
    >
      {@render children()}
    </div>
  </div>
{/if}
