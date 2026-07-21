<script lang="ts">
  import { notifications } from "$lib/stores/notifications.js";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import Card from "$lib/components/ui/card/index.svelte";
  import CardHeader from "$lib/components/ui/card/card-header.svelte";
  import CardTitle from "$lib/components/ui/card/card-title.svelte";
  import CardContent from "$lib/components/ui/card/card-content.svelte";
  import Button from "$lib/components/ui/button/index.svelte";
  import { BarChart3, Download, FileText, Users, Bed, DollarSign } from "@lucide/svelte";

  function generateReport(title: string) {
    notifications.add({
      type: "success",
      title: "Report Generated",
      message: `${title} report has been generated and is ready for download`,
    });
  }

  const reports = [
    { title: "Daily Summary", description: "Overview of today's operations", icon: BarChart3 },
    { title: "Patient Demographics", description: "Age, gender, and blood group distribution", icon: Users },
    { title: "Bed Utilization", description: "Ward occupancy and bed usage statistics", icon: Bed },
    { title: "Revenue Report", description: "Financial summary and payment analysis", icon: DollarSign },
    { title: "Lab Test Summary", description: "Test orders and turnaround times", icon: FileText },
    { title: "Pharmacy Consumption", description: "Medication usage and stock levels", icon: Download },
  ];
</script>

<PageLayout title="Reports" description="Generate and view hospital reports">
  <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
    {#each reports as report}
      <Card class="hover:shadow-md transition-shadow">
        <CardContent class="p-6">
          <div class="flex items-start gap-4">
            <div class="rounded-lg bg-primary/10 p-3">
              <report.icon class="h-6 w-6 text-primary" />
            </div>
            <div class="flex-1">
              <h3 class="font-semibold">{report.title}</h3>
              <p class="text-sm text-muted-foreground mt-1">{report.description}</p>
              <Button variant="link" class="p-0 h-auto mt-2" onclick={() => generateReport(report.title)}>
                Generate Report
              </Button>
            </div>
          </div>
        </CardContent>
      </Card>
    {/each}
  </div>
</PageLayout>
