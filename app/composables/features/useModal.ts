import { ref, computed, shallowRef, type Ref } from 'vue'

export interface ModalState<T = unknown> {
  isOpen: Ref<boolean>
  activeItem: Ref<T | null>
  isLoading: Ref<boolean>
  error: Ref<string | null>
}

export interface ModalActions<T = unknown> {
  open: (item?: T) => void
  close: () => void
  openEdit: (id: number, getItemFn: (id: number) => T | null) => void
  openAdd: (defaultItem?: Partial<T>) => void
  setLoading: (loading: boolean) => void
  setError: (error: string | null) => void
  reset: () => void
}

export interface ModalComputed {
  hasActiveItem: Ref<boolean>
  isModalDisabled: Ref<boolean>
  modalTitle: Ref<string>
}

export function useModal<T = unknown>(
  options: {
    defaultItem?: T
    onOpen?: (item: T | null) => void
    onClose?: (item: T | null) => void
    getItemById?: (id: number) => T | null
  } = {}
): ModalState<T> & ModalActions<T> & ModalComputed {
  
  // State
  const isOpen = ref(false)
  const activeItem = shallowRef<T | null>(null)
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Computed
  const hasActiveItem = computed(() => activeItem.value !== null)
  const isModalDisabled = computed(() => isLoading.value || error.value !== null)
  const modalTitle = computed(() => {
    if (!hasActiveItem.value) return 'Add New Item'
    return 'Edit Item'
  })

  // Actions
  const open = (item?: T) => {
    activeItem.value = item || options.defaultItem || null
    isOpen.value = true
    error.value = null
    options.onOpen?.(activeItem.value)
  }

  const close = () => {
    const currentItem = activeItem.value
    isOpen.value = false
    activeItem.value = null
    isLoading.value = false
    error.value = null
    options.onClose?.(currentItem)
  }

  const openEdit = (id: number, getItemFn?: (id: number) => T | null) => {
    const getItem = getItemFn || options.getItemById
    if (!getItem) {
      error.value = 'No item getter function provided'
      return
    }
    
    const item = getItem(id)
    if (item) {
      activeItem.value = { ...item }
      isOpen.value = true
      error.value = null
      options.onOpen?.(activeItem.value)
    } else {
      error.value = 'Item not found'
    }
  }

  const openAdd = (defaultItem?: Partial<T>) => {
    const newItem = { ...options.defaultItem, ...defaultItem } as T
    activeItem.value = newItem
    isOpen.value = true
    error.value = null
    options.onOpen?.(activeItem.value)
  }

  const setLoading = (loading: boolean) => {
    isLoading.value = loading
  }

  const setError = (errorMsg: string | null) => {
    error.value = errorMsg
  }

  const reset = () => {
    isOpen.value = false
    activeItem.value = null
    isLoading.value = false
    error.value = null
  }

  return {
    // State
    isOpen,
    activeItem,
    isLoading,
    error,
    // Computed
    hasActiveItem,
    isModalDisabled,
    modalTitle,
    // Actions
    open,
    close,
    openEdit,
    openAdd,
    setLoading,
    setError,
    reset
  }
}
