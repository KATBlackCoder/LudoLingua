<template>
  <div>
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center">
            <h3 class="text-lg font-semibold">
              {{ gameDataFile.name }}
              <UBadge color="primary" size="sm" class="ml-2">
                {{ gameDataFile.text_unit_count }}
              </UBadge>
            </h3>
            <span class="text-sm text-gray-500 ml-3">
              {{ gameDataFile.path }}
            </span>
          </div>
        </div>
      </template>

      <div v-if="gameDataFile.text_units.length === 0" class="py-4 text-center text-gray-500">
        No translation units found in this file.
      </div>

      <UTable v-else :data="gameDataFile.text_units" :loading="loading" />
    </UCard>
  </div>
</template>

<script setup lang="ts">
import type { GameDataFile } from '~/types/engine';

defineProps<{
  gameDataFile: GameDataFile;
  loading?: boolean;
}>();
</script> 