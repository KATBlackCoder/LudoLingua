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
        v-if="showCopy"
        :color="copyButtonColor"
        :variant="copyVariant"
        size="sm"
        :disabled="loading || copyDisabled"
        :loading="copyLoading"
        @click="handleCopy"
      >
        <UIcon v-if="!copyLoading" :name="copyButtonIcon" class="w-4 h-4 mr-1" />
        {{ copyButtonLabel }}
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
  
  // Copy button
  showCopy?: boolean
  copyLabel?: string
  copyColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  copyVariant?: 'solid' | 'soft' | 'outline' | 'subtle' | 'ghost'
  copyIcon?: string
  copyDisabled?: boolean
  copyLoading?: boolean
  copyText?: string // Text to copy to clipboard
  
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

const props = withDefaults(defineProps<Props>(), {
  showCancel: true,
  cancelLabel: 'Cancel',
  showCopy: false,
  copyLabel: 'Copy',
  copyColor: 'neutral',
  copyVariant: 'ghost',
  copyIcon: 'i-lucide-copy',
  copyDisabled: false,
  copyLoading: false,
  copyText: '',
  showSave: true,
  saveLabel: 'Save',
  saveColor: 'primary',
  saveVariant: 'solid',
  loading: false,
  disabled: false,
  statusInfo: '',
  keyboardShortcuts: ''
})

// Clipboard functionality
const { copy: copyToClipboard, copied } = useClipboard()

const handleCopy = async () => {
  if (!props.copyText?.trim()) return
  try {
    await copyToClipboard(props.copyText)
  } catch (error) {
    console.error('Failed to copy to clipboard:', error)
  }
}

// Dynamic copy button properties based on clipboard state
const copyButtonLabel = computed(() => {
  if (copied.value) return 'Copied!'
  return props.copyLabel
})

const copyButtonColor = computed(() => {
  if (copied.value) return 'success'
  return props.copyColor
})

const copyButtonIcon = computed(() => {
  if (copied.value) return 'i-lucide-check'
  return props.copyIcon
})

defineEmits<{
  cancel: []
  copy: []
  save: []
}>()
</script>
