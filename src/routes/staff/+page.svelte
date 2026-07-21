<script lang="ts">
  import { onMount } from "svelte";
  import { getStaffList } from "$lib/lib/api.js";
  import type { Staff } from "$lib/lib/types.js";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import Card from "$lib/components/ui/card/index.svelte";
  import CardHeader from "$lib/components/ui/card/card-header.svelte";
  import CardTitle from "$lib/components/ui/card/card-title.svelte";
  import CardContent from "$lib/components/ui/card/card-content.svelte";
  import Badge from "$lib/components/ui/badge/index.svelte";
  import Avatar from "$lib/components/ui/avatar/index.svelte";
  import AvatarFallback from "$lib/components/ui/avatar/avatar-fallback.svelte";
  import Input from "$lib/components/ui/input/index.svelte";
  import { UserCog, Stethoscope, Heart, Shield, Search } from "@lucide/svelte";

  let staff = $state<Staff[]>([]);
  let loading = $state(true);
  let search = $state("");
  let filterRole = $state("all");

  onMount(async () => {
    try {
      staff = await getStaffList();
    } finally {
      loading = false;
    }
  });

  function getRoleIcon(role: string) {
    switch (role) {
      case "doctor": return Stethoscope;
      case "nurse": return Heart;
      default: return Shield;
    }
  }

  function getRoleColor(role: string) {
    const colors: Record<string, string> = {
      doctor: "bg-primary/10 text-primary",
      nurse: "bg-success/10 text-success",
      pharmacist: "bg-info/10 text-info",
      lab_tech: "bg-warning/10 text-warning",
      receptionist: "bg-secondary text-secondary-foreground",
    };
    return colors[role] || "bg-muted text-muted-foreground";
  }

  function getInitials(first: string, last: string) {
    return `${first[0]}${last[0]}`.toUpperCase();
  }

  const filteredStaff = $derived(
    staff.filter((s) => {
      const matchesSearch =
        !search ||
        `${s.first_name} ${s.last_name}`.toLowerCase().includes(search.toLowerCase()) ||
        (s.specialization && s.specialization.toLowerCase().includes(search.toLowerCase()));
      const matchesRole = filterRole === "all" || s.role === filterRole;
      return matchesSearch && matchesRole;
    })
  );

  const doctorCount = $derived(staff.filter((s) => s.role === "doctor").length);
  const nurseCount = $derived(staff.filter((s) => s.role === "nurse").length);
  const otherCount = $derived(staff.filter((s) => s.role !== "doctor" && s.role !== "nurse").length);
</script>

<PageLayout title="Staff" description="Manage hospital staff and assignments">
  <div class="grid gap-4 md:grid-cols-3">
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">Doctors</p>
            <p class="text-3xl font-bold">{doctorCount}</p>
          </div>
          <Stethoscope class="h-8 w-8 text-primary" />
        </div>
      </CardContent>
    </Card>
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">Nurses</p>
            <p class="text-3xl font-bold">{nurseCount}</p>
          </div>
          <Heart class="h-8 w-8 text-success" />
        </div>
      </CardContent>
    </Card>
    <Card>
      <CardContent class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <p class="text-sm text-muted-foreground">Other Staff</p>
            <p class="text-3xl font-bold">{otherCount}</p>
          </div>
          <Shield class="h-8 w-8 text-info" />
        </div>
      </CardContent>
    </Card>
  </div>

  <Card>
    <CardHeader>
      <div class="flex items-center justify-between">
        <CardTitle>Staff Directory</CardTitle>
        <div class="flex items-center gap-3">
          <div class="relative w-64">
            <Search class="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
            <Input placeholder="Search staff..." bind:value={search} class="pl-9" />
          </div>
          <select
            bind:value={filterRole}
            class="rounded-md border bg-background px-3 py-2 text-sm"
          >
            <option value="all">All Roles</option>
            <option value="doctor">Doctors</option>
            <option value="nurse">Nurses</option>
            <option value="pharmacist">Pharmacists</option>
            <option value="lab_tech">Lab Techs</option>
            <option value="receptionist">Receptionists</option>
          </select>
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
          {#each filteredStaff as member}
            {@const RoleIcon = getRoleIcon(member.role)}
            <div class="flex items-center justify-between rounded-lg border p-4">
              <div class="flex items-center gap-4">
                <Avatar>
                  <AvatarFallback class={getRoleColor(member.role)}>
                    {getInitials(member.first_name, member.last_name)}
                  </AvatarFallback>
                </Avatar>
                <div>
                  <p class="font-medium">{member.first_name} {member.last_name}</p>
                  <p class="text-sm text-muted-foreground">
                    {member.specialization || member.role.replace("_", " ")}
                    {#if member.qualification}
                      · {member.qualification}
                    {/if}
                  </p>
                </div>
              </div>
              <div class="flex items-center gap-3">
                <Badge class={getRoleColor(member.role)}>
                  <RoleIcon class="h-3 w-3 mr-1" />
                  {member.role.replace("_", " ")}
                </Badge>
                <Badge variant={member.is_active ? "default" : "secondary"}>
                  {member.is_active ? "Active" : "On Leave"}
                </Badge>
              </div>
            </div>
          {/each}
        </div>
      {/if}
    </CardContent>
  </Card>
</PageLayout>
