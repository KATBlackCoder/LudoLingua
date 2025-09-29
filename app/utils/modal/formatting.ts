/**
 * Modal-specific formatting utilities
 * Centralized text and data formatting for modal components
 */

import { formatTimeAgo, formatFileSize, sanitizeText, truncateText, getBadgeColor } from '~/utils/ui'

export type MetadataItemType = 'text' | 'date' | 'file-size' | 'badge'

/**
 * Format value based on type for MetadataCard
 */
export function formatMetadataValue(value: string, type: MetadataItemType): string {
  switch (type) {
    case 'date':
      return formatTimeAgo(new Date(value))
    case 'file-size':
      return formatFileSize(parseInt(value))
    case 'badge':
    case 'text':
    default:
      return value
  }
}

/**
 * Process text for TextCard with sanitization and truncation
 */
export function processText(
  text: string, 
  options: {
    sanitize?: boolean
    truncate?: boolean
    maxLength?: number
  } = {}
): string {
  const { sanitize = true, truncate = false, maxLength = 1000 } = options
  
  let processedText = text || ''
  
  if (sanitize) {
    processedText = sanitizeText(processedText)
  }
  
  if (truncate && processedText.length > maxLength) {
    processedText = truncateText(processedText, maxLength)
  }
  
  return processedText
}

/**
 * Get character count for text display
 */
export function getCharacterCount(text: string): number {
  return text?.length || 0
}

/**
 * Check if text exceeds maximum length
 */
export function isTextOverLimit(text: string, maxLength: number): boolean {
  return getCharacterCount(text) > maxLength
}

/**
 * Get badge color for metadata items
 */
export function getMetadataBadgeColor(
  item: { badgeColor?: string; type?: MetadataItemType; value: string }
): string {
  if (item.badgeColor) return item.badgeColor
  if (item.type === 'badge') {
    return getBadgeColor(item.value)
  }
  return 'neutral'
}
