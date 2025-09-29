/**
 * Modal formatting composable
 * Reactive text and data formatting for modal components
 */

import { computed, type Ref } from 'vue'
import { processText, getCharacterCount, isTextOverLimit, formatMetadataValue, type MetadataItemType } from '~/utils/modal'
import { getBadgeColor } from '~/utils/ui'

/**
 * Composable for managing text formatting in TextCard
 */
export function useModalTextFormatting(
  text: Ref<string>,
  options: {
    sanitize?: Ref<boolean>
    truncate?: Ref<boolean>
    maxLength?: Ref<number>
  } = {}
) {
  const characterCount = computed(() => getCharacterCount(text.value))
  
  const displayText = computed(() => 
    processText(text.value, {
      sanitize: options.sanitize?.value ?? true,
      truncate: options.truncate?.value ?? false,
      maxLength: options.maxLength?.value ?? 1000
    })
  )
  
  const isOverLimit = computed(() => 
    isTextOverLimit(text.value, options.maxLength?.value ?? 1000)
  )
  
  return {
    characterCount,
    displayText,
    isOverLimit
  }
}

/**
 * Composable for managing metadata formatting in MetadataCard
 */
export function useModalMetadataFormatting() {
  const formatValue = (item: { value: string; type?: MetadataItemType }) => 
    formatMetadataValue(item.value, item.type || 'text')
  
  const getItemBadgeColor = (item: { badgeColor?: string; type?: MetadataItemType; value: string }) => {
    if (item.badgeColor) return item.badgeColor
    if (item.type === 'badge') {
      return getBadgeColor(item.value)
    }
    return 'neutral'
  }
  
  return {
    formatValue,
    getItemBadgeColor
  }
}
