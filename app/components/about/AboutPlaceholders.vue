<template>
  <section>
    <div class="flex items-center gap-2 mb-2">
      <h3 class="text-lg font-medium">Placeholders Reference</h3>
      <UBadge color="primary" variant="soft" size="xs">Do not edit inside [ … ]</UBadge>
    </div>

    <UAlert color="error" variant="soft" icon="i-lucide-info" class="mb-3">
      <template #title>Preserve placeholders exactly</template>
      <template #description>
        <div class="text-sm">
          Any token enclosed in square brackets (e.g., <code>[COLOR_5]</code>) is a placeholder that encodes engine semantics. Do not translate, change, remove, or re-order these unless you are sure you know what you are doing. They will be restored on export.
        </div>
      </template>
    </UAlert>

    <!-- Engine Selection Tabs -->
    <div class="mt-6">
      <div class="flex gap-2 mb-4">
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
  </section>
</template>

<script setup lang="ts">
import AboutCommonPlaceholders from '~/components/about/placeholders/AboutCommonPlaceholders.vue'
import AboutRpgMakerPlaceholders from '~/components/about/placeholders/AboutRpgMakerPlaceholders.vue'
import AboutWolfRpgPlaceholders from '~/components/about/placeholders/AboutWolfRpgPlaceholders.vue'

const activeEngine = ref<'common' | 'rpg_maker' | 'wolf_rpg'>('common')
</script>


