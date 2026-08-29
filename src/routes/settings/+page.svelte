<script lang="ts">
  import { auth, userName, userRole } from "$lib/stores/auth.js";
  import { sidebar } from "$lib/stores/sidebar.js";
  import { updateMyProfile, changePassword } from "$lib/lib/api.js";
  import PageLayout from "$lib/components/layout/PageLayout.svelte";
  import { Card, CardContent, CardHeader, CardTitle, CardDescription } from "$lib/components/ui/card/index.js";
  import Button from "$lib/components/ui/button/index.svelte";
  import Input from "$lib/components/ui/input/index.svelte";
  import Label from "$lib/components/ui/label/index.svelte";
  import Avatar from "$lib/components/ui/avatar/index.svelte";
  import AvatarFallback from "$lib/components/ui/avatar/avatar-fallback.svelte";
  import { User, Shield, Palette, Check } from "@lucide/svelte";
  import { getInitials } from "$lib/utils/index.js";
  import { profileUpdateSchema, passwordChangeSchema } from "$lib/lib/validation.js";
  import type { ProfileUpdateFormData, PasswordChangeFormData } from "$lib/lib/validation.js";

  let profileFirstName = $state($userName.split(" ")[0] || "");
  let profileLastName = $state($userName.split(" ").slice(1).join(" ") || "");
  let profileEmail = $state("");
  let currentPassword = $state("");
  let newPassword = $state("");
  let confirmPassword = $state("");
  let theme = $state("light");
  let savingProfile = $state(false);
  let savingPassword = $state(false);
  let profileErrors = $state<Record<string, string>>({});
  let passwordErrors = $state<Record<string, string>>({});

  function validateProfile(): boolean {
    profileErrors = {};
    const result = profileUpdateSchema.safeParse({
      first_name: profileFirstName || "",
      last_name: profileLastName || "",
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
      await updateMyProfile({
        first_name: profileFirstName || undefined,
        last_name: profileLastName || undefined,
        email: profileEmail || undefined,
      });
      const fullName = `${profileFirstName} ${profileLastName}`.trim();
      auth.update((user) => user ? { ...user, full_name: fullName } : null);
    } catch (e) {
      // Error handled by UI state
    } finally {
      savingProfile = false;
    }
  }

  async function updatePassword() {
    if (!validatePassword()) return;
    savingPassword = true;
    try {
      await changePassword(currentPassword, newPassword);
      currentPassword = "";
      newPassword = "";
      confirmPassword = "";
      passwordErrors = {};
    } catch (e: any) {
      // Error handled by UI state
    } finally {
      savingPassword = false;
    }
  }

  function setTheme(t: string) {
    theme = t;
    document.documentElement.classList.toggle("dark", t === "dark");
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
          <Label for="first-name">First Name</Label>
          <Input id="first-name" bind:value={profileFirstName} />
          {#if profileErrors.first_name}
            <p class="text-sm text-destructive">{profileErrors.first_name}</p>
          {/if}
        </div>
        <div class="space-y-2">
          <Label for="last-name">Last Name</Label>
          <Input id="last-name" bind:value={profileLastName} />
          {#if profileErrors.last_name}
            <p class="text-sm text-destructive">{profileErrors.last_name}</p>
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
