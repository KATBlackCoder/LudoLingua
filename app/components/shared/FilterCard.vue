<template>
  <UCard :variant="variant === 'default' ? undefined : variant">
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <UIcon :name="icon" class="text-gray-500 w-4 h-4" />
          <span class="text-sm font-medium text-gray-700 dark:text-gray-300">{{ title }}</span>
        </div>
        <div class="flex items-center gap-2">
          <UBadge :color="badgeColor" variant="soft" size="sm">
            {{ badgeText }}
          </UBadge>
          <UButton 
            v-if="showClearButton"
            variant="soft"
            color="error"
            icon="i-lucide-x" 
            size="xs"
            @click="$emit('clear')"
          >
            Clear
          </UButton>
          <UButton
            v-if="collapsible"
            variant="ghost"
            :icon="isCollapsed ? 'i-lucide-chevron-down' : 'i-lucide-chevron-up'"
            size="xs"
            @click="toggleCollapse"
          />
        </div>
      </div>
    </template>

    <div v-if="!isCollapsed" class="space-y-4">
      <div :class="gridClass">
        <slot />
      </div>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface Props {
  title?: string
  icon?: string
  badgeText?: string
  badgeColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  showClearButton?: boolean
  gridCols?: number
  collapsible?: boolean
  defaultCollapsed?: boolean
  variant?: 'default' | 'soft' | 'outline'
}

interface Emits {
  (e: 'clear'): void
  (e: 'toggle-collapse', collapsed: boolean): void
}

const props = withDefaults(defineProps<Props>(), {
  title: 'Filters & Search',
  icon: 'i-lucide-filter',
  badgeText: '0 items',
  badgeColor: 'primary',
  showClearButton: false,
  gridCols: 4,
  collapsible: false,
  defaultCollapsed: false,
  variant: 'default'
})

const emit = defineEmits<Emits>()

// Collapsible state
const isCollapsed = ref(props.defaultCollapsed)

// Methods
const toggleCollapse = () => {
  isCollapsed.value = !isCollapsed.value
  emit('toggle-collapse', isCollapsed.value)
}

const gridClass = computed(() => {
  const baseClass = 'grid grid-cols-1 gap-4'
  const responsiveClass = props.gridCols === 4 
    ? 'md:grid-cols-4' 
    : props.gridCols === 5 
    ? 'md:grid-cols-5'
    : `md:grid-cols-${props.gridCols}`
  
  return `${baseClass} ${responsiveClass}`
})
</script>
