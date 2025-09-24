<template>
  <div v-if="engineStore.hasProject">
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
              <UIcon name="i-lucide-bar-chart-3" class="text-primary w-5 h-5" />
            </div>
            <div>
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Project Statistics</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">Translation progress and configuration</p>
            </div>
          </div>
          <div class="flex items-center gap-2">
            <UBadge :color="getEngineColor(engineStore.engineType)" variant="soft" size="sm">
              {{ engineStore.engineType }}
            </UBadge>
            <UButton 
              size="sm" 
              color="neutral" 
              variant="outline"
              icon="i-lucide-refresh-cw" 
              @click="refresh" 
            />
          </div>
        </div>
      </template>

      <div class="space-y-6">
        <!-- Progress Overview -->
        <UCard :ui="{ body: 'p-0' }">
          <div class="p-6">
            <div class="flex items-center justify-between mb-4">
              <div class="flex items-center gap-2">
                <UIcon name="i-lucide-target" class="text-primary w-5 h-5" />
                <h4 class="text-lg font-semibold text-gray-900 dark:text-white">Translation Progress</h4>
              </div>
              <div class="flex items-center gap-2">
                <UBadge 
                  :color="progressPercent >= 100 ? 'success' : 'primary'" 
                  variant="soft" 
                  size="lg"
                >
                  {{ progressPercent }}%
                </UBadge>
                <UBadge :color="databaseStatusColor" variant="soft" size="sm">
                  <UIcon :name="databaseStatusIcon" class="w-3 h-3 mr-1" />
                  {{ databaseStatusText }}
                </UBadge>
              </div>
            </div>
            
            <UProgress :value="progressPercent" size="lg" class="mb-4" />
            
            <div class="grid grid-cols-2 gap-4 text-sm">
              <div class="space-y-1">
                <p class="text-gray-600 dark:text-gray-400">Translated</p>
                <p class="text-2xl font-bold text-primary">{{ translatedCount }}</p>
                <p class="text-xs text-gray-500">of {{ engineStore.totalTextUnits }} units</p>
              </div>
              <div class="space-y-1">
                <p class="text-gray-600 dark:text-gray-400">Remaining</p>
                <p class="text-2xl font-bold text-orange-500">{{ remainingCount }}</p>
                <p class="text-xs text-gray-500">units to translate</p>
              </div>
            </div>
          </div>
        </UCard>

        <!-- Project Details Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <!-- Translation Scope -->
          <UCard>
            <template #header>
              <div class="flex items-center gap-2">
                <UIcon name="i-lucide-files" class="text-blue-500 w-4 h-4" />
                <span class="font-semibold text-gray-900 dark:text-white">Translation Scope</span>
              </div>
            </template>
            <div class="space-y-3">
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-400">Text Units</span>
                <span class="font-semibold text-gray-900 dark:text-white">{{ engineStore.totalTextUnits }}</span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-400">Files</span>
                <span class="font-semibold text-gray-900 dark:text-white">{{ engineStore.gameDataFiles.length }}</span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-400">Project</span>
                <UBadge color="neutral" variant="soft" size="sm">{{ engineStore.projectName }}</UBadge>
              </div>
            </div>
          </UCard>

          <!-- AI Configuration -->
          <UCard>
            <template #header>
              <div class="flex items-center gap-2">
                <UIcon name="i-lucide-brain" class="text-purple-500 w-4 h-4" />
                <span class="font-semibold text-gray-900 dark:text-white">AI Configuration</span>
              </div>
            </template>
            <div class="space-y-3">
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-400">Provider</span>
                <UBadge color="primary" variant="soft" size="sm">{{ settingsStore.userSettings.provider }}</UBadge>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-400">Model</span>
                <span class="font-semibold text-gray-900 dark:text-white text-xs">{{ settingsStore.userSettings.model.display_name }}</span>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-400">Temperature</span>
                <span class="font-semibold text-gray-900 dark:text-white">{{ settingsStore.userSettings.temperature }}</span>
              </div>
            </div>
          </UCard>

          <!-- Language Configuration -->
          <UCard>
            <template #header>
              <div class="flex items-center gap-2">
                <UIcon name="i-lucide-globe" class="text-green-500 w-4 h-4" />
                <span class="font-semibold text-gray-900 dark:text-white">Language Configuration</span>
              </div>
            </template>
            <div class="space-y-3">
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-400">Source</span>
                <UBadge color="neutral" variant="soft" size="sm">{{ settingsStore.userSettings.source_language.label }}</UBadge>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-400">Target</span>
                <UBadge color="success" variant="soft" size="sm">{{ settingsStore.userSettings.target_language.label }}</UBadge>
              </div>
              <div class="flex items-center justify-between">
                <span class="text-sm text-gray-600 dark:text-gray-400">Manifest</span>
                <UBadge :color="manifestExists ? 'success' : 'neutral'" variant="soft" size="sm">
                  {{ manifestExists ? 'Present' : 'Missing' }}
                </UBadge>
              </div>
            </div>
          </UCard>
        </div>

        <!-- Time Estimates -->
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-lucide-clock" class="text-orange-500 w-4 h-4" />
              <span class="font-semibold text-gray-900 dark:text-white">Time Estimates</span>
            </div>
          </template>
          <div class="grid grid-cols-2 gap-4">
            <div class="text-center p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
              <UIcon name="i-lucide-timer" class="text-blue-500 w-6 h-6 mx-auto mb-2" />
              <p class="text-sm text-gray-600 dark:text-gray-400 mb-1">Elapsed Time</p>
              <p class="text-lg font-bold text-gray-900 dark:text-white">{{ elapsedText }}</p>
            </div>
            <div class="text-center p-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
              <UIcon name="i-lucide-hourglass" class="text-orange-500 w-6 h-6 mx-auto mb-2" />
              <p class="text-sm text-gray-600 dark:text-gray-400 mb-1">Estimated Remaining</p>
              <p class="text-lg font-bold text-gray-900 dark:text-white">{{ remainingText }}</p>
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
import { useTranslator } from '~/composables/useTranslator';

const engineStore = useEngineStore();
const settingsStore = useSettingsStore();
const { elapsedText, remainingText } = useTranslator();

// Check if manifest file actually exists in project directory
const manifestExists = computed(() => {
  if (!engineStore.hasProject) return false;

  // Check if manifest hash is available in project info
  return engineStore.projectInfo?.manifest_hash !== null;
});

const databaseStatusText = computed(() => {
  if (translatedCount.value === 0) return 'Ready';
  return 'Active';
});

const databaseStatusIcon = computed(() => {
  if (translatedCount.value === 0) return 'i-lucide-check-circle';
  return 'i-lucide-server';
});

const databaseStatusColor = computed(() => {
  if (translatedCount.value === 0) return 'neutral';
  return 'success';
});

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

// Database status updates automatically based on translation count

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