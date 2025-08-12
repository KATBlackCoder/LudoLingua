<template>
  <div v-if="engineStore.hasProject">
    <UCard>
      <template #header>
        <div class="flex items-center justify-between gap-3">
          <div class="flex items-center gap-2 min-w-0">
            <h3 class="text-lg font-semibold truncate">Project Statistics</h3>
            <UBadge :color="getEngineColor(engineStore.engineType)" size="sm">{{ engineStore.engineType }}</UBadge>
          </div>
          <div class="flex items-center gap-2">
            <UBadge size="sm" color="neutral" variant="soft">{{ engineStore.projectName }}</UBadge>
            <UButton size="xs" color="neutral" icon="i-heroicons-arrow-path" @click="refresh" />
          </div>
        </div>
      </template>

      <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-2">
        <!-- Progress -->
        <UCard>
          <template #header>
            <div class="flex items-center justify-between">
              <span class="font-medium">Progress</span>
              <UBadge size="xs" :color="progressPercent >= 100 ? 'success' : 'primary'" variant="soft">{{ progressPercent }}%</UBadge>
            </div>
          </template>
          <div class="space-y-2">
            <UProgress :value="progressPercent" />
            <div class="flex justify-between text-sm text-muted">
              <span>{{ translatedCount }} / {{ engineStore.totalTextUnits }} translated</span>
              <span>{{ remainingCount }} remaining</span>
            </div>
            <div class="grid grid-cols-2 gap-2 text-sm">
              <div class="flex items-center justify-between"><span>Elapsed</span><span class="font-medium">{{ elapsedText }}</span></div>
              <div class="flex items-center justify-between"><span>ETA</span><span class="font-medium">{{ remainingText }}</span></div>
            </div>
          </div>
        </UCard>

        <!-- Translation Scope -->
        <UCard>
          <template #header>
            <span class="font-medium">Translation Scope</span>
          </template>
          <div class="space-y-2 text-sm">
            <div class="flex items-center justify-between"><span>Text Units</span><span class="font-medium">{{ engineStore.totalTextUnits }}</span></div>
            <div class="flex items-center justify-between"><span>Files</span><span class="font-medium">{{ engineStore.gameDataFiles.length }}</span></div>
          </div>
        </UCard>

        <!-- AI Configuration -->
        <UCard>
          <template #header>
            <span class="font-medium">AI Configuration</span>
          </template>
          <div class="space-y-2 text-sm">
            <div class="flex items-center justify-between"><span>Provider</span><span class="font-medium">{{ settingsStore.userSettings.provider }}</span></div>
            <div class="flex items-center justify-between"><span>Model</span><span class="font-medium">{{ settingsStore.userSettings.model.display_name }}</span></div>
          </div>
        </UCard>

        <!-- Language Configuration -->
        <UCard>
          <template #header>
            <span class="font-medium">Language Configuration</span>
          </template>
          <div class="space-y-2 text-sm">
            <div class="flex items-center justify-between"><span>Source</span><span class="font-medium">{{ settingsStore.userSettings.source_language.label }}</span></div>
            <div class="flex items-center justify-between"><span>Target</span><span class="font-medium">{{ settingsStore.userSettings.target_language.label }}</span></div>
          </div>
        </UCard>

        <!-- Token Settings -->
        <UCard>
          <template #header>
            <span class="font-medium">Token Settings</span>
          </template>
          <div class="space-y-2 text-sm">
            <div class="flex items-center justify-between">
              <span>Temperature</span>
              <span class="font-medium">{{ settingsStore.userSettings.temperature }}</span>
            </div>
            <div class="flex items-center justify-between">
              <span>Max Tokens (per unit)</span>
              <span class="font-medium flex items-center gap-2">
                {{ settingsStore.userSettings.max_tokens }}
                <UBadge size="xs" :color="settingsStore.userSettings.max_tokens > 512 ? 'warning' : 'neutral'" variant="soft">
                  {{ settingsStore.userSettings.max_tokens > 512 ? 'High' : 'Normal' }}
                </UBadge>
              </span>
            </div>
          </div>
        </UCard>
      </div>
    </UCard>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useEngineStore } from '~/stores/engine';
import { useSettingsStore } from '~/stores/settings';
import { useTranslation } from '~/composables/useTranslation';

const engineStore = useEngineStore();
const settingsStore = useSettingsStore();
const { elapsedText, remainingText } = useTranslation();

// Initialize settings and log configuration for debugging
onMounted(async () => {
  try {
    await settingsStore.initializeStores()
    
    // Log configuration after settings are loaded
    console.log('=== AI Configuration ===');
    console.log('Provider:', settingsStore.userSettings.provider);
    console.log('Model:', settingsStore.userSettings.model?.display_name || 'Not set');
    console.log('Base URL:', settingsStore.userSettings.base_url);
    console.log('Temperature:', settingsStore.userSettings.temperature);
    console.log('Max Tokens:', settingsStore.userSettings.max_tokens);
    console.log('=== Language Configuration ===');
    console.log('Source Language:', settingsStore.userSettings.source_language.label);
    console.log('Target Language:', settingsStore.userSettings.target_language.label);
    console.log('========================');
  } catch (error) {
    console.error('Failed to initialize settings:', error)
  }
})

// Simplified: remove noisy token estimates

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

const progressPercent = computed(() => {
  const total = engineStore.totalTextUnits || 0
  return total ? Math.round((translatedCount.value / total) * 100) : 0
});

const refresh = async () => {
  try {
    await engineStore.refreshProject()
  } catch (e) {
    console.error('Failed to refresh project:', e)
  }
}

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