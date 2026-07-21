<script lang="ts">
  import type { Snippet } from "svelte";
  import type { Component } from "svelte";
  import { cn } from "$lib/utils/index.js";

  let {
    class: className = "",
    title,
    value,
    change,
    changeType = "neutral",
    icon,
    children,
    ...restProps
  }: {
    class?: string;
    title: string;
    value: string | number;
    change?: string;
    changeType?: "positive" | "negative" | "neutral";
    icon?: Component;
    children?: Snippet;
  } = $props();

  const changeColors = {
    positive: "text-success",
    negative: "text-destructive",
    neutral: "text-muted-foreground",
  };
</script>

<div
  class={cn(
    "rounded-xl border bg-card p-6 shadow-sm hover:shadow-md transition-shadow",
    className
  )}
  {...restProps}
>
  <div class="flex items-center justify-between">
    <p class="text-sm font-medium text-muted-foreground">{title}</p>
    {#if icon}
      <div class="rounded-lg bg-primary/10 p-2">
        <icon class="h-5 w-5 text-primary"></icon>
      </div>
    {/if}
  </div>
  <div class="mt-3">
    <p class="text-3xl font-bold tracking-tight">{value}</p>
    {#if change}
      <p class={cn("text-xs mt-1", changeColors[changeType])}>
        {change}
      </p>
    {/if}
  </div>
  {#if children}
    <div class="mt-4">
      {@render children()}
    </div>
  {/if}
</div>
