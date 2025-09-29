<template>
  <UCard :variant="variant">
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <UIcon :name="icon" class="w-4 h-4" :class="iconColorClass" />
          <h4 class="font-medium">{{ title }}</h4>
        </div>
        
        <div v-if="showCharacterCount" class="flex items-center gap-2 text-xs text-gray-500 dark:text-gray-400">
          <span>{{ characterCount }} chars</span>
          <UBadge 
            v-if="characterCount > maxLength" 
            color="warning" 
            variant="soft" 
            size="xs"
          >
            Over limit
          </UBadge>
        </div>
      </div>
    </template>
    
    <div class="space-y-2">
      <!-- Text content -->
      <div 
        v-if="displayText" 
        class="text-sm leading-relaxed"
        :class="textClass"
      >
        {{ displayText }}
      </div>
      
      <!-- Empty state -->
      <div v-else class="text-center py-4 text-gray-500 dark:text-gray-400">
        <UIcon name="i-lucide-file-text" class="w-8 h-8 mx-auto mb-2 opacity-50" />
        <p class="text-sm">No text content</p>
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
import { truncateText, sanitizeText } from '~/utils/ui'
import { getIconColorClass } from '~/utils/modal/colors'

interface Props {
  title: string
  text?: string
  icon?: string
  iconColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  variant?: 'outline' | 'solid' | 'soft' | 'subtle'
  showCharacterCount?: boolean
  maxLength?: number
  textClass?: string
  truncate?: boolean
  sanitize?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  text: '',
  icon: 'i-lucide-file-text',
  iconColor: 'primary',
  variant: 'outline',
  showCharacterCount: true,
  maxLength: 1000,
  textClass: '',
  truncate: false,
  sanitize: true
})

const characterCount = computed(() => {
  return props.text?.length || 0
})

const displayText = computed(() => {
  let text = props.text || ''
  
  if (props.sanitize) {
    text = sanitizeText(text)
  }
  
  if (props.truncate && text.length > props.maxLength) {
    text = truncateText(text, props.maxLength)
  }
  
  return text
})

const iconColorClass = computed(() => getIconColorClass(props.iconColor))
</script>
