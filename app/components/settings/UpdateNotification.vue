<template>
  <UCard v-if="hasUpdate" class="mb-4">
    <template #header>
      <div class="flex items-center gap-2">
        <UIcon name="i-lucide-download" class="text-primary" />
        <h3 class="text-lg font-semibold">Update Available</h3>
      </div>
    </template>

    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <div>
          <p class="text-sm text-gray-600 dark:text-gray-400">
            A new version is available for download
          </p>
          <p class="font-medium">
            Version {{ updateInfo?.version }}
          </p>
        </div>
        <UBadge color="success" variant="soft">
          New
        </UBadge>
      </div>

      <div v-if="updateInfo?.body" class="text-sm text-gray-600 dark:text-gray-400">
        <p class="font-medium mb-1">What's new:</p>
        <div class="prose prose-sm max-w-none dark:prose-invert">
          <pre class="whitespace-pre-wrap text-sm">{{ updateInfo.body }}</pre>
        </div>
      </div>

      <div class="flex gap-2">
        <UButton
          :loading="isBusy"
          :disabled="isBusy"
          color="primary"
          size="sm"
          @click="downloadAndInstall"
        >
          <UIcon name="i-lucide-download" class="mr-1" />
          Download & Install
        </UButton>
        
        <UButton
          :disabled="isBusy"
          variant="ghost"
          size="sm"
          @click="dismissUpdate"
        >
          <UIcon name="i-lucide-x" class="mr-1" />
          Dismiss
        </UButton>
      </div>

      <div v-if="error" class="p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-alert-triangle" class="text-red-500" />
          <p class="text-sm text-red-700 dark:text-red-300">
            {{ error }}
          </p>
        </div>
      </div>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { useUpdater } from '~/composables/useUpdater'

const {
  hasUpdate,
  updateInfo,
  error,
  isBusy,
  downloadAndInstall,
  dismissUpdate
} = useUpdater()

</script>
