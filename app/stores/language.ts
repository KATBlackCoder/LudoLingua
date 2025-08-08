import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { useAppToast } from "~/composables/useAppToast";
import { load } from "@tauri-apps/plugin-store";
import { invoke } from "@tauri-apps/api/core";
import type { Language } from "~/types/language";

export const useLanguageStore = defineStore("language", () => {
  // State
  const { showToast } = useAppToast();
  const languageOptions = ref<Language[]>([]);

  const currentSourceLanguage = ref("en");
  const currentTargetLanguage = ref("fr");

  // Computed

  const getLanguage = computed(() => {
    return {
      source_language: languageOptions.value.find(
        (lang: Language) => lang.id === currentSourceLanguage.value
      ),
      target_language: languageOptions.value.find(
        (lang: Language) => lang.id === currentTargetLanguage.value
      ),
    };
  });

  // Actions
  async function loadLanguageSettings() {
    const store = await load("ludollingua-settings.json", { autoSave: false });
    const val = await store.get<{
      source_language: { id: string; label: string; native_name: string };
      target_language: { id: string; label: string; native_name: string };
    }>("user_settings");
    currentSourceLanguage.value = val?.source_language.id || "en";
    currentTargetLanguage.value = val?.target_language.id || "fr";
  }
  
  async function fetchLanguageOptions() {
    try {
      const langs = await invoke<Language[]>("get_languages");
      languageOptions.value = langs;
    } catch (e) {
      // Fallback to a minimal set if backend fails
      languageOptions.value = [
        { id: "en", label: "English", native_name: "English", dir: 'ltr', enabled: true },
        { id: "fr", label: "French", native_name: "Français", dir: 'ltr', enabled: true },
        { id: "ja", label: "Japanese", native_name: "日本語", dir: 'ltr', enabled: true },
      ];
      console.error("Failed to load language options:", e);
    }
  }

  Promise.all([fetchLanguageOptions(), loadLanguageSettings()])
    .then(() => {
      const tgt = languageOptions.value.find(l => l.id === currentTargetLanguage.value)
      if (tgt?.dir) document.documentElement.setAttribute('dir', tgt.dir)
    })
    .catch(() => {})

  function setLanguage(sourceLanguage: string, targetLanguage: string) {
    currentSourceLanguage.value = sourceLanguage;
    currentTargetLanguage.value = targetLanguage;
    const tgt = languageOptions.value.find(l => l.id === targetLanguage)
    if (tgt?.dir) document.documentElement.setAttribute('dir', tgt.dir)
    showToast(
      "Language set to " +
        getLanguage.value.source_language?.label +
        " to " +
        getLanguage.value.target_language?.label,
      "You can change this in the settings",
      "primary",
      1000,
      "i-heroicons-check-circle"
    );
  }

  const resetToDefaults = () => {
    currentSourceLanguage.value = "en";
    currentTargetLanguage.value = "fr";
  };

  return {
    // State
    currentSourceLanguage,
    currentTargetLanguage,

    // Computed
    languageOptions,
    getLanguage,

    // Actions
    setLanguage,
    resetToDefaults,
  };
});
