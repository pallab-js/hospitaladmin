<script lang="ts">
  import { notifications } from "$lib/stores/notifications.js";
  import { X, CheckCircle, AlertCircle, AlertTriangle, Info } from "@lucide/svelte";
  import { cn } from "$lib/utils/index.js";

  const icons = {
    success: CheckCircle,
    error: AlertCircle,
    warning: AlertTriangle,
    info: Info,
  };

  const colors = {
    success: "bg-success/10 text-success border-success/20",
    error: "bg-destructive/10 text-destructive border-destructive/20",
    warning: "bg-warning/10 text-warning border-warning/20",
    info: "bg-info/10 text-info border-info/20",
  };
</script>

<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
  {#each $notifications as notification (notification.id)}
    {@const Icon = icons[notification.type]}
    <div
      class={cn(
        "flex items-start gap-3 rounded-lg border p-4 shadow-lg animate-in slide-in-from-right-full min-w-[300px] max-w-[400px]",
        colors[notification.type]
      )}
    >
      <Icon class="h-5 w-5 shrink-0 mt-0.5" />
      <div class="flex-1">
        <p class="text-sm font-medium">{notification.title}</p>
        {#if notification.message}
          <p class="text-sm opacity-80 mt-1">{notification.message}</p>
        {/if}
      </div>
      <button
        class="shrink-0 opacity-60 hover:opacity-100"
        onclick={() => notifications.remove(notification.id)}
      >
        <X class="h-4 w-4" />
      </button>
    </div>
  {/each}
</div>
