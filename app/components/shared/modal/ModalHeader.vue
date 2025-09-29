<template>
  <div class="flex items-center gap-3">
    <div 
      v-if="icon" 
      class="p-2 rounded-lg"
      :class="iconBackgroundClass"
    >
      <UIcon 
        :name="icon" 
        class="w-5 h-5"
        :class="iconColorClass"
      />
    </div>
    
    <div class="flex-1">
      <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
        {{ title }}
      </h3>
      <p 
        v-if="description" 
        class="text-sm text-gray-500 dark:text-gray-400 mt-1"
      >
        {{ description }}
      </p>
    </div>
    
    <div v-if="$slots.actions" class="flex items-center gap-2">
      <slot name="actions" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { getIconColorClass, getIconBackgroundClass } from '~/utils/modal/colors'

interface Props {
  title: string
  description?: string
  icon?: string
  iconColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  iconVariant?: 'solid' | 'soft' | 'outline' | 'subtle' | 'ghost'
}

const props = withDefaults(defineProps<Props>(), {
  description: '',
  icon: '',
  iconColor: 'primary',
  iconVariant: 'soft'
})

const iconBackgroundClass = computed(() => getIconBackgroundClass(props.iconColor))
const iconColorClass = computed(() => getIconColorClass(props.iconColor))
</script>
