<script lang="ts">
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { getPatientById } from "$lib/lib/api.js";
  import { notifications } from "$lib/stores/notifications.js";
  import type { Patient } from "$lib/lib/types.js";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import Button from "$lib/components/ui/button/index.svelte";
  import Card from "$lib/components/ui/card/index.svelte";
  import CardHeader from "$lib/components/ui/card/card-header.svelte";
  import CardTitle from "$lib/components/ui/card/card-title.svelte";
  import CardContent from "$lib/components/ui/card/card-content.svelte";
  import Badge from "$lib/components/ui/badge/index.svelte";
  import Separator from "$lib/components/ui/separator/index.svelte";
  import { ArrowLeft, Edit, Phone, Mail, MapPin, AlertTriangle, Shield, History, Printer } from "@lucide/svelte";
  import { formatDate } from "$lib/utils/index.js";

  let patient = $state<Patient | null>(null);
  let loading = $state(true);

  onMount(async () => {
    const id = $page.params.id ?? "";
    try {
      patient = await getPatientById(id);
    } catch (e) {
      console.error("Failed to load patient:", e);
    } finally {
      loading = false;
    }
  });

  function getGenderBadge(gender: string) {
    const colors: Record<string, string> = {
      male: "bg-blue-100 text-blue-800",
      female: "bg-pink-100 text-pink-800",
      other: "bg-purple-100 text-purple-800",
    };
    return colors[gender] || "bg-gray-100 text-gray-800";
  }
</script>

<PageLayout title={patient ? `${patient.first_name} ${patient.last_name}` : "Patient Details"}>
  {#snippet actions()}
    <a href="/patients">
      <Button variant="outline">
        <ArrowLeft class="h-4 w-4 mr-2" />
        Back
      </Button>
    </a>
    <Button variant="outline" onclick={() => notifications.add({ type: "info", title: "Coming Soon", message: "Patient editing will be available in a future update" })}>
      <Edit class="h-4 w-4 mr-2" />
      Edit
    </Button>
  {/snippet}

  {#if loading}
    <div class="grid gap-6 lg:grid-cols-3">
      {#each Array(3) as _}
        <div class="h-64 animate-pulse rounded-xl bg-muted"></div>
      {/each}
    </div>
  {:else if patient}
    <div class="grid gap-6 lg:grid-cols-3">
      <!-- Patient Info Card -->
      <Card class="lg:col-span-2">
        <CardHeader>
          <div class="flex items-center justify-between">
            <CardTitle>Patient Information</CardTitle>
            <div class="flex gap-2">
              <Badge class={getGenderBadge(patient.gender)}>
                {patient.gender}
              </Badge>
              <Badge variant="outline">{patient.patient_uid}</Badge>
            </div>
          </div>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div>
              <p class="text-sm text-muted-foreground">Full Name</p>
              <p class="font-medium">{patient.first_name} {patient.last_name}</p>
            </div>
            <div>
              <p class="text-sm text-muted-foreground">Date of Birth</p>
              <p class="font-medium">{formatDate(patient.date_of_birth)}</p>
            </div>
            <div>
              <p class="text-sm text-muted-foreground">Blood Group</p>
              <p class="font-medium">{patient.blood_group || "Not specified"}</p>
            </div>
            <div>
              <p class="text-sm text-muted-foreground">Registered</p>
              <p class="font-medium">{formatDate(patient.created_at)}</p>
            </div>
          </div>

          <Separator />

          <div class="space-y-3">
            <h4 class="font-medium">Contact Information</h4>
            <div class="grid grid-cols-2 gap-4">
              <div class="flex items-center gap-2">
                <Phone class="h-4 w-4 text-muted-foreground" />
                <span>{patient.phone || "Not provided"}</span>
              </div>
              <div class="flex items-center gap-2">
                <Mail class="h-4 w-4 text-muted-foreground" />
                <span>{patient.email || "Not provided"}</span>
              </div>
            </div>
            {#if patient.address}
              <div class="flex items-center gap-2">
                <MapPin class="h-4 w-4 text-muted-foreground" />
                <span>{patient.address}</span>
              </div>
            {/if}
          </div>

          {#if patient.emergency_contact_name}
            <Separator />
            <div class="space-y-3">
              <h4 class="font-medium">Emergency Contact</h4>
              <p>{patient.emergency_contact_name} - {patient.emergency_contact_phone || "No phone"}</p>
            </div>
          {/if}
        </CardContent>
      </Card>

      <!-- Side Info -->
      <div class="space-y-6">
        <!-- Insurance -->
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-2 text-base">
              <Shield class="h-4 w-4" />
              Insurance
            </CardTitle>
          </CardHeader>
          <CardContent>
            {#if patient.insurance_provider}
              <div class="space-y-2">
                <div>
                  <p class="text-sm text-muted-foreground">Provider</p>
                  <p class="font-medium">{patient.insurance_provider}</p>
                </div>
                <div>
                  <p class="text-sm text-muted-foreground">Policy ID</p>
                  <p class="font-medium">{patient.insurance_id || "Not provided"}</p>
                </div>
              </div>
            {:else}
              <p class="text-sm text-muted-foreground">No insurance information</p>
            {/if}
          </CardContent>
        </Card>

        <!-- Medical -->
        <Card>
          <CardHeader>
            <CardTitle class="flex items-center gap-2 text-base">
              <AlertTriangle class="h-4 w-4" />
              Medical Information
            </CardTitle>
          </CardHeader>
          <CardContent class="space-y-3">
            <div>
              <p class="text-sm text-muted-foreground">Allergies</p>
              <p class="font-medium">{patient.allergies || "None reported"}</p>
            </div>
            <div>
              <p class="text-sm text-muted-foreground">Medical History</p>
              <p class="font-medium">{patient.medical_history || "None reported"}</p>
            </div>
          </CardContent>
        </Card>

        <!-- Quick Actions -->
        <Card>
          <CardHeader>
            <CardTitle class="text-base">Quick Actions</CardTitle>
          </CardHeader>
          <CardContent class="space-y-2">
            <a href="/appointments/new?patient={patient.id}" class="block">
              <Button variant="outline" class="w-full justify-start">
                Book Appointment
              </Button>
            </a>
            <Button variant="outline" class="w-full justify-start" onclick={() => notifications.add({ type: "info", title: "Coming Soon", message: "Patient history view will be available in a future update" })}>
              <History class="h-4 w-4 mr-2" />
              View History
            </Button>
            <Button variant="outline" class="w-full justify-start" onclick={() => window.print()}>
              <Printer class="h-4 w-4 mr-2" />
              Print Summary
            </Button>
          </CardContent>
        </Card>
      </div>
    </div>
  {:else}
    <Card>
      <CardContent class="p-8 text-center">
        <p class="text-muted-foreground">Patient not found</p>
      </CardContent>
    </Card>
  {/if}
</PageLayout>
