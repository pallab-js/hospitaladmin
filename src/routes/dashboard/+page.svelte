<script lang="ts">
  import { onMount } from "svelte";
  import { userRole, userName } from "$lib/stores/auth.js";
  import {
    getDashboardStats,
    getRevenueChart,
    getDepartmentLoad,
    getAppointments,
    getWardOccupancy,
    getLabOrders,
    getInvoices,
  } from "$lib/lib/api.js";
  import type {
    DashboardStats,
    RevenueData,
    DepartmentLoad,
    AppointmentWithDetails,
    WardOccupancy,
    LabOrder,
    Invoice,
  } from "$lib/lib/types.js";
  import Card from "$lib/components/ui/card/index.svelte";
  import CardHeader from "$lib/components/ui/card/card-header.svelte";
  import CardTitle from "$lib/components/ui/card/card-title.svelte";
  import CardContent from "$lib/components/ui/card/card-content.svelte";
  import StatusBadge from "$lib/components/ui/StatusBadge.svelte";
  import Badge from "$lib/components/ui/badge/index.svelte";
  import KPICard from "$lib/components/domain/KPICard.svelte";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import { formatCurrency, formatDate } from "$lib/utils/index.js";
  import {
    Users,
    Calendar,
    Bed,
    DollarSign,
    TestTube,
    Activity,
    UserCheck,
    UserPlus,
    Clock,
    AlertTriangle,
    FileText,
    Stethoscope,
    ClipboardList,
    TrendingUp,
    Pill,
    CheckCircle,
    XCircle,
    Timer,
  } from "@lucide/svelte";

  let stats = $state<DashboardStats | null>(null);
  let revenueData = $state<RevenueData[]>([]);
  let departmentData = $state<DepartmentLoad[]>([]);
  let appointments = $state<AppointmentWithDetails[]>([]);
  let wards = $state<WardOccupancy[]>([]);
  let labOrders = $state<LabOrder[]>([]);
  let invoices = $state<Invoice[]>([]);
  let loading = $state(true);

  onMount(async () => {
    try {
      const results = await Promise.allSettled([
        getDashboardStats(),
        getRevenueChart(7),
        getDepartmentLoad(),
        getAppointments(),
        getWardOccupancy(),
        getLabOrders(),
        getInvoices(),
      ]);
      if (results[0].status === "fulfilled") stats = results[0].value;
      if (results[1].status === "fulfilled") revenueData = results[1].value;
      if (results[2].status === "fulfilled") departmentData = results[2].value;
      if (results[3].status === "fulfilled") appointments = results[3].value;
      if (results[4].status === "fulfilled") wards = results[4].value;
      if (results[5].status === "fulfilled") labOrders = results[5].value;
      if (results[6].status === "fulfilled") invoices = results[6].value;
    } catch (e) {
      console.error("Failed to load dashboard:", e);
    } finally {
      loading = false;
    }
  });

  const maxRevenue = $derived(
    revenueData.length > 0 ? Math.max(...revenueData.map((r) => r.amount)) : 1
  );
  const maxDeptLoad = $derived(
    departmentData.length > 0
      ? Math.max(...departmentData.map((d) => d.appointment_count), 1)
      : 1
  );

  </script>

<PageLayout
  title="Welcome back, {$userName}"
  description={$userRole === "doctor"
    ? "Here's your clinical overview for today."
    : $userRole === "management"
      ? "Here's the hospital overview for today."
      : "Here's your operational overview for today."}
>
  {#if loading}
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
      {#each Array(8) as _}
        <div class="h-32 animate-pulse rounded-xl bg-muted"></div>
      {/each}
    </div>
    <div class="grid gap-4 lg:grid-cols-2">
      {#each Array(2) as _}
        <div class="h-64 animate-pulse rounded-xl bg-muted"></div>
      {/each}
    </div>
  {:else}
    <!-- ═══════════════ MANAGEMENT DASHBOARD ═══════════════ -->
    {#if $userRole === "management"}
      <!-- KPI Row -->
      <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <KPICard title="Patients Today" value={stats?.total_patients_today ?? 0} change="+12% from yesterday" changeType="positive" icon={Users} />
        <KPICard title="Appointments" value={stats?.total_appointments_today ?? 0} change="Scheduled today" changeType="neutral" icon={Calendar} />
        <KPICard title="Bed Occupancy" value="{(stats?.bed_occupancy_rate ?? 0).toFixed(1)}%" change={stats && stats.bed_occupancy_rate > 80 ? "High" : "Normal"} changeType={stats && stats.bed_occupancy_rate > 80 ? "negative" : "positive"} icon={Bed} />
        <KPICard title="Revenue Today" value={formatCurrency(stats?.revenue_today ?? 0)} change="Collected" changeType="positive" icon={DollarSign} />
      </div>
      <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <KPICard title="Pending Labs" value={stats?.pending_lab_orders ?? 0} change="Awaiting results" changeType={stats && stats.pending_lab_orders > 5 ? "negative" : "neutral"} icon={TestTube} />
        <KPICard title="Active Admissions" value={stats?.active_admissions ?? 0} change="Currently admitted" changeType="neutral" icon={Activity} />
        <KPICard title="Staff On Duty" value={stats?.staff_on_duty ?? 0} change="Working now" changeType="neutral" icon={UserCheck} />
        <KPICard title="New Patients (Month)" value={stats?.patients_registered_this_month ?? 0} change="This month" changeType="positive" icon={UserPlus} />
      </div>

      <!-- Revenue + Department Load -->
      <div class="grid gap-4 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-2"><TrendingUp class="h-5 w-5" /> Revenue (7 Days)</CardTitle>
          </CardHeader>
          <CardContent>
            {#if revenueData.length > 0}
              <div class="space-y-3">
                {#each revenueData as item}
                  <div class="flex items-center gap-3">
                    <span class="w-20 text-xs text-muted-foreground">{item.date.slice(5)}</span>
                    <div class="flex-1">
                      <div class="h-6 w-full overflow-hidden rounded bg-muted">
                        <div class="h-full rounded bg-primary transition-all duration-500" style="width: {(item.amount / maxRevenue) * 100}%"></div>
                      </div>
                    </div>
                    <span class="w-24 text-right text-sm font-medium">{formatCurrency(item.amount)}</span>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-sm text-muted-foreground text-center py-8">No revenue data</p>
            {/if}
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-2"><ClipboardList class="h-5 w-5" /> Department Load</CardTitle>
          </CardHeader>
          <CardContent>
            {#if departmentData.length > 0}
              <div class="space-y-4">
                {#each departmentData as dept}
                  <div class="space-y-2">
                    <div class="flex items-center justify-between">
                      <span class="text-sm font-medium">{dept.department_name}</span>
                      <span class="text-sm text-muted-foreground">{dept.appointment_count} appts</span>
                    </div>
                    <div class="h-3 w-full overflow-hidden rounded-full bg-muted">
                      <div class="h-full rounded-full bg-primary transition-all duration-500" style="width: {(dept.appointment_count / maxDeptLoad) * 100}%"></div>
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-sm text-muted-foreground text-center py-8">No department data</p>
            {/if}
          </CardContent>
        </Card>
      </div>

      <!-- Ward Status + Financial Summary -->
      <div class="grid gap-4 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-2"><Bed class="h-5 w-5" /> Ward Occupancy</CardTitle>
          </CardHeader>
          <CardContent>
            {#if wards.length > 0}
              <div class="space-y-3">
                {#each wards as w}
                  <div class="flex items-center justify-between">
                    <span class="text-sm">{w.ward_name}</span>
                    <div class="flex items-center gap-3">
                      <div class="h-2 w-24 overflow-hidden rounded-full bg-muted">
                        <div class="h-full rounded-full {w.occupancy_rate >= 90 ? 'bg-destructive' : w.occupancy_rate >= 70 ? 'bg-yellow-500' : 'bg-green-500'}" style="width: {w.occupancy_rate}%"></div>
                      </div>
                      <span class="w-12 text-right text-sm font-medium">{w.occupancy_rate}%</span>
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-sm text-muted-foreground text-center py-8">No ward data</p>
            {/if}
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-2"><DollarSign class="h-5 w-5" /> Recent Invoices</CardTitle>
          </CardHeader>
          <CardContent>
            {#if invoices.length > 0}
              <div class="space-y-3">
                {#each invoices.slice(0, 5) as inv}
                  <div class="flex items-center justify-between">
                    <div>
                      <p class="text-sm font-medium">{inv.invoice_number}</p>
                      <p class="text-xs text-muted-foreground">{formatDate(inv.invoice_date)}</p>
                    </div>
                    <div class="flex items-center gap-3">
                      <span class="text-sm font-medium">{formatCurrency(inv.total)}</span>
                      <StatusBadge status={inv.status} />
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-sm text-muted-foreground text-center py-8">No invoices</p>
            {/if}
          </CardContent>
        </Card>
      </div>

      <!-- Quick Actions -->
      <Card>
        <CardHeader><CardTitle>Quick Actions</CardTitle></CardHeader>
        <CardContent>
          <div class="grid gap-3 md:grid-cols-2 lg:grid-cols-5">
            <a href="/patients/new" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <Users class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">Register Patient</p></div>
            </a>
            <a href="/appointments/new" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <Calendar class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">Book Appointment</p></div>
            </a>
            <a href="/wards" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <Bed class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">Ward Status</p></div>
            </a>
            <a href="/billing" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <DollarSign class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">Billing</p></div>
            </a>
            <a href="/reports" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <FileText class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">Reports</p></div>
            </a>
          </div>
        </CardContent>
      </Card>

    <!-- ═══════════════ DOCTOR DASHBOARD ═══════════════ -->
    {:else if $userRole === "doctor"}
      <!-- KPI Row -->
      <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <KPICard title="My Patients Today" value={stats?.total_patients_today ?? 0} change="Seen today" changeType="positive" icon={Stethoscope} />
        <KPICard title="Appointments" value={stats?.total_appointments_today ?? 0} change="Scheduled" changeType="neutral" icon={Calendar} />
        <KPICard title="Pending Lab Orders" value={stats?.pending_lab_orders ?? 0} change="Awaiting results" changeType={stats && stats.pending_lab_orders > 5 ? "negative" : "neutral"} icon={TestTube} />
        <KPICard title="Active Admissions" value={stats?.active_admissions ?? 0} change="Currently admitted" changeType="neutral" icon={Activity} />
      </div>

      <!-- Today's Appointments -->
      <Card>
        <CardHeader>
          <CardTitle class="flex items-center gap-2"><Calendar class="h-5 w-5" /> Today's Appointments</CardTitle>
        </CardHeader>
        <CardContent>
          {#if appointments.length > 0}
            <div class="space-y-3">
              {#each appointments as apt}
                <div class="flex items-center justify-between rounded-lg border p-3">
                  <div class="flex items-center gap-3">
                    <div class="flex h-10 w-10 items-center justify-center rounded-full bg-primary/10">
                      <Stethoscope class="h-5 w-5 text-primary" />
                    </div>
                    <div>
                      <p class="font-medium text-sm">{apt.patient_name}</p>
                      <p class="text-xs text-muted-foreground">{apt.patient_uid} · {apt.appointment.visit_type}</p>
                    </div>
                  </div>
                  <div class="flex items-center gap-3">
                    <div class="text-right">
                      <p class="text-sm font-medium">{apt.appointment.appointment_time}</p>
                      <p class="text-xs text-muted-foreground">{apt.appointment.reason || "No reason"}</p>
                    </div>
                    <StatusBadge status={apt.appointment.status} />
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <p class="text-sm text-muted-foreground text-center py-8">No appointments scheduled</p>
          {/if}
        </CardContent>
      </Card>

      <!-- Lab Orders + Ward Overview -->
      <div class="grid gap-4 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-2"><TestTube class="h-5 w-5" /> Recent Lab Orders</CardTitle>
          </CardHeader>
          <CardContent>
            {#if labOrders.length > 0}
              <div class="space-y-3">
                {#each labOrders.slice(0, 5) as order}
                  <div class="flex items-center justify-between rounded-lg border p-3">
                    <div>
                      <p class="text-sm font-medium">{order.patient_name}</p>
                      <p class="text-xs text-muted-foreground">{order.patient_uid} · {order.order_date}</p>
                    </div>
                    <StatusBadge status={order.status} />
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-sm text-muted-foreground text-center py-8">No lab orders</p>
            {/if}
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-2"><Bed class="h-5 w-5" /> Ward Overview</CardTitle>
          </CardHeader>
          <CardContent>
            {#if wards.length > 0}
              <div class="space-y-3">
                {#each wards as w}
                  <div class="flex items-center justify-between">
                    <span class="text-sm">{w.ward_name}</span>
                    <div class="flex items-center gap-3">
                      <span class="text-xs text-muted-foreground">{w.occupied}/{w.total_beds} beds</span>
                      <div class="h-2 w-20 overflow-hidden rounded-full bg-muted">
                        <div class="h-full rounded-full {w.occupancy_rate >= 90 ? 'bg-destructive' : w.occupancy_rate >= 70 ? 'bg-yellow-500' : 'bg-green-500'}" style="width: {w.occupancy_rate}%"></div>
                      </div>
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-sm text-muted-foreground text-center py-8">No ward data</p>
            {/if}
          </CardContent>
        </Card>
      </div>

      <!-- Quick Actions -->
      <Card>
        <CardHeader><CardTitle>Quick Actions</CardTitle></CardHeader>
        <CardContent>
          <div class="grid gap-3 md:grid-cols-2 lg:grid-cols-4">
            <a href="/patients" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <Users class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">View Patients</p></div>
            </a>
            <a href="/appointments" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <Calendar class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">Appointments</p></div>
            </a>
            <a href="/lab" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <TestTube class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">Lab Orders</p></div>
            </a>
            <a href="/wards" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <Bed class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">Ward Status</p></div>
            </a>
          </div>
        </CardContent>
      </Card>

    <!-- ═══════════════ STAFF DASHBOARD ═══════════════ -->
    {:else}
      <!-- KPI Row -->
      <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <KPICard title="Patients Today" value={stats?.total_patients_today ?? 0} change="Registered" changeType="positive" icon={Users} />
        <KPICard title="Appointments" value={stats?.total_appointments_today ?? 0} change="Scheduled" changeType="neutral" icon={Calendar} />
        <KPICard title="Bed Occupancy" value="{(stats?.bed_occupancy_rate ?? 0).toFixed(1)}%" change="Current" changeType="neutral" icon={Bed} />
        <KPICard title="Pending Labs" value={stats?.pending_lab_orders ?? 0} change="To process" changeType={stats && stats.pending_lab_orders > 5 ? "negative" : "neutral"} icon={TestTube} />
      </div>

      <!-- Appointments + Ward Status -->
      <div class="grid gap-4 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-2"><Calendar class="h-5 w-5" /> Today's Appointments</CardTitle>
          </CardHeader>
          <CardContent>
            {#if appointments.length > 0}
              <div class="space-y-3">
                {#each appointments as apt}
                  <div class="flex items-center justify-between rounded-lg border p-3">
                    <div>
                      <p class="font-medium text-sm">{apt.patient_name}</p>
                      <p class="text-xs text-muted-foreground">{apt.doctor_name} · {apt.appointment.appointment_time}</p>
                    </div>
                    <StatusBadge status={apt.appointment.status} />
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-sm text-muted-foreground text-center py-8">No appointments</p>
            {/if}
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-2"><Bed class="h-5 w-5" /> Ward Occupancy</CardTitle>
          </CardHeader>
          <CardContent>
            {#if wards.length > 0}
              <div class="space-y-3">
                {#each wards as w}
                  <div class="flex items-center justify-between">
                    <span class="text-sm">{w.ward_name}</span>
                    <div class="flex items-center gap-3">
                      <span class="text-xs text-muted-foreground">{w.occupied}/{w.total_beds}</span>
                      <div class="h-2 w-20 overflow-hidden rounded-full bg-muted">
                        <div class="h-full rounded-full {w.occupancy_rate >= 90 ? 'bg-destructive' : w.occupancy_rate >= 70 ? 'bg-yellow-500' : 'bg-green-500'}" style="width: {w.occupancy_rate}%"></div>
                      </div>
                      <span class="w-10 text-right text-xs font-medium">{w.occupancy_rate}%</span>
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-sm text-muted-foreground text-center py-8">No ward data</p>
            {/if}
          </CardContent>
        </Card>
      </div>

      <!-- Lab Orders + Invoices -->
      <div class="grid gap-4 lg:grid-cols-2">
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-2"><TestTube class="h-5 w-5" /> Lab Orders</CardTitle>
          </CardHeader>
          <CardContent>
            {#if labOrders.length > 0}
              <div class="space-y-3">
                {#each labOrders.slice(0, 5) as order}
                  <div class="flex items-center justify-between rounded-lg border p-3">
                    <div>
                      <p class="text-sm font-medium">{order.patient_name}</p>
                      <p class="text-xs text-muted-foreground">{order.order_date}</p>
                    </div>
                    <StatusBadge status={order.status} />
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-sm text-muted-foreground text-center py-8">No lab orders</p>
            {/if}
          </CardContent>
        </Card>
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-2"><FileText class="h-5 w-5" /> Recent Invoices</CardTitle>
          </CardHeader>
          <CardContent>
            {#if invoices.length > 0}
              <div class="space-y-3">
                {#each invoices.slice(0, 5) as inv}
                  <div class="flex items-center justify-between rounded-lg border p-3">
                    <div>
                      <p class="text-sm font-medium">{inv.invoice_number}</p>
                      <p class="text-xs text-muted-foreground">{formatDate(inv.invoice_date)}</p>
                    </div>
                    <div class="flex items-center gap-2">
                      <span class="text-sm font-medium">{formatCurrency(inv.total)}</span>
                      <StatusBadge status={inv.status} />
                    </div>
                  </div>
                {/each}
              </div>
            {:else}
              <p class="text-sm text-muted-foreground text-center py-8">No invoices</p>
            {/if}
          </CardContent>
        </Card>
      </div>

      <!-- Quick Actions -->
      <Card>
        <CardHeader><CardTitle>Quick Actions</CardTitle></CardHeader>
        <CardContent>
          <div class="grid gap-3 md:grid-cols-2 lg:grid-cols-5">
            <a href="/patients" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <Users class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">Patients</p></div>
            </a>
            <a href="/wards" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <Bed class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">Wards</p></div>
            </a>
            <a href="/lab" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <TestTube class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">Lab</p></div>
            </a>
            <a href="/pharmacy" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <Pill class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">Pharmacy</p></div>
            </a>
            <a href="/billing" class="flex items-center gap-3 rounded-lg border p-4 hover:bg-accent transition-colors">
              <DollarSign class="h-5 w-5 text-primary" />
              <div><p class="font-medium text-sm">Billing</p></div>
            </a>
          </div>
        </CardContent>
      </Card>
    {/if}
  {/if}
</PageLayout>
