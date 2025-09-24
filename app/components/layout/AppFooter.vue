<template>
  <UFooter>
    <template #left>
      <div class="flex items-center gap-4">
        <div class="flex items-center gap-2 text-sm text-gray-500 dark:text-gray-400">
          <div class="p-1 bg-gray-100 dark:bg-gray-800 rounded">
            <UIcon name="i-lucide-box" class="w-3 h-3" />
          </div>
          <span>{{ AppName }} v{{ AppVersion }}</span>
        </div>
        <div class="text-xs text-gray-400 dark:text-gray-500">
          Offline RPG Translation Tool
        </div>
      </div>
    </template>
    
    <template #right>
      <div class="flex items-center gap-3">
        <UTooltip text="View on GitHub">
          <UButton 
            color="neutral" 
            variant="ghost" 
            icon="i-lucide-github" 
            size="sm"
            target="_blank" 
            href="https://github.com/KATBlackCoder/LudoLingua" 
          />
        </UTooltip>
        <UButton
          color="neutral"
          variant="ghost"
          icon="i-lucide-info"
          size="sm"
          to="/about"
        >
          About
        </UButton>
      </div>
    </template>
  </UFooter>
</template>

<script setup lang="ts">
import { useAppInfo } from '~/composables/useAppInfo';

const AppName = ref();
const AppVersion = ref();

useAppInfo().getAppInfo().then((info) => {
  AppName.value = info.name;
  AppVersion.value = info.version;
});
</script> 