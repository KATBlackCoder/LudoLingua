// Provider Configuration and Translation Types
// These types mirror the Rust structs in the backend and are used for all provider-related settings.
import type { Language } from './language';

export type Provider = 'ollama' ;

/**
 * ProviderConfig defines all settings required to connect to a language model provider.
 * This matches the Rust struct in the backend and is used for both Ollama and future providers.
 */
export interface ProviderConfig {
  provider: Provider;
  model: ModelInfo; // Model selection (matches backend ModelInfo)
  base_url?: string;
  api_key?: string;
  temperature: number;
  max_tokens: number;
  extra_config: Record<string, string>;
}

/**
 * TranslationSettings groups the source/target language and provider config for a translation session.
 */
export interface TranslationSettings {
  source_language: Language;
  target_language: Language;
  provider_config: ProviderConfig;
}

/**
 * ModelInfo describes a model available from a provider.
 * Model lists are now loaded from the backend (JSON), not hardcoded here.
 */
export interface ModelInfo {
  model_name: string;
  display_name: string;
}

// Model lists are now loaded from the backend (JSON), not hardcoded here.
// See backend /models/*.json for available models per provider.

/**
 * Default provider configuration for Ollama (can be used as a template for other providers).
 */
export const defaultProviderConfig: ProviderConfig = {
  provider: 'ollama',
  model: { model_name: 'mistral:latest', display_name: 'Mistral 7B' },
  base_url: 'http://localhost:11434',
  api_key: undefined,
  temperature: 0.7,
  max_tokens: 2048,
  extra_config: {},
};
