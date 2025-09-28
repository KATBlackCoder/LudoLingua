<template>
  <UAlert
    v-if="selectedCount > 0"
    :color="alertColor"
    variant="soft"
    icon="i-lucide-check-square"
    :title="`${selectedCount} item(s) selected`"
    class="p-4"
    :actions="actionButtons"
  />
</template>

<script setup lang="ts">
import type { BulkAction } from '~/composables/features/useBulkActions'

interface Props {
  selectedCount: number
  actions?: BulkAction[]
  alertColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
}

const props = withDefaults(defineProps<Props>(), {
  actions: () => [],
  alertColor: 'info'
})

const actionButtons = computed(() => {
  return props.actions
    .filter(action => action.label) // Filter out undefined labels
    .map(action => ({
      label: action.label,
      color: action.color,
      variant: action.variant,
      icon: action.icon,
      loading: action.loading,
      disabled: action.disabled || props.selectedCount === 0,
      size: action.size || 'sm',
      onClick: action.onClick
    }))
})
</script>
