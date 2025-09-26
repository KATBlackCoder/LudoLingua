/**
 * Shared translation utilities
 * 
 * Common functions used across translation-related composables and components.
 * This eliminates code duplication while maintaining separation of concerns.
 */

/**
 * Get human-readable label for translation status
 */
export function getStatusLabel(status: string): string {
  switch (status) {
    case 'NotTranslated': return 'Not Translated'
    case 'MachineTranslated': return 'Machine Translated'
    case 'HumanReviewed': return 'Human Reviewed'
    case 'Ignored': return 'Ignored'
    default: return status
  }
}

/**
 * Get color class for translation status
 * Uses colors defined in app.config.ts
 */
export function getStatusColor(status: string): string {
  switch (status) {
    case 'NotTranslated': return 'neutral' // gray
    case 'MachineTranslated': return 'warning' // amber/yellow
    case 'HumanReviewed': return 'success' // emerald/green
    case 'Ignored': return 'error' // red
    default: return 'neutral' // gray
  }
}

/**
 * Format duration in milliseconds to human-readable format
 * @param ms Duration in milliseconds
 * @returns Formatted duration string (HH:MM:SS or MM:SS)
 */
export function formatDuration(ms: number): string {
  if (!Number.isFinite(ms) || ms < 0) ms = 0
  const totalSeconds = Math.floor(ms / 1000)
  const hours = Math.floor(totalSeconds / 3600)
  const minutes = Math.floor((totalSeconds % 3600) / 60)
  const seconds = totalSeconds % 60
  const hh = hours.toString().padStart(2, '0')
  const mm = minutes.toString().padStart(2, '0')
  const ss = seconds.toString().padStart(2, '0')
  return hours > 0 ? `${hh}:${mm}:${ss}` : `${mm}:${ss}`
}

/**
 * Get status options for select components
 */
export const statusOptions = [
  { label: 'Not Translated', value: 'NotTranslated' },
  { label: 'Machine Translated', value: 'MachineTranslated' },
  { label: 'Human Reviewed', value: 'HumanReviewed' },
  { label: 'Ignored', value: 'Ignored' }
]

/**
 * Get prompt type options for select components
 */
export const promptTypeOptions = [
  { label: 'Character', value: 'Character' },
  { label: 'Dialogue', value: 'Dialogue' },
  { label: 'System', value: 'System' },
  { label: 'Equipment', value: 'Equipment' },
  { label: 'Skill', value: 'Skill' },
  { label: 'Class', value: 'Class' },
  { label: 'State', value: 'State' },
  { label: 'Other', value: 'Other' }
]

/**
 * Get all status options including "All" for filtering
 */
export const statusFilterOptions = [
  { label: 'All', value: 'All' },
  ...statusOptions
]

/**
 * Get all prompt type options including "All" for filtering
 */
export const promptTypeFilterOptions = [
  { label: 'All', value: 'All' },
  ...promptTypeOptions
]

