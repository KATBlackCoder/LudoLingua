<template>
  <UCard>
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
            <UIcon name="i-lucide-cpu" class="text-primary w-5 h-5" />
          </div>
          <div>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Technology Stack</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Modern technologies and frameworks</p>
          </div>
        </div>
        <UBadge color="primary" variant="soft" size="sm">
          <UIcon name="i-lucide-code" class="mr-1" />
          Modern
        </UBadge>
      </div>
    </template>

    <div class="space-y-6">
      <!-- Core Technologies -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-layers" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Core Technologies</span>
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

      <!-- Settings Warning -->
      <UAlert
        color="warning"
        variant="soft"
        icon="i-lucide-triangle-alert"
      >
        <template #title>Save settings to apply changes</template>
        <template #description>
          <div class="text-sm">
            After choosing a different model or languages in <strong>Settings</strong>, click <strong>Save</strong>. If you don't save, the configuration won't be used by translation or connection tests.
          </div>
        </template>
      </UAlert>

      <!-- LLM Parameters -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-sliders" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">LLM Parameters</span>
          </div>
        </template>
        
        <div class="space-y-4">
          <UAlert 
            color="warning" 
            variant="soft" 
            icon="i-lucide-settings"
            title="Connection tests require saved settings"
            description="Tests only run once your settings file exists. Open Settings and click Save at least once to create ludollingua-settings.json."
          />
          
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <UCard class="p-4 bg-primary-50 dark:bg-primary-900/20 border border-primary-200 dark:border-primary-800">
              <div class="flex items-start gap-3">
                <div class="p-2 bg-primary-100 dark:bg-primary-900/30 rounded-lg">
                  <UIcon name="i-lucide-thermometer" class="text-primary w-4 h-4" />
                </div>
                <div class="flex-1">
                  <h4 class="font-semibold text-primary-800 dark:text-primary-200 mb-1">Temperature</h4>
                  <p class="text-xs text-primary-700 dark:text-primary-300 mb-2">
                    Controls randomness/creativity. Lower = more literal and consistent; higher = more diverse but riskier.
                  </p>
                  <UBadge color="primary" variant="soft" size="sm">
                    Recommended: 0.2–0.3 (default: 0.3)
                  </UBadge>
                </div>
              </div>
            </UCard>
            
            <UCard class="p-4 bg-primary-50 dark:bg-primary-900/20 border border-primary-200 dark:border-primary-800">
              <div class="flex items-start gap-3">
                <div class="p-2 bg-primary-100 dark:bg-primary-900/30 rounded-lg">
                  <UIcon name="i-lucide-hash" class="text-primary w-4 h-4" />
                </div>
                <div class="flex-1">
                  <h4 class="font-semibold text-primary-800 dark:text-primary-200 mb-1">Max Tokens (num_predict)</h4>
                  <p class="text-xs text-primary-700 dark:text-primary-300 mb-2">
                    Caps the length of the generated translation. Prevents overly long outputs and improves latency.
                  </p>
                  <div class="space-y-1">
                    <UBadge color="primary" variant="soft" size="sm">
                      Recommended: 1024
                    </UBadge>
                    <p class="text-xs text-primary-600 dark:text-primary-400">
                      Use 2048 for longer paragraphs; smaller for short UI strings.
                    </p>
                  </div>
                </div>
              </div>
            </UCard>
          </div>
          
          <UAlert 
            color="neutral" 
            variant="soft" 
            icon="i-lucide-info"
            title="How it's applied"
            description="These settings are sent to your AI provider as ModelOptions (temperature, num_predict) during generation."
          />
        </div>
      </UCard>

      <!-- AI Provider Configuration -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-settings" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">AI Provider Configuration</span>
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
    </div>
  </UCard>
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


