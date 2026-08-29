<script lang="ts">
  import { cn } from "$lib/utils/index.js";
  import Badge from "$lib/components/ui/badge/index.svelte";

  let { status, class: className }: { status: string; class?: string } = $props();

  const statusStyles: Record<string, string> = {
    scheduled: "bg-blue-500/15 text-blue-700 dark:text-blue-300",
    confirmed: "bg-green-500/15 text-green-700 dark:text-green-300",
    in_progress: "bg-yellow-500/15 text-yellow-700 dark:text-yellow-300",
    completed: "bg-green-500/15 text-green-700 dark:text-green-300",
    cancelled: "bg-red-500/15 text-red-700 dark:text-red-300",
    no_show: "bg-gray-500/15 text-gray-700 dark:text-gray-300",
    ordered: "bg-blue-500/15 text-blue-700 dark:text-blue-300",
    paid: "bg-green-500/15 text-green-700 dark:text-green-300",
    pending: "bg-yellow-500/15 text-yellow-700 dark:text-yellow-300",
    partial: "bg-orange-500/15 text-orange-700 dark:text-orange-300",
    overdue: "bg-red-500/15 text-red-700 dark:text-red-300",
    available: "bg-green-500/15 text-green-700 dark:text-green-300",
    occupied: "bg-red-500/15 text-red-700 dark:text-red-300",
    reserved: "bg-yellow-500/15 text-yellow-700 dark:text-yellow-300",
    cleaning: "bg-blue-500/15 text-blue-700 dark:text-blue-300",
    maintenance: "bg-gray-500/15 text-gray-700 dark:text-gray-300",
    active: "bg-green-500/15 text-green-700 dark:text-green-300",
    discharged: "bg-gray-500/15 text-gray-700 dark:text-gray-300",
    transferred: "bg-purple-500/15 text-purple-700 dark:text-purple-300",
    routine: "bg-blue-500/15 text-blue-700 dark:text-blue-300",
    urgent: "bg-orange-500/15 text-orange-700 dark:text-orange-300",
    stat: "bg-red-500/15 text-red-700 dark:text-red-300",
  };

  let style = $derived(statusStyles[status] || "bg-gray-500/15 text-gray-700 dark:text-gray-300");
  let label = $derived(status.replace(/_/g, " "));
</script>

<Badge class={cn(style, className)}>
  {label}
</Badge>
