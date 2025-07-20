import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { useAppToast } from "~/composables/useAppToast";

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
