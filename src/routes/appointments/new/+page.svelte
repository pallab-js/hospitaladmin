<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { createAppointment, getPatients, getDoctors } from "$lib/lib/api.js";
  import { notifications } from "$lib/stores/notifications.js";
  import type { Patient } from "$lib/lib/types.js";
  import { onMount } from "svelte";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import Button from "$lib/components/ui/button/index.svelte";
  import Input from "$lib/components/ui/input/index.svelte";
  import Label from "$lib/components/ui/label/index.svelte";
  import Card from "$lib/components/ui/card/index.svelte";
  import CardHeader from "$lib/components/ui/card/card-header.svelte";
  import CardTitle from "$lib/components/ui/card/card-title.svelte";
  import CardContent from "$lib/components/ui/card/card-content.svelte";
  import Select from "$lib/components/ui/select/index.svelte";
  import { ArrowLeft, Save } from "@lucide/svelte";

  let loading = $state(false);
  let patients = $state<Patient[]>([]);
  let doctors = $state<{ id: string; full_name: string; department_id: string | null }[]>([]);
  let loadingData = $state(true);

  let form = $state({
    patient_id: $page.url.searchParams.get("patient") || "",
    doctor_id: "",
    appointment_date: new Date().toISOString().slice(0, 10),
    appointment_time: "09:00",
    visit_type: "consultation",
    reason: "",
  });

  onMount(async () => {
    try {
      const [p, d] = await Promise.all([getPatients(), getDoctors()]);
      patients = p;
      doctors = d;
    } finally {
      loadingData = false;
    }
  });

  async function handleSubmit() {
    if (!form.patient_id || !form.doctor_id || !form.appointment_date || !form.appointment_time) {
      notifications.add({
        type: "error",
        title: "Validation Error",
        message: "Please fill in all required fields",
      });
      return;
    }

    loading = true;
    try {
      const selectedDoctor = doctors.find((d) => d.id === form.doctor_id);
      await createAppointment({
        ...form,
        department_id: selectedDoctor?.department_id || undefined,
      });
      notifications.add({
        type: "success",
        title: "Appointment Booked",
        message: `Scheduled for ${form.appointment_date} at ${form.appointment_time}`,
      });
      goto("/appointments");
    } catch (e) {
      notifications.add({
        type: "error",
        title: "Booking Failed",
        message: "An error occurred while booking the appointment",
      });
    } finally {
      loading = false;
    }
  }
</script>

<PageLayout title="Book Appointment" description="Schedule a new patient appointment">
  {#snippet actions()}
    <a href="/appointments">
      <Button variant="outline">
        <ArrowLeft class="h-4 w-4 mr-2" />
        Back
      </Button>
    </a>
  {/snippet}

  <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
    <Card class="max-w-2xl">
      <CardHeader>
        <CardTitle>Appointment Details</CardTitle>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="space-y-2">
          <Label for="patient">Patient *</Label>
          <Select id="patient" bind:value={form.patient_id}>
            <option value="">Select a patient</option>
            {#each patients as p}
              <option value={p.id}>{p.first_name} {p.last_name} ({p.patient_uid})</option>
            {/each}
          </Select>
        </div>

        <div class="space-y-2">
          <Label for="doctor">Doctor *</Label>
          <Select id="doctor" bind:value={form.doctor_id}>
            <option value="">Select a doctor</option>
            {#each doctors as d}
              <option value={d.id}>{d.full_name}</option>
            {/each}
          </Select>
        </div>

        <div class="grid grid-cols-2 gap-4">
          <div class="space-y-2">
            <Label for="date">Date *</Label>
            <Input id="date" type="date" bind:value={form.appointment_date} required />
          </div>
          <div class="space-y-2">
            <Label for="time">Time *</Label>
            <Input id="time" type="time" bind:value={form.appointment_time} required />
          </div>
        </div>

        <div class="space-y-2">
          <Label for="type">Type</Label>
          <Select id="type" bind:value={form.visit_type}>
            <option value="consultation">Consultation</option>
            <option value="follow_up">Follow-up</option>
            <option value="emergency">Emergency</option>
            <option value="routine_checkup">Routine Checkup</option>
          </Select>
        </div>

        <div class="space-y-2">
          <Label for="reason">Reason for Visit</Label>
          <Input id="reason" bind:value={form.reason} placeholder="Brief description of the visit reason" />
        </div>

        <div class="flex justify-end gap-4 pt-4">
          <a href="/appointments">
            <Button type="button" variant="outline">Cancel</Button>
          </a>
          <Button type="submit" disabled={loading}>
            {#if loading}
              <div class="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent mr-2"></div>
              Booking...
            {:else}
              <Save class="h-4 w-4 mr-2" />
              Book Appointment
            {/if}
          </Button>
        </div>
      </CardContent>
    </Card>
  </form>
</PageLayout>
