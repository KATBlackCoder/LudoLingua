<template>
  <UCard>
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
            <UIcon name="i-lucide-sparkles" class="text-primary w-5 h-5" />
          </div>
          <div>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Key Features</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Core functionality and capabilities</p>
          </div>
        </div>
        <UBadge color="primary" variant="soft" size="sm">
          <UIcon name="i-lucide-star" class="mr-1" />
          Core
        </UBadge>
      </div>
    </template>

    <div class="space-y-6">
      <!-- Features Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
        <UCard v-for="feature in features" :key="feature.title" class="p-4">
          <div class="flex items-start gap-3">
            <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
              <UIcon :name="feature.icon" class="w-5 h-5 text-primary" />
            </div>
            <div class="flex-1">
              <h4 class="font-medium text-sm mb-1">{{ feature.title }}</h4>
              <p class="text-xs text-gray-600 dark:text-gray-400 leading-relaxed">{{ feature.description }}</p>
            </div>
          </div>
        </UCard>
      </div>

      <!-- Complete Workflow -->
      <UCard>
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-lucide-workflow" class="text-gray-500 w-4 h-4" />
            <span class="text-sm font-medium text-gray-700 dark:text-gray-300">Complete Workflow</span>
          </div>
        </template>
        <div class="space-y-3">
          <p class="text-sm text-gray-600 dark:text-gray-400">
            LudoLingua provides a complete end-to-end solution for RPG Maker game translation, from project loading to final export.
          </p>
          <div class="flex items-center justify-between text-xs">
            <span class="text-gray-500">Load Project</span>
            <UIcon name="i-lucide-arrow-right" class="text-gray-400" />
            <span class="text-gray-500">Extract Text</span>
            <UIcon name="i-lucide-arrow-right" class="text-gray-400" />
            <span class="text-gray-500">Translate</span>
            <UIcon name="i-lucide-arrow-right" class="text-gray-400" />
            <span class="text-gray-500">Export</span>
          </div>
        </div>
      </UCard>
    </div>
  </UCard>
</template>

<script setup lang="ts">
const features = [
  {
    title: 'Project Loading',
    icon: 'i-lucide-folder-open',
    description: 'Load and parse RPG Maker MV/MZ projects with automatic file detection and structure analysis.'
  },
  {
    title: 'Text Extraction',
    icon: 'i-lucide-file-text',
    description: 'Extract all translatable text from game files while preserving formatting and placeholders.'
  },
  {
    title: 'Smart Organization',
    icon: 'i-lucide-layers',
    description: 'Organize content by file type, prompt type, and translation status for efficient management.'
  },
  {
    title: 'AI Translation',
    icon: 'i-lucide-sparkles',
    description: 'AI-assisted translation with customizable prompts and context-aware terminology.'
  },
  {
    title: 'Translation Management',
    icon: 'i-lucide-list-checks',
    description: 'Comprehensive translation review, editing, and status tracking with bulk operations.'
  },
  {
    title: 'Glossary System',
    icon: 'i-lucide-book-open',
    description: 'Built-in glossary and termbase with automatic prompt injection for consistent translations.'
  },
  {
    title: 'File Export',
    icon: 'i-lucide-download',
    description: 'Inject translations back to original game files with proper formatting preservation.'
  },
  {
    title: 'Offline Operation',
    icon: 'i-lucide-shield-check',
    description: 'Work completely offline with local AI models or connect to cloud services as needed.'
  }
]
</script>


