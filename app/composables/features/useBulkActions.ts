import { ref, computed, type Ref } from 'vue'

export interface BulkAction {
  label: string
  color: 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'
  variant: 'solid' | 'soft' | 'outline' | 'subtle' | 'ghost'
  icon: string
  loading?: boolean
  disabled?: boolean
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  onClick?: () => void | Promise<void>
}

export interface BulkActionsState {
  isProcessing: Ref<boolean>
  currentAction: Ref<string | null>
  errors: Ref<string[]>
}

export interface BulkActionsActions {
  executeAction: (action: BulkAction) => Promise<void>
  setProcessing: (processing: boolean) => void
  setCurrentAction: (action: string | null) => void
  addError: (error: string) => void
  clearErrors: () => void
}

export interface BulkActionsComputed {
  hasErrors: Ref<boolean>
  canExecute: Ref<boolean>
}

export function useBulkActions(
  selectedCount: Ref<number>,
  options: {
    onActionStart?: (action: string) => void
    onActionComplete?: (action: string) => void
    onActionError?: (action: string, error: string) => void
  } = {}
): BulkActionsState & BulkActionsActions & BulkActionsComputed {
  
  // State
  const isProcessing = ref(false)
  const currentAction = ref<string | null>(null)
  const errors = ref<string[]>([])

  // Computed
  const hasErrors = computed(() => errors.value.length > 0)
  const canExecute = computed(() => selectedCount.value > 0 && !isProcessing.value)

  // Actions
  const executeAction = async (action: BulkAction) => {
    if (!action.onClick || !canExecute.value) return

    try {
      isProcessing.value = true
      currentAction.value = action.label
      options.onActionStart?.(action.label)

      await action.onClick()
      
      options.onActionComplete?.(action.label)
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred'
      errors.value.push(`${action.label}: ${errorMessage}`)
      options.onActionError?.(action.label, errorMessage)
    } finally {
      isProcessing.value = false
      currentAction.value = null
    }
  }

  const setProcessing = (processing: boolean) => {
    isProcessing.value = processing
  }

  const setCurrentAction = (action: string | null) => {
    currentAction.value = action
  }

  const addError = (error: string) => {
    errors.value.push(error)
  }

  const clearErrors = () => {
    errors.value = []
  }

  return {
    // State
    isProcessing,
    currentAction,
    errors,
    // Computed
    hasErrors,
    canExecute,
    // Actions
    executeAction,
    setProcessing,
    setCurrentAction,
    addError,
    clearErrors
  }
}

/**
 * Creates bulk action configurations with dynamic labels and counts
 */
export function createBulkActions(
  selectedCount: Ref<number>,
  actions: {
    label: string
    color: BulkAction['color']
    variant: BulkAction['variant']
    icon: string
    onClick: () => void | Promise<void>
    minSelection?: number
  }[]
): BulkAction[] {
  return actions
    .filter(action => selectedCount.value >= (action.minSelection || 1))
    .map(action => ({
      ...action,
      label: action.minSelection && action.minSelection > 1 
        ? `${action.label} (${selectedCount.value})`
        : action.label,
      disabled: false,
      loading: false,
      size: 'sm' as const,
      onClick: action.onClick
    }))
}
