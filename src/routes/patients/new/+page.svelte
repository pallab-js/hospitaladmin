<script lang="ts">
  import { goto } from "$app/navigation";
  import { createPatient } from "$lib/lib/api.js";
  import { notifications } from "$lib/stores/notifications.js";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import Button from "$lib/components/ui/button/index.svelte";
  import Input from "$lib/components/ui/input/index.svelte";
  import Label from "$lib/components/ui/label/index.svelte";
  import Card from "$lib/components/ui/card/index.svelte";
  import CardHeader from "$lib/components/ui/card/card-header.svelte";
  import CardTitle from "$lib/components/ui/card/card-title.svelte";
  import CardContent from "$lib/components/ui/card/card-content.svelte";
  import CardFooter from "$lib/components/ui/card/card-footer.svelte";
  import Select from "$lib/components/ui/select/index.svelte";
  import Separator from "$lib/components/ui/separator/index.svelte";
  import { ArrowLeft, Save } from "@lucide/svelte";

  let loading = $state(false);
  let form = $state({
    first_name: "",
    last_name: "",
    date_of_birth: "",
    gender: "male",
    blood_group: "",
    phone: "",
    email: "",
    address: "",
    emergency_contact_name: "",
    emergency_contact_phone: "",
    insurance_provider: "",
    insurance_id: "",
    allergies: "",
    medical_history: "",
  });

  async function handleSubmit() {
    if (!form.first_name || !form.last_name || !form.date_of_birth) {
      notifications.add({
        type: "error",
        title: "Validation Error",
        message: "Please fill in all required fields",
      });
      return;
    }

    loading = true;
    try {
      const patient = await createPatient(form);
      notifications.add({
        type: "success",
        title: "Patient Registered",
        message: `${patient.first_name} ${patient.last_name} (${patient.patient_uid})`,
      });
      goto(`/patients/${patient.id}`);
    } catch (e) {
      notifications.add({
        type: "error",
        title: "Registration Failed",
        message: "An error occurred while registering the patient",
      });
    } finally {
      loading = false;
    }
  }
</script>

<PageLayout title="Register New Patient" description="Enter patient information to create a new record">
  {#snippet actions()}
    <a href="/patients">
      <Button variant="outline">
        <ArrowLeft class="h-4 w-4 mr-2" />
        Back to Patients
      </Button>
    </a>
  {/snippet}

  <form onsubmit={(e) => { e.preventDefault(); handleSubmit(); }}>
    <div class="grid gap-6 lg:grid-cols-2">
      <!-- Personal Information -->
      <Card>
        <CardHeader>
          <CardTitle>Personal Information</CardTitle>
        </CardHeader>
        <CardContent class="space-y-4">
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="first_name">First Name *</Label>
              <Input id="first_name" bind:value={form.first_name} placeholder="Enter first name" required />
            </div>
            <div class="space-y-2">
              <Label for="last_name">Last Name *</Label>
              <Input id="last_name" bind:value={form.last_name} placeholder="Enter last name" required />
            </div>
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="dob">Date of Birth *</Label>
              <Input id="dob" type="date" bind:value={form.date_of_birth} required />
            </div>
            <div class="space-y-2">
              <Label for="gender">Gender *</Label>
              <Select id="gender" bind:value={form.gender}>
                <option value="male">Male</option>
                <option value="female">Female</option>
                <option value="other">Other</option>
              </Select>
            </div>
          </div>
          <div class="grid grid-cols-2 gap-4">
            <div class="space-y-2">
              <Label for="blood_group">Blood Group</Label>
              <Select id="blood_group" bind:value={form.blood_group}>
                <option value="">Select</option>
                <option value="A+">A+</option>
                <option value="A-">A-</option>
                <option value="B+">B+</option>
                <option value="B-">B-</option>
                <option value="AB+">AB+</option>
                <option value="AB-">AB-</option>
                <option value="O+">O+</option>
                <option value="O-">O-</option>
              </Select>
            </div>
            <div class="space-y-2">
              <Label for="phone">Phone</Label>
              <Input id="phone" type="tel" bind:value={form.phone} placeholder="Enter phone number" />
            </div>
          </div>
          <div class="space-y-2">
            <Label for="email">Email</Label>
            <Input id="email" type="email" bind:value={form.email} placeholder="Enter email address" />
          </div>
          <div class="space-y-2">
            <Label for="address">Address</Label>
            <Input id="address" bind:value={form.address} placeholder="Enter full address" />
          </div>
        </CardContent>
      </Card>

      <!-- Emergency & Medical -->
      <div class="space-y-6">
        <Card>
          <CardHeader>
            <CardTitle>Emergency Contact</CardTitle>
          </CardHeader>
          <CardContent class="space-y-4">
            <div class="space-y-2">
              <Label for="emergency_name">Contact Name</Label>
              <Input id="emergency_name" bind:value={form.emergency_contact_name} placeholder="Enter contact name" />
            </div>
            <div class="space-y-2">
              <Label for="emergency_phone">Contact Phone</Label>
              <Input id="emergency_phone" type="tel" bind:value={form.emergency_contact_phone} placeholder="Enter contact phone" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Insurance Information</CardTitle>
          </CardHeader>
          <CardContent class="space-y-4">
            <div class="space-y-2">
              <Label for="insurance_provider">Insurance Provider</Label>
              <Input id="insurance_provider" bind:value={form.insurance_provider} placeholder="Enter provider name" />
            </div>
            <div class="space-y-2">
              <Label for="insurance_id">Policy ID</Label>
              <Input id="insurance_id" bind:value={form.insurance_id} placeholder="Enter policy ID" />
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle>Medical Information</CardTitle>
          </CardHeader>
          <CardContent class="space-y-4">
            <div class="space-y-2">
              <Label for="allergies">Known Allergies</Label>
              <Input id="allergies" bind:value={form.allergies} placeholder="List any known allergies" />
            </div>
            <div class="space-y-2">
              <Label for="medical_history">Medical History</Label>
              <Input id="medical_history" bind:value={form.medical_history} placeholder="Relevant medical history" />
            </div>
          </CardContent>
        </Card>
      </div>
    </div>

    <!-- Submit -->
    <div class="mt-6 flex justify-end gap-4">
      <a href="/patients">
        <Button type="button" variant="outline">Cancel</Button>
      </a>
      <Button type="submit" disabled={loading}>
        {#if loading}
          <div class="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent mr-2"></div>
          Registering...
        {:else}
          <Save class="h-4 w-4 mr-2" />
          Register Patient
        {/if}
      </Button>
    </div>
  </form>
</PageLayout>
