import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { EngineInfo, GameDataFile } from '../types/engine';
import type { TextUnit } from '../types/translation';
import { useLanguageStore } from './language';

export const useEngineStore = defineStore('engine', () => {
  // Toast
  const toast = useToast();
  // State
  const projectInfo = ref<EngineInfo | null>(null);
  const textUnits = ref<TextUnit[]>([]);
  const gameDataFiles = ref<GameDataFile[]>([]);
  const isLoading = ref(false);
  const error = ref<string | null>(null);
  // Store references
  const languageStore = useLanguageStore();
  // Computed
  const hasProject = computed(() => projectInfo.value !== null);
  const projectName = computed(() => projectInfo.value?.name || 'No Project');
  const projectPath = computed(() => projectInfo.value?.path || '');
  const engineType = computed(() => projectInfo.value?.engine_type || 'Unknown');
  const getTextUnits = computed(() => textUnits.value);
  const getGameDataFiles = computed(() => gameDataFiles.value);
  const totalTextUnits = computed(() => textUnits.value.length);

  // Actions
  async function loadProject(projectPath: string) {
    try {
      isLoading.value = true;
      error.value = null;
         
      // Get language settings from the settings store
      const sourceLanguage = languageStore.getLanguage.source_language;
      const targetLanguage = languageStore.getLanguage.target_language;
      
      // Call the backend command to load the project with language parameters
      const result = await invoke<EngineInfo>('load_project', { 
        projectPath,
        sourceLanguage,
        targetLanguage
      });
      
      projectInfo.value = result;
      
      // Extract text units from the loaded project
      const extractedUnits = await invoke<TextUnit[]>('extract_text', { projectInfo: result });
      setTextUnits(extractedUnits);
      
      // Get game data files directly from the backend
      const files = await invoke<GameDataFile[]>('extract_game_data_files', { projectInfo: result });
      setGameDataFiles(files);

    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to load project';
      error.value = errorMessage;
      console.error('Failed to load project:', err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function refreshProject() {
    if (!projectInfo.value) {
      throw new Error('No project loaded');
    }
    
    await loadProject(projectInfo.value.path);
  }

  async function saveProject() {
    if (!projectInfo.value) {
      throw new Error('No project loaded');
    }

    try {
      isLoading.value = true;
      error.value = null;
          
      // Call the backend command to inject all text units back into project files
      await invoke('inject_text_units', {
        projectInfo: projectInfo.value,
        textUnits: textUnits.value
      });
      
      toast.add({
        title: 'Project saved successfully',
        description: 'All translations have been injected back into the project files',
        color: 'success',
        icon: 'i-heroicons-check-circle'
      });
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to save project';
      error.value = errorMessage;
      console.error('Failed to save project:', err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  function setGameDataFiles(files: GameDataFile[]) {
    gameDataFiles.value = files;
  }

  function setTextUnits(units: TextUnit[]) {
    textUnits.value = units;
  }

  function updateTextUnit(updatedUnit: TextUnit) {
    // Update in global text units array
    const globalIndex = textUnits.value.findIndex(u => u.id === updatedUnit.id);
    if (globalIndex !== -1) {
      textUnits.value[globalIndex] = updatedUnit;
    }

    // Update in game data files
    for (const file of gameDataFiles.value) {
      const unitIndex = file.text_units.findIndex(u => u.id === updatedUnit.id);
      if (unitIndex !== -1) {
        file.text_units[unitIndex] = updatedUnit;
        break;
      }
    }
  }

  function getTextUnitById(id: string): TextUnit | undefined {
    return textUnits.value.find(unit => unit.id === id);
  }

  function getTextUnitsByFile(filePath: string): TextUnit[] {
    const file = gameDataFiles.value.find(f => f.path === filePath);
    return file?.text_units || [];
  }

  function clearError() {
    error.value = null;
  }

  function reset() {
    projectInfo.value = null;
    textUnits.value = [];
    gameDataFiles.value = [];
    error.value = null;
    console.log('Engine store reset');
  }

  return {
    // State
    projectInfo,
    textUnits,
    gameDataFiles,
    isLoading,
    error,
    
    // Computed
    hasProject,
    projectName,
    projectPath,
    engineType,
    totalTextUnits,
    getTextUnits,
    getGameDataFiles,
    
    // Actions
    loadProject,
    refreshProject,
    saveProject,
    setGameDataFiles,
    setTextUnits,
    updateTextUnit,
    getTextUnitById,
    getTextUnitsByFile,
    clearError,
    reset,
  };
}); 