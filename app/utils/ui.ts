/**
 * Pure UI utility functions
 * No state management - just pure functions for styling and formatting
 */

/**
 * Badge color mapping functions
 */
export function getBadgeColor(status: string): 'primary' | 'secondary' | 'tertiary' | 'info' | 'success' | 'warning' | 'error' | 'neutral' {
  const statusLower = status.toLowerCase()
  
  if (statusLower.includes('success') || statusLower.includes('complete') || statusLower.includes('done')) {
    return 'success'
  }
  if (statusLower.includes('error') || statusLower.includes('failed')) {
    return 'error'
  }
  if (statusLower.includes('warning') || statusLower.includes('pending')) {
    return 'warning'
  }
  if (statusLower.includes('info') || statusLower.includes('processing')) {
    return 'info'
  }
  
  return 'neutral'
}

/**
 * Icon mapping utilities
 */
export function getStatusIcon(status: string): string {
  const statusLower = status.toLowerCase()
  
  if (statusLower.includes('success') || statusLower.includes('complete')) {
    return 'i-lucide-check-circle'
  }
  if (statusLower.includes('error') || statusLower.includes('failed')) {
    return 'i-lucide-x-circle'
  }
  if (statusLower.includes('warning') || statusLower.includes('pending')) {
    return 'i-lucide-alert-circle'
  }
  if (statusLower.includes('info') || statusLower.includes('processing')) {
    return 'i-lucide-info'
  }
  
  return 'i-lucide-help-circle'
}

export function getPromptTypeIcon(promptType: string): string {
  const iconMap: Record<string, string> = {
    'Character': 'i-lucide-user',
    'Class': 'i-lucide-shield',
    'Skill': 'i-lucide-zap',
    'Equipment': 'i-lucide-sword',
    'State': 'i-lucide-heart',
    'System': 'i-lucide-settings',
    'Dialogue': 'i-lucide-message-circle',
    'Other': 'i-lucide-file-text'
  }
  return iconMap[promptType] || 'i-lucide-file-text'
}

export function getActionIcon(action: string): string {
  const actionLower = action.toLowerCase()
  
  if (actionLower.includes('edit') || actionLower.includes('modify')) {
    return 'i-lucide-edit'
  }
  if (actionLower.includes('delete') || actionLower.includes('remove')) {
    return 'i-lucide-trash-2'
  }
  if (actionLower.includes('translate') || actionLower.includes('process')) {
    return 'i-lucide-languages'
  }
  if (actionLower.includes('add') || actionLower.includes('create')) {
    return 'i-lucide-plus'
  }
  if (actionLower.includes('view') || actionLower.includes('show')) {
    return 'i-lucide-eye'
  }
  if (actionLower.includes('copy') || actionLower.includes('duplicate')) {
    return 'i-lucide-copy'
  }
  
  return 'i-lucide-more-horizontal'
}

/**
 * Text formatting helpers
 */
export function truncateText(text: string, maxLength: number): string {
  if (text.length <= maxLength) return text
  return text.substring(0, maxLength) + '...'
}

export function formatFileSize(bytes: number): string {
  const units = ['B', 'KB', 'MB', 'GB', 'TB']
  let size = bytes
  let unitIndex = 0
  
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024
    unitIndex++
  }
  
  return `${size.toFixed(1)} ${units[unitIndex]}`
}

export function formatTimeAgo(date: Date): string {
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffSeconds = Math.floor(diffMs / 1000)
  const diffMinutes = Math.floor(diffSeconds / 60)
  const diffHours = Math.floor(diffMinutes / 60)
  const diffDays = Math.floor(diffHours / 24)
  
  if (diffDays > 0) {
    return `${diffDays} day${diffDays > 1 ? 's' : ''} ago`
  }
  if (diffHours > 0) {
    return `${diffHours} hour${diffHours > 1 ? 's' : ''} ago`
  }
  if (diffMinutes > 0) {
    return `${diffMinutes} minute${diffMinutes > 1 ? 's' : ''} ago`
  }
  return 'Just now'
}

/**
 * Responsive breakpoint helpers
 */
export function getResponsiveClass(baseClass: string, isFullscreen: boolean): string {
  if (isFullscreen) {
    return baseClass.replace('md:', '').replace('lg:', '')
  }
  return baseClass
}

export function getTableWidthClass(isFullscreen: boolean): string {
  return isFullscreen ? 'w-full' : 'w-full md:w-auto'
}

export function getButtonSizeClass(isFullscreen: boolean, size: 'xs' | 'sm' | 'md' | 'lg' = 'sm'): string {
  if (isFullscreen) {
    const sizeMap = { xs: 'sm', sm: 'md', md: 'lg', lg: 'xl' }
    return sizeMap[size] || 'md'
  }
  return size
}

/**
 * Color utilities
 */
export function getColorVariant(baseColor: string, variant: 'solid' | 'soft' | 'outline' | 'subtle' | 'ghost' = 'soft'): string {
  return `${baseColor}-${variant}`
}

export function getTextColor(bgColor: string): string {
  const darkColors = ['red', 'blue', 'green', 'purple', 'indigo']
  const isDark = darkColors.some(color => bgColor.includes(color))
  return isDark ? 'text-white' : 'text-gray-900'
}

/**
 * Validation helpers
 */
export function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  return emailRegex.test(email)
}

export function isValidUrl(url: string): boolean {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}

export function sanitizeText(text: string): string {
  return text
    .replace(/[<>]/g, '') // Remove potential HTML tags
    .trim()
    .substring(0, 1000) // Limit length
}
