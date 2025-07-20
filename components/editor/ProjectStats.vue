<template>
  <div v-if="engineStore.hasProject">
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <h3 class="text-lg font-semibold">Project Statistics</h3>
          <UBadge :color="getEngineColor(engineStore.engineType)" size="sm">
            {{ engineStore.engineType }}
          </UBadge>
        </div>
      </template>
      
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <!-- Translation Scope -->
        <div class="space-y-2">
          <h4 class="text-sm font-medium text-gray-600 dark:text-gray-400">Translation Scope</h4>
          <div class="space-y-1">
            <div class="flex justify-between">
              <span class="text-sm">Text Units:</span>
              <span class="text-sm font-medium">{{ engineStore.totalTextUnits }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-sm">Files:</span>
              <span class="text-sm font-medium">{{ engineStore.gameDataFiles.length }}</span>
            </div>
          </div>
        </div>

        <!-- AI Configuration -->
        <div class="space-y-2">
          <h4 class="text-sm font-medium text-gray-600 dark:text-gray-400">AI Configuration</h4>
          <div class="space-y-1">
            <div class="flex justify-between">
              <span class="text-sm">Provider:</span>
              <span class="text-sm font-medium">{{ settingsStore.userSettings.provider }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-sm">Model:</span>
              <span class="text-sm font-medium">{{ settingsStore.userSettings.model.display_name }}</span>
            </div>
          </div>
        </div>

        <!-- Cost Estimation -->
        <div class="space-y-2">
          <h4 class="text-sm font-medium text-gray-600 dark:text-gray-400">Cost Estimation</h4>
          <div class="space-y-1">
            <div class="flex justify-between">
              <span class="text-sm">Est. Tokens:</span>
              <span class="text-sm font-medium">{{ estimatedTokens }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-sm">Est. Cost:</span>
              <span class="text-sm font-medium">{{ estimatedCost }}</span>
            </div>
          </div>
        </div>

        <!-- Translation Progress -->
        <div class="space-y-2">
          <h4 class="text-sm font-medium text-gray-600 dark:text-gray-400">Progress</h4>
          <div class="space-y-1">
            <div class="flex justify-between">
              <span class="text-sm">Translated:</span>
              <span class="text-sm font-medium">{{ translatedCount }}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-sm">Remaining:</span>
              <span class="text-sm font-medium">{{ remainingCount }}</span>
            </div>
          </div>
        </div>
      </div>
    </UCard>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useEngineStore } from '~/stores/engine';
import { useSettingsStore } from '~/stores/settings';

const engineStore = useEngineStore();
const settingsStore = useSettingsStore();

// Computed properties for statistics
const estimatedTokens = computed(() => {
  // Rough estimation: 1 token per 4 characters for source text
  const totalChars = engineStore.textUnits.reduce((sum, unit) => sum + unit.source_text.length, 0);
  return Math.ceil(totalChars / 4);
});

const estimatedCost = computed(() => {
  // Very rough estimation - would need actual pricing data
  const tokens = estimatedTokens.value;
  // Assuming ~$0.002 per 1K tokens (typical LLM pricing)
  const cost = (tokens / 1000) * 0.002;
  return cost > 0.01 ? `$${cost.toFixed(2)}` : '< $0.01';
});

const translatedCount = computed(() => {
  return engineStore.textUnits.filter(unit => 
    unit.status === 'MachineTranslated' || unit.status === 'HumanReviewed'
  ).length;
});

const remainingCount = computed(() => {
  return engineStore.textUnits.filter(unit => 
    unit.status === 'NotTranslated'
  ).length;
});

const getEngineColor = (engineType: string) => {
  switch (engineType.toLowerCase()) {
    case 'rpg maker mv':
      return 'primary';
    case 'rpg maker mz':
      return 'success';
    default:
      return 'neutral';
  }
};
</script>

<style>

</style>