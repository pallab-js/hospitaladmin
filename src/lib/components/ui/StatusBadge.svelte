<script lang="ts">
  import { cn } from "$lib/utils/index.js";
  import Badge from "$lib/components/ui/badge/index.svelte";

  let { status, class: className }: { status: string; class?: string } = $props();

  const statusStyles: Record<string, string> = {
    scheduled: "bg-blue-100 text-blue-800",
    confirmed: "bg-green-100 text-green-800",
    in_progress: "bg-yellow-100 text-yellow-800",
    completed: "bg-green-100 text-green-800",
    cancelled: "bg-red-100 text-red-800",
    no_show: "bg-gray-100 text-gray-800",
    ordered: "bg-blue-100 text-blue-800",
    paid: "bg-green-100 text-green-800",
    pending: "bg-yellow-100 text-yellow-800",
    partial: "bg-orange-100 text-orange-800",
    overdue: "bg-red-100 text-red-800",
    available: "bg-green-100 text-green-800",
    occupied: "bg-red-100 text-red-800",
    reserved: "bg-yellow-100 text-yellow-800",
    cleaning: "bg-blue-100 text-blue-800",
    maintenance: "bg-gray-100 text-gray-800",
    active: "bg-green-100 text-green-800",
    discharged: "bg-gray-100 text-gray-800",
    transferred: "bg-purple-100 text-purple-800",
    routine: "bg-blue-100 text-blue-800",
    urgent: "bg-orange-100 text-orange-800",
    stat: "bg-red-100 text-red-800",
  };

  let style = $derived(statusStyles[status] || "bg-gray-100 text-gray-800");
  let label = $derived(status.replace(/_/g, " "));
</script>

<Badge class={cn(style, className)}>
  {label}
</Badge>
