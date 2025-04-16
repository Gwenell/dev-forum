<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { api } from '$lib/api';
  import { authStore } from '$lib/stores/auth';
  
  let formData = {
    username: '',
    email: '',
    password: '',
    confirmPassword: ''
  };
  
  let loading = false;
  let error = '';
  let fieldErrors = {
    username: '',
    email: '',
    password: '',
    confirmPassword: ''
  };

  // Track focus states
  let focusedField = '';
  
  // Form validation
  function validateForm() {
    let isValid = true;
    
    // Reset errors
    fieldErrors = {
      username: '',
      email: '',
      password: '',
      confirmPassword: ''
    };
    
    // Username validation
    if (!formData.username.trim()) {
      fieldErrors.username = 'Username is required';
      isValid = false;
    } else if (formData.username.length < 3) {
      fieldErrors.username = 'Username must be at least 3 characters';
      isValid = false;
    }
    
    // Email validation
    if (!formData.email.trim()) {
      fieldErrors.email = 'Email is required';
      isValid = false;
    } else if (!/\S+@\S+\.\S+/.test(formData.email)) {
      fieldErrors.email = 'Please enter a valid email address';
      isValid = false;
    }
    
    // Password validation
    if (!formData.password) {
      fieldErrors.password = 'Password is required';
      isValid = false;
    } else if (formData.password.length < 8) {
      fieldErrors.password = 'Password must be at least 8 characters';
      isValid = false;
    }
    
    // Confirm password validation
    if (formData.password !== formData.confirmPassword) {
      fieldErrors.confirmPassword = 'Passwords do not match';
      isValid = false;
    }
    
    return isValid;
  }
  
  // Form submission
  async function handleSubmit() {
    if (!validateForm()) return;
    
    loading = true;
    error = '';
    
    try {
      const { username, email, password } = formData;
      // Use the authStore register method which handles token setting and user fetching
      const success = await authStore.register(username, email, password);
      
      if (success) {
        // Redirect to home page on success
        goto('/');
      } else {
        error = $authStore.error || 'Registration failed. Please try again.';
      }
    } catch (err) {
      console.error('Registration failed:', err);
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
  <title>Register - Developer Forum</title>
  <meta name="description" content="Join the Developer Forum community and connect with fellow developers." />
</svelte:head>

<div class="max-w-md mx-auto my-12 p-6 bg-white dark:bg-gray-800 rounded-lg shadow-md">
  <h1 class="text-2xl font-bold mb-6 text-center">Create an Account</h1>
  
  {#if error}
    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded mb-4">
      <p>{error}</p>
    </div>
  {/if}
  
  <form on:submit|preventDefault={handleSubmit} class="space-y-5">
    <div class="relative">
      <label for="username" class="block text-sm font-medium mb-1">Username</label>
      <div class="relative">
        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
          </svg>
        </div>
        <input 
          type="text"
          id="username"
          bind:value={formData.username}
          on:focus={() => focusedField = 'username'}
          on:blur={() => focusedField = ''}
          class="w-full pl-10 px-3 py-2 border rounded-md 
                 focus:outline-none focus:ring-2 focus:ring-blue-500 
                 dark:bg-gray-700 dark:border-gray-600
                 {fieldErrors.username ? 'border-red-500' : 'border-gray-300'}
                 {focusedField === 'username' ? 'ring-2 ring-blue-500' : ''}"
          placeholder="Choose a username"
          autocomplete="username"
        />
        {#if fieldErrors.username}
          <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
            <svg class="h-5 w-5 text-red-500" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"></path>
            </svg>
          </div>
        {/if}
      </div>
      {#if fieldErrors.username}
        <p class="text-red-500 text-sm mt-1">{fieldErrors.username}</p>
      {/if}
    </div>
    
    <div class="relative">
      <label for="email" class="block text-sm font-medium mb-1">Email</label>
      <div class="relative">
        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
          </svg>
        </div>
        <input 
          type="email"
          id="email"
          bind:value={formData.email}
          on:focus={() => focusedField = 'email'}
          on:blur={() => focusedField = ''}
          class="w-full pl-10 px-3 py-2 border rounded-md 
                 focus:outline-none focus:ring-2 focus:ring-blue-500 
                 dark:bg-gray-700 dark:border-gray-600
                 {fieldErrors.email ? 'border-red-500' : 'border-gray-300'}
                 {focusedField === 'email' ? 'ring-2 ring-blue-500' : ''}"
          placeholder="Enter your email"
          autocomplete="email"
        />
        {#if fieldErrors.email}
          <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
            <svg class="h-5 w-5 text-red-500" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"></path>
            </svg>
          </div>
        {/if}
      </div>
      {#if fieldErrors.email}
        <p class="text-red-500 text-sm mt-1">{fieldErrors.email}</p>
      {/if}
    </div>
    
    <div class="relative">
      <label for="password" class="block text-sm font-medium mb-1">Password</label>
      <div class="relative">
        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
          </svg>
        </div>
        <input 
          type="password"
          id="password"
          bind:value={formData.password}
          on:focus={() => focusedField = 'password'}
          on:blur={() => focusedField = ''}
          class="w-full pl-10 px-3 py-2 border rounded-md 
                 focus:outline-none focus:ring-2 focus:ring-blue-500
                 dark:bg-gray-700 dark:border-gray-600
                 {fieldErrors.password ? 'border-red-500' : 'border-gray-300'}
                 {focusedField === 'password' ? 'ring-2 ring-blue-500' : ''}"
          placeholder="Create a password"
          autocomplete="new-password"
        />
        {#if fieldErrors.password}
          <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
            <svg class="h-5 w-5 text-red-500" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"></path>
            </svg>
          </div>
        {/if}
      </div>
      {#if fieldErrors.password}
        <p class="text-red-500 text-sm mt-1">{fieldErrors.password}</p>
      {/if}
    </div>
    
    <div class="relative">
      <label for="confirmPassword" class="block text-sm font-medium mb-1">Confirm Password</label>
      <div class="relative">
        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
          </svg>
        </div>
        <input 
          type="password"
          id="confirmPassword"
          bind:value={formData.confirmPassword}
          on:focus={() => focusedField = 'confirmPassword'}
          on:blur={() => focusedField = ''}
          class="w-full pl-10 px-3 py-2 border rounded-md 
                 focus:outline-none focus:ring-2 focus:ring-blue-500
                 dark:bg-gray-700 dark:border-gray-600
                 {fieldErrors.confirmPassword ? 'border-red-500' : 'border-gray-300'}
                 {focusedField === 'confirmPassword' ? 'ring-2 ring-blue-500' : ''}"
          placeholder="Confirm your password"
          autocomplete="new-password"
        />
        {#if fieldErrors.confirmPassword}
          <div class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none">
            <svg class="h-5 w-5 text-red-500" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"></path>
            </svg>
          </div>
        {/if}
      </div>
      {#if fieldErrors.confirmPassword}
        <p class="text-red-500 text-sm mt-1">{fieldErrors.confirmPassword}</p>
      {/if}
    </div>
    
    <button 
      type="submit"
      class="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded-md font-medium transition-all
             disabled:opacity-50 disabled:cursor-not-allowed shadow-sm hover:shadow-md"
      disabled={loading}
    >
      {#if loading}
        <span class="inline-block animate-spin mr-2">↻</span>
        Creating Account...
      {:else}
        Register
      {/if}
    </button>
  </form>
  
  <p class="mt-4 text-center text-sm">
    Already have an account? <a href="/login" class="text-blue-500 hover:underline">Log in</a>
  </p>
  
  <div class="mt-6 pt-6 border-t border-gray-200 dark:border-gray-700">
    <p class="text-center text-sm font-medium mb-3">Or sign up with</p>
    <div class="flex space-x-4 justify-center">
      <button class="flex-1 flex items-center justify-center py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium 
                     hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
        <svg class="h-5 w-5 mr-2" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12.545,10.239v3.821h5.445c-0.712,2.315-2.647,3.972-5.445,3.972c-3.332,0-6.033-2.701-6.033-6.032s2.701-6.032,6.033-6.032c1.498,0,2.866,0.549,3.921,1.453l2.814-2.814C17.503,2.988,15.139,2,12.545,2C7.021,2,2.543,6.477,2.543,12s4.478,10,10.002,10c8.396,0,10.249-7.85,9.426-11.748L12.545,10.239z"/>
        </svg>
        Google
      </button>
      <button class="flex-1 flex items-center justify-center py-2 px-4 border border-gray-300 rounded-md shadow-sm text-sm font-medium 
                     hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors">
        <svg class="h-5 w-5 mr-2" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
        </svg>
        GitHub
      </button>
    </div>
  </div>
</div> 