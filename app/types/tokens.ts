import type { PromptType } from './translation'

/** Token estimation for a single text unit */
export interface TokenEstimate {
  /** Input tokens (prompt + source text + glossary) */
  input_tokens: number
  /** Estimated output tokens (translated text) */
  output_tokens: number
  /** Total tokens (input + output) */
  total_tokens: number
}

/** Token estimation for the entire project */
export interface ProjectTokenEstimate {
  /** Total input tokens for all text units */
  total_input_tokens: number
  /** Total estimated output tokens for all text units */
  total_output_tokens: number
  /** Grand total tokens */
  total_tokens: number
  /** Number of text units to translate */
  text_units_count: number
  /** Breakdown by prompt type */
  by_prompt_type: PromptTypeEstimate[]
}

/** Token estimation breakdown by prompt type */
export interface PromptTypeEstimate {
  prompt_type: PromptType
  count: number
  input_tokens: number
  output_tokens: number
  total_tokens: number
}

/** Token pricing information */
export interface TokenPricing {
  /** USD per 1K input tokens */
  input_price_per_1k: number
  /** USD per 1K output tokens */
  output_price_per_1k: number
  /** Currency code (e.g., "USD") */
  currency: string
}

/** Model information with pricing and details */
export interface ModelInfo {
  /** User-friendly display name for the model */
  display_name: string
  /** Actual model name used for API calls */
  model_name: string
  /** Provider name (e.g., "Ollama", "OpenAI", "Anthropic") */
  provider: string
  /** Brief description of the model's capabilities */
  description?: string
  /** Token pricing information */
  pricing: TokenPricing
  /** Approximate context window size in tokens */
  context_window?: number
  /** Whether the model is available/enabled */
  enabled: boolean
}

/** Helper function to format large numbers with appropriate suffixes */
export function formatTokenCount(count: number): string {
  if (count < 1000) return count.toString()
  if (count < 1000000) return `${(count / 1000).toFixed(1)}K`
  return `${(count / 1000000).toFixed(1)}M`
}

/** Helper function to format cost estimates */
export function formatCost(cost: number, currency = 'USD'): string {
  if (cost === 0) return 'Free'
  if (cost < 0.01) return `< $0.01`
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency,
    minimumFractionDigits: 2,
    maximumFractionDigits: 4,
  }).format(cost)
}

/** Calculate estimated cost based on token estimate and model pricing */
export function calculateCost(
  estimate: ProjectTokenEstimate,
  pricing: TokenPricing
): number {
  const inputCost = (estimate.total_input_tokens / 1000) * pricing.input_price_per_1k
  const outputCost = (estimate.total_output_tokens / 1000) * pricing.output_price_per_1k
  return inputCost + outputCost
}
