/**
 * Modal validation composable
 * Reactive validation management for modal components
 */

import { computed, type Ref } from 'vue'
import { getAutoValidation, getValidationIcon, getValidationColor, type ValidationType } from '~/utils/modal'

/**
 * Composable for managing modal form validation
 * Used by FormCard for smart validation
 */
export function useModalValidation(
  value: Ref<string>,
  validationType: Ref<ValidationType>,
  customValidation?: Ref<boolean>
) {
  const autoValidation = computed(() => 
    getAutoValidation(value.value, validationType.value, customValidation?.value)
  )
  
  const validationIcon = computed(() => 
    getValidationIcon(autoValidation.value, false)
  )
  
  const validationColor = computed(() => 
    getValidationColor(autoValidation.value, false)
  )
  
  return {
    autoValidation,
    validationIcon,
    validationColor
  }
}

/**
 * Composable for managing modal validation with error states
 * Used by FormCard with error handling
 */
export function useModalValidationWithErrors(
  value: Ref<string>,
  validationType: Ref<ValidationType>,
  hasError: Ref<boolean>,
  customValidation?: Ref<boolean>
) {
  const autoValidation = computed(() => 
    getAutoValidation(value.value, validationType.value, customValidation?.value)
  )
  
  const validationIcon = computed(() => 
    getValidationIcon(autoValidation.value, hasError.value)
  )
  
  const validationColor = computed(() => 
    getValidationColor(autoValidation.value, hasError.value)
  )
  
  return {
    autoValidation,
    validationIcon,
    validationColor
  }
}
