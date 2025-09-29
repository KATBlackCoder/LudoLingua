// Provider/LLM configuration types (Ollama-only)
import type { Language } from './language';
import type { ModelInfo } from './tokens';

// Supported providers (front-end view)
export type Provider =
  | 'Ollama'
  | 'RunPod'
;

/** Simplified config matching Rust `LlmConfig` (Ollama-only) */
export interface LlmConfig {
  model: ModelInfo;
  base_url?: string;
  api_key?: string;
  temperature: number;
  max_tokens: number;
}

/**
 * TranslationSettings groups the source/target language and provider config for a translation session.
 */
export interface TranslationSettings {
  source_language: Language;
  target_language: Language;
  provider_config: LlmConfig;
}

// Model lists are now loaded from the backend (JSON), not hardcoded here.
// See backend /models/*.json for available models per provider.

/**
 * Default provider configuration for Ollama (can be used as a template for other providers).
 * Note: base_url is always 'http://localhost:11434' for Ollama - backend handles this automatically.
 */
export const defaultOllamaConfig: LlmConfig = {
  model: {
    display_name: 'Qwen2.5 7B',
    model_name: 'qwen2.5:7b',
    provider: 'Ollama',
    description: 'Qwen2.5 7B - Lightweight model with excellent multilingual capabilities, especially strong for Asian languages. Perfect for quick translations and UI text. No content filtering.',
    pricing: {
      input_price_per_1k: 0.0,
      output_price_per_1k: 0.0,
      currency: 'USD'
    },
    context_window: 131072,
    enabled: true
  },
  base_url: 'http://localhost:11434', // Always localhost for Ollama
  api_key: undefined,
  temperature: 0.3,
  max_tokens: 512,
};

/**
 * Default provider configuration for RunPod (uses Ollama models on GPU).
 * Note: base_url should contain the user's pod ID (e.g., "abc123"). 
 * The backend automatically formats this to "https://abc123-11434.proxy.runpod.net".
 */
export const defaultRunPodConfig: LlmConfig = {
  model: {
    display_name: 'Qwen3 30B',
    model_name: 'qwen3:30b',
    provider: 'RunPod',
    description: 'Qwen3 30B - Advanced model with 256K context window, superior Japanese→English translation, and excellent adult content handling. Perfect for complex RPG texts with long context requirements.',
    pricing: {
      input_price_per_1k: 0.0,
      output_price_per_1k: 0.0,
      currency: 'USD'
    },
    context_window: 262144,
    enabled: true
  },
  base_url: '', // User needs to enter their pod ID (e.g., "abc123")
  api_key: undefined,
  temperature: 0.3,
  max_tokens: 512,
};
