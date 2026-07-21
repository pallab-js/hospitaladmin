<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";
  import { cn } from "$lib/utils/index.js";
  import { Search } from "@lucide/svelte";

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
  <div class="fixed inset-0 z-50 flex items-start justify-center pt-[20vh]">
    <button
      class="fixed inset-0 bg-black/50"
      onclick={() => (open = false)}
      onkeydown={(e) => e.key === 'Escape' && (open = false)}
      aria-label="Close search"
    ></button>
    <div
      class={cn(
        "relative z-50 w-full max-w-lg overflow-hidden rounded-lg border bg-background shadow-lg",
        className
      )}
      {...restProps}
    >
      <div class="flex items-center border-b px-4">
        <Search class="mr-2 h-4 w-4 shrink-0 opacity-50" />
        <input
          class="flex h-12 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground"
          placeholder="Search patients, appointments..."
        />
      </div>
      <div class="max-h-[300px] overflow-y-auto p-2">
        {@render children()}
      </div>
    </div>
  </div>
{/if}
