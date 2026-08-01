<script lang="ts">
  import { auth, userName, userRole } from "$lib/stores/auth.js";
  import { sidebar } from "$lib/stores/sidebar.js";
  import { notifications } from "$lib/stores/notifications.js";
  import { updateMyProfile, changePassword } from "$lib/lib/api.js";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "$lib/components/ui/card/index.js";
  import Button from "$lib/components/ui/button/index.svelte";
  import Input from "$lib/components/ui/input/index.svelte";
  import Label from "$lib/components/ui/label/index.svelte";
  import Avatar from "$lib/components/ui/avatar/index.svelte";
  import AvatarFallback from "$lib/components/ui/avatar/avatar-fallback.svelte";
  import { User, Bell, Shield, Palette, Check } from "@lucide/svelte";
  import { getInitials } from "$lib/utils/index.js";
  import { profileUpdateSchema, passwordChangeSchema } from "$lib/lib/validation.js";
  import type { ProfileUpdateFormData, PasswordChangeFormData } from "$lib/lib/validation.js";

  let profileName = $state($userName);
  let profileEmail = $state("");
  let currentPassword = $state("");
  let newPassword = $state("");
  let confirmPassword = $state("");
  let theme = $state("light");
  let emailNotifications = $state(true);
  let lowStockAlerts = $state(true);
  let appointmentReminders = $state(false);
  let savingProfile = $state(false);
  let savingPassword = $state(false);
  let profileErrors = $state<Record<string, string>>({});
  let passwordErrors = $state<Record<string, string>>({});

  function validateProfile(): boolean {
    profileErrors = {};
    const result = profileUpdateSchema.safeParse({
      first_name: profileName.split(" ")[0] || "",
      last_name: profileName.split(" ").slice(1).join(" ") || "",
      email: profileEmail,
    });
    if (!result.success) {
      for (const issue of result.error.issues) {
        const field = String(issue.path[0]);
        profileErrors[field] = issue.message;
      }
      return false;
    }
    return true;
  }

  function validatePassword(): boolean {
    passwordErrors = {};
    const result = passwordChangeSchema.safeParse({
      current_password: currentPassword,
      new_password: newPassword,
      confirm_password: confirmPassword,
    });
    if (!result.success) {
      for (const issue of result.error.issues) {
        const field = String(issue.path[0]);
        passwordErrors[field] = issue.message;
      }
      return false;
    }
    return true;
  }

  async function saveProfile() {
    if (!validateProfile()) return;
    savingProfile = true;
    try {
      const nameParts = profileName.split(" ");
      await updateMyProfile({
        first_name: nameParts[0] || undefined,
        last_name: nameParts.slice(1).join(" ") || undefined,
        email: profileEmail || undefined,
      });
      auth.update((user) => user ? { ...user, full_name: profileName } : null);
      notifications.add({ type: "success", title: "Profile Updated", message: "Your profile has been saved" });
    } catch (e) {
      notifications.add({ type: "error", title: "Error", message: "Failed to update profile" });
    } finally {
      savingProfile = false;
    }
  }

  async function updatePassword() {
    if (!validatePassword()) return;
    savingPassword = true;
    try {
      await changePassword(currentPassword, newPassword);
      notifications.add({ type: "success", title: "Password Updated", message: "Your password has been changed" });
      currentPassword = "";
      newPassword = "";
      confirmPassword = "";
      passwordErrors = {};
    } catch (e: any) {
      const message = e?.message || "Failed to update password";
      notifications.add({ type: "error", title: "Error", message });
    } finally {
      savingPassword = false;
    }
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
          {#if profileErrors.first_name}
            <p class="text-sm text-destructive">{profileErrors.first_name}</p>
          {/if}
        </div>
        <div class="space-y-2">
          <Label for="email">Email</Label>
          <Input id="email" type="email" placeholder="your@email.com" bind:value={profileEmail} />
          {#if profileErrors.email}
            <p class="text-sm text-destructive">{profileErrors.email}</p>
          {/if}
        </div>
        <Button onclick={saveProfile} disabled={savingProfile}>
          {#if savingProfile}
            <div class="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent mr-2"></div>
            Saving...
          {:else}
            Save Changes
          {/if}
        </Button>
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
          {#if passwordErrors.current_password}
            <p class="text-sm text-destructive">{passwordErrors.current_password}</p>
          {/if}
        </div>
        <div class="space-y-2">
          <Label for="new-password">New Password</Label>
          <Input id="new-password" type="password" bind:value={newPassword} />
          {#if passwordErrors.new_password}
            <p class="text-sm text-destructive">{passwordErrors.new_password}</p>
          {/if}
        </div>
        <div class="space-y-2">
          <Label for="confirm-password">Confirm Password</Label>
          <Input id="confirm-password" type="password" bind:value={confirmPassword} />
          {#if passwordErrors.confirm_password}
            <p class="text-sm text-destructive">{passwordErrors.confirm_password}</p>
          {/if}
        </div>
        <Button onclick={updatePassword} disabled={savingPassword}>
          {#if savingPassword}
            <div class="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent mr-2"></div>
            Updating...
          {:else}
            Update Password
          {/if}
        </Button>
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
