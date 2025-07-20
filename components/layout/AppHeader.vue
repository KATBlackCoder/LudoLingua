<template>
  <header class="border-b border-gray-200 dark:border-gray-800">
    <UContainer class="py-4">
      <div class="flex justify-between items-center">
        <h1 class="text-2xl font-bold">LudoLingua</h1>
        <nav>
          <UButtonGroup>
            <UButton to="/" variant="ghost" :active="$route.path === '/'">
              <template #leading>
                <UIcon name="i-heroicons-home" />
              </template>
              Home
            </UButton>
            <UButton
              to="/glossary"
              variant="ghost"
              :active="$route.path === '/glossary'"
            >
              <template #leading>
                <UIcon name="i-heroicons-book-open" />
              </template>
              Glossary
            </UButton>
            <UButton
              to="/settings"
              variant="ghost"
              :active="$route.path === '/settings'"
            >
              <template #leading>
                <UIcon name="i-heroicons-cog-6-tooth" />
              </template>
              Settings
            </UButton>
            <UButton
              to="/about"
              variant="ghost"
              :active="$route.path === '/about'"
            >
              <template #leading>
                <UIcon name="i-heroicons-information-circle" />
              </template>
              About
            </UButton>
          </UButtonGroup>
        </nav>
      </div>
    </UContainer>
  </header>
</template>

<script setup lang="ts">
// Header component logic can go here
</script>
