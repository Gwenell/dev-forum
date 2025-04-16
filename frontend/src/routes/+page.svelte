<script lang="ts">
  import { onMount } from 'svelte';
  import { api } from '$lib/api';
  
  // Store categories data
  let categories = [];
  let loading = true;
  let error = null;
  
  // Load categories on mount
  onMount(async () => {
    try {
      categories = await api.categories.getAll();
      loading = false;
    } catch (err) {
      console.error('Failed to load categories:', err);
      error = 'Failed to load categories. Please try again later.';
      loading = false;
    }
  });
</script>

<svelte:head>
  <title>Developer Forum - Home</title>
  <meta name="description" content="A community forum for developers to discuss programming languages, tools, and more." />
</svelte:head>

<section class="hero bg-gradient-to-r from-blue-500 to-indigo-600 text-white rounded-lg p-8 mb-8">
  <div class="max-w-3xl">
    <h1 class="text-4xl font-bold mb-4">Welcome to Developer Forum</h1>
    <p class="text-xl mb-6">A community for developers to discuss programming languages, tools, share code, and more.</p>
    <div class="flex space-x-4">
      <a href="/categories" class="bg-white text-blue-600 hover:bg-gray-100 px-6 py-2 rounded-md font-medium">
        Browse Categories
      </a>
      <a href="/register" class="bg-transparent hover:bg-blue-700 border border-white px-6 py-2 rounded-md font-medium">
        Join the Community
      </a>
    </div>
  </div>
</section>

<section class="mb-8">
  <h2 class="text-2xl font-bold mb-4">Forum Categories</h2>
  
  {#if loading}
    <div class="flex justify-center p-8">
      <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"></div>
    </div>
  {:else if error}
    <div class="bg-red-100 border border-red-400 text-red-700 px-4 py-3 rounded">
      <p>{error}</p>
      <button 
        class="underline mt-2" 
        on:click={() => {
          loading = true;
          error = null;
          api.categories.getAll()
            .then(data => {
              categories = data;
              loading = false;
            })
            .catch(err => {
              console.error('Failed to load categories:', err);
              error = 'Failed to load categories. Please try again later.';
              loading = false;
            });
        }}
      >
        Try again
      </button>
    </div>
  {:else if categories.length === 0}
    <div class="text-center p-8 bg-gray-100 dark:bg-gray-800 rounded-lg">
      <p class="text-lg text-gray-600 dark:text-gray-400">No categories found.</p>
    </div>
  {:else}
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
      {#each categories as category}
        <div class="bg-white dark:bg-gray-800 rounded-lg shadow-md overflow-hidden hover:shadow-lg transition-shadow">
          <div class="p-6">
            <h3 class="text-xl font-bold mb-2">
              <a href="/category/{category.slug}" class="hover:text-blue-500">{category.name}</a>
            </h3>
            <p class="text-gray-600 dark:text-gray-400 mb-4">{category.description}</p>
            
            <div class="text-sm text-gray-500 dark:text-gray-400">
              {#if category.subcategories && category.subcategories.length > 0}
                <div class="flex flex-wrap gap-2 mt-4">
                  {#each category.subcategories.slice(0, 5) as subcategory}
                    <a 
                      href="/category/{category.slug}/{subcategory.slug}" 
                      class="bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600"
                    >
                      {subcategory.name}
                    </a>
                  {/each}
                  {#if category.subcategories.length > 5}
                    <span class="px-2 py-1 text-gray-500 dark:text-gray-400">+{category.subcategories.length - 5} more</span>
                  {/if}
                </div>
              {:else}
                <p>No subcategories</p>
              {/if}
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</section>

<section class="mb-8">
  <h2 class="text-2xl font-bold mb-4">Features</h2>
  
  <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
    <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md">
      <div class="text-blue-500 mb-4">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
        </svg>
      </div>
      <h3 class="text-xl font-bold mb-2">Real-time Chat</h3>
      <p class="text-gray-600 dark:text-gray-400">
        Connect with other developers through our real-time chat functionality. Share ideas, ask questions, and collaborate on projects.
      </p>
    </div>
    
    <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md">
      <div class="text-blue-500 mb-4">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
        </svg>
      </div>
      <h3 class="text-xl font-bold mb-2">Code Sharing</h3>
      <p class="text-gray-600 dark:text-gray-400">
        Share code snippets with syntax highlighting. Get feedback and help from experienced developers in our community.
      </p>
    </div>
    
    <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md">
      <div class="text-blue-500 mb-4">
        <svg xmlns="http://www.w3.org/2000/svg" class="h-10 w-10" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 10l4.553-2.276A1 1 0 0121 8.618v6.764a1 1 0 01-1.447.894L15 14M5 18h8a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v8a2 2 0 002 2z" />
        </svg>
      </div>
      <h3 class="text-xl font-bold mb-2">Live Screen Sharing</h3>
      <p class="text-gray-600 dark:text-gray-400">
        Host live coding sessions with screen sharing capabilities. Teach, learn, and collaborate in real-time.
      </p>
    </div>
  </div>
</section>

<section>
  <h2 class="text-2xl font-bold mb-4">Join Our Community</h2>
  
  <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md">
    <p class="text-lg mb-4">
      Ready to join a community of passionate developers? Register today to participate in discussions, share your knowledge, and connect with like-minded professionals.
    </p>
    
    <div class="flex space-x-4">
      <a href="/register" class="bg-blue-500 hover:bg-blue-600 text-white px-6 py-2 rounded-md font-medium">
        Register Now
      </a>
      <a href="/login" class="bg-gray-200 dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600 px-6 py-2 rounded-md font-medium">
        Login
      </a>
    </div>
  </div>
</section>
