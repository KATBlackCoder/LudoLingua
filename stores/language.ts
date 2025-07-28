import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { useAppToast } from "~/composables/useAppToast";
import { load } from "@tauri-apps/plugin-store";

export const useLanguageStore = defineStore("language", () => {
  // State
  const { showToast } = useAppToast();
  const languageOptions = [
    { id: "en", label: "English", native_name: "English" },
    { id: "ja", label: "Japanese", native_name: "日本語" },
    { id: "zh", label: "Chinese", native_name: "中文" },
    { id: "es", label: "Spanish", native_name: "Español" },
    { id: "fr", label: "French", native_name: "Français" },
    { id: "de", label: "German", native_name: "Deutsch" },
    { id: "ko", label: "Korean", native_name: "한국어" },
    { id: "pt", label: "Portuguese", native_name: "Português" },
    { id: "ru", label: "Russian", native_name: "Русский" },
    { id: "it", label: "Italian", native_name: "Italiano" },
  ];

  const currentSourceLanguage = ref("en");
  const currentTargetLanguage = ref("fr");

  // Computed

  const getLanguage = computed(() => {
    return {
      source_language: languageOptions.find(
        (lang) => lang.id === currentSourceLanguage.value
      ),
      target_language: languageOptions.find(
        (lang) => lang.id === currentTargetLanguage.value
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
  loadLanguageSettings()

  function setLanguage(sourceLanguage: string, targetLanguage: string) {
    currentSourceLanguage.value = sourceLanguage;
    currentTargetLanguage.value = targetLanguage;
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
