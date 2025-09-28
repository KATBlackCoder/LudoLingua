import { ref, readonly, onMounted, onUnmounted } from 'vue'

/**
 * Composable for detecting fullscreen state based on window width
 * Eliminates 5x duplication across table components
 */
export function useFullscreen() {
  const isFullscreen = ref(false)

  const handleResize = () => {
    isFullscreen.value = window.innerWidth >= 1920 // Consider 1920px+ as fullscreen
  }

  onMounted(() => {
    handleResize()
    window.addEventListener('resize', handleResize)
  })

  onUnmounted(() => {
    window.removeEventListener('resize', handleResize)
  })

  return {
    isFullscreen: readonly(isFullscreen)
  }
}
