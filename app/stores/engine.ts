import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { EngineInfo, GameDataFile } from '../types/engine';
import type { TextUnit } from '../types/translation';
import { useLanguageStore } from './language';

/**
 * Engine store
 *
 * Manages the currently loaded project (`EngineInfo`), extracted `TextUnit`s,
 * and structured `GameDataFile`s. Provides actions to load, refresh, and save
 * a project by invoking Tauri commands, plus utilities to query and update
 * text units.
 */
export const useEngineStore = defineStore('engine', () => {
  // UI notifications are handled in components/composables, keep store UI-agnostic
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

      // If this is an exported subset with a manifest, merge pre-translated units
      const merged = await invoke<TextUnit[] | null>('load_subset_with_manifest', { projectInfo: result });
      if (merged && Array.isArray(merged) && merged.length > 0) {
        setTextUnits(merged);
      }
      
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
      
      // success handled by caller (UI/composable)
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to save project';
      error.value = errorMessage;
      console.error('Failed to save project:', err);
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  // removed exportTranslatedCopy

  // Export only the minimal subtree (e.g., www/data + detection artifacts) and inject into that copy
  async function exportTranslatedSubset(destinationRoot: string) {
    if (!projectInfo.value) {
      throw new Error('No project loaded');
    }

    try {
      isLoading.value = true;
      error.value = null;

      const exportedPath = await invoke<string>('export_translated_subset', {
        projectInfo: projectInfo.value,
        textUnits: textUnits.value,
        destinationRoot
      });

      return exportedPath;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to export translated subset';
      error.value = errorMessage;
      console.error('Failed to export translated subset:', err);
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
    exportTranslatedSubset,
    setGameDataFiles,
    setTextUnits,
    updateTextUnit,
    getTextUnitById,
    getTextUnitsByFile,
    clearError,
    reset,
  };
}); 