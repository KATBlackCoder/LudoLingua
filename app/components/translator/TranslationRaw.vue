<template>
  <div class="space-y-6">
    <!-- Header Section -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <div class="p-2 bg-orange-50 dark:bg-orange-900/20 rounded-lg">
              <UIcon name="i-lucide-file-text" class="text-orange-500 w-5 h-5" />
            </div>
            <div>
              <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Raw Text</h3>
              <p class="text-sm text-gray-500 dark:text-gray-400">Untranslated text units ready for processing</p>
            </div>
          </div>
          <div class="flex items-center gap-2">
            <UBadge color="tertiary" variant="soft" size="sm">
              {{ rows.length }} items
            </UBadge>
          </div>
        </div>
      </template>
    </UCard>

    <!-- Raw Text Table -->
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-3">
            <UIcon name="i-lucide-table" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Untranslated Text Units</span>
          </div>
          <div class="flex items-center gap-2">
            <span class="text-xs text-gray-500 dark:text-gray-400">
              Page {{ page }} of {{ pageCount }}
            </span>
            <UPagination 
              v-model:page="page" 
              :total="rows.length" 
              :items-per-page="pageSize"
              size="sm"
            />
          </div>
        </div>
      </template>

      <UTable :data="pagedRows" class="text-sm">
        <template #prompt_type-data="{ row }">
          <UBadge :color="getPromptTypeColor(row.original.prompt_type)" variant="soft" size="sm">
            {{ row.original.prompt_type }}
          </UBadge>
        </template>
        <template #source_text-data="{ row }">
          <div class="max-w-md">
            <span class="whitespace-pre-wrap break-words text-sm">{{ row.original.source_text }}</span>
          </div>
        </template>
      </UTable>

      <template #footer>
        <div class="flex items-center justify-between text-xs text-gray-500 dark:text-gray-400">
          <span>
            Showing {{ pagedRows.length }} of {{ rows.length }} raw text units
          </span>
          <span>
            These items are ready for translation
          </span>
        </div>
      </template>
    </UCard>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { useTranslator } from '~/composables/useTranslator'

const { textUnits } = useTranslator()

const rows = computed(() => textUnits.value
  .filter(u => u.status === 'NotTranslated')
  .map(u => ({ id: u.id, prompt_type: u.prompt_type, source_text: u.source_text })))
const page = ref(1)
const pageSize = ref(25)
const pageCount = computed(() => Math.max(1, Math.ceil(rows.value.length / pageSize.value)))
const pagedRows = computed(() => {
  const start = (page.value - 1) * pageSize.value
  return rows.value.slice(start, start + pageSize.value)
})

const getPromptTypeColor = (promptType: string) => {
  const colorMap: Record<string, 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral'> = {
    'Character': 'primary',
    'Class': 'success',
    'Skill': 'info',
    'Equipment': 'warning',
    'State': 'error',
    'System': 'neutral',
    'Dialogue': 'secondary',
    'Other': 'neutral'
  }
  return colorMap[promptType] || 'neutral'
}
</script>


