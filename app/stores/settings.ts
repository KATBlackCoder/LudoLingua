import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { OllamaConfig, ModelInfo } from '~/types/provider';
import type { Language } from '~/types/language';
import { Store } from '@tauri-apps/plugin-store';
import { useProviderStore } from './provider';
import { useLanguageStore } from './language';

/**
 * Unified user settings structure
 */
export interface UserSettings {
  provider: string;
  model: ModelInfo;
  source_language: Language;
  target_language: Language;
  base_url?: string;
  api_key?: string;
  temperature: number;
  max_tokens: number;
}

/**
 * Default user settings
 */
const defaultUserSettings: UserSettings = {
  provider: 'ollama',
  model: { model_name: 'mistral:latest', display_name: 'Mistral 7B' },
  source_language: { id: 'en', label: 'English', native_name: 'English', dir: 'ltr', enabled: true },
  target_language: { id: 'fr', label: 'French', native_name: 'Français', dir: 'ltr', enabled: true },
  base_url: 'http://localhost:11434',
  api_key: undefined,
  temperature: 0.3,
  max_tokens: 256,
};

/**
 * Settings Store
 * 
 * Manages persistent application configuration using Tauri's Store plugin.
 * Handles unified user settings including provider and language preferences.
 */
export const useSettingsStore = defineStore('settings', () => {
  // ========================================
  // STATE
  // ========================================
  
  /** Current user settings (provider + language configuration) */
  const userSettings = ref<UserSettings>(defaultUserSettings);
  
  /** Loading state for async operations */
  const isLoading = ref(false);
  
  /** Current error message, null if no error */
  const error = ref<string | null>(null);

  // ========================================
  // STORE REFERENCES
  // ========================================
  
  /** Provider store for LLM provider and model management */
  const providerStore = useProviderStore();
  
  /** Language store for source/target language management */
  const languageStore = useLanguageStore();

  // ========================================
  // COMPUTED PROPERTIES
  // ========================================
  
  /**
   * Validates if the current user settings are complete
   * @returns true if provider, model, and base_url are all set
   */
  const hasValidConfig = computed(() => {
    return userSettings.value.provider && 
           userSettings.value.model && 
           userSettings.value.base_url;
  });

  /**
   * Provider configuration extracted from user settings
   */
  const providerConfig = computed((): OllamaConfig => ({
    model: userSettings.value.model,
    base_url: userSettings.value.base_url,
    temperature: userSettings.value.temperature,
    max_tokens: userSettings.value.max_tokens,
  }));

  /**
   * Complete translation settings combining language and provider config
   * Used for translation operations
   */
  const currentTranslationSettings = computed(() => ({
    source_language: languageStore.getLanguage.source_language,
    target_language: languageStore.getLanguage.target_language,
    provider_config: providerConfig.value,
  }));

  // ========================================
  // CORE SETTINGS MANAGEMENT
  // ========================================

  /**
   * Load user settings from persistent storage
   * Automatically syncs with provider and language stores after loading
   */
  const loadUserSettings = async (): Promise<void> => {
    try {
      isLoading.value = true;
      error.value = null;
      
      const store = await Store.load('ludollingua-settings.json');
      const settingsValue = await store.get<UserSettings>('user_settings');
      
      // Use loaded settings or fall back to defaults
      const settings = settingsValue || defaultUserSettings;
      userSettings.value = settings;
      
      // Sync with provider store to update UI components
      await providerStore.setProvider(settings.provider);
      if (settings.model) {
        providerStore.setModel(settings.model);
      }
      
      // Sync with language store to update UI components
      languageStore.setLanguage(settings.source_language.id, settings.target_language.id, { silent: true });
      
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to load user settings';
      console.error('Failed to load user settings:', e);
    } finally {
      isLoading.value = false;
    }
  };

  /**
   * Save user settings to persistent storage
   * @param settings - The user settings to save
   */
  const saveUserSettings = async (settings: UserSettings): Promise<void> => {
    try {
      isLoading.value = true;
      error.value = null;
      
      const store = await Store.load('ludollingua-settings.json');
      await store.set('user_settings', settings);
      await store.save();
      
      // Update local state
      userSettings.value = settings;
      
      // Sync with provider store to update UI components
      await providerStore.setProvider(settings.provider);
      if (settings.model) {
        providerStore.setModel(settings.model);
      }
      
      // Sync with language store to update UI components
      languageStore.setLanguage(settings.source_language.id, settings.target_language.id, { silent: true });
      
      console.log('User settings saved:', settings);
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to save user settings';
      console.error('Failed to save user settings:', e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  };

  /**
   * Reset user settings to default values
   * Saves the defaults to persistent storage
   */
  const resetUserSettings = async (): Promise<void> => {
    try {
      isLoading.value = true;
      error.value = null;
      
      // Create a fresh copy of defaults to avoid reference issues
      const settings = { ...defaultUserSettings };
      await saveUserSettings(settings);
      
      console.log('User settings reset to defaults:', settings);
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to reset user settings';
      console.error('Failed to reset user settings:', e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  };

  /**
   * Delete user settings from persistent storage
   * Resets local state to defaults but doesn't save defaults to storage
   */
  const deleteUserSettings = async (): Promise<void> => {
    try {
      isLoading.value = true;
      error.value = null;
      
      const store = await Store.load('ludollingua-settings.json');
      await store.delete('user_settings');
      await store.save();
      
      // Reset to defaults locally
      userSettings.value = { ...defaultUserSettings };
      
      console.log('User settings deleted');
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to delete user settings';
      console.error('Failed to delete user settings:', e);
      throw e;
    } finally {
      isLoading.value = false;
    }
  };

  /**
   * Update specific user settings
   * @param updates - Partial settings object with changes
   */
  const updateUserSettings = async (updates: Partial<UserSettings>): Promise<void> => {
    const newSettings = { ...userSettings.value, ...updates };
    await saveUserSettings(newSettings);
  };

  // ========================================
  // UTILITIES
  // ========================================

  /**
   * Clear the current error state
   */
  const clearError = (): void => {
    error.value = null;
  };

  /**
   * Initialize all related stores and load configurations
   * Should be called when the application starts
   */
  const initializeStores = async (): Promise<void> => {
    try {
      // Load data from all stores in parallel for better performance
      await Promise.all([
        providerStore.fetchProviders(),
        providerStore.fetchModels(),
        loadUserSettings(),
      ]);
    } catch (e) {
      console.error('Failed to initialize settings stores:', e);
    }
  };

  /**
   * Check whether a persisted settings file with `user_settings` exists.
   * Used to redirect first-time users to Settings and gate certain actions.
   */
  const hasPersistedUserSettings = async (): Promise<boolean> => {
    try {
      const store = await Store.load('ludollingua-settings.json', { autoSave: false });
      const existing = await store.get<UserSettings>('user_settings');
      return !!existing;
    } catch {
      return false;
    }
  };

  // ========================================
  // STORE INTERFACE
  // ========================================

  return {
    // State
    userSettings,
    isLoading,
    error,
    
    // Computed
    hasValidConfig,
    providerConfig,
    currentTranslationSettings,
    
    // Store references
    providerStore,
    languageStore,
    
    // Core settings management
    loadUserSettings,
    saveUserSettings,
    resetUserSettings,
    deleteUserSettings,
    updateUserSettings,
    
    // Utilities
    clearError,
    initializeStores,
    hasPersistedUserSettings,
  };
}); 