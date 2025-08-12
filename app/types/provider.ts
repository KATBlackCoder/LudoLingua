// Provider/LLM configuration types (Ollama-only)
import type { Language } from './language';

/** Simplified config matching Rust `LlmConfig` (Ollama-only) */
export interface OllamaConfig {
  model: ModelInfo;
  base_url?: string;
  temperature: number;
  max_tokens: number;
}

/**
 * TranslationSettings groups the source/target language and provider config for a translation session.
 */
export interface TranslationSettings {
  source_language: Language;
  target_language: Language;
  provider_config: OllamaConfig;
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
export const defaultOllamaConfig: OllamaConfig = {
  model: { model_name: 'mistral:latest', display_name: 'Mistral 7B' },
  base_url: 'http://localhost:11434',
  temperature: 0.3,
  max_tokens: 256,
};
