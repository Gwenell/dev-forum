<script lang="ts">
	import { onMount } from 'svelte';
	import { authStore } from '$lib/stores/auth';
	import { themeStore } from '$lib/stores/theme';
	import '../app.css';
	
	// Initialize auth and theme stores on mount
	onMount(() => {
		authStore.initialize();
		themeStore.initialize();
	});
</script>

<div class="app" data-theme={$themeStore.currentTheme}>
	<header>
		<div class="container mx-auto px-4 py-2 flex justify-between items-center">
			<a href="/" class="text-xl font-bold">Developer Forum</a>
			
			<div class="flex items-center space-x-4">
				<!-- Theme toggle -->
				<button 
					on:click={() => themeStore.toggle()} 
					class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700"
					aria-label="Toggle theme"
				>
					{#if $themeStore.currentTheme === 'dark'}
						<!-- Sun icon for light mode -->
						<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
						</svg>
					{:else}
						<!-- Moon icon for dark mode -->
						<svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
							<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
						</svg>
					{/if}
				</button>
				
				<!-- User menu -->
				{#if $authStore.isAuthenticated && $authStore.user}
					<div class="relative group">
						<button class="flex items-center space-x-1">
							<img 
								src={$authStore.user.avatar_url || '/default-avatar.png'} 
								alt="User avatar" 
								class="w-8 h-8 rounded-full"
							/>
							<span>{$authStore.user.display_name || $authStore.user.username}</span>
							<svg xmlns="http://www.w3.org/2000/svg" class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
								<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
							</svg>
						</button>
						
						<div class="absolute right-0 mt-2 w-48 bg-white dark:bg-gray-800 rounded-md shadow-lg py-1 hidden group-hover:block">
							<a href="/profile" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700">Profile</a>
							{#if $authStore.user.is_admin}
								<a href="/admin" class="block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700">Admin Panel</a>
							{/if}
							<button 
								on:click={() => authStore.logout()} 
								class="w-full text-left px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-700 text-red-600"
							>
								Logout
							</button>
						</div>
					</div>
				{:else}
					<div class="space-x-2">
						<a href="/login" class="px-4 py-2 rounded-md bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600">
							Login
						</a>
						<a href="/register" class="px-4 py-2 rounded-md bg-blue-500 text-white hover:bg-blue-600">
							Register
						</a>
					</div>
				{/if}
			</div>
		</div>
	</header>
	
	<main class="container mx-auto px-4 py-8">
		<slot />
	</main>
	
	<footer class="bg-gray-100 dark:bg-gray-800 mt-8">
		<div class="container mx-auto px-4 py-6">
			<div class="flex justify-between items-center">
				<div>
					<p class="text-sm text-gray-600 dark:text-gray-400">© 2025 Developer Forum. All rights reserved.</p>
				</div>
				<div>
					<a href="/about" class="text-sm text-gray-600 dark:text-gray-400 hover:underline">About</a>
					<a href="/terms" class="ml-4 text-sm text-gray-600 dark:text-gray-400 hover:underline">Terms</a>
					<a href="/privacy" class="ml-4 text-sm text-gray-600 dark:text-gray-400 hover:underline">Privacy</a>
				</div>
			</div>
		</div>
	</footer>
</div>

<style>
	:global(:root) {
		--color-primary: #F2F0E3;
		--color-secondary: #2E2E2E;
		--color-text: #2E2E2E;
		--color-textSecondary: #4A4A4A;
		--color-background: #FFFFFF;
		--color-card: #F8F8F8;
		--color-border: #E0E0E0;
		--color-accent: #4A87C7;
		--color-success: #4CAF50;
		--color-warning: #FF9800;
		--color-error: #E53935;
	}

	:global([data-theme='dark']) {
		--color-primary: #1F1F1F;
		--color-secondary: #D1CFC0;
		--color-text: #D1CFC0;
		--color-textSecondary: #A0A0A0;
		--color-background: #121212;
		--color-card: #2A2A2A;
		--color-border: #3A3A3A;
		--color-accent: #5D93CC;
		--color-success: #43A047;
		--color-warning: #FB8C00;
		--color-error: #E53935;
	}

	:global(html, body) {
		color: var(--color-text);
		background-color: var(--color-background);
	}

	.app {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
	}

	main {
		flex: 1;
	}

	header {
		background-color: var(--color-primary);
		color: var(--color-secondary);
		border-bottom: 1px solid var(--color-border);
	}

	footer {
		background-color: var(--color-primary);
		color: var(--color-secondary);
		border-top: 1px solid var(--color-border);
	}
</style>
