<script lang="ts">
  import { onMount } from "svelte";
  import { getWardOccupancy } from "$lib/lib/api.js";
  import type { WardOccupancy } from "$lib/lib/types.js";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import Card from "$lib/components/ui/card/index.svelte";
  import CardHeader from "$lib/components/ui/card/card-header.svelte";
  import CardTitle from "$lib/components/ui/card/card-title.svelte";
  import CardContent from "$lib/components/ui/card/card-content.svelte";
  import Badge from "$lib/components/ui/badge/index.svelte";
  import { Building2, AlertTriangle, CheckCircle } from "@lucide/svelte";

  let wards = $state<WardOccupancy[]>([]);
  let loading = $state(true);

  onMount(async () => {
    try {
      wards = await getWardOccupancy();
    } finally {
      loading = false;
    }
  });

  function getOccupancyColor(occupancy: number) {
    if (occupancy >= 90) return "text-destructive";
    if (occupancy >= 70) return "text-warning";
    return "text-success";
  }

  function getBedStatusColor(status: string) {
    const colors: Record<string, string> = {
      available: "bg-success",
      occupied: "bg-destructive",
      reserved: "bg-warning",
      cleaning: "bg-info",
      maintenance: "bg-muted",
    };
    return colors[status] || "bg-muted";
  }
</script>

<PageLayout title="Wards & Beds" description="Monitor ward occupancy and bed management">
  {#if loading}
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      {#each Array(5) as _}
        <div class="h-64 animate-pulse rounded-xl bg-muted"></div>
      {/each}
    </div>
  {:else}
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      {#each wards as ward}
        {@const occupancy = ward.occupancy_rate}
        <Card>
          <CardHeader class="pb-3">
            <div class="flex items-center justify-between">
              <CardTitle class="text-lg">{ward.ward_name}</CardTitle>
              <Badge variant="outline">{ward.total_beds} beds</Badge>
            </div>
          </CardHeader>
          <CardContent>
            <div class="space-y-4">
              <div>
                <div class="flex items-center justify-between text-sm mb-2">
                  <span class="text-muted-foreground">Occupancy</span>
                  <span class="font-medium {getOccupancyColor(occupancy)}">{occupancy}%</span>
                </div>
                <div class="h-3 w-full overflow-hidden rounded-full bg-muted">
                  <div
                    class="h-full rounded-full transition-all {occupancy >= 90
                      ? 'bg-destructive'
                      : occupancy >= 70
                        ? 'bg-warning'
                        : 'bg-success'}"
                    style="width: {occupancy}%"
                  ></div>
                </div>
              </div>

              <div class="grid grid-cols-3 gap-4 text-center">
                <div>
                  <p class="text-2xl font-bold">{ward.total_beds}</p>
                  <p class="text-xs text-muted-foreground">Total</p>
                </div>
                <div>
                  <p class="text-2xl font-bold text-destructive">{ward.occupied}</p>
                  <p class="text-xs text-muted-foreground">Occupied</p>
                </div>
                <div>
                  <p class="text-2xl font-bold text-success">{ward.available}</p>
                  <p class="text-xs text-muted-foreground">Available</p>
                </div>
              </div>

              <div class="flex flex-wrap gap-1.5">
                {#each Array(ward.total_beds) as _, i}
                  <div
                    class="h-6 w-6 rounded {getBedStatusColor(
                      i < ward.occupied ? 'occupied' : 'available'
                    )}"
                    title="Bed {i + 1}"
                  ></div>
                {/each}
              </div>

              <div class="flex items-center gap-4 text-xs text-muted-foreground">
                <div class="flex items-center gap-1">
                  <div class="h-3 w-3 rounded bg-success"></div>
                  <span>Available</span>
                </div>
                <div class="flex items-center gap-1">
                  <div class="h-3 w-3 rounded bg-destructive"></div>
                  <span>Occupied</span>
                </div>
                <div class="flex items-center gap-1">
                  <div class="h-3 w-3 rounded bg-warning"></div>
                  <span>Reserved</span>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
      {/each}
    </div>

    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2">
          <AlertTriangle class="h-5 w-5 text-warning" />
          Alerts
        </CardTitle>
      </CardHeader>
      <CardContent>
        <div class="space-y-2">
          {#each wards.filter(w => w.occupancy_rate >= 80) as w}
            <div class="flex items-center gap-3 rounded-lg border border-warning/20 bg-warning/5 p-3">
              <AlertTriangle class="h-4 w-4 text-warning" />
              <span class="text-sm">{w.ward_name} is at {w.occupancy_rate}% capacity - consider transfers if needed</span>
            </div>
          {/each}
          {#each wards.filter(w => w.available > 0) as w}
            <div class="flex items-center gap-3 rounded-lg border border-success/20 bg-success/5 p-3">
              <CheckCircle class="h-4 w-4 text-success" />
              <span class="text-sm">{w.ward_name} has {w.available} available beds</span>
            </div>
          {/each}
        </div>
      </CardContent>
    </Card>
  {/if}
</PageLayout>
