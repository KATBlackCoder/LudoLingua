<template>
  <div class="flex items-center gap-2" :class="containerClass">
    <UButton
      v-for="action in actions"
      :key="action.id"
      :color="action.color"
      :variant="action.variant"
      :size="action.size"
      :icon="action.icon"
      :loading="action.loading"
      :disabled="action.disabled"
      :class="action.class"
      @click="action.onClick"
    >
      {{ action.label }}
    </UButton>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

export interface ActionButton {
  id: string
  label: string
  color?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  variant?: 'solid' | 'soft' | 'outline' | 'subtle' | 'ghost'
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  icon?: string
  loading?: boolean
  disabled?: boolean
  class?: string
  onClick: () => void | Promise<void>
}

interface Props {
  actions?: ActionButton[]
  direction?: 'horizontal' | 'vertical'
  justify?: 'start' | 'center' | 'end' | 'between' | 'around'
  wrap?: boolean
  spacing?: 'tight' | 'normal' | 'loose'
}

const props = withDefaults(defineProps<Props>(), {
  actions: () => [],
  direction: 'horizontal',
  justify: 'start',
  wrap: false,
  spacing: 'normal'
})

const containerClass = computed(() => {
  const classes = []
  
  // Direction
  if (props.direction === 'vertical') {
    classes.push('flex-col')
  } else {
    classes.push('flex-row')
  }
  
  // Justify
  switch (props.justify) {
    case 'start':
      classes.push('justify-start')
      break
    case 'center':
      classes.push('justify-center')
      break
    case 'end':
      classes.push('justify-end')
      break
    case 'between':
      classes.push('justify-between')
      break
    case 'around':
      classes.push('justify-around')
      break
  }
  
  // Wrap
  if (props.wrap) {
    classes.push('flex-wrap')
  }
  
  // Spacing
  switch (props.spacing) {
    case 'tight':
      classes.push(props.direction === 'vertical' ? 'gap-1' : 'gap-1')
      break
    case 'normal':
      classes.push(props.direction === 'vertical' ? 'gap-2' : 'gap-2')
      break
    case 'loose':
      classes.push(props.direction === 'vertical' ? 'gap-4' : 'gap-4')
      break
  }
  
  return classes.join(' ')
})
</script>
