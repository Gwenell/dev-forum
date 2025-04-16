<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { authStore } from '$lib/stores/auth';
  
  let email = '';
  let password = '';
  let rememberMe = false;
  
  let loading = false;
  let error = '';
  
  async function handleSubmit() {
    if (!email || !password) {
      error = 'Please enter both email and password';
      return;
    }
    
    loading = true;
    error = '';
    
    try {
      // Use the auth store login method which handles token setting and user fetching
      const success = await authStore.login(email, password);
      
      if (success) {
        // Redirect to home page on success
        goto('/');
      } else {
        error = $authStore.error || 'Login failed. Please try again.';
      }
    } catch (err) {
      console.error('Login failed:', err);
      error = 'An unexpected error occurred. Please try again.';
    } finally {
      loading = false;
    }
  }
  
  // Check if user is already logged in
  onMount(() => {
    if ($authStore.isAuthenticated) {
      goto('/');
    }
  });
</script>

<svelte:head>
  <title>Login - Developer Forum</title>
  <meta name="description" content="Log in to your Developer Forum account." />
</svelte:head>

<div class="max-w-md mx-auto my-12 p-6 bg-white dark:bg-gray-800 rounded-lg shadow-md">
  <h1 class="text-2xl font-bold mb-6 text-center">Log In</h1>
  
  {#if error}
    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      <p>{error}</p>
    </div>
  {/if}
  
  <form on:submit|preventDefault={handleSubmit}>
    <div class="mb-4">
      <label for="email" class="block text-sm font-medium mb-1">Email</label>
      <input 
        type="email"
        id="email"
        bind:value={email}
        class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600"
        placeholder="Enter your email"
      />
    </div>
    
    <div class="mb-4">
      <label for="password" class="block text-sm font-medium mb-1">Password</label>
      <input 
        type="password"
        id="password"
        bind:value={password}
        class="w-full px-3 py-2 border rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:border-gray-600"
        placeholder="Enter your password"
      />
    </div>
    
    <div class="flex items-center justify-between mb-6">
      <div class="flex items-center">
        <input 
          type="checkbox"
          id="rememberMe"
          bind:checked={rememberMe}
          class="h-4 w-4 rounded border-gray-300 text-blue-600 focus:ring-blue-500"
        />
        <label for="rememberMe" class="ml-2 block text-sm text-gray-700 dark:text-gray-300">
          Remember me
        </label>
      </div>
      
      <div class="text-sm">
        <a href="/forgot-password" class="text-blue-500 hover:underline">Forgot password?</a>
      </div>
    </div>
    
    <button 
      type="submit"
      class="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-md font-medium disabled:opacity-50 disabled:cursor-not-allowed"
      disabled={loading}
    >
      {#if loading}
        <span class="inline-block animate-spin mr-2">↻</span>
        Logging in...
      {:else}
        Log In
      {/if}
    </button>
  </form>
  
  <p class="mt-4 text-center text-sm">
    Don't have an account? <a href="/register" class="text-blue-500 hover:underline">Register</a>
  </p>
  
  <div class="mt-6 pt-6 border-t border-gray-200 dark:border-gray-700">
    <p class="text-center text-sm font-medium mb-3">Or continue with</p>
    <div class="flex space-x-4 justify-center">
      <button class="flex-1 flex items-center justify-center py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium hover:bg-gray-50 dark:hover:bg-gray-700">
        <svg class="h-5 w-5 mr-2" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12.545,10.239v3.821h5.445c-0.712,2.315-2.647,3.972-5.445,3.972c-3.332,0-6.033-2.701-6.033-6.032s2.701-6.032,6.033-6.032c1.498,0,2.866,0.549,3.921,1.453l2.814-2.814C17.503,2.988,15.139,2,12.545,2C7.021,2,2.543,6.477,2.543,12s4.478,10,10.002,10c8.396,0,10.249-7.85,9.426-11.748L12.545,10.239z"/>
        </svg>
        Google
      </button>
      <button class="flex-1 flex items-center justify-center py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium hover:bg-gray-50 dark:hover:bg-gray-700">
        <svg class="h-5 w-5 mr-2" fill="currentColor" viewBox="0 0 24 24">
          <path d="M13.397 20.997v-8.196h2.765l.411-3.209h-3.176V7.548c0-.926.258-1.56 1.587-1.56h1.684V3.127A22.336 22.336 0 0014.201 3c-2.444 0-4.122 1.492-4.122 4.231v2.355H7.332v3.209h2.753v8.202h3.312z"/>
        </svg>
        GitHub
      </button>
    </div>
  </div>
</div> 