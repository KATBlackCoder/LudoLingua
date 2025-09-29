<template>
  <div class="flex items-center gap-2">
    <UBadge
      v-if="status"
      :color="computedStatusColor"
      :variant="statusVariant"
      size="sm"
    >
      {{ status }}
    </UBadge>
    
    <UBadge
      v-if="category"
      :color="computedCategoryColor"
      :variant="categoryVariant"
      size="sm"
    >
      {{ category }}
    </UBadge>
    
    <UBadge
      v-if="metadata"
      :color="computedMetadataColor"
      :variant="metadataVariant"
      size="sm"
    >
      {{ metadata }}
    </UBadge>
    
    <div v-if="$slots.default" class="flex items-center gap-2">
      <slot />
    </div>
  </div>
</template>

<script setup lang="ts">
import { getBadgeColor } from '~/utils/ui'
import { getPromptTypeColor } from '~/utils/table'

interface Props {
  status?: string
  statusColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  statusVariant?: 'solid' | 'soft' | 'outline' | 'subtle'
  
  category?: string
  categoryColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  categoryVariant?: 'solid' | 'soft' | 'outline' | 'subtle'
  
  metadata?: string
  metadataColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  metadataVariant?: 'solid' | 'soft' | 'outline' | 'subtle'
}

const props = withDefaults(defineProps<Props>(), {
  status: '',
  statusColor: 'neutral',
  statusVariant: 'soft',
  category: '',
  categoryColor: 'info',
  categoryVariant: 'soft',
  metadata: '',
  metadataColor: 'neutral',
  metadataVariant: 'outline'
})

// Auto-detect colors based on content if not explicitly provided
const computedStatusColor = computed(() => {
  if (props.status && props.statusColor === 'neutral') {
    return getBadgeColor(props.status)
  }
  return props.statusColor
})

const computedCategoryColor = computed(() => {
  if (props.category && props.categoryColor === 'info') {
    return getPromptTypeColor(props.category)
  }
  return props.categoryColor
})

const computedMetadataColor = computed(() => {
  if (props.metadata && props.metadataColor === 'neutral') {
    return getBadgeColor(props.metadata)
  }
  return props.metadataColor
})
</script>
