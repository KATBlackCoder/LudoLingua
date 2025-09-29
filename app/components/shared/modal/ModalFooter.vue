<template>
  <div class="flex items-center justify-between">
    <!-- Left side - Status info -->
    <div class="flex items-center gap-4 text-sm text-gray-500 dark:text-gray-400">
      <span v-if="statusInfo" class="flex items-center gap-2">
        <UIcon name="i-lucide-info" class="w-4 h-4" />
        {{ statusInfo }}
      </span>
      
      <span v-if="keyboardShortcuts" class="flex items-center gap-2">
        <UIcon name="i-lucide-keyboard" class="w-4 h-4" />
        {{ keyboardShortcuts }}
      </span>
    </div>
    
    <!-- Right side - Action buttons -->
    <div class="flex items-center gap-2">
      <UButton
        v-if="showCancel"
        variant="soft"
        size="sm"
        :disabled="loading"
        @click="$emit('cancel')"
      >
        {{ cancelLabel }}
      </UButton>
      
      <UButton
        v-if="showSave"
        :color="saveColor"
        :variant="saveVariant"
        size="sm"
        :loading="loading"
        :disabled="loading || disabled"
        @click="$emit('save')"
      >
        {{ saveLabel }}
      </UButton>
      
      <!-- Custom actions slot -->
      <slot name="actions" />
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  // Cancel button
  showCancel?: boolean
  cancelLabel?: string
  
  // Save button
  showSave?: boolean
  saveLabel?: string
  saveColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  saveVariant?: 'solid' | 'soft' | 'outline' | 'subtle'
  
  // State
  loading?: boolean
  disabled?: boolean
  
  // Status info
  statusInfo?: string
  keyboardShortcuts?: string
}

withDefaults(defineProps<Props>(), {
  showCancel: true,
  cancelLabel: 'Cancel',
  showSave: true,
  saveLabel: 'Save',
  saveColor: 'primary',
  saveVariant: 'solid',
  loading: false,
  disabled: false,
  statusInfo: '',
  keyboardShortcuts: ''
})

defineEmits<{
  cancel: []
  save: []
}>()
</script>
