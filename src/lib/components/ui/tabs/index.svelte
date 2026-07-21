<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLAttributes } from "svelte/elements";
  import { cn } from "$lib/utils/index.js";
  import { setTabsContext } from "./tabs-context.js";

  let {
    class: className = "",
    value = $bindable(""),
    children,
    ...restProps
  }: HTMLAttributes<HTMLDivElement> & {
    value?: string;
    children: Snippet;
  } = $props();

  setTabsContext({
    get value() {
      return value;
    },
    set value(v) {
      value = v;
    },
  });
</script>

<div class={cn("w-full", className)} data-orientation="horizontal" {...restProps}>
  {@render children()}
</div>
