import { defineStore } from 'pinia';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { EngineInfo, GameDataFile } from '../types/engine';
import type { TextUnit } from '../types/translation';

export const useProjectStore = defineStore('project', () => {
  // State
  const projectInfo = ref<EngineInfo | null>(null);
  const textUnits = ref<TextUnit[]>([]);
  const gameDataFiles = ref<GameDataFile[]>([]);
  const isLoading = ref(false);
  const currentFile = ref<GameDataFile | null>(null);

  // Actions
  async function loadProject(projectPath: string) {
    try {
      isLoading.value = true;
      
      // Call the backend command to load the project
      const result = await invoke<EngineInfo>('load_project', { projectPath });
      
      projectInfo.value = result;
      
      // Extract text units from the loaded project
      const extractedUnits = await invoke<TextUnit[]>('extract_text', { projectInfo: result });
      textUnits.value = extractedUnits;
      
      // Get game data files directly from the backend
      const files = await invoke<GameDataFile[]>('extract_game_data_files', { projectInfo: result });
      gameDataFiles.value = files;
      
      // Set the first file as current if available
      if (files.length > 0) {
        currentFile.value = files[0];
      }
    } catch (error) {
      console.error('Failed to load project:', error);
    } finally {
      isLoading.value = false;
    }
  }

  function setCurrentFile(file: GameDataFile) {
    currentFile.value = file;
  }

  function reset() {
    projectInfo.value = null;
    textUnits.value = [];
    gameDataFiles.value = [];
    currentFile.value = null;
  }
  
  // Get text units for the current file
  function getCurrentFileTextUnits() {
    return currentFile.value?.text_units || [];
  }

  return {
    // State
    projectInfo,
    textUnits,
    gameDataFiles,
    isLoading,
    currentFile,
    
    // Computed
    getCurrentFileTextUnits,
    
    // Actions
    loadProject,
    setCurrentFile,
    reset,
  };
}); 