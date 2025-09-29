<template>
  <UModal
    v-model:open="open"
    :dismissible="dismissible"
    :ui="modalUi"
    :title="title || undefined"
    :description="description"
  >
    <!-- Header -->
    <template v-if="showHeader" #title>
      <ModalHeader
        :title="title || 'Modal'"
        :description="description"
        :icon="headerIcon"
        :icon-color="headerIconColor"
        :icon-background="headerIconBackground"
      />
    </template>

    <!-- Actions -->
    <template v-if="showActions" #actions>
      <ModalActions
        :status="status"
        :status-color="statusColor"
        :status-variant="statusVariant"
        :category="category"
        :category-variant="categoryVariant"
        :metadata="metadata"
        :metadata-color="metadataColor"
        :metadata-variant="metadataBadgeVariant"
      >
        <slot name="actions" />
      </ModalActions>
    </template>

    <!-- Body -->
    <template #body>
      <ModalBody
        :loading="loading"
        :loading-message="loadingMessage"
        :error="error"
        :error-title="errorTitle"
      >
        <!-- Two-column layout for main content -->
        <div v-if="layout === 'two-column'" class="grid gap-6 lg:grid-cols-2">
          <!-- Source/Input Card -->
          <TextCard
            v-if="sourceText"
            :title="sourceTitle"
            :text="sourceText"
            :icon="sourceIcon"
            :icon-color="sourceIconColor"
            :variant="sourceVariant"
            :show-character-count="showCharacterCount"
            :max-length="maxLength"
            :truncate="sourceTruncate"
            :sanitize="sourceSanitize"
          >
            <template v-if="$slots.sourceFooter" #footer>
              <slot name="sourceFooter" />
            </template>
          </TextCard>

          <!-- Translation/Output Card -->
          <FormCard
            v-if="showTranslationCard"
            :title="translationTitle"
            :icon="translationIcon"
            :icon-color="translationIconColor"
            :variant="translationVariant"
            :validation-type="translationValidationType"
            :validation-value="translationValue"
            :is-valid="translationValid"
            :has-error="translationError"
            :error-message="translationErrorMessage"
            :help-text="translationHelpText"
          >
            <slot name="translationContent" />
            
            <!-- Retranslation section -->
            <div v-if="showRetranslation" class="mt-4">
              <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
                <div class="flex items-center gap-3">
                  <UIcon :name="retranslationIcon" class="text-primary w-4 h-4" />
                  <div>
                    <p class="text-sm font-medium text-gray-900 dark:text-white">{{ retranslationLabel }}</p>
                    <p class="text-xs text-gray-500 dark:text-gray-400">{{ retranslationDescription }}</p>
                  </div>
                </div>
                <UButton
                  color="primary"
                  variant="soft"
                  size="sm"
                  :loading="isRetranslating"
                  :disabled="isRetranslating || retranslationDisabled"
                  @click="emit('retranslate')"
                >
                  <UIcon :name="retranslationButtonIcon" class="w-4 h-4 mr-2" />
                  {{ isRetranslating ? retranslationLoadingText : retranslationButtonText }}
                </UButton>
              </div>
            </div>
          </FormCard>
        </div>

        <!-- Single column layout -->
        <div v-else-if="layout === 'single-column'" class="space-y-6">
          <!-- Text content -->
          <TextCard
            v-if="textContent"
            :title="textTitle"
            :text="textContent"
            :icon="textIcon"
            :icon-color="textIconColor"
            :variant="textVariant"
            :show-character-count="showCharacterCount"
            :max-length="maxLength"
            :truncate="textTruncate"
            :sanitize="textSanitize"
          />

          <!-- Form content -->
          <FormCard
            v-if="showFormCard"
            :title="formTitle"
            :icon="formIcon"
            :icon-color="formIconColor"
            :variant="formVariant"
            :validation-type="formValidationType"
            :validation-value="formValue"
            :is-valid="formValid"
            :has-error="formError"
            :error-message="formErrorMessage"
            :help-text="formHelpText"
          >
            <slot name="formContent" />
          </FormCard>

          <!-- Metadata content -->
          <MetadataCard
            v-if="metadataItems && metadataItems.length > 0"
            :title="metadataTitle"
            :items="metadataItems"
            :icon="metadataIcon"
            :icon-color="metadataIconColor"
            :variant="metadataVariant"
          >
            <template v-if="$slots.metadataFooter" #footer>
              <slot name="metadataFooter" />
            </template>
          </MetadataCard>
        </div>

        <!-- Custom layout -->
        <div v-else-if="layout === 'custom'" class="space-y-6">
          <slot name="content" />
        </div>

        <!-- Alert/Info section -->
        <UAlert
          v-if="alertMessage"
          :color="alertColor"
          :variant="alertVariant"
          :icon="alertIcon"
          :title="alertTitle"
          :description="alertMessage"
        />
      </ModalBody>
    </template>

    <!-- Footer -->
    <template v-if="showFooter" #footer>
      <ModalFooter
        :show-cancel="showCancel"
        :cancel-label="cancelLabel"
        :show-save="showSave"
        :save-label="saveLabel"
        :save-color="saveColor"
        :save-variant="saveVariant"
        :loading="loading"
        :disabled="disabled"
        :status-info="statusInfo"
        :keyboard-shortcuts="keyboardShortcuts"
        @cancel="$emit('cancel')"
        @save="$emit('save')"
      >
        <template v-if="$slots.footerActions" #actions>
          <slot name="footerActions" />
        </template>
      </ModalFooter>
    </template>
  </UModal>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import ModalHeader from './ModalHeader.vue'
import ModalActions from './ModalActions.vue'
import ModalBody from './ModalBody.vue'
import ModalFooter from './ModalFooter.vue'
import TextCard from './TextCard.vue'
import FormCard from './FormCard.vue'
import MetadataCard from './MetadataCard.vue'
import type { ModalColor } from '~/utils/modal'
// Define MetadataItem interface locally since it's not exported from MetadataCard
interface MetadataItem {
  label: string
  value: string
  icon?: string
  badge?: string
  badgeColor?: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  badgeVariant?: 'solid' | 'soft' | 'outline' | 'subtle'
  type?: 'text' | 'date' | 'file-size' | 'badge'
}

// Layout types
type LayoutType = 'two-column' | 'single-column' | 'custom'
type AlertColor = 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
type AlertVariant = 'solid' | 'soft' | 'outline' | 'subtle'

interface Props {
  // Modal state
  open: boolean
  dismissible?: boolean
  
  // Layout
  layout?: LayoutType
  
  // Header
  showHeader?: boolean
  title?: string
  description?: string
  headerIcon?: string
  headerIconColor?: ModalColor
  headerIconBackground?: ModalColor
  
  // Actions
  showActions?: boolean
  status?: string
  statusColor?: ModalColor
  statusVariant?: 'solid' | 'soft' | 'outline' | 'subtle'
  category?: string
  categoryVariant?: 'solid' | 'soft' | 'outline' | 'subtle'
  metadata?: string
  metadataColor?: ModalColor
  metadataBadgeVariant?: 'solid' | 'soft' | 'outline' | 'subtle'
  
  // Body state
  loading?: boolean
  loadingMessage?: string
  error?: string
  errorTitle?: string
  
  // Two-column layout props
  sourceText?: string
  sourceTitle?: string
  sourceIcon?: string
  sourceIconColor?: ModalColor
  sourceVariant?: 'outline' | 'solid' | 'soft' | 'subtle'
  sourceTruncate?: boolean
  sourceSanitize?: boolean
  
  showTranslationCard?: boolean
  translationTitle?: string
  translationIcon?: string
  translationIconColor?: ModalColor
  translationVariant?: 'outline' | 'solid' | 'soft' | 'subtle'
  translationValidationType?: 'text' | 'translation' | 'required' | 'custom'
  translationValue?: string
  translationValid?: boolean
  translationError?: boolean
  translationErrorMessage?: string
  translationHelpText?: string
  
  // Single column layout props
  textContent?: string
  textTitle?: string
  textIcon?: string
  textIconColor?: ModalColor
  textVariant?: 'outline' | 'solid' | 'soft' | 'subtle'
  textTruncate?: boolean
  textSanitize?: boolean
  
  showFormCard?: boolean
  formTitle?: string
  formIcon?: string
  formIconColor?: ModalColor
  formVariant?: 'outline' | 'solid' | 'soft' | 'subtle'
  formValidationType?: 'text' | 'translation' | 'required' | 'custom'
  formValue?: string
  formValid?: boolean
  formError?: boolean
  formErrorMessage?: string
  formHelpText?: string
  
  // Metadata
  metadataItems?: MetadataItem[]
  metadataTitle?: string
  metadataIcon?: string
  metadataIconColor?: ModalColor
  metadataVariant?: 'outline' | 'solid' | 'soft' | 'subtle'
  
  // Alert
  alertMessage?: string
  alertTitle?: string
  alertColor?: AlertColor
  alertVariant?: AlertVariant
  alertIcon?: string
  
  // Footer
  showFooter?: boolean
  showCancel?: boolean
  cancelLabel?: string
  showSave?: boolean
  saveLabel?: string
  saveColor?: ModalColor
  saveVariant?: 'solid' | 'soft' | 'outline' | 'subtle'
  disabled?: boolean
  statusInfo?: string
  keyboardShortcuts?: string
  
  // Common props
  showCharacterCount?: boolean
  maxLength?: number
  
  // Retranslation props
  showRetranslation?: boolean
  isRetranslating?: boolean
  retranslationDisabled?: boolean
  retranslationLabel?: string
  retranslationDescription?: string
  retranslationIcon?: string
  retranslationButtonIcon?: string
  retranslationButtonText?: string
  retranslationLoadingText?: string
  
  // Modal UI customization
  modalUi?: Record<string, string | number | boolean>
}

const props = withDefaults(defineProps<Props>(), {
  dismissible: true,
  layout: 'two-column',
  
  // Header defaults
  showHeader: true,
  title: 'Modal',
  description: '',
  headerIcon: 'i-lucide-edit-3',
  headerIconColor: 'primary',
  headerIconBackground: 'primary',
  
  // Actions defaults
  showActions: true,
  status: '',
  statusColor: 'neutral',
  statusVariant: 'soft',
  category: '',
  categoryVariant: 'soft',
  metadata: '',
  metadataColor: 'neutral',
  metadataBadgeVariant: 'outline',
  
  // Body defaults
  loading: false,
  loadingMessage: 'Loading...',
  error: '',
  errorTitle: 'Error',
  
  // Two-column defaults
  sourceText: '',
  sourceTitle: 'Source Text',
  sourceIcon: 'i-lucide-file-text',
  sourceIconColor: 'neutral',
  sourceVariant: 'outline',
  sourceTruncate: false,
  sourceSanitize: true,
  
  showTranslationCard: true,
  translationTitle: 'Translation',
  translationIcon: 'i-lucide-languages',
  translationIconColor: 'primary',
  translationVariant: 'outline',
  translationValidationType: 'translation',
  translationValue: '',
  translationErrorMessage: '',
  translationHelpText: '',
  
  // Single column defaults
  textContent: '',
  textTitle: 'Text Content',
  textIcon: 'i-lucide-file-text',
  textIconColor: 'primary',
  textVariant: 'outline',
  textTruncate: false,
  textSanitize: true,
  
  showFormCard: true,
  formTitle: 'Form',
  formIcon: 'i-lucide-edit',
  formIconColor: 'primary',
  formVariant: 'outline',
  formValidationType: 'text',
  formValue: '',
  formErrorMessage: '',
  formHelpText: '',
  
  // Metadata defaults
  metadataItems: () => [],
  metadataTitle: 'Metadata',
  metadataIcon: 'i-lucide-info',
  metadataIconColor: 'info',
  metadataVariant: 'outline',
  
  // Alert defaults
  alertMessage: '',
  alertTitle: '',
  alertColor: 'info',
  alertVariant: 'soft',
  alertIcon: 'i-lucide-info',
  
  // Footer defaults
  showFooter: true,
  showCancel: true,
  cancelLabel: 'Cancel',
  showSave: true,
  saveLabel: 'Save',
  saveColor: 'primary',
  saveVariant: 'solid',
  disabled: false,
  statusInfo: '',
  keyboardShortcuts: '',
  
  // Common defaults
  showCharacterCount: true,
  maxLength: 1000,
  
  // Retranslation defaults
  showRetranslation: false,
  isRetranslating: false,
  retranslationDisabled: false,
  retranslationLabel: 'AI Retranslation',
  retranslationDescription: 'Generate new translation using AI',
  retranslationIcon: 'i-lucide-refresh-cw',
  retranslationButtonIcon: 'i-lucide-sparkles',
  retranslationButtonText: 'Retranslate',
  retranslationLoadingText: 'Retranslating...',
  
  // Modal UI defaults
  modalUi: () => ({ content: 'max-w-6xl' })
})

const emit = defineEmits<{
  'update:open': [value: boolean]
  cancel: []
  save: []
  retranslate: []
}>()

// Computed properties for better reactivity
const open = computed({
  get: () => props.open,
  set: (value: boolean) => emit('update:open', value)
})

// No utilities to expose - Modal.vue is a pure UI component
</script>
