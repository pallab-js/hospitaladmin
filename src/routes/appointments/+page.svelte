<script lang="ts">
  import { onMount } from "svelte";
  import { getAppointments } from "$lib/lib/api.js";
  import type { AppointmentWithDetails } from "$lib/lib/types.js";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import Button from "$lib/components/ui/button/index.svelte";
  import Card from "$lib/components/ui/card/index.svelte"; import CardContent from "$lib/components/ui/card/card-content.svelte";
  import Badge from "$lib/components/ui/badge/index.svelte";
  import Table from "$lib/components/ui/table/index.svelte"; import TableHeader from "$lib/components/ui/table/table-header.svelte"; import TableBody from "$lib/components/ui/table/table-body.svelte"; import TableRow from "$lib/components/ui/table/table-row.svelte"; import TableHead from "$lib/components/ui/table/table-head.svelte"; import TableCell from "$lib/components/ui/table/table-cell.svelte";
  import { Plus, Calendar, Clock } from "@lucide/svelte";
  import { formatDate, formatTime, getStatusColor } from "$lib/utils/index.js";

  let appointments = $state<AppointmentWithDetails[]>([]);
  let loading = $state(true);

  onMount(async () => {
    await loadAppointments();
  });

  async function loadAppointments() {
    loading = true;
    try {
      appointments = await getAppointments();
    } catch (e) {
      console.error("Failed to load appointments:", e);
    } finally {
      loading = false;
    }
  }

  function getStatusLabel(status: string) {
    const labels: Record<string, string> = {
      scheduled: "Scheduled",
      confirmed: "Confirmed",
      in_progress: "In Progress",
      completed: "Completed",
      cancelled: "Cancelled",
      no_show: "No Show",
    };
    return labels[status] || status;
  }
</script>

<PageLayout title="Appointments" description="Manage patient appointments and scheduling">
  {#snippet actions()}
    <a href="/appointments/new">
      <Button>
        <Plus class="h-4 w-4 mr-2" />
        Book Appointment
      </Button>
    </a>
  {/snippet}

  <Card>
    <CardContent class="p-0">
      {#if loading}
        <div class="p-8 text-center">
          <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent mx-auto"></div>
          <p class="mt-4 text-sm text-muted-foreground">Loading appointments...</p>
        </div>
      {:else if appointments.length === 0}
        <div class="p-8 text-center">
          <Calendar class="h-12 w-12 mx-auto text-muted-foreground mb-4" />
          <p class="text-muted-foreground">No appointments found</p>
          <p class="text-sm text-muted-foreground mt-1">Book a new appointment to get started</p>
        </div>
      {:else}
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Patient</TableHead>
              <TableHead>Doctor</TableHead>
              <TableHead>Date & Time</TableHead>
              <TableHead>Type</TableHead>
              <TableHead>Status</TableHead>
              <TableHead>Reason</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each appointments as apt}
              <TableRow>
                <TableCell>
                  <div>
                    <p class="font-medium">{apt.patient_name}</p>
                    <p class="text-xs text-muted-foreground">{apt.patient_uid}</p>
                  </div>
                </TableCell>
                <TableCell>{apt.doctor_name}</TableCell>
                <TableCell>
                  <div class="flex items-center gap-2">
                    <Calendar class="h-4 w-4 text-muted-foreground" />
                    <span>{formatDate(apt.appointment.appointment_date)}</span>
                  </div>
                  <div class="flex items-center gap-2 mt-1">
                    <Clock class="h-4 w-4 text-muted-foreground" />
                    <span>{formatTime(apt.appointment.appointment_time)}</span>
                  </div>
                </TableCell>
                <TableCell>
                  <Badge variant="outline">{apt.appointment.visit_type}</Badge>
                </TableCell>
                <TableCell>
                  <Badge class={getStatusColor(apt.appointment.status)}>
                    {getStatusLabel(apt.appointment.status)}
                  </Badge>
                </TableCell>
                <TableCell class="max-w-[200px] truncate">
                  {apt.appointment.reason || "-"}
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
      {/if}
    </CardContent>
  </Card>
</PageLayout>
