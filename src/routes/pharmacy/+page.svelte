<script lang="ts">
  import { onMount } from "svelte";
  import { getMedications, getMedicationStock } from "$lib/lib/api.js";
  import type { Medication } from "$lib/lib/types.js";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import Card from "$lib/components/ui/card/index.svelte";
  import CardHeader from "$lib/components/ui/card/card-header.svelte";
  import CardTitle from "$lib/components/ui/card/card-title.svelte";
  import CardContent from "$lib/components/ui/card/card-content.svelte";
  import Badge from "$lib/components/ui/badge/index.svelte";
  import Input from "$lib/components/ui/input/index.svelte";
  import { Pill, Package, AlertTriangle, TrendingDown, Search } from "@lucide/svelte";

  let medications = $state<Medication[]>([]);
  let stock = $state<{ medication_id: string; quantity: number; low_threshold: number; expiry_date: string }[]>([]);
  let loading = $state(true);
  let search = $state("");

  onMount(async () => {
    try {
      const [m, s] = await Promise.all([getMedications(), getMedicationStock()]);
      medications = m;
      stock = s;
    } finally {
      loading = false;
    }
  });

  function getStockQty(medId: string) {
    return stock.find((s) => s.medication_id === medId);
  }

  const totalMedications = $derived(medications.length);
  const lowStockCount = $derived(
    stock.filter((s) => s.quantity < s.low_threshold).length
  );
  const expiringCount = $derived(
    stock.filter((s) => {
      const exp = new Date(s.expiry_date);
      const now = new Date();
      const threeMonths = new Date(now.getTime() + 90 * 24 * 60 * 60 * 1000);
      return exp <= threeMonths;
    }).length
  );

  const filteredMedications = $derived(
    search
      ? medications.filter(
          (m) =>
            m.name.toLowerCase().includes(search.toLowerCase()) ||
            m.category.toLowerCase().includes(search.toLowerCase()) ||
            (m.generic_name && m.generic_name.toLowerCase().includes(search.toLowerCase()))
        )
      : medications
  );
</script>

<PageLayout title="Pharmacy" description="Manage medications and inventory">
  <div class="grid gap-4 md:grid-cols-3">
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">Total Medicines</p>
            <p class="text-3xl font-bold">{totalMedications}</p>
          </div>
          <Pill class="h-8 w-8 text-primary" />
        </div>
      </CardContent>
    </Card>
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">Low Stock</p>
            <p class="text-3xl font-bold text-warning">{lowStockCount}</p>
          </div>
          <TrendingDown class="h-8 w-8 text-warning" />
        </div>
      </CardContent>
    </Card>
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">Expiring Soon</p>
            <p class="text-3xl font-bold text-destructive">{expiringCount}</p>
          </div>
          <AlertTriangle class="h-8 w-8 text-destructive" />
        </div>
      </CardContent>
    </Card>
  </div>

  <Card>
    <CardHeader>
      <div class="flex items-center justify-between">
        <CardTitle>Medication Inventory</CardTitle>
        <div class="relative w-64">
          <Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
          <Input placeholder="Search medications..." bind:value={search} class="pl-9" />
        </div>
      </div>
    </CardHeader>
    <CardContent>
      {#if loading}
        <div class="space-y-4">
          {#each Array(5) as _}
            <div class="h-16 animate-pulse rounded-lg bg-muted"></div>
          {/each}
        </div>
      {:else}
        <div class="space-y-4">
          {#each filteredMedications as med}
            {@const stockInfo = getStockQty(med.id)}
            {@const isLow = stockInfo && stockInfo.quantity < stockInfo.low_threshold}
            <div class="flex items-center justify-between rounded-lg border p-4 {isLow ? 'border-warning/50 bg-warning/5' : ''}">
              <div class="flex items-center gap-4">
                <Package class="h-5 w-5 {isLow ? 'text-warning' : 'text-primary'}" />
                <div>
                  <p class="font-medium">{med.name}</p>
                  <p class="text-sm text-muted-foreground">{med.category} · {med.dosage_form} · {med.strength || ""}</p>
                </div>
              </div>
              <div class="flex items-center gap-4">
                <div class="text-right">
                  <p class="font-medium">{stockInfo?.quantity ?? 0} units</p>
                  <p class="text-xs text-muted-foreground">Exp: {stockInfo?.expiry_date ?? "N/A"}</p>
                </div>
                <Badge class={isLow ? "bg-warning/10 text-warning" : "bg-success/10 text-success"}>
                  {isLow ? "Low Stock" : "In Stock"}
                </Badge>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </CardContent>
  </Card>
</PageLayout>
