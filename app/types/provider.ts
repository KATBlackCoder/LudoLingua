// Provider/LLM configuration types (Ollama-only)
import type { Language } from './language';
import type { ModelInfo } from './tokens';

// Supported providers (front-end view)
export type Provider =
  | 'Ollama'
  | 'OpenAI'
  | 'OpenRouter'
  | 'RunPod'
  | 'DeepL'
  | 'Anthropic'
  | 'Google'
  | 'Groq'
  | 'Mistral'
  | 'Cohere'
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
 */
export const defaultOllamaConfig: LlmConfig = {
  model: {
    display_name: 'Mistral 7B',
    model_name: 'mistral:latest',
    provider: 'Ollama',
    description: 'A 7B parameter model trained by Mistral AI, good for general-purpose text generation and translation',
    pricing: {
      input_price_per_1k: 0.0,
      output_price_per_1k: 0.0,
      currency: 'USD'
    },
    context_window: 32768,
    enabled: true
  },
  base_url: 'http://localhost:11434',
  api_key: undefined,
  temperature: 0.3,
  max_tokens: 256,
};

/**
 * Default provider configuration for RunPod (uses Ollama models on GPU).
 */
export const defaultRunPodConfig: LlmConfig = {
  model: {
    display_name: 'Llama 3.1 8B (RunPod)',
    model_name: 'llama3.1:8b',
    provider: 'RunPod',
    description: 'Meta Llama 3.1 8B model optimized for translation on RunPod GPU infrastructure',
    pricing: {
      input_price_per_1k: 0.0,
      output_price_per_1k: 0.0,
      currency: 'USD'
    },
    context_window: 131072,
    enabled: true
  },
  base_url: '', // User needs to enter their pod ID
  api_key: undefined,
  temperature: 0.3,
  max_tokens: 1000,
};
