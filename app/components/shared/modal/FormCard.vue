<template>
  <UCard :variant="variant">
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <UIcon :name="icon" class="w-4 h-4" :class="iconColorClass" />
          <h4 class="font-medium">{{ title }}</h4>
        </div>
        
        <div v-if="showValidation" class="flex items-center gap-2">
          <UIcon 
            v-if="autoValidation" 
            name="i-lucide-check-circle" 
            class="w-4 h-4 text-success" 
          />
          <UIcon 
            v-else-if="hasError" 
            name="i-lucide-x-circle" 
            class="w-4 h-4 text-error" 
          />
        </div>
      </div>
    </template>
    
    <div class="space-y-4">
      <!-- Form content -->
      <slot />
      
      <!-- Validation errors -->
      <div v-if="hasError && errorMessage" class="text-sm text-error">
        {{ errorMessage }}
      </div>
    </div>
    
    <template v-if="showFooter" #footer>
      <div class="flex items-center justify-between">
        <div v-if="helpText" class="text-xs text-gray-500 dark:text-gray-400">
          {{ helpText }}
        </div>
        
        <div class="flex items-center gap-2">
          <slot name="footer-actions" />
        </div>
      </div>
    </template>
  </UCard>
</template>

<script setup lang="ts">
import { getAutoValidation } from '~/utils/modal'
import { getIconColorClass } from '~/utils/modal/colors'

interface Props {
  title: string
  icon?: string
  iconColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  variant?: 'outline' | 'solid' | 'soft' | 'subtle'
  showValidation?: boolean
  isValid?: boolean
  hasError?: boolean
  errorMessage?: string
  showFooter?: boolean
  helpText?: string
  validationType?: 'text' | 'translation' | 'required' | 'custom'
  validationValue?: string
}

const props = withDefaults(defineProps<Props>(), {
  icon: 'i-lucide-edit',
  iconColor: 'primary',
  variant: 'outline',
  showValidation: true,
  isValid: false,
  hasError: false,
  errorMessage: '',
  showFooter: false,
  helpText: '',
  validationType: 'text',
  validationValue: ''
})

// Auto-validation based on validation type
const autoValidation = computed(() => {
  const value = props.validationValue || props.title
  
  if (!props.validationType) return props.isValid
  
  return getAutoValidation(value, props.validationType)
})

const iconColorClass = computed(() => getIconColorClass(props.iconColor))
</script>
