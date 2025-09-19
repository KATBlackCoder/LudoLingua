<template>
  <UFooter>
    <template #left>
      <div class="text-sm text-gray-500 dark:text-gray-400 flex items-center gap-2">
        <UIcon name="i-lucide-box" />
        <span>{{ AppName }} v{{ AppVersion }}</span>
      </div>
    </template>
    
    <template #right>
      <div class="flex items-center gap-2">
        <UTooltip text="View on GitHub">
          <UButton 
            color="neutral" 
            variant="ghost" 
            icon="i-lucide-github" 
            square 
            target="_blank" 
            href="https://github.com/yourusername/ludolingua" 
          />
        </UTooltip>
        <UColorModeButton />
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