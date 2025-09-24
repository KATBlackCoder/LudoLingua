<template>
  <UHeader>
    <template #title>
      <div class="flex items-center gap-3">
        <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
          <UIcon name="i-lucide-languages" class="text-primary w-5 h-5" />
        </div>
        <div>
          <h1 class="text-lg font-bold text-gray-900 dark:text-white">LudoLingua</h1>
          <p class="text-xs text-gray-500 dark:text-gray-400">RPG Translation Tool</p>
        </div>
      </div>
    </template>

    <UNavigationMenu :items="navigationItems" />

    <template #right>
      <div class="flex items-center gap-2">
        <UColorModeButton />
        <UButton
          v-if="engineStore.hasProject"
          color="primary"
          variant="soft"
          size="sm"
          icon="i-lucide-folder-open"
          @click="goToTranslator"
        >
          Open Workspace
        </UButton>
      </div>
    </template>
  </UHeader>
</template>

<script setup lang="ts">
import { useEngineStore } from '~/stores/engine'
import type { NavigationMenuItem } from '@nuxt/ui'

const engineStore = useEngineStore()
const route = useRoute()

const goToTranslator = () => {
  navigateTo('/translator')
}

const navigationItems = computed<NavigationMenuItem[]>(() => [
  {
    label: 'Home',
    to: '/',
    icon: 'i-lucide-home',
    active: route.path === '/'
  },
  ...(engineStore.hasProject && !engineStore.isLoading ? [{
    label: 'Translator',
    to: '/translator',
    icon: 'i-lucide-languages',
    active: route.path === '/translator'
  }] : []),
  {
    label: 'Translations',
    to: '/translations',
    icon: 'i-lucide-list',
    active: route.path === '/translations'
  },
  {
    label: 'Glossary',
    to: '/glossary',
    icon: 'i-lucide-book-open',
    active: route.path === '/glossary'
  },
  {
    label: 'Settings',
    to: '/settings',
    icon: 'i-lucide-settings',
    active: route.path === '/settings'
  },
  {
    label: 'About',
    to: '/about',
    icon: 'i-lucide-info',
    active: route.path === '/about'
  }
])
</script>
