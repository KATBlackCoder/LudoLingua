<template>
  <UHeader>
    <template #title>
      <div class="flex items-center gap-2">
        <UIcon name="i-lucide-languages" class="w-6 h-6" />
        <span>LudoLingua</span>
      </div>
    </template>

    <UNavigationMenu :items="navigationItems" />

    <template #right>
      <UColorModeButton />
    </template>
  </UHeader>
</template>

<script setup lang="ts">
import { useEngineStore } from '~/stores/engine'
import type { NavigationMenuItem } from '@nuxt/ui'

const engineStore = useEngineStore()
const route = useRoute()

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
