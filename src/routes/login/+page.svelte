<script lang="ts">
  import { goto } from "$app/navigation";
  import { auth } from "$lib/stores/auth.js";
  import { notifications } from "$lib/stores/notifications.js";
  import { login } from "$lib/lib/api.js";
  import Button from "$lib/components/ui/button/index.svelte";
  import Input from "$lib/components/ui/input/index.svelte";
  import Label from "$lib/components/ui/label/index.svelte";
  import Card from "$lib/components/ui/card/index.svelte";
  import CardHeader from "$lib/components/ui/card/card-header.svelte";
  import CardTitle from "$lib/components/ui/card/card-title.svelte";
  import CardDescription from "$lib/components/ui/card/card-description.svelte";
  import CardContent from "$lib/components/ui/card/card-content.svelte";
  import { Activity } from "@lucide/svelte";

  let username = $state("");
  let password = $state("");
  let loading = $state(false);
  let error = $state("");

  async function handleLogin() {
    if (!username || !password) {
      error = "Please enter username and password";
      return;
    }

    loading = true;
    error = "";

    try {
      const result = await login(username, password);
      if (result.success && result.user) {
        auth.login(result.user);
        notifications.add({
          type: "success",
          title: "Welcome back!",
          message: `Logged in as ${result.user.full_name || result.user.username}`,
        });
        goto("/dashboard");
      } else {
        error = result.message;
      }
    } catch (e) {
      error = "Login failed. Please try again.";
    } finally {
      loading = false;
    }
  }
</script>

<div class="flex min-h-screen items-center justify-center bg-gradient-to-br from-primary/5 via-background to-primary/10 p-4">
  <Card class="w-full max-w-md">
    <CardHeader class="space-y-1 text-center">
      <div class="mx-auto mb-4 flex h-12 w-12 items-center justify-center rounded-xl bg-primary">
        <Activity class="h-6 w-6 text-primary-foreground" />
      </div>
      <CardTitle class="text-2xl font-bold">Hospital Management System</CardTitle>
      <CardDescription>Sign in to your account</CardDescription>
    </CardHeader>
    <CardContent>
      <form
        class="space-y-4"
        onsubmit={(e) => {
          e.preventDefault();
          handleLogin();
        }}
      >
        {#if error}
          <div class="rounded-lg bg-destructive/10 p-3 text-sm text-destructive">
            {error}
          </div>
        {/if}

        <div class="space-y-2">
          <Label for="username">Username</Label>
          <Input
            id="username"
            type="text"
            placeholder="Enter your username"
            bind:value={username}
            disabled={loading}
          />
        </div>

        <div class="space-y-2">
          <Label for="password">Password</Label>
          <Input
            id="password"
            type="password"
            placeholder="Enter your password"
            bind:value={password}
            disabled={loading}
          />
        </div>

        <Button type="submit" class="w-full" disabled={loading}>
          {#if loading}
            <div class="h-4 w-4 animate-spin rounded-full border-2 border-current border-t-transparent"></div>
            Signing in...
          {:else}
            Sign in
          {/if}
        </Button>


      </form>
    </CardContent>
  </Card>
</div>
