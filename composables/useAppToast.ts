export const useAppToast = () => {
  const toast = useToast()
  const showToast = (message: string, description: string, color?: 'primary' | 'success' | 'error' | 'warning' | 'info', duration?: number, icon?: string) => {
    toast.add({
      title: message,
      description: description,
      color: color || 'primary',
      duration: duration || 3000,
      icon: icon,
    })
  }

  return {
    showToast
  }
}
