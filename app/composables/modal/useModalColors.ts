/**
 * Modal color composable
 * Reactive color management for modal components
 */

import { computed, type Ref } from 'vue'
import { getIconColorClass, getIconBackgroundClass, getComputedColor, type ModalColor } from '~/utils/modal'

/**
 * Composable for managing modal icon colors
 * Used by ModalHeader, FormCard, TextCard, MetadataCard
 */
export function useModalIconColors(iconColor: Ref<ModalColor>) {
  const iconColorClass = computed(() => getIconColorClass(iconColor.value))
  const iconBackgroundClass = computed(() => getIconBackgroundClass(iconColor.value))
  
  return {
    iconColorClass,
    iconBackgroundClass
  }
}

/**
 * Composable for managing modal action colors
 * Used by ModalActions for smart color detection
 */
export function useModalActionColors(
  content: Ref<string>,
  explicitColor?: Ref<ModalColor>,
  fallbackColor: ModalColor = 'neutral'
) {
  const computedColor = computed(() => 
    getComputedColor(content.value, explicitColor?.value, fallbackColor)
  )
  
  return {
    computedColor
  }
}

/**
 * Composable for managing multiple action colors
 * Used by ModalActions with multiple badges
 */
export function useModalMultipleActionColors(
  status: Ref<string>,
  category: Ref<string>,
  metadata: Ref<string>,
  statusColor?: Ref<ModalColor>,
  categoryColor?: Ref<ModalColor>,
  metadataColor?: Ref<ModalColor>
) {
  const computedStatusColor = computed(() => 
    getComputedColor(status.value, statusColor?.value, 'neutral')
  )
  
  const computedCategoryColor = computed(() => 
    getComputedColor(category.value, categoryColor?.value, 'info')
  )
  
  const computedMetadataColor = computed(() => 
    getComputedColor(metadata.value, metadataColor?.value, 'neutral')
  )
  
  return {
    computedStatusColor,
    computedCategoryColor,
    computedMetadataColor
  }
}
