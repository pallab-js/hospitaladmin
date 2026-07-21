<script lang="ts">
  import { onMount } from "svelte";
  import { getInvoices, getPatients } from "$lib/lib/api.js";
  import type { Invoice, Patient } from "$lib/lib/types.js";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import Card from "$lib/components/ui/card/index.svelte";
  import CardHeader from "$lib/components/ui/card/card-header.svelte";
  import CardTitle from "$lib/components/ui/card/card-title.svelte";
  import CardContent from "$lib/components/ui/card/card-content.svelte";
  import Badge from "$lib/components/ui/badge/index.svelte";
  import { DollarSign, CreditCard, FileText, AlertCircle } from "@lucide/svelte";
  import { formatCurrency, formatDate } from "$lib/utils/index.js";

  let invoices = $state<Invoice[]>([]);
  let patients = $state<Patient[]>([]);
  let loading = $state(true);

  onMount(async () => {
    try {
      const [i, p] = await Promise.all([getInvoices(), getPatients()]);
      invoices = i;
      patients = p;
    } finally {
      loading = false;
    }
  });

  function getPatientName(id: string) {
    const p = patients.find((p) => p.id === id);
    return p ? `${p.first_name} ${p.last_name}` : "Unknown";
  }

  function getStatusColor(status: string) {
    const colors: Record<string, string> = {
      paid: "bg-success/10 text-success",
      pending: "bg-warning/10 text-warning",
      partial: "bg-info/10 text-info",
      overdue: "bg-destructive/10 text-destructive",
      cancelled: "bg-muted text-muted-foreground",
    };
    return colors[status] || "bg-muted text-muted-foreground";
  }

  const totalRevenue = $derived(
    invoices.filter((i) => i.status === "paid").reduce((sum, i) => sum + i.total, 0)
  );
  const pendingAmount = $derived(
    invoices
      .filter((i) => i.status === "pending" || i.status === "partial")
      .reduce((sum, i) => sum + i.total, 0)
  );
  const overdueAmount = $derived(
    invoices.filter((i) => i.status === "overdue").reduce((sum, i) => sum + i.total, 0)
  );
  const todayCount = $derived(
    invoices.filter((i) => i.invoice_date === "2024-01-15").length
  );
</script>

<PageLayout title="Billing" description="Manage invoices and payments">
  <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">Revenue (Paid)</p>
            <p class="text-2xl font-bold text-success">{formatCurrency(totalRevenue)}</p>
          </div>
          <DollarSign class="h-8 w-8 text-success" />
        </div>
      </CardContent>
    </Card>
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">Pending Payments</p>
            <p class="text-2xl font-bold text-warning">{formatCurrency(pendingAmount)}</p>
          </div>
          <CreditCard class="h-8 w-8 text-warning" />
        </div>
      </CardContent>
    </Card>
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">Invoices Today</p>
            <p class="text-2xl font-bold">{todayCount}</p>
          </div>
          <FileText class="h-8 w-8 text-primary" />
        </div>
      </CardContent>
    </Card>
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">Overdue</p>
            <p class="text-2xl font-bold text-destructive">{formatCurrency(overdueAmount)}</p>
          </div>
          <AlertCircle class="h-8 w-8 text-destructive" />
        </div>
      </CardContent>
    </Card>
  </div>

  <Card>
    <CardHeader>
      <CardTitle>Recent Invoices</CardTitle>
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
          {#each invoices as invoice}
            <div class="flex items-center justify-between rounded-lg border p-4">
              <div class="flex items-center gap-4">
                <FileText class="h-5 w-5 text-primary" />
                <div>
                  <p class="font-medium">{invoice.invoice_number}</p>
                  <p class="text-sm text-muted-foreground">{getPatientName(invoice.patient_id)}</p>
                </div>
              </div>
              <div class="flex items-center gap-4">
                <div class="text-right">
                  <p class="font-medium">{formatCurrency(invoice.total)}</p>
                  <p class="text-xs text-muted-foreground">{formatDate(invoice.invoice_date)}</p>
                </div>
                <Badge class={getStatusColor(invoice.status)}>
                  {invoice.status}
                </Badge>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </CardContent>
  </Card>
</PageLayout>
