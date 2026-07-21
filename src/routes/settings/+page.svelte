<script lang="ts">
  import { auth, userName, userRole } from "$lib/stores/auth.js";
  import { sidebar } from "$lib/stores/sidebar.js";
  import { notifications } from "$lib/stores/notifications.js";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "$lib/components/ui/card/index.js";
  import Button from "$lib/components/ui/button/index.svelte";
  import Input from "$lib/components/ui/input/index.svelte";
  import Label from "$lib/components/ui/label/index.svelte";
  import Avatar from "$lib/components/ui/avatar/index.svelte";
  import AvatarFallback from "$lib/components/ui/avatar/avatar-fallback.svelte";
  import { User, Bell, Shield, Palette, Check } from "@lucide/svelte";

  let profileName = $state($userName);
  let profileEmail = $state("");
  let currentPassword = $state("");
  let newPassword = $state("");
  let confirmPassword = $state("");
  let theme = $state("light");
  let emailNotifications = $state(true);
  let lowStockAlerts = $state(true);
  let appointmentReminders = $state(false);

  function getInitials(name: string) {
    return name.split(" ").map(n => n[0]).join("").toUpperCase().slice(0, 2);
  }

  function saveProfile() {
    notifications.add({ type: "success", title: "Profile Updated", message: "Your profile has been saved" });
  }

  function updatePassword() {
    if (!currentPassword || !newPassword) {
      notifications.add({ type: "error", title: "Error", message: "Please fill in all password fields" });
      return;
    }
    if (newPassword !== confirmPassword) {
      notifications.add({ type: "error", title: "Error", message: "New passwords do not match" });
      return;
    }
    notifications.add({ type: "success", title: "Password Updated", message: "Your password has been changed" });
    currentPassword = "";
    newPassword = "";
    confirmPassword = "";
  }

  function setTheme(t: string) {
    theme = t;
    document.documentElement.classList.toggle("dark", t === "dark");
    notifications.add({ type: "info", title: "Theme Changed", message: `Switched to ${t} theme` });
  }
</script>

<PageLayout title="Settings" description="Manage your account and application settings">
  <div class="grid gap-6 lg:grid-cols-2">
    <!-- Profile Settings -->
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2">
          <User class="h-5 w-5" />
          Profile
        </CardTitle>
        <CardDescription>Manage your personal information</CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="flex items-center gap-4">
          <Avatar class="h-20 w-20">
            <AvatarFallback class="bg-primary text-primary-foreground text-xl">
              {getInitials($userName)}
            </AvatarFallback>
          </Avatar>
          <div>
            <p class="font-medium text-lg">{$userName}</p>
            <p class="text-muted-foreground capitalize">{$userRole}</p>
          </div>
        </div>
        <div class="space-y-2">
          <Label for="name">Full Name</Label>
          <Input id="name" bind:value={profileName} />
        </div>
        <div class="space-y-2">
          <Label for="email">Email</Label>
          <Input id="email" type="email" placeholder="your@email.com" bind:value={profileEmail} />
        </div>
        <Button onclick={saveProfile}>Save Changes</Button>
      </CardContent>
    </Card>

    <!-- Security Settings -->
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2">
          <Shield class="h-5 w-5" />
          Security
        </CardTitle>
        <CardDescription>Manage your password and security</CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="space-y-2">
          <Label for="current-password">Current Password</Label>
          <Input id="current-password" type="password" bind:value={currentPassword} />
        </div>
        <div class="space-y-2">
          <Label for="new-password">New Password</Label>
          <Input id="new-password" type="password" bind:value={newPassword} />
        </div>
        <div class="space-y-2">
          <Label for="confirm-password">Confirm Password</Label>
          <Input id="confirm-password" type="password" bind:value={confirmPassword} />
        </div>
        <Button onclick={updatePassword}>Update Password</Button>
      </CardContent>
    </Card>

    <!-- Notification Settings -->
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2">
          <Bell class="h-5 w-5" />
          Notifications
        </CardTitle>
        <CardDescription>Configure notification preferences</CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <label class="flex items-center justify-between cursor-pointer">
          <div>
            <p class="font-medium">Email Notifications</p>
            <p class="text-sm text-muted-foreground">Receive email alerts for important events</p>
          </div>
          <input type="checkbox" class="h-5 w-5 rounded border-gray-300" bind:checked={emailNotifications} />
        </label>
        <label class="flex items-center justify-between cursor-pointer">
          <div>
            <p class="font-medium">Low Stock Alerts</p>
            <p class="text-sm text-muted-foreground">Get notified when inventory is low</p>
          </div>
          <input type="checkbox" class="h-5 w-5 rounded border-gray-300" bind:checked={lowStockAlerts} />
        </label>
        <label class="flex items-center justify-between cursor-pointer">
          <div>
            <p class="font-medium">Appointment Reminders</p>
            <p class="text-sm text-muted-foreground">Send reminders for upcoming appointments</p>
          </div>
          <input type="checkbox" class="h-5 w-5 rounded border-gray-300" bind:checked={appointmentReminders} />
        </label>
        <Button onclick={() => notifications.add({ type: "success", title: "Saved", message: "Notification preferences updated" })}>
          Save Preferences
        </Button>
      </CardContent>
    </Card>

    <!-- Appearance Settings -->
    <Card>
      <CardHeader>
        <CardTitle class="flex items-center gap-2">
          <Palette class="h-5 w-5" />
          Appearance
        </CardTitle>
        <CardDescription>Customize the application appearance</CardDescription>
      </CardHeader>
      <CardContent class="space-y-4">
        <div class="space-y-2">
          <Label>Theme</Label>
          <div class="flex gap-2">
            <Button variant={theme === "light" ? "default" : "outline"} class="flex-1" onclick={() => setTheme("light")}>
              {#if theme === "light"}<Check class="h-4 w-4 mr-1" />{/if}Light
            </Button>
            <Button variant={theme === "dark" ? "default" : "outline"} class="flex-1" onclick={() => setTheme("dark")}>
              {#if theme === "dark"}<Check class="h-4 w-4 mr-1" />{/if}Dark
            </Button>
            <Button variant={theme === "system" ? "default" : "outline"} class="flex-1" onclick={() => setTheme("system")}>
              {#if theme === "system"}<Check class="h-4 w-4 mr-1" />{/if}System
            </Button>
          </div>
        </div>
        <div class="space-y-2">
          <Label>Sidebar</Label>
          <div class="flex gap-2">
            <Button variant={$sidebar.isOpen ? "default" : "outline"} class="flex-1" onclick={() => sidebar.open()}>
              Expanded
            </Button>
            <Button variant={!$sidebar.isOpen ? "default" : "outline"} class="flex-1" onclick={() => sidebar.close()}>
              Collapsed
            </Button>
          </div>
        </div>
      </CardContent>
    </Card>
  </div>
</PageLayout>
