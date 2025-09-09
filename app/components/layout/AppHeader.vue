<template>
  <header class="border-b border-gray-200 dark:border-gray-800 sticky top-0 z-50 bg-white/80 dark:bg-gray-900/80 backdrop-blur">
    <UContainer class="py-3">
      <div class="flex justify-between items-center gap-3">
        <div class="flex items-center gap-2">
          <UIcon name="i-heroicons-language" class="w-6 h-6" />
          <h1 class="text-xl font-semibold">LudoLingua</h1>
        </div>
        <nav>
          <UButtonGroup>
            <UButton to="/" variant="ghost" :active="$route.path === '/'" icon="i-heroicons-home">Home</UButton>
            <UButton to="/translator" variant="ghost" :active="$route.path === '/translator'" icon="i-heroicons-language">Translator</UButton>
            <UButton to="/settings" variant="ghost" :active="$route.path === '/settings'" icon="i-heroicons-cog-6-tooth">Settings</UButton>
            <UButton to="/glossary" variant="ghost" :active="$route.path === '/glossary'" icon="i-heroicons-book-open">Glossary</UButton>
            <UButton to="/about" variant="ghost" :active="$route.path === '/about'" icon="i-heroicons-information-circle">About</UButton>
          </UButtonGroup>
        </nav>
      </div>
    </UContainer>
  </header>
</template>

<script setup lang="ts">
// Header component logic can go here
</script>
