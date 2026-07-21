<script lang="ts">
  import { onMount } from "svelte";
  import { getLabOrders, getLabTests, getPatients } from "$lib/lib/api.js";
  import type { LabOrder, LabTest, Patient } from "$lib/lib/types.js";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import Card from "$lib/components/ui/card/index.svelte";
  import CardHeader from "$lib/components/ui/card/card-header.svelte";
  import CardTitle from "$lib/components/ui/card/card-title.svelte";
  import CardContent from "$lib/components/ui/card/card-content.svelte";
  import Badge from "$lib/components/ui/badge/index.svelte";
  import { TestTube, Clock, CheckCircle, AlertCircle } from "@lucide/svelte";

  let orders = $state<LabOrder[]>([]);
  let tests = $state<LabTest[]>([]);
  let patients = $state<Patient[]>([]);
  let loading = $state(true);

  onMount(async () => {
    try {
      const [o, t, p] = await Promise.all([getLabOrders(), getLabTests(), getPatients()]);
      orders = o;
      tests = t;
      patients = p;
    } finally {
      loading = false;
    }
  });

  function getPatientName(id: string) {
    const p = patients.find((p) => p.id === id);
    return p ? `${p.first_name} ${p.last_name}` : "Unknown";
  }

  function getTestName(orderId: string) {
    // demo: map order to test by index
    const idx = parseInt(orderId.replace("lo", "")) - 1;
    return tests[idx]?.name || "Lab Test";
  }

  function getStatusColor(status: string) {
    const colors: Record<string, string> = {
      ordered: "bg-warning/10 text-warning",
      in_progress: "bg-info/10 text-info",
      completed: "bg-success/10 text-success",
      cancelled: "bg-destructive/10 text-destructive",
    };
    return colors[status] || "bg-muted text-muted-foreground";
  }

  function getPriorityColor(priority: string) {
    const colors: Record<string, string> = {
      stat: "bg-destructive/10 text-destructive",
      urgent: "bg-warning/10 text-warning",
      routine: "bg-muted text-muted-foreground",
    };
    return colors[priority] || "bg-muted text-muted-foreground";
  }

  const pendingCount = $derived(orders.filter((o) => o.status === "ordered").length);
  const inProgressCount = $derived(orders.filter((o) => o.status === "in_progress").length);
  const completedCount = $derived(orders.filter((o) => o.status === "completed").length);
</script>

<PageLayout title="Lab & Diagnostics" description="Manage lab orders and test results">
  <div class="grid gap-4 md:grid-cols-3">
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">Pending Orders</p>
            <p class="text-3xl font-bold">{pendingCount}</p>
          </div>
          <Clock class="h-8 w-8 text-warning" />
        </div>
      </CardContent>
    </Card>
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">In Progress</p>
            <p class="text-3xl font-bold text-info">{inProgressCount}</p>
          </div>
          <AlertCircle class="h-8 w-8 text-info" />
        </div>
      </CardContent>
    </Card>
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">Completed Today</p>
            <p class="text-3xl font-bold text-success">{completedCount}</p>
          </div>
          <CheckCircle class="h-8 w-8 text-success" />
        </div>
      </CardContent>
    </Card>
  </div>

  <div class="grid gap-4 lg:grid-cols-2">
    <Card>
      <CardHeader>
        <CardTitle>Recent Lab Orders</CardTitle>
      </CardHeader>
      <CardContent>
        {#if loading}
          <div class="space-y-4">
            {#each Array(4) as _}
              <div class="h-16 animate-pulse rounded-lg bg-muted"></div>
            {/each}
          </div>
        {:else}
          <div class="space-y-4">
            {#each orders as order}
              <div class="flex items-center justify-between rounded-lg border p-4">
                <div class="flex items-center gap-4">
                  <TestTube class="h-5 w-5 text-primary" />
                  <div>
                    <p class="font-medium">{getTestName(order.id)}</p>
                    <p class="text-sm text-muted-foreground">{getPatientName(order.patient_id)}</p>
                  </div>
                </div>
                <div class="flex items-center gap-2">
                  <Badge class={getPriorityColor(order.priority)}>
                    {order.priority}
                  </Badge>
                  <Badge class={getStatusColor(order.status)}>
                    {order.status.replace("_", " ")}
                  </Badge>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </CardContent>
    </Card>

    <Card>
      <CardHeader>
        <CardTitle>Available Tests</CardTitle>
      </CardHeader>
      <CardContent>
        {#if loading}
          <div class="space-y-4">
            {#each Array(4) as _}
              <div class="h-16 animate-pulse rounded-lg bg-muted"></div>
            {/each}
          </div>
        {:else}
          <div class="space-y-4">
            {#each tests as test}
              <div class="flex items-center justify-between rounded-lg border p-4">
                <div>
                  <p class="font-medium">{test.name}</p>
                  <p class="text-sm text-muted-foreground">{test.category} · {test.code}</p>
                </div>
                <div class="text-right">
                  <p class="font-medium">₹{test.price}</p>
                  <p class="text-xs text-muted-foreground">{test.normal_range || ""}</p>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      </CardContent>
    </Card>
  </div>
</PageLayout>
