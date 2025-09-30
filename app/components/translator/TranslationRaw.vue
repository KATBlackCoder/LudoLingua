<template>
  <div class="space-y-6 w-full">
    <!-- Raw Text Table using DataTable -->
    <DataTable
      :data="rows"
      :columns="tableColumns as any"
      :loading="false"
      title="Raw Text"
      icon="i-lucide-file-text"
      :show-filters="true"
      :show-search="true"
      :show-pagination="true"
      :show-row-count="true"
      :initial-page-size="25"
      :show-selection="false"
      :show-bulk-actions="false"
      :show-row-actions="false"
      :search-fields="['source_text', 'field_type']"
      filter-title="Search Raw Text"
      filter-icon="i-lucide-search"
      class="w-full"
    >
      <template #prompt_type-data="{ row }">
        <UBadge :color="getPromptTypeColor((row.original as { prompt_type: string }).prompt_type)" variant="soft" size="sm">
          {{ (row.original as { prompt_type: string }).prompt_type }}
        </UBadge>
      </template>
      <template #field_type-data="{ row }">
        <UBadge color="neutral" variant="outline" size="sm">
          {{ (row.original as { field_type: string }).field_type }}
        </UBadge>
      </template>
      <template #source_text-data="{ row }">
        <div class="max-w-md">
          <span class="whitespace-pre-wrap break-words text-sm">{{ (row.original as { source_text: string }).source_text }}</span>
        </div>
      </template>
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useTranslator } from '~/composables/useTranslator'
import DataTable from '~/components/shared/DataTable.vue'
import { getPromptTypeColor } from '~/utils/table'
import type { TableColumn } from '#ui/types'

const { textUnits } = useTranslator()

const rows = computed(() => textUnits.value
  .filter(u => u.status === 'NotTranslated')
  .map(u => ({ id: u.id, prompt_type: u.prompt_type, field_type: u.field_type, source_text: u.source_text })))

// Create table columns manually
const tableColumns: TableColumn<{ id: string; prompt_type: string; field_type: string; source_text: string }>[] = [
  {
    accessorKey: 'prompt_type',
    header: 'Type',
    size: 120,
    enableSorting: true
  },
  {
    accessorKey: 'field_type',
    header: 'Field Type',
    size: 120,
    enableSorting: true
  },
  {
    accessorKey: 'source_text',
    header: 'Source Text',
    size: 200,
    enableSorting: false
  }
]

// Using shared getPromptTypeColor utility function from utils/table.ts
</script>


