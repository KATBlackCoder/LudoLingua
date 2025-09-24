<template>
  <UCard>
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
            <UIcon name="i-lucide-code" class="text-primary w-5 h-5" />
          </div>
          <div>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Placeholders Reference</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Engine-specific formatting codes</p>
          </div>
        </div>
        <UBadge color="primary" variant="soft" size="sm">
          <UIcon name="i-lucide-shield" class="mr-1" />
          Do not edit inside [ … ]
        </UBadge>
      </div>
    </template>

    <div class="space-y-6">
      <!-- Preserve placeholders exactly -->
      <UAlert color="error" variant="soft" icon="i-lucide-info">
        <template #title>Preserve placeholders exactly</template>
        <template #description>
          <div class="text-sm">
            Any token enclosed in square brackets (e.g., <code>[COLOR_5]</code>) is a placeholder that encodes engine semantics. Do not translate, change, remove, or re-order these unless you are sure you know what you are doing. They will be restored on export.
          </div>
        </template>
      </UAlert>

      <!-- Engine Selection Tabs -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-settings" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Engine-Specific Placeholders</span>
          </div>
        </template>
        <div class="space-y-4">
          <div class="flex gap-2 flex-wrap">
            <UButton
              :variant="activeEngine === 'common' ? 'solid' : 'outline'"
              :color="activeEngine === 'common' ? 'primary' : 'neutral'"
              icon="i-lucide-shield"
              @click="activeEngine = 'common'"
            >
              Common
            </UButton>
            <UButton
              :variant="activeEngine === 'rpg_maker' ? 'solid' : 'outline'"
              :color="activeEngine === 'rpg_maker' ? 'info' : 'neutral'"
              icon="i-lucide-gamepad-2"
              @click="activeEngine = 'rpg_maker'"
            >
              RPG Maker MV/MZ
            </UButton>
            <UButton
              :variant="activeEngine === 'wolf_rpg' ? 'solid' : 'outline'"
              :color="activeEngine === 'wolf_rpg' ? 'error' : 'neutral'"
              icon="i-lucide-dog"
              @click="activeEngine = 'wolf_rpg'"
            >
              Wolf RPG
            </UButton>
          </div>

          <!-- Engine Content -->
          <div>
            <AboutCommonPlaceholders v-if="activeEngine === 'common'" />
            <AboutRpgMakerPlaceholders v-else-if="activeEngine === 'rpg_maker'" />
            <AboutWolfRpgPlaceholders v-else-if="activeEngine === 'wolf_rpg'" />
          </div>
        </div>
      </UCard>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import AboutCommonPlaceholders from '~/components/about/placeholders/AboutCommonPlaceholders.vue'
import AboutRpgMakerPlaceholders from '~/components/about/placeholders/AboutRpgMakerPlaceholders.vue'
import AboutWolfRpgPlaceholders from '~/components/about/placeholders/AboutWolfRpgPlaceholders.vue'

const activeEngine = ref<'common' | 'rpg_maker' | 'wolf_rpg'>('common')
</script>


