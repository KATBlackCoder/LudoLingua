<template>
  <div v-if="show" :class="containerClass">
    <label :class="labelClass">
      {{ label }}
    </label>
    
    <!-- Select input -->
    <USelect
      v-if="type === 'select'"
      v-model="selectValue"
      :items="items"
      :placeholder="placeholder"
      :multiple="multiple"
      :disabled="disabled"
      :loading="loading"
      :size="inputSize"
      :ui="{ content: dropdownMinWidth }"
    />
    
    <!-- Input menu (autocomplete) -->
    <UInputMenu
      v-else-if="type === 'input-menu'"
      v-model="selectValue"
      :items="items"
      :placeholder="placeholder"
      :multiple="multiple"
      :disabled="disabled"
      :loading="loading"
      :value-key="(valueKey as any) || 'value'"
      :size="inputSize"
      :ui="{ content: dropdownMinWidth }"
    />
    
    <!-- Input field -->
    <UInput
      v-else-if="type === 'input'"
      v-model="inputValue"
      :placeholder="placeholder"
      :disabled="disabled"
      :loading="loading"
      :size="inputSize"
    />
    
    <!-- Range slider -->
    <div v-else-if="type === 'range'">
      <div class="flex items-center gap-2">
        <!-- Decrement button -->
        <UButton
          class="shrink-0"
          :disabled="disabled || (rangeValue as number) <= rangeMin"
          variant="outline"
          :size="isCompactMode ? 'xs' : 'sm'"
          icon="i-lucide-minus"
          @click="decrementRange"
        />
        
        <!-- Slider -->
        <USlider
          v-model="rangeValue"
          :min="rangeMin"
          :max="rangeMax"
          :step="rangeStep"
          :disabled="disabled"
          :size="inputSize"
          class="flex-1"
        />
        
        <!-- Increment button -->
        <UButton
          class="shrink-0"
          :disabled="disabled || (rangeValue as number) >= rangeMax"
          variant="outline"
          :size="isCompactMode ? 'xs' : 'sm'"
          icon="i-lucide-plus"
          @click="incrementRange"
        />
      </div>
      
      <div v-if="rangeLabel" class="text-xs text-gray-500 dark:text-gray-400 text-center mt-1">
        {{ rangeLabel }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  // Visibility
  show?: boolean
  
  // Field configuration
  label: string
  placeholder?: string
  type?: 'select' | 'input-menu' | 'input' | 'range'
  
  // Layout configuration
  compact?: boolean
  totalFilters?: number
  
  // Select configuration
  items?: Array<{ label: string; value: string }>
  multiple?: boolean
  disabled?: boolean
  loading?: boolean
  valueKey?: string
  
  // Range configuration
  rangeMin?: number
  rangeMax?: number
  rangeStep?: number
  rangeLabel?: string
  
  // Model value
  modelValue: string | string[] | number
}

interface Emits {
  (e: 'update:modelValue', value: string | string[] | number): void
}

const props = withDefaults(defineProps<Props>(), {
  show: true,
  placeholder: 'All',
  type: 'select',
  compact: false,
  totalFilters: 1,
  items: () => [],
  multiple: false,
  disabled: false,
  loading: false,
  valueKey: 'value',
  rangeMin: 0,
  rangeMax: 100,
  rangeStep: 1,
  rangeLabel: ''
})

const emit = defineEmits<Emits>()

// Responsive design computed properties
const isCompactMode = computed(() => {
  return props.compact || props.totalFilters > 3
})

const containerClass = computed(() => {
  return isCompactMode.value ? 'space-y-0.5' : 'space-y-1'
})

const labelClass = computed(() => {
  return isCompactMode.value 
    ? 'text-xs font-medium text-gray-600 dark:text-gray-400' 
    : 'text-sm font-medium text-gray-700 dark:text-gray-300'
})

const inputSize = computed(() => {
  return isCompactMode.value ? 'xs' : 'sm'
})

const dropdownMinWidth = computed(() => {
  return isCompactMode.value ? 'min-w-[160px]' : 'min-w-[200px]'
})

// Separate computed properties for different input types
const selectValue = computed({
  get: () => props.modelValue as string | string[],
  set: (value: string | string[]) => emit('update:modelValue', value)
})

const inputValue = computed({
  get: () => props.modelValue as string,
  set: (value: string) => emit('update:modelValue', value)
})

const rangeValue = computed({
  get: () => props.modelValue as number,
  set: (value: number) => emit('update:modelValue', value)
})

// Range increment/decrement methods
const incrementRange = () => {
  const currentValue = props.modelValue as number
  if (typeof currentValue === 'number' && currentValue < props.rangeMax) {
    const newValue = Math.min(currentValue + props.rangeStep, props.rangeMax)
    emit('update:modelValue', newValue)
  }
}

const decrementRange = () => {
  const currentValue = props.modelValue as number
  if (typeof currentValue === 'number' && currentValue > props.rangeMin) {
    const newValue = Math.max(currentValue - props.rangeStep, props.rangeMin)
    emit('update:modelValue', newValue)
  }
}
</script>
