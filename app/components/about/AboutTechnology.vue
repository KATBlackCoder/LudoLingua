<template>
  <section>
    <div class="flex items-center gap-3 mb-4">
      <div class="flex items-center gap-2">
        <UIcon name="i-lucide-cpu" class="text-primary" />
        <h3 class="text-xl font-semibold">Technology Stack</h3>
      </div>
      <UBadge color="primary" variant="soft" size="sm">
        <UIcon name="i-lucide-code" class="mr-1" />
        Modern
      </UBadge>
    </div>

    <UCard class="mb-6">
      <template #header>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-layers" class="text-primary" />
          <span class="font-medium">Core Technologies</span>
        </div>
      </template>
      <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-3">
        <div v-for="tech in technologies" :key="tech.name" class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
          <UIcon :name="tech.icon" class="w-6 h-6 mx-auto mb-2 text-primary" />
          <div class="text-sm font-medium">{{ tech.name }}</div>
          <div class="text-xs text-gray-500 dark:text-gray-400">{{ tech.description }}</div>
        </div>
      </div>
    </UCard>

    <UAlert
      color="warning"
      variant="soft"
      icon="i-lucide-triangle-alert"
      class="mb-6"
    >
      <template #title>Save settings to apply changes</template>
      <template #description>
        <div class="text-sm">
          After choosing a different model or languages in <strong>Settings</strong>, click <strong>Save</strong>. If you don't save, the configuration won't be used by translation or connection tests.
        </div>
      </template>
    </UAlert>

    <!-- Provider Selection Tabs -->
    <UCard>
      <template #header>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-settings" class="text-primary" />
          <span class="font-medium">AI Provider Configuration</span>
        </div>
      </template>
      <div class="space-y-4">
        <div class="flex gap-2">
          <UButton
            :variant="activeProvider === 'ollama' ? 'solid' : 'outline'"
            :color="activeProvider === 'ollama' ? 'primary' : 'neutral'"
            icon="i-lucide-cpu"
            @click="activeProvider = 'ollama'"
          >
            Ollama
          </UButton>
          <UButton
            :variant="activeProvider === 'runpod' ? 'solid' : 'outline'"
            :color="activeProvider === 'runpod' ? 'primary' : 'neutral'"
            icon="i-lucide-cloud"
            @click="activeProvider = 'runpod'"
          >
            RunPod
          </UButton>
        </div>

        <!-- Provider Content -->
        <div>
          <AboutOllama v-if="activeProvider === 'ollama'" />
          <AboutRunPod v-else-if="activeProvider === 'runpod'" />
        </div>
      </div>
    </UCard>
  </section>
</template>

<script setup lang="ts">
import AboutOllama from '~/components/about/technologies/AboutOllama.vue'
import AboutRunPod from '~/components/about/technologies/AboutRunPod.vue'

const activeProvider = ref<'ollama' | 'runpod'>('ollama')

const technologies = [
  {
    name: 'Tauri',
    icon: 'i-lucide-shield',
    description: 'Desktop framework'
  },
  {
    name: 'Rust',
    icon: 'i-lucide-cpu',
    description: 'Backend core'
  },
  {
    name: 'Nuxt 4',
    icon: 'i-lucide-zap',
    description: 'Frontend framework'
  },
  {
    name: 'TypeScript',
    icon: 'i-lucide-code',
    description: 'Type safety'
  },
  {
    name: 'Nuxt UI',
    icon: 'i-lucide-palette',
    description: 'UI components'
  },
  {
    name: 'Tailwind CSS',
    icon: 'i-lucide-paintbrush',
    description: 'Styling'
  }
]
</script>


