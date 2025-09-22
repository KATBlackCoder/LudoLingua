<template>
  <UCard>
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-refresh-cw" class="text-primary" />
          <h3 class="text-lg font-semibold">Updates</h3>
        </div>
        <UButton
          :loading="isChecking"
          :disabled="isBusy"
          variant="outline"
          size="sm"
          @click="() => checkForUpdates()"
        >
          <UIcon name="i-lucide-refresh-cw" class="mr-1" />
          Check for Updates
        </UButton>
      </div>
    </template>

    <div class="space-y-4">
      <!-- Current Version Info -->
      <div class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg">
        <div>
          <p class="text-sm text-gray-600 dark:text-gray-400">Current Version</p>
          <p class="font-medium">{{ currentVersion }}</p>
        </div>
        <UBadge color="neutral" variant="soft">
          Installed
        </UBadge>
      </div>

      <!-- Update Status -->
      <div v-if="!hasUpdate && !isChecking" class="text-center py-4">
        <UIcon name="i-lucide-check-circle" class="text-green-500 text-2xl mx-auto mb-2" />
        <p class="text-sm text-gray-600 dark:text-gray-400">
          You are running the latest version
        </p>
      </div>

      <!-- Checking Status -->
      <div v-if="isChecking" class="text-center py-4">
        <UIcon name="i-lucide-refresh-cw" class="text-primary text-2xl mx-auto mb-2 animate-spin" />
        <p class="text-sm text-gray-600 dark:text-gray-400">
          Checking for updates...
        </p>
      </div>

      <!-- Error State -->
      <div v-if="error && !isChecking" class="p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg">
        <div class="flex items-center gap-2">
          <UIcon name="i-lucide-alert-triangle" class="text-red-500" />
          <div>
            <p class="text-sm font-medium text-red-700 dark:text-red-300">
              Update Check Failed
            </p>
            <p class="text-xs text-red-600 dark:text-red-400">
              {{ error }}
            </p>
          </div>
        </div>
        <UButton
          variant="ghost"
          size="xs"
          class="mt-2"
          @click="() => checkForUpdates()"
        >
          Try Again
        </UButton>
      </div>

      <!-- Auto-Update Settings -->
      <div class="border-t pt-4">
        <h4 class="text-sm font-medium mb-3">Update Preferences</h4>
        <div class="space-y-3">
          <UCheckbox
            v-model="autoCheckEnabled"
            label="Automatically check for updates on startup"
            :disabled="isBusy"
          />
          <UCheckbox
            v-model="showUpdateNotifications"
            label="Show notifications for available updates"
            :disabled="isBusy"
          />
        </div>
      </div>

      <!-- Manual Update Actions -->
      <div class="border-t pt-4">
        <h4 class="text-sm font-medium mb-3">Manual Actions</h4>
        <div class="flex gap-2">
          <UButton
            :loading="isChecking"
            :disabled="isBusy"
            variant="outline"
            size="sm"
            @click="() => checkForUpdates()"
          >
            <UIcon name="i-lucide-refresh-cw" class="mr-1" />
            Check Now
          </UButton>
          
          <UButton
            v-if="hasUpdate"
            :loading="isBusy"
            :disabled="isBusy"
            color="primary"
            size="sm"
            @click="downloadAndInstall"
          >
            <UIcon name="i-lucide-download" class="mr-1" />
            Download Update
          </UButton>
        </div>
      </div>
    </div>
  </UCard>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useUpdater } from '~/composables/useUpdater'
import { useAppInfo } from '~/composables/useAppInfo'

const {
  hasUpdate,
  error,
  isChecking,
  isBusy,
  checkForUpdates,
  downloadAndInstall
} = useUpdater()

const { getAppInfo } = useAppInfo()
const currentVersion = ref('')

// Auto-update settings
const autoCheckEnabled = ref(true)
const showUpdateNotifications = ref(true)

// Check for updates on component mount if auto-check is enabled
onMounted(async () => {
  const appInfo = await getAppInfo()
  currentVersion.value = appInfo.version
  
  if (autoCheckEnabled.value) {
    await checkForUpdates(true) // Silent check
  }
})
</script>
