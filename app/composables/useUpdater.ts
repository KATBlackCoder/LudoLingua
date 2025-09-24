import { ref, computed } from 'vue'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import { useNotifications } from './useNotifications'

export interface UpdateInfo {
  version: string
  date?: string
  body?: string
  size?: number
}

export const useUpdater = () => {
  const { notify } = useNotifications()
  
  const isChecking = ref(false)
  const isDownloading = ref(false)
  const isInstalling = ref(false)
  const updateInfo = ref<UpdateInfo | null>(null)
  const error = ref<string | null>(null)
  const downloadProgress = ref(0)
  const downloadStatus = ref('')
  const totalDownloaded = ref(0)

  const hasUpdate = computed(() => updateInfo.value !== null)
  const isBusy = computed(() => isChecking.value || isDownloading.value || isInstalling.value)

  // Auto-check for updates on initialization
  const initializeUpdater = async () => {
    try {
      await checkForUpdates(true) // Silent check on init
    } catch (err) {
      console.warn('Auto-update check failed:', err)
    }
  }

  const checkForUpdates = async (silent = false) => {
    if (isBusy.value) return

    try {
      isChecking.value = true
      error.value = null

      console.log('🔍 Checking for updates...')
      const update = await check()
      console.log('📦 Update check result:', update)
      
      if (update) {
        console.log('✅ Update available:', update.version)
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
        console.log('ℹ️ No updates available')
        updateInfo.value = null
        if (!silent) {
          await notify('No Updates', 'You are running the latest version')
        }
      }
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to check for updates'
      error.value = errorMessage
      console.error('Update check failed:', err)
      console.error('Error details:', {
        message: errorMessage,
        stack: err instanceof Error ? err.stack : 'No stack trace',
        type: typeof err
      })
      
      if (!silent) {
        await notify('Update Check Failed', errorMessage)
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
        // Log only safe progress info, not the entire event object
        if (event.event === 'Progress') {
          console.log('Download progress:', event.event, 'chunk:', event.data.chunkLength)
        } else {
          console.log('Download event:', event.event)
        }
        
        if (event.event === 'Progress') {
          const { chunkLength } = event.data
          // Accumulate total downloaded bytes
          totalDownloaded.value += chunkLength
          
          // Show only downloaded size
          const downloadedMB = Math.round(totalDownloaded.value / 1024 / 1024)
          downloadStatus.value = `Downloading... ${downloadedMB}MB`
        } else if (event.event === 'Started') {
          downloadStatus.value = 'Starting download...'
          downloadProgress.value = 0
          totalDownloaded.value = 0 // Reset counter
        } else if (event.event === 'Finished') {
          downloadStatus.value = 'Download completed!'
          downloadProgress.value = 100
        }
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
    downloadProgress.value = 0
    downloadStatus.value = ''
    totalDownloaded.value = 0
  }

    return {
    // State
    isChecking,
    isDownloading,
    isInstalling,
    updateInfo,
    error,
    downloadProgress,
    downloadStatus,
    totalDownloaded,
    
    // Computed
    hasUpdate,
    isBusy,
    
    // Actions
    checkForUpdates,
    downloadAndInstall,
    dismissUpdate,
    initializeUpdater
  }
}
