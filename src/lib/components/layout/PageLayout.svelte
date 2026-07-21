<script lang="ts">
  import type { Snippet } from "svelte";
  import { cn } from "$lib/utils/index.js";

  let {
    class: className = "",
    title,
    description,
    actions,
    children,
    ...restProps
  }: {
    class?: string;
    title?: string;
    description?: string;
    actions?: Snippet;
    children: Snippet;
  } = $props();
</script>

<div class={cn("space-y-6", className)} {...restProps}>
  {#if title || actions}
    <div class="flex flex-col gap-4 sm:flex-row sm:items-center sm:justify-between">
      <div>
        {#if title}
          <h1 class="text-3xl font-bold tracking-tight">{title}</h1>
        {/if}
        {#if description}
          <p class="text-muted-foreground">{description}</p>
        {/if}
      </div>
      {#if actions}
        <div class="flex items-center gap-2">
          {@render actions()}
        </div>
      {/if}
    </div>
  {/if}
  {@render children()}
</div>
