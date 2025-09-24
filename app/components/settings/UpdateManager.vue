<template>
  <UCard>
    <template #header>
      <div class="flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="p-2 bg-primary-50 dark:bg-primary-900/20 rounded-lg">
            <UIcon name="i-lucide-refresh-cw" class="text-primary w-5 h-5" />
          </div>
          <div>
            <h3 class="text-lg font-semibold text-gray-900 dark:text-white">Application Updates</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">Keep your app up to date with the latest features</p>
          </div>
        </div>
        <UButton
          :loading="isChecking"
          :disabled="isBusy"
          variant="outline"
          size="sm"
          @click="() => checkForUpdates()"
        >
          <UIcon name="i-lucide-refresh-cw" class="w-4 h-4 mr-2" />
          Check for Updates
        </UButton>
      </div>
    </template>

    <div class="space-y-6">
      <!-- Version Info Card -->
      <UCard 
        :ui="{ body: 'p-0' }"
        :class="hasUpdate 
          ? 'ring-2 ring-primary-200 dark:ring-primary-800' 
          : ''"
      >
        <div class="p-6">
          <div class="flex items-center justify-between">
            <div class="space-y-1">
              <div class="flex items-center gap-2">
                <UIcon 
                  :name="hasUpdate ? 'i-lucide-arrow-up-circle' : 'i-lucide-check-circle'" 
                  :class="hasUpdate ? 'text-primary' : 'text-green-500'"
                  class="w-5 h-5"
                />
                <p class="text-sm font-medium text-gray-600 dark:text-gray-400">
                  {{ hasUpdate ? 'Available Version' : 'Current Version' }}
                </p>
              </div>
              <p class="text-2xl font-bold text-gray-900 dark:text-white">
                {{ hasUpdate ? updateInfo?.version : currentVersion }}
              </p>
            </div>
            <UBadge 
              :color="hasUpdate ? 'primary' : 'success'" 
              variant="soft"
              size="lg"
              class="text-sm font-medium"
            >
              {{ hasUpdate ? 'New' : 'Installed' }}
            </UBadge>
          </div>
        </div>
      </UCard>

      <!-- Status Messages -->
      <div v-if="!hasUpdate && !isChecking && !error" class="text-center py-8">
        <div class="inline-flex items-center justify-center w-16 h-16 bg-green-100 dark:bg-green-900/20 rounded-full mb-4">
          <UIcon name="i-lucide-check-circle" class="text-green-500 w-8 h-8" />
        </div>
        <h4 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">You're up to date!</h4>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          You are running the latest version
        </p>
      </div>

      <div v-if="isChecking" class="text-center py-8">
        <div class="inline-flex items-center justify-center w-16 h-16 bg-primary-100 dark:bg-primary-900/20 rounded-full mb-4">
          <UIcon name="i-lucide-refresh-cw" class="text-primary w-8 h-8 animate-spin" />
        </div>
        <h4 class="text-lg font-semibold text-gray-900 dark:text-white mb-2">Checking for updates...</h4>
        <p class="text-sm text-gray-600 dark:text-gray-400">
          Please wait while we check for the latest version
        </p>
      </div>

      <!-- Download Progress -->
      <UAlert
        v-if="isDownloading"
        color="info"
        variant="soft"
        icon="i-lucide-download"
        title="Downloading Update"
        :description="downloadStatus"
        class="p-4"
      >
        <template #default>
          <div class="space-y-3">
            <UProgress 
              :model-value="null"
              color="info"
              size="lg"
              animation="swing"
            />
          </div>
        </template>
      </UAlert>

      <!-- Installation Progress -->
      <UAlert
        v-if="isInstalling"
        color="success"
        variant="soft"
        icon="i-lucide-refresh-cw"
        title="Installing Update"
        description="The update is being installed. The app will restart automatically."
        class="p-4"
      >
        <template #default>
          <div class="space-y-3">
            <UProgress 
              :model-value="null"
              color="success"
              size="lg"
              animation="swing"
            />
          </div>
        </template>
      </UAlert>

      <!-- Error State -->
      <UAlert
        v-if="error && !isChecking"
        color="error"
        variant="soft"
        icon="i-lucide-alert-triangle"
        title="Update Check Failed"
        :description="error"
        class="p-4"
      >
        <template #actions>
          <UButton
            variant="ghost"
            size="sm"
            @click="() => checkForUpdates()"
          >
            <UIcon name="i-lucide-refresh-cw" class="w-4 h-4 mr-2" />
            Try Again
          </UButton>
        </template>
      </UAlert>

      <!-- Update Actions -->
      <div v-if="hasUpdate && !isDownloading && !isInstalling" class="space-y-4">
        <div class="border-t pt-6">
          <h4 class="text-sm font-semibold text-gray-900 dark:text-white mb-4">Update Actions</h4>
          <div class="flex gap-3">
            <UButton
              v-if="!isDownloading && !isInstalling"
              :loading="isBusy"
              :disabled="isBusy"
              color="primary"
              size="lg"
              @click="downloadAndInstall"
            >
              <UIcon name="i-lucide-download" class="w-4 h-4 mr-2" />
              Download & Install Update
            </UButton>
          </div>
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
  updateInfo,
  error,
  isChecking,
  isBusy,
  isDownloading,
  isInstalling,
  downloadStatus,
  checkForUpdates,
  downloadAndInstall,
  initializeUpdater
} = useUpdater()

const { getAppInfo } = useAppInfo()
const currentVersion = ref('')

// Initialize current version and check for updates on component mount
onMounted(async () => {
  const appInfo = await getAppInfo()
  currentVersion.value = appInfo.version
  
  // Auto-check for updates when component mounts
  await initializeUpdater()
})
</script>
