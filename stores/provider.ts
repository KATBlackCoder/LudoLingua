import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { load } from '@tauri-apps/plugin-store'
import type { Provider, ModelInfo, ProviderConfig } from '~/types/provider'
import { useAppToast } from '~/composables/useAppToast'

export const useProviderStore = defineStore('provider', () => {
  const { showToast } = useAppToast();
  
  // State
  const selectedProvider = ref<string>('ollama')
  const availableProviders = ref<string[]>([])
  const availableModels = ref<ModelInfo[]>([])
  const selectedModel = ref<ModelInfo | null>(null)
  const connectionStatus = ref<'unknown' | 'connected' | 'disconnected'>('unknown')
  const lastConnectionTest = ref<Date | null>(null)
  const error = ref<string | null>(null)
  const isLoading = ref(false)
  
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

  const currentProviderConfig = computed((): ProviderConfig => ({
    provider: selectedProvider.value as Provider,
    model: selectedModel.value || { model_name: 'mistral:latest', display_name: 'Mistral 7B' },
    base_url: selectedProvider.value === 'ollama' ? 'http://localhost:11434' : undefined,
    api_key: undefined,
    temperature: 0.7,
    max_tokens: 2048,
    extra_config: {},
  }))
  
  // Action: Fetch models for the current provider from the backend
  async function fetchModels() {
    try {
      const models = await invoke<ModelInfo[]>('get_available_models', { provider: selectedProvider.value })
      availableModels.value = models
      selectedModel.value = models[0] ?? null
    } catch {
      availableModels.value = []
      selectedModel.value = null
    }
  }

  // Action: Fetch providers from the backend
  async function fetchProviders() {
    try {
      const providers = await invoke<Provider[]>('get_available_providers')
      availableProviders.value = providers
    } catch {
      availableProviders.value = []
    }
  }

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
  async function testConnection(config?: ProviderConfig) {
    try {
      isLoading.value = true
      error.value = null
      
      const testConfig = config || currentProviderConfig.value
      const result = await invoke<boolean>('test_llm_connection', { config: testConfig })
      
      connectionStatus.value = result ? 'connected' : 'disconnected'
      lastConnectionTest.value = new Date()
      
      const message = result ? 'Connection successful' : 'Connection failed'
      showToast('Connection Test', message, result ? 'success' : 'error', 700)
      
      console.log('Provider connection test result:', result)
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
    currentProviderConfig,

    // Actions
    fetchProviders,
    fetchModels,
    setProvider,
    setModel,
    testConnection,
    clearError,
  }
})
