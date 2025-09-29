<template>
  <UCard :variant="variant">
    <template #header>
      <div class="flex items-center gap-2">
        <UIcon :name="icon" class="w-4 h-4" :class="iconColorClass" />
        <h4 class="font-medium">{{ title }}</h4>
      </div>
    </template>
    
    <div class="space-y-3">
      <!-- Metadata items -->
      <div 
        v-for="(item, index) in items" 
        :key="index"
        class="flex items-center justify-between py-2 border-b border-gray-100 dark:border-gray-800 last:border-b-0"
      >
        <div class="flex items-center gap-2">
          <UIcon 
            v-if="item.icon" 
            :name="item.icon" 
            class="w-4 h-4 text-gray-500 dark:text-gray-400" 
          />
          <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
            {{ item.label }}
          </span>
        </div>
        
        <div class="flex items-center gap-2">
          <span class="text-sm text-gray-600 dark:text-gray-400">
            {{ formatValue(item) }}
          </span>
          
          <UBadge 
            v-if="item.badge || item.type === 'badge'" 
            :color="getItemBadgeColor(item)"
            :variant="item.badgeVariant || 'soft'"
            size="xs"
          >
            {{ item.badge || item.value }}
          </UBadge>
        </div>
      </div>
      
      <!-- Custom content slot -->
      <slot />
    </div>
    
    <template v-if="$slots.footer" #footer>
      <slot name="footer" />
    </template>
  </UCard>
</template>

<script setup lang="ts">
import { formatTimeAgo, formatFileSize, getBadgeColor } from '~/utils/ui'
import { getIconColorClass } from '~/utils/modal/colors'

interface MetadataItem {
  label: string
  value: string
  icon?: string
  badge?: string
  badgeColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  badgeVariant?: 'solid' | 'soft' | 'outline' | 'subtle'
  type?: 'text' | 'date' | 'file-size' | 'badge'
}

interface Props {
  title: string
  items: MetadataItem[]
  icon?: string
  iconColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  variant?: 'outline' | 'solid' | 'soft' | 'subtle'
}

const props = withDefaults(defineProps<Props>(), {
  icon: 'i-lucide-info',
  iconColor: 'info',
  variant: 'outline'
})

const iconColorClass = computed(() => getIconColorClass(props.iconColor))

// Format value based on type
const formatValue = (item: MetadataItem) => {
  if (item.type === 'date') {
    return formatTimeAgo(new Date(item.value))
  }
  if (item.type === 'file-size') {
    return formatFileSize(parseInt(item.value))
  }
  return item.value
}

// Get badge color for badge type items
const getItemBadgeColor = (item: MetadataItem) => {
  if (item.badgeColor) return item.badgeColor
  if (item.type === 'badge') {
    return getBadgeColor(item.value)
  }
  return 'neutral'
}
</script>
