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

      <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-4 gap-2">
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

        <!-- Token Usage Estimate -->
        <UCard>
          <template #header>
            <div class="flex items-center justify-between">
              <span class="font-medium">Token Usage</span>
              <UButton 
                size="xs" 
                color="primary" 
                icon="i-heroicons-calculator" 
                :loading="isEstimating"
                @click="handleEstimateClick"
              >
                {{ tokenEstimate ? 'Refresh' : 'Estimate' }}
              </UButton>
            </div>
          </template>
          
          <div v-if="tokenEstimate" class="space-y-2 text-sm">
            <div class="flex items-center justify-between">
              <span>Total Tokens</span>
              <span class="font-medium">{{ formatTokenCount(tokenEstimate.total_tokens) }}</span>
            </div>
            <div class="flex items-center justify-between">
              <span>Input</span>
              <span class="font-medium">{{ formatTokenCount(tokenEstimate.total_input_tokens) }}</span>
            </div>
            <div class="flex items-center justify-between">
              <span>Output (est.)</span>
              <span class="font-medium">{{ formatTokenCount(tokenEstimate.total_output_tokens) }}</span>
            </div>
            <USeparator />
            <div class="flex items-center justify-between">
              <span>Estimated Cost</span>
              <UBadge color="success" variant="soft">{{ estimatedCost }}</UBadge>
            </div>
          </div>
          
          <div v-else-if="isEstimating" class="flex items-center justify-center py-4">
            <UProgress size="sm" animation="carousel" />
          </div>
          
          <div v-else class="text-center py-4 text-gray-500">
            <p class="text-sm">Click "Estimate" to calculate token usage</p>
          </div>
        </UCard>

        <!-- Actual Token Usage Card -->
        <UCard v-if="translateStore.actualTokenUsage.length > 0" class="w-full">
          <template #header>
            <div class="flex items-center gap-2">
              <Icon name="i-heroicons-chart-bar" class="w-5 h-5" />
              <h3 class="text-lg font-semibold">Actual Token Usage</h3>
            </div>
          </template>
          
          <div class="space-y-2 text-sm">
            <div class="flex items-center justify-between">
              <span>Total Tokens</span>
              <span class="font-medium">{{ formatTokenCount(translateStore.totalActualTokenUsage.total_tokens) }}</span>
            </div>
            <div class="flex items-center justify-between">
              <span>Input</span>
              <span class="font-medium">{{ formatTokenCount(translateStore.totalActualTokenUsage.input_tokens) }}</span>
            </div>
            <div class="flex items-center justify-between">
              <span>Output</span>
              <span class="font-medium">{{ formatTokenCount(translateStore.totalActualTokenUsage.output_tokens) }}</span>
            </div>
            <div class="flex items-center justify-between">
              <span>Translations</span>
              <span class="font-medium">{{ translateStore.actualTokenUsage.length }}</span>
            </div>
            <USeparator />
            <div class="flex items-center justify-between">
              <span>Actual Cost</span>
              <UBadge color="primary" variant="soft">{{ actualCost }}</UBadge>
            </div>
            <USeparator />
            <div class="flex items-center justify-between">
              <span>Estimation Accuracy</span>
              <UBadge :color="accuracyColor" variant="soft">{{ estimationAccuracy }}</UBadge>
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
import { useTranslateStore } from '~/stores/translate';
import { useTranslation } from '~/composables/useTranslation';
import { formatTokenCount, formatCost, calculateCost } from '~/types/tokens';

const engineStore = useEngineStore();
const settingsStore = useSettingsStore();
const translateStore = useTranslateStore();
const { elapsedText, remainingText, tokenEstimate, isEstimating, estimateTokens } = useTranslation();

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

const estimatedCost = computed(() => {
  if (!tokenEstimate.value) return 'Unknown'
  
  // Get pricing directly from the model configuration
  const model = settingsStore.userSettings.model
  if (!model) return 'Unknown'
  
  // Debug: log model structure
  console.log('Model structure:', model)
  
  // Fallback to Ollama pricing if pricing is missing (for backwards compatibility)
  const pricing = model.pricing || {
    input_price_per_1k: 0.0,
    output_price_per_1k: 0.0,
    currency: 'USD'
  }
  
  const cost = calculateCost(tokenEstimate.value, pricing)
  return formatCost(cost)
});

const actualCost = computed(() => {
  if (!translateStore.totalActualTokenUsage || !settingsStore.userSettings.model.pricing) {
    return 'Unknown'
  }
  
  const pricing = settingsStore.userSettings.model.pricing
  const usage = translateStore.totalActualTokenUsage
  const inputCost = (usage.input_tokens / 1000.0) * pricing.input_price_per_1k
  const outputCost = (usage.output_tokens / 1000.0) * pricing.output_price_per_1k
  const totalCost = inputCost + outputCost
  
  return formatCost(totalCost)
});

const estimationAccuracy = computed(() => {
  if (!tokenEstimate.value || !translateStore.totalActualTokenUsage || translateStore.totalActualTokenUsage.total_tokens === 0) {
    return 'N/A'
  }
  
  const estimated = tokenEstimate.value.total_tokens
  const actual = translateStore.totalActualTokenUsage.total_tokens
  const accuracy = (estimated / actual) * 100
  
  return `${accuracy.toFixed(0)}%`
});

const accuracyColor = computed(() => {
  if (!tokenEstimate.value || !translateStore.totalActualTokenUsage) {
    return 'neutral'
  }
  
  const estimated = tokenEstimate.value.total_tokens
  const actual = translateStore.totalActualTokenUsage.total_tokens
  const accuracy = (estimated / actual) * 100
  
  if (accuracy >= 90 && accuracy <= 110) return 'success'      // Very accurate
  if (accuracy >= 80 && accuracy <= 120) return 'info'     // Fairly accurate
  if (accuracy >= 70 && accuracy <= 130) return 'warning'     // Somewhat accurate
  return 'error'                                                // Poor accuracy
});

const refresh = async () => {
  try {
    await engineStore.refreshProject()
  } catch (e) {
    console.error('Failed to refresh project:', e)
  }
}

const handleEstimateClick = async () => {
  await estimateTokens()
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