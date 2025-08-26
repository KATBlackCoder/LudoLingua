import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { load } from '@tauri-apps/plugin-store'
import type { ModelInfo } from '~/types/tokens'
import type { LlmConfig, Provider } from '~/types/provider'
import { useAppToast } from '~/composables/useAppToast'
import { useSettingsStore } from './settings'

/**
 * Provider store (Multi-provider ready)
 *
 * Manages model lists, selected model, and connection status for the
 * LLM provider. Exposes a minimal config for translation calls and
 * basic health checks. Model list is loaded from the backend.
 */
export const useProviderStore = defineStore('provider', () => {
  const { showToast } = useAppToast();
  const settingsStore = useSettingsStore();
  
  // State
  const selectedProvider = ref<Provider>('Ollama')
  const availableProviders = ref<Provider[]>(['Ollama'])
  const availableModels = ref<ModelInfo[]>([])
  const selectedModel = ref<ModelInfo | null>(null)
  const connectionStatus = ref<'unknown' | 'connected' | 'disconnected'>('unknown')
  const lastConnectionTest = ref<Date | null>(null)
  const error = ref<string | null>(null)
  const isLoading = ref(false)
  const STALE_MS = 60_000
  
  // Getters
  const getProvider = computed(() =>
    availableProviders.value.map(p => ({ label: p, value: p }))
  )
 
  const getModels = computed(() =>
    availableModels.value.map((m: ModelInfo) => ({
      label: m.display_name,
      value: m.model_name
    }))
  )

  const isConnected = computed(() => connectionStatus.value === 'connected')
  const isDisconnected = computed(() => connectionStatus.value === 'disconnected')
  const shouldRetest = computed(() => {
    if (!lastConnectionTest.value) return true
    const age = Date.now() - lastConnectionTest.value.getTime()
    return connectionStatus.value === 'unknown' || age > STALE_MS
  })
  const connectionStatusText = computed(() => {
    switch (connectionStatus.value) {
      case 'connected':
        return 'Connected';
      case 'disconnected':
        return 'Disconnected';
      default:
        return 'Unknown';
    }
  })

  const currentProviderConfig = computed((): LlmConfig => {
    const s = settingsStore.userSettings
    return {
      model: selectedModel.value || s.model,
      base_url: s.base_url,
      api_key: s.api_key,
      temperature: s.temperature,
      max_tokens: s.max_tokens,
    }
  })
  
  // Action: Fetch models for the current provider from the backend
  async function fetchModels() {
    try {
      const models = await invoke<ModelInfo[]>('get_provider_models', { provider: selectedProvider.value })
      availableModels.value = models
      selectedModel.value = models[0] ?? null
    } catch (e) {
      console.error('Failed to fetch models for provider', selectedProvider.value, e)
      availableModels.value = []
      selectedModel.value = null
    }
  }

  // Providers list (static for now)
  async function fetchProviders() {
    availableProviders.value = ['Ollama', 'OpenAI', 'OpenRouter', 'RunPod', 'Groq'] as Provider[]
    if (!availableProviders.value.includes(selectedProvider.value)) {
      selectedProvider.value = 'Ollama'
    }
  }

  // Action: Set the provider and refresh models
  async function setProvider(provider: Provider) {
    selectedProvider.value = provider
    await fetchModels()
    // Reset connection status when provider changes
    connectionStatus.value = 'unknown'
  }

  // Action: Set the selected model
  function setModel(model: ModelInfo) {
    selectedModel.value = model
    // Reset connection status when model changes
    connectionStatus.value = 'unknown'
  }

  // Action: Test connection with current or provided config
  async function testConnection(config?: LlmConfig, opts?: { silent?: boolean }) {
    try {
      isLoading.value = true
      error.value = null
      
      const testConfig = config || currentProviderConfig.value
      const result = await invoke<boolean>('test_llm_connection', { config: testConfig })
      
      connectionStatus.value = result ? 'connected' : 'disconnected'
      lastConnectionTest.value = new Date()
      
      const message = result ? `Connected to ${selectedProvider.value}` : `Failed to connect to ${selectedProvider.value}`
      if (!opts?.silent) {
        showToast('Connection Test', message, result ? 'success' : 'error', 700)
      }
      
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to test provider connection'
      console.error('Failed to test provider connection:', e)
      connectionStatus.value = 'disconnected'
      lastConnectionTest.value = new Date()
      
      if (!opts?.silent) {
        showToast('Connection Test Failed', error.value, 'error', 700)
      }
      
      return false
    } finally {
      isLoading.value = false
    }
  }

  // Action: Load provider settings from persistent storage
  async function loadProviderSettings() {
    try {
      const store = await load('ludollingua-settings.json', { autoSave: false });
      const val = await store.get<{
        provider: Provider;
        model: ModelInfo;
        base_url?: string;
        temperature: number;
        max_tokens: number;
      }>('provider_settings');
      
      if (val) {
        selectedProvider.value = (val.provider as Provider) || 'Ollama';
        selectedModel.value = val.model || null;
      }
    } catch (error) {
      console.error('Failed to load provider settings:', error);
      // Keep default values if loading fails
    }
  }

  // Initialize settings when store is created
  loadProviderSettings();

  function clearError() {
    error.value = null
  }

  async function ensureConnected(config?: LlmConfig): Promise<boolean> {
    if (isConnected.value && !shouldRetest.value) return true
    const ok = await testConnection(config)
    return !!ok
  }

  return {
    // State
    selectedProvider,
    availableProviders,
    availableModels,
    selectedModel,
    connectionStatus,
    lastConnectionTest,
    error,
    isLoading,

    // Getters
    getProvider,
    getModels,
    isConnected,
    isDisconnected,
    connectionStatusText,
    shouldRetest,
    currentProviderConfig,

    // Actions
    fetchProviders,
    fetchModels,
    setProvider,
    setModel,
    testConnection,
    ensureConnected,
    clearError,
  }
})
