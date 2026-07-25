<script lang="ts">
  import { onMount } from "svelte";
  import { getPatients, searchPatients } from "$lib/lib/api.js";
  import type { Patient } from "$lib/lib/types.js";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import Button from "$lib/components/ui/button/index.svelte";
  import Input from "$lib/components/ui/input/index.svelte";
  import Card from "$lib/components/ui/card/index.svelte"; import CardContent from "$lib/components/ui/card/card-content.svelte";
  import Badge from "$lib/components/ui/badge/index.svelte";
  import Table from "$lib/components/ui/table/index.svelte"; import TableHeader from "$lib/components/ui/table/table-header.svelte"; import TableBody from "$lib/components/ui/table/table-body.svelte"; import TableRow from "$lib/components/ui/table/table-row.svelte"; import TableHead from "$lib/components/ui/table/table-head.svelte"; import TableCell from "$lib/components/ui/table/table-cell.svelte";
  import { Plus, Search, Eye, ChevronLeft, ChevronRight } from "@lucide/svelte";
  import { debounce, getGenderBadge } from "$lib/utils/index.js";

  let patients = $state<Patient[]>([]);
  let loading = $state(true);
  let searchQuery = $state("");
  let currentPage = $state(1);
  let totalCount = $state(0);
  const pageSize = 10;

  onMount(async () => {
    await loadPatients();
  });

  async function loadPatients() {
    loading = true;
    try {
      if (searchQuery) {
        patients = await searchPatients({ query: searchQuery, page: currentPage, limit: pageSize });
      } else {
        patients = await getPatients(currentPage, pageSize);
      }
      totalCount = patients.length;
    } catch (e) {
      console.error("Failed to load patients:", e);
    } finally {
      loading = false;
    }
  }

  const handleSearch = debounce(async () => {
    currentPage = 1;
    await loadPatients();
  }, 300);

  function prevPage() {
    if (currentPage > 1) {
      currentPage--;
      loadPatients();
    }
  }

  function nextPage() {
    currentPage++;
    loadPatients();
  }
</script>

<PageLayout title="Patients" description="Manage patient records and information">
  {#snippet actions()}
    <a href="/patients/new">
      <Button>
        <Plus class="h-4 w-4 mr-2" />
        Register Patient
      </Button>
    </a>
  {/snippet}

  <!-- Search -->
  <div class="flex items-center gap-4">
    <div class="relative flex-1 max-w-sm">
      <Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
      <Input
        type="text"
        placeholder="Search by name, ID, or phone..."
        class="pl-9"
        bind:value={searchQuery}
        oninput={handleSearch}
      />
    </div>
  </div>

  <!-- Table -->
  <Card>
    <CardContent class="p-0">
      {#if loading}
        <div class="p-8 text-center">
          <div class="h-8 w-8 animate-spin rounded-full border-4 border-primary border-t-transparent mx-auto"></div>
          <p class="mt-4 text-sm text-muted-foreground">Loading patients...</p>
        </div>
      {:else if patients.length === 0}
        <div class="p-8 text-center">
          <p class="text-muted-foreground">No patients found</p>
        </div>
      {:else}
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Patient ID</TableHead>
              <TableHead>Name</TableHead>
              <TableHead>Gender</TableHead>
              <TableHead>Phone</TableHead>
              <TableHead>Blood Group</TableHead>
              <TableHead>Registered</TableHead>
              <TableHead class="text-right">Actions</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {#each patients as patient}
              <TableRow>
                <TableCell class="font-mono text-sm">{patient.patient_uid}</TableCell>
                <TableCell class="font-medium">
                  {patient.first_name} {patient.last_name}
                </TableCell>
                <TableCell>
                  <Badge class={getGenderBadge(patient.gender)}>
                    {patient.gender}
                  </Badge>
                </TableCell>
                <TableCell>{patient.phone || "-"}</TableCell>
                <TableCell>{patient.blood_group || "-"}</TableCell>
                <TableCell>{new Date(patient.created_at).toLocaleDateString()}</TableCell>
                <TableCell class="text-right">
                  <a href="/patients/{patient.id}">
                    <Button variant="ghost" size="icon" aria-label="View patient {patient.first_name} {patient.last_name}">
                      <Eye class="h-4 w-4" />
                    </Button>
                  </a>
                </TableCell>
              </TableRow>
            {/each}
          </TableBody>
        </Table>
        {#if patients.length >= pageSize}
          <div class="flex items-center justify-between border-t px-4 py-3">
            <p class="text-sm text-muted-foreground">Page {currentPage}</p>
            <div class="flex gap-2">
              <Button variant="outline" size="sm" onclick={prevPage} disabled={currentPage <= 1}>
                <ChevronLeft class="h-4 w-4 mr-1" />
                Previous
              </Button>
              <Button variant="outline" size="sm" onclick={nextPage}>
                Next
                <ChevronRight class="h-4 w-4 ml-1" />
              </Button>
            </div>
          </div>
        {/if}
      {/if}
    </CardContent>
  </Card>
</PageLayout>
