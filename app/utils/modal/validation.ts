/**
 * Modal-specific validation utilities
 * Centralized validation logic for modal components
 */

export type ValidationType = 'text' | 'translation' | 'required' | 'custom'

/**
 * Auto-validation based on validation type
 * Used by FormCard for smart validation
 */
export function getAutoValidation(
  value: string, 
  validationType: ValidationType, 
  customValidation?: boolean
): boolean {
  if (!value) return false
  
  switch (validationType) {
    case 'translation':
      return isValidTranslation(value)
    case 'custom':
      return customValidation ?? false
    case 'text':
    case 'required':
    default:
      return value.trim().length > 0
  }
}

/**
 * Translation validation
 */
export function isValidTranslation(text: string): boolean {
  return text.trim().length > 0
}

/**
 * Get validation icon based on validation state
 */
export function getValidationIcon(isValid: boolean, hasError: boolean): string {
  if (hasError) return 'i-lucide-x-circle'
  if (isValid) return 'i-lucide-check-circle'
  return 'i-lucide-help-circle'
}

/**
 * Get validation color based on validation state
 */
export function getValidationColor(isValid: boolean, hasError: boolean): string {
  if (hasError) return 'text-error'
  if (isValid) return 'text-success'
  return 'text-gray-500 dark:text-gray-400'
}
