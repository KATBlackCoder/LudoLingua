<template>
  <div class="flex items-center justify-between">
    <div class="flex items-center gap-3">
      <UIcon :name="icon" class="text-gray-500 w-4 h-4" />
      <span class="text-sm font-medium text-gray-700 dark:text-gray-300">{{ title }}</span>
    </div>
    <div class="flex items-center gap-2">
      <span v-if="showPaginationInfo" class="text-xs text-gray-500 dark:text-gray-400">
        Page {{ currentPage }} of {{ totalPages }}
      </span>
      <UPagination
        v-if="showPagination"
        v-model:page="currentPage"
        :items-per-page="pageSize"
        :total="totalItems"
        size="sm"
      />
      <slot name="actions" />
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  title?: string
  icon?: string
  showPagination?: boolean
  showPaginationInfo?: boolean
  currentPage?: number
  totalPages?: number
  pageSize?: number
  totalItems?: number
}

interface Emits {
  (e: 'update:currentPage', page: number): void
}

const props = withDefaults(defineProps<Props>(), {
  title: 'Table Data',
  icon: 'i-lucide-table',
  showPagination: false,
  showPaginationInfo: false,
  currentPage: 1,
  totalPages: 1,
  pageSize: 25,
  totalItems: 0
})

const emit = defineEmits<Emits>()

const currentPage = computed({
  get: () => props.currentPage,
  set: (page: number) => emit('update:currentPage', page)
})
</script>
