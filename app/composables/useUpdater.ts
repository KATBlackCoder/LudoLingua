import { ref, computed } from 'vue'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import { useNotifications } from './useNotifications'

export interface UpdateInfo {
  version: string
  date?: string
  body?: string
}

export const useUpdater = () => {
  const { notify } = useNotifications()
  
  const isChecking = ref(false)
  const isDownloading = ref(false)
  const isInstalling = ref(false)
  const updateInfo = ref<UpdateInfo | null>(null)
  const error = ref<string | null>(null)

  const hasUpdate = computed(() => updateInfo.value !== null)
  const isBusy = computed(() => isChecking.value || isDownloading.value || isInstalling.value)

  const checkForUpdates = async (silent = false) => {
    if (isBusy.value) return

    try {
      isChecking.value = true
      error.value = null

      const update = await check()
      
      if (update) {
        updateInfo.value = {
          version: update.version,
          date: update.date,
          body: update.body
        }
        
        if (!silent) {
          // Show notification for available update
          await notify('Update Available', `Version ${update.version} is available for download`)
        }
      } else {
        updateInfo.value = null
        if (!silent) {
          await notify('No Updates', 'You are running the latest version')
        }
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to check for updates'
      console.error('Update check failed:', err)
      
      if (!silent) {
        await notify('Update Check Failed', error.value)
      }
    } finally {
      isChecking.value = false
    }
  }

  const downloadAndInstall = async () => {
    if (!hasUpdate.value || isBusy.value) return

    try {
      isDownloading.value = true
      error.value = null

      const update = await check()
      if (!update) {
        throw new Error('No update available')
      }

      // Download the update
      await update.downloadAndInstall((event) => {
        console.log('Download progress:', event)
        // You can emit progress events here if needed
      })

      isDownloading.value = false
      isInstalling.value = true

      // Show completion notification
      await notify('Update Downloaded', 'The update has been downloaded and will be installed on restart')

      // Restart the application
      await relaunch()
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Failed to download update'
      console.error('Update download failed:', err)
      
      await notify('Update Failed', error.value)
    } finally {
      isDownloading.value = false
      isInstalling.value = false
    }
  }

  const dismissUpdate = () => {
    updateInfo.value = null
    error.value = null
  }

  return {
    // State
    isChecking,
    isDownloading,
    isInstalling,
    updateInfo,
    error,
    
    // Computed
    hasUpdate,
    isBusy,
    
    // Actions
    checkForUpdates,
    downloadAndInstall,
    dismissUpdate
  }
}
