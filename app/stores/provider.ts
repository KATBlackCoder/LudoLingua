import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { load } from '@tauri-apps/plugin-store'
import type { ModelInfo, OllamaConfig } from '~/types/provider'
import { useAppToast } from '~/composables/useAppToast'

/**
 * Provider store (Ollama-only)
 *
 * Manages model lists, selected model, and connection status for the
 * local Ollama service. Exposes a minimal config for translation calls
 * and basic health checks. Model list is loaded from the backend.
 */
export const useProviderStore = defineStore('provider', () => {
  const { showToast } = useAppToast();
  
  // State
  const selectedProvider = ref<string>('ollama')
  const availableProviders = ref<string[]>(['ollama'])
  const availableModels = ref<ModelInfo[]>([])
  const selectedModel = ref<ModelInfo | null>(null)
  const connectionStatus = ref<'unknown' | 'connected' | 'disconnected'>('unknown')
  const lastConnectionTest = ref<Date | null>(null)
  const error = ref<string | null>(null)
  const isLoading = ref(false)
  const STALE_MS = 60_000
  
  // Getters
  const getProvider = computed(() =>
    availableProviders.value.map(p => p.charAt(0).toUpperCase() + p.slice(1))
  )
 
  const getModels = computed(() =>
    availableModels.value.map(m => ({
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

  const currentProviderConfig = computed((): OllamaConfig => ({
    model: selectedModel.value || { model_name: 'mistral:latest', display_name: 'Mistral 7B' },
    base_url: 'http://localhost:11434',
    temperature: 0.7,
    max_tokens: 2048,
  }))
  
  // Action: Fetch models for the current provider from the backend
  async function fetchModels() {
    try {
      const models = await invoke<ModelInfo[]>('get_ollama_models')
      availableModels.value = models
      selectedModel.value = models[0] ?? null
    } catch {
      availableModels.value = []
      selectedModel.value = null
    }
  }

  // Ollama-only: providers list is static
  async function fetchProviders() { availableProviders.value = ['ollama'] }

  // Action: Set the provider and refresh models
  async function setProvider(provider: string) {
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
  async function testConnection(config?: OllamaConfig) {
    try {
      isLoading.value = true
      error.value = null
      
      const testConfig = config || currentProviderConfig.value
      const result = await invoke<boolean>('test_llm_connection', { config: testConfig })
      
      connectionStatus.value = result ? 'connected' : 'disconnected'
      lastConnectionTest.value = new Date()
      
      const message = result ? 'Connection successful' : 'Connection failed'
      showToast('Connection Test', message, result ? 'success' : 'error', 700)
      
      return result
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to test provider connection'
      console.error('Failed to test provider connection:', e)
      connectionStatus.value = 'disconnected'
      lastConnectionTest.value = new Date()
      
      showToast('Connection Test Failed', error.value, 'error', 700)
      
      return false
    } finally {
      isLoading.value = false
    }
  }

  // Action: Load provider settings from persistent storage
  async function loadProviderSettings() {
    try {
      const store = await load("ludollingua-settings.json", { autoSave: false });
      const val = await store.get<{
        provider: string;
        model: ModelInfo;
        base_url?: string;
        temperature: number;
        max_tokens: number;
      }>("provider_settings");
      
      if (val) {
        selectedProvider.value = val.provider || 'ollama';
        selectedModel.value = val.model || null;
        // Note: base_url, temperature, max_tokens would be used if we had them in state
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

  async function ensureConnected(config?: OllamaConfig): Promise<boolean> {
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
