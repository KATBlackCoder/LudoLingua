<template>
  <div class="min-h-screen flex flex-col bg-gray-50 dark:bg-gray-900">
    <header class="bg-white dark:bg-gray-800 shadow">
      <div class="container mx-auto px-4 py-4">
        <div class="flex justify-between items-center">
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">LudoLingua</h1>
          <nav>
            <ul class="flex space-x-4">
              <li>
                <NuxtLink to="/" class="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white">
                  Home
                </NuxtLink>
              </li>
              <li>
                <NuxtLink to="/glossary" class="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white">
                  Glossary
                </NuxtLink>
              </li>
              <li>
                <NuxtLink to="/settings" class="text-gray-600 hover:text-gray-900 dark:text-gray-300 dark:hover:text-white">
                  Settings
                </NuxtLink>
              </li>
            </ul>
          </nav>
        </div>
      </div>
    </header>
    
    <main class="container mx-auto px-4 py-6 flex-grow">
      <!-- Use slot to render the page content -->
      <slot />
    </main>
    
    <footer class="bg-white dark:bg-gray-800 shadow">
      <div class="container mx-auto px-4 py-4 text-center text-gray-600 dark:text-gray-400">
        <p>LudoLingua v0.1.0</p>
      </div>
    </footer>
  </div>
</template>

<script setup lang="ts">
// Layout setup code can go here
</script> 