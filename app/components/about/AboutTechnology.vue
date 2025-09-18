<template>
  <section>
    <h3 class="text-lg font-medium mb-2">Technology</h3>
    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-2 mt-2">
      <UBadge color="primary" variant="soft" class="justify-center">Tauri</UBadge>
      <UBadge color="primary" variant="soft" class="justify-center">Rust</UBadge>
      <UBadge color="primary" variant="soft" class="justify-center">Nuxt 4</UBadge>
      <UBadge color="primary" variant="soft" class="justify-center">TypeScript</UBadge>
      <UBadge color="primary" variant="soft" class="justify-center">Nuxt UI</UBadge>
      <UBadge color="primary" variant="soft" class="justify-center">Tailwind CSS</UBadge>
    </div>
    <div class="mt-4">
      <UAlert
        color="warning"
        variant="soft"
        icon="i-lucide-triangle-alert"
        title="Save settings to apply changes"
      >
        <template #description>
          <div class="text-sm">
            After choosing a different model or languages in <strong>Settings</strong>, click <strong>Save</strong>. If you don't save, the configuration won't be used by translation or connection tests.
          </div>
        </template>
      </UAlert>
    </div>

    <!-- Provider Selection Tabs -->
    <div class="mt-6">
      <div class="flex gap-2 mb-4">
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
  </section>
</template>

<script setup lang="ts">
import AboutOllama from '~/components/about/technologies/AboutOllama.vue'
import AboutRunPod from '~/components/about/technologies/AboutRunPod.vue'

const activeProvider = ref<'ollama' | 'runpod'>('ollama')
</script>


