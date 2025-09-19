<template>
  <section>
    <div class="flex items-center gap-3 mb-4">
      <div class="flex items-center gap-2">
        <UIcon name="i-lucide-languages" class="text-primary" />
        <h3 class="text-xl font-semibold">Translation Management</h3>
      </div>
      <UBadge color="primary" variant="soft" size="sm">
        <UIcon name="i-lucide-list-checks" class="mr-1" />
        Complete
      </UBadge>
    </div>
    
    <!-- Overview -->
    <UCard class="mb-6">
      <template #header>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-info" class="text-primary" />
          <span class="font-medium">Overview</span>
        </div>
      </template>
      <p class="text-sm text-gray-600 dark:text-gray-400 leading-relaxed">
        LudoLingua provides a comprehensive translation management system that allows you to view, edit, and organize all translations in your projects. 
        Access it through the <strong>Translations</strong> page once you have loaded a project.
      </p>
    </UCard>

    <!-- Translation Workflow -->
    <UCard class="mb-6">
      <template #header>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-workflow" class="w-5 h-5" />
          <span class="font-medium">Translation Workflow</span>
        </div>
      </template>
      <div class="space-y-4">
        <div class="flex items-start gap-3">
          <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
            <UIcon name="i-lucide-folder-open" class="w-4 h-4 text-primary" />
          </div>
          <div>
            <div class="font-medium">Load Project</div>
            <div class="text-sm text-gray-600 dark:text-gray-400">Extract translatable text from RPG Maker files</div>
          </div>
        </div>
        <div class="flex items-start gap-3">
          <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
            <UIcon name="i-lucide-sparkles" class="w-4 h-4 text-primary" />
          </div>
          <div>
            <div class="font-medium">Translate Text</div>
            <div class="text-sm text-gray-600 dark:text-gray-400">Use AI assistance in the Translator workspace</div>
          </div>
        </div>
        <div class="flex items-start gap-3">
          <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
            <UIcon name="i-lucide-list-checks" class="w-4 h-4 text-primary" />
          </div>
          <div>
            <div class="font-medium">Manage Translations</div>
            <div class="text-sm text-gray-600 dark:text-gray-400">Review, edit, and organize in the Translations page</div>
          </div>
        </div>
        <div class="flex items-start gap-3">
          <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
            <UIcon name="i-lucide-download" class="w-4 h-4 text-primary" />
          </div>
          <div>
            <div class="font-medium">Export Results</div>
            <div class="text-sm text-gray-600 dark:text-gray-400">Save translations back to your game files</div>
          </div>
        </div>
      </div>
    </UCard>

    <!-- Translation Features -->
    <UCard class="mb-6">
      <template #header>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-list-checks" class="w-5 h-5" />
          <span class="font-medium">Management Features</span>
        </div>
      </template>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <div class="flex items-start gap-3 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
          <UIcon name="i-lucide-table" class="w-5 h-5 text-primary mt-0.5" />
          <div>
            <div class="font-medium text-sm">Searchable Table</div>
            <div class="text-xs text-gray-600 dark:text-gray-400">View all translations in a comprehensive table</div>
          </div>
        </div>
        <div class="flex items-start gap-3 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
          <UIcon name="i-lucide-pencil" class="w-5 h-5 text-primary mt-0.5" />
          <div>
            <div class="font-medium text-sm">Edit & Update</div>
            <div class="text-xs text-gray-600 dark:text-gray-400">Edit translation text and status</div>
          </div>
        </div>
        <div class="flex items-start gap-3 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
          <UIcon name="i-lucide-filter" class="w-5 h-5 text-primary mt-0.5" />
          <div>
            <div class="font-medium text-sm">Smart Filtering</div>
            <div class="text-xs text-gray-600 dark:text-gray-400">Filter by status and prompt type</div>
          </div>
        </div>
        <div class="flex items-start gap-3 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
          <UIcon name="i-lucide-search" class="w-5 h-5 text-primary mt-0.5" />
          <div>
            <div class="font-medium text-sm">Text Search</div>
            <div class="text-xs text-gray-600 dark:text-gray-400">Search source and translated text</div>
          </div>
        </div>
        <div class="flex items-start gap-3 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
          <UIcon name="i-lucide-check-square" class="w-5 h-5 text-primary mt-0.5" />
          <div>
            <div class="font-medium text-sm">Bulk Operations</div>
            <div class="text-xs text-gray-600 dark:text-gray-400">Bulk select and delete operations</div>
          </div>
        </div>
        <div class="flex items-start gap-3 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
          <UIcon name="i-lucide-folder-x" class="w-5 h-5 text-primary mt-0.5" />
          <div>
            <div class="font-medium text-sm">Project Cleanup</div>
            <div class="text-xs text-gray-600 dark:text-gray-400">Project-level deletion and cleanup</div>
          </div>
        </div>
      </div>
    </UCard>

    <!-- Translation Status -->
    <UCard class="mb-6">
      <template #header>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-tags" class="w-5 h-5" />
          <span class="font-medium">Translation Status</span>
        </div>
      </template>
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <div class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
          <UBadge color="neutral" variant="soft" size="sm">Not Translated</UBadge>
          <div>
            <div class="text-sm font-medium">Untranslated</div>
            <div class="text-xs text-gray-600 dark:text-gray-400">Text that hasn't been translated yet</div>
          </div>
        </div>
        <div class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
          <UBadge color="info" variant="soft" size="sm">Machine Translated</UBadge>
          <div>
            <div class="text-sm font-medium">AI Generated</div>
            <div class="text-xs text-gray-600 dark:text-gray-400">AI-generated translations ready for review</div>
          </div>
        </div>
        <div class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
          <UBadge color="success" variant="soft" size="sm">Human Reviewed</UBadge>
          <div>
            <div class="text-sm font-medium">Verified</div>
            <div class="text-xs text-gray-600 dark:text-gray-400">Translations verified and approved by humans</div>
          </div>
        </div>
        <div class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-800/50 rounded-lg">
          <UBadge color="warning" variant="soft" size="sm">Ignored</UBadge>
          <div>
            <div class="text-sm font-medium">Skipped</div>
            <div class="text-xs text-gray-600 dark:text-gray-400">Text marked to skip during translation</div>
          </div>
        </div>
      </div>
    </UCard>

    <!-- Prompt Types -->
    <UCard class="mb-6">
      <template #header>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-message-square-text" class="w-5 h-5" />
          <span class="font-medium">Content Types</span>
        </div>
      </template>
      <div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
        <UBadge color="secondary" variant="soft" class="justify-center">Character</UBadge>
        <UBadge color="secondary" variant="soft" class="justify-center">Dialogue</UBadge>
        <UBadge color="secondary" variant="soft" class="justify-center">System</UBadge>
        <UBadge color="secondary" variant="soft" class="justify-center">Equipment</UBadge>
        <UBadge color="secondary" variant="soft" class="justify-center">Skill</UBadge>
        <UBadge color="secondary" variant="soft" class="justify-center">Class</UBadge>
        <UBadge color="secondary" variant="soft" class="justify-center">State</UBadge>
        <UBadge color="secondary" variant="soft" class="justify-center">Other</UBadge>
      </div>
      <div class="mt-3">
        <p class="text-sm text-muted">
          Each text unit is categorized by content type, allowing for specialized translation prompts and better organization.
        </p>
      </div>
    </UCard>

    <!-- Tips & Best Practices -->
    <UAlert color="info" variant="soft" icon="i-lucide-lightbulb">
      <template #title>Tips for Managing Translations</template>
      <template #description>
        <ul class="text-sm space-y-1 mt-2">
          <li>• Use filters to focus on specific content types or translation statuses</li>
          <li>• Review machine translations and mark them as "Human Reviewed" when verified</li>
          <li>• Use bulk operations to efficiently manage large numbers of translations</li>
          <li>• Regularly clean up old project data to keep your database organized</li>
          <li>• Check translation statistics to track your progress</li>
        </ul>
      </template>
    </UAlert>
  </section>
</template>
