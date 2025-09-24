<template>
  <div class="space-y-6">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div class="flex items-center gap-3">
        <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
          <UIcon name="i-lucide-book-open" class="text-primary w-5 h-5" />
        </div>
        <div>
          <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Glossary Manager</h1>
          <p class="text-sm text-gray-500 dark:text-gray-400">Define canonical terms used by the translator</p>
        </div>
      </div>
      <div class="flex items-center gap-2">
        <UButton 
          to="/translator" 
          variant="outline" 
          icon="i-lucide-arrow-left"
        >
          Back to Translator
        </UButton>
      </div>
    </div>

    <!-- Glossary Table -->
    <UCard>
      <GlossaryTable />
    </UCard>
  </div>
</template>

<script setup lang="ts">
import GlossaryTable from '~/components/glossaries/GlossaryTable.vue'
</script>


