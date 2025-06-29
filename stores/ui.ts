import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useUiStore = defineStore('ui', () => {
  // State
  const isSidebarOpen = ref(false);
  const isDarkMode = ref(false);
  const isLoading = ref(false);
  const loadingMessage = ref('');

  // Actions
  function toggleSidebar() {
    isSidebarOpen.value = !isSidebarOpen.value;
  }

  function toggleDarkMode() {
    isDarkMode.value = !isDarkMode.value;
  }

  function startLoading(message = 'Loading...') {
    isLoading.value = true;
    loadingMessage.value = message;
  }

  function stopLoading() {
    isLoading.value = false;
    loadingMessage.value = '';
  }

  return {
    // State
    isSidebarOpen,
    isDarkMode,
    isLoading,
    loadingMessage,
    
    // Actions
    toggleSidebar,
    toggleDarkMode,
    startLoading,
    stopLoading,
  };
}); 