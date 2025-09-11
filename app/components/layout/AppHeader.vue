<template>
  <header class="border-b border-gray-200 dark:border-gray-800 sticky top-0 z-50 bg-white/80 dark:bg-gray-900/80 backdrop-blur">
    <UContainer class="py-3">
      <div class="flex justify-between items-center gap-3">
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-languages" class="w-6 h-6" />
          <h1 class="text-xl font-semibold">LudoLingua</h1>
        </div>
        <nav>
          <UFieldGroup>
            <UButton to="/" variant="ghost" :active="$route.path === '/'" icon="i-lucide-home">Home</UButton>
            <UButton 
              v-if="engineStore.hasProject && !engineStore.isLoading"
              to="/translator" 
              variant="ghost" 
              :active="$route.path === '/translator'" 
              icon="i-lucide-languages"
            >
              Translator
            </UButton>
            <UButton to="/translations" variant="ghost" :active="$route.path === '/translations'" icon="i-lucide-list">Translations</UButton>
            <UButton to="/settings" variant="ghost" :active="$route.path === '/settings'" icon="i-lucide-settings">Settings</UButton>
            <UButton to="/glossary" variant="ghost" :active="$route.path === '/glossary'" icon="i-lucide-book-open">Glossary</UButton>
            <UButton to="/about" variant="ghost" :active="$route.path === '/about'" icon="i-lucide-info">About</UButton>
          </UFieldGroup>
        </nav>
      </div>
    </UContainer>
  </header>
</template>

<script setup lang="ts">
import { useEngineStore } from '~/stores/engine'

const engineStore = useEngineStore()
</script>
