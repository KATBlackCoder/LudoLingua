<template>
  <UCard>
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
            <UIcon name="i-lucide-info" class="text-primary w-5 h-5" />
          </div>
          <div>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">What is LudoLingua?</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Desktop RPG Maker translation tool</p>
          </div>
        </div>
        <UBadge color="primary" variant="soft" size="sm">
          <UIcon name="i-lucide-heart" class="mr-1" />
          Open Source
        </UBadge>
      </div>
    </template>

    <div class="space-y-6">
      <!-- Overview -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-book-open" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Overview</span>
          </div>
        </template>
        <p class="text-sm text-gray-600 dark:text-gray-400 leading-relaxed">
          LudoLingua is a desktop application designed for translating RPG Maker game files. It helps game developers and translators extract, translate, and manage game text using AI assistance with comprehensive translation management tools.
        </p>
      </UCard>

      <!-- Perfect for Game Localization -->
      <UAlert color="info" variant="soft" icon="i-lucide-lightbulb">
        <template #title>Perfect for Game Localization</template>
        <template #description>
          <div class="text-sm">
            Whether you're a solo developer or part of a localization team, LudoLingua streamlines the entire translation workflow from extraction to final export, making RPG Maker game localization faster and more efficient.
          </div>
        </template>
      </UAlert>

      <!-- Key Capabilities -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-zap" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Key Capabilities</span>
          </div>
        </template>
        <div class="grid grid-cols-2 sm:grid-cols-3 gap-3">
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <UIcon name="i-lucide-cpu" class="w-6 h-6 mx-auto mb-2 text-primary" />
            <div class="text-sm font-medium">Local AI</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Offline processing</div>
          </div>
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <UIcon name="i-lucide-cloud" class="w-6 h-6 mx-auto mb-2 text-primary" />
            <div class="text-sm font-medium">Cloud AI</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Online services</div>
          </div>
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <UIcon name="i-lucide-shield-check" class="w-6 h-6 mx-auto mb-2 text-primary" />
            <div class="text-sm font-medium">Local Files</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Secure processing</div>
          </div>
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <UIcon name="i-lucide-gamepad-2" class="w-6 h-6 mx-auto mb-2 text-primary" />
            <div class="text-sm font-medium">RPG Maker MV</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Full support</div>
          </div>
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <UIcon name="i-lucide-gamepad-2" class="w-6 h-6 mx-auto mb-2 text-primary" />
            <div class="text-sm font-medium">RPG Maker MZ</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Full support</div>
          </div>
          <div class="text-center p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
            <UIcon name="i-lucide-palette" class="w-6 h-6 mx-auto mb-2 text-primary" />
            <div class="text-sm font-medium">Modern UI</div>
            <div class="text-xs text-gray-500 dark:text-gray-400">Nuxt UI design</div>
          </div>
        </div>
      </UCard>

      <!-- Support Our Project -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-heart" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Support Our Project</span>
          </div>
        </template>
        <div class="space-y-4">
          <div class="flex gap-2 flex-wrap">
            <UButton
              :variant="activeReferral === 'freebitcoin' ? 'solid' : 'outline'"
              :color="activeReferral === 'freebitcoin' ? 'warning' : 'neutral'"
              icon="i-lucide-bitcoin"
              @click="activeReferral = 'freebitcoin'"
            >
              FreeBitcoin
            </UButton>
            <UButton
              :variant="activeReferral === 'runpod' ? 'solid' : 'outline'"
              :color="activeReferral === 'runpod' ? 'info' : 'neutral'"
              icon="i-lucide-cloud"
              @click="activeReferral = 'runpod'"
            >
              RunPod
            </UButton>
            <UButton
              :variant="activeReferral === 'nutaku' ? 'solid' : 'outline'"
              :color="activeReferral === 'nutaku' ? 'secondary' : 'neutral'"
              icon="i-lucide-gamepad-2"
              @click="activeReferral = 'nutaku'"
            >
              Nutaku (18+)
            </UButton>
          </div>

          <!-- Referral Content -->
          <div>
            <AboutFreeBitcoin v-if="activeReferral === 'freebitcoin'" />
            <AboutRunPod v-else-if="activeReferral === 'runpod'" />
            <AboutNutaku v-else-if="activeReferral === 'nutaku'" />
          </div>
        </div>
      </UCard>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import AboutFreeBitcoin from '~/components/about/referral/AboutFreeBitcoin.vue'
import AboutRunPod from '~/components/about/referral/AboutRunPod.vue'
import AboutNutaku from '~/components/about/referral/AboutNutaku.vue'

const activeReferral = ref<'freebitcoin' | 'runpod' | 'nutaku'>('freebitcoin')
</script>


