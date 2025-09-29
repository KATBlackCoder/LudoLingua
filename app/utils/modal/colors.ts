/**
 * Modal-specific color utilities
 * Centralized color mapping and detection for modal components
 */

export type ModalColor = 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'

/**
 * Get icon color class for modal components
 * Used by ModalHeader, FormCard, TextCard, MetadataCard
 */
export function getIconColorClass(iconColor: ModalColor): string {
  const colorMap: Record<ModalColor, string> = {
    primary: 'text-primary',
    secondary: 'text-secondary',
    tertiary: 'text-tertiary',
    info: 'text-info',
    success: 'text-success',
    warning: 'text-warning',
    error: 'text-error',
    neutral: 'text-gray-500 dark:text-gray-400'
  }
  return colorMap[iconColor]
}

/**
 * Get icon background class for modal components
 * Used by ModalHeader
 */
export function getIconBackgroundClass(iconColor: ModalColor): string {
  const colorMap: Record<ModalColor, string> = {
    primary: 'bg-primary-50 dark:bg-primary-900/20',
    secondary: 'bg-secondary-50 dark:bg-secondary-900/20',
    tertiary: 'bg-tertiary-50 dark:bg-tertiary-900/20',
    info: 'bg-info-50 dark:bg-info-900/20',
    success: 'bg-success-50 dark:bg-success-900/20',
    warning: 'bg-warning-50 dark:bg-warning-900/20',
    error: 'bg-error-50 dark:bg-error-900/20',
    neutral: 'bg-gray-50 dark:bg-gray-800/50'
  }
  return colorMap[iconColor]
}

/**
 * Auto-detect color based on content
 * Used by ModalActions for smart badge color detection
 */
export function getAutoDetectedColor(content: string, fallbackColor: ModalColor = 'neutral'): ModalColor {
  if (!content) return fallbackColor
  
  const contentLower = content.toLowerCase()
  
  if (contentLower.includes('success') || contentLower.includes('complete') || contentLower.includes('done')) {
    return 'success'
  }
  if (contentLower.includes('error') || contentLower.includes('failed')) {
    return 'error'
  }
  if (contentLower.includes('warning') || contentLower.includes('pending')) {
    return 'warning'
  }
  if (contentLower.includes('info') || contentLower.includes('processing')) {
    return 'info'
  }
  
  return fallbackColor
}

/**
 * Get computed color for modal actions
 * Combines auto-detection with explicit color preference
 */
export function getComputedColor(
  content: string, 
  explicitColor?: ModalColor, 
  fallbackColor: ModalColor = 'neutral'
): ModalColor {
  if (explicitColor) return explicitColor
  return getAutoDetectedColor(content, fallbackColor)
}
