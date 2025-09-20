import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification'

export const useNotifications = () => {
  /**
   * Check if notification permission is granted and request if needed
   */
  const checkAndRequestPermission = async (): Promise<boolean> => {
    try {
      let permissionGranted = await isPermissionGranted()
      
      if (!permissionGranted) {
        const result = await requestPermission()
        permissionGranted = result === 'granted'
      }
      
      return permissionGranted
    } catch (error) {
      console.warn('Notification permission error:', error)
      return false
    }
  }

  /**
   * Send a notification if permissions are granted
   */
  const notify = async (title: string, body: string): Promise<void> => {
    try {
      const hasPermission = await checkAndRequestPermission()
      
      if (hasPermission) {
        await sendNotification({
          title,
          body
        })
      }
    } catch (error) {
      console.warn('Failed to send notification:', error)
    }
  }

  /**
   * Notify when a project is loaded successfully
   */
  const notifyProjectLoaded = async (projectName: string, textUnitCount: number, engineType: string): Promise<void> => {
    await notify(
      'Project Loaded',
      `${projectName} (${textUnitCount} text units, ${engineType})`
    )
  }

  /**
   * Notify when batch translation completes
   */
  const notifyTranslationComplete = async (successCount: number, totalCount: number, failedCount: number): Promise<void> => {
    const failedText = failedCount > 0 ? `, ${failedCount} failed` : ''
    await notify(
      'Translation Complete',
      `${successCount}/${totalCount} units translated${failedText}`
    )
  }

  return {
    checkAndRequestPermission,
    notify,
    notifyProjectLoaded,
    notifyTranslationComplete
  }
}
