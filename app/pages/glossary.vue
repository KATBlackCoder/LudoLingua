<template>
  <UContainer class="py-6 space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-semibold">Glossary Manager</h1>
        <p class="text-sm text-muted">Define canonical terms used by the translator.</p>
      </div>
      <UButton to="/translation" variant="ghost" icon="i-heroicons-arrow-left">Back to Translation</UButton>
    </div>
    <GlossaryTable />
  </UContainer>
  
</template>

<script setup lang="ts">
import GlossaryTable from '~/components/glossaries/GlossaryTable.vue'
</script>


