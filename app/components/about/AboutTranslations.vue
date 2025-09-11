<template>
  <section>
    <h3 class="text-lg font-medium mb-4">Translation Management</h3>
    
    <!-- Overview -->
    <div class="mb-6">
      <p class="text-muted mb-3">
        LudoLingua provides a comprehensive translation management system that allows you to view, edit, and organize all translations in your projects. 
        Access it through the <strong>Translations</strong> page once you have loaded a project.
      </p>
    </div>

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
          <UBadge color="primary" variant="solid" size="sm">1</UBadge>
          <div>
            <div class="font-medium">Load Project</div>
            <div class="text-sm text-muted">Extract translatable text from RPG Maker files</div>
          </div>
        </div>
        <div class="flex items-start gap-3">
          <UBadge color="primary" variant="solid" size="sm">2</UBadge>
          <div>
            <div class="font-medium">Translate Text</div>
            <div class="text-sm text-muted">Use AI assistance in the Translator workspace</div>
          </div>
        </div>
        <div class="flex items-start gap-3">
          <UBadge color="primary" variant="solid" size="sm">3</UBadge>
          <div>
            <div class="font-medium">Manage Translations</div>
            <div class="text-sm text-muted">Review, edit, and organize in the Translations page</div>
          </div>
        </div>
        <div class="flex items-start gap-3">
          <UBadge color="primary" variant="solid" size="sm">4</UBadge>
          <div>
            <div class="font-medium">Export Results</div>
            <div class="text-sm text-muted">Save translations back to your game files</div>
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
      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3 text-sm">
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-table" class="w-4 h-4 text-primary-500" />
          <span>View all translations in a searchable table</span>
        </div>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-pencil" class="w-4 h-4 text-primary-500" />
          <span>Edit translation text and status</span>
        </div>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-filter" class="w-4 h-4 text-primary-500" />
          <span>Filter by status and prompt type</span>
        </div>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-search" class="w-4 h-4 text-primary-500" />
          <span>Search source and translated text</span>
        </div>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-check-square" class="w-4 h-4 text-primary-500" />
          <span>Bulk select and delete operations</span>
        </div>
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-folder-x" class="w-4 h-4 text-primary-500" />
          <span>Project-level deletion and cleanup</span>
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
      <div class="space-y-3">
        <div class="flex items-center gap-3">
          <UBadge color="neutral" variant="soft" size="sm">Not Translated</UBadge>
          <span class="text-sm">Text that hasn't been translated yet</span>
        </div>
        <div class="flex items-center gap-3">
          <UBadge color="info" variant="soft" size="sm">Machine Translated</UBadge>
          <span class="text-sm">AI-generated translations ready for review</span>
        </div>
        <div class="flex items-center gap-3">
          <UBadge color="success" variant="soft" size="sm">Human Reviewed</UBadge>
          <span class="text-sm">Translations verified and approved by humans</span>
        </div>
        <div class="flex items-center gap-3">
          <UBadge color="warning" variant="soft" size="sm">Ignored</UBadge>
          <span class="text-sm">Text marked to skip during translation</span>
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
