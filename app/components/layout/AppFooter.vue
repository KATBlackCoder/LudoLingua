<template>
  <footer class="border-t border-gray-200 dark:border-gray-800 py-4 mt-auto">
    <UContainer>
      <div class="flex justify-between items-center gap-3">
        <div class="text-sm text-gray-500 dark:text-gray-400 flex items-center gap-2">
          <UIcon name="i-lucide-box" />
          <span> {{ AppName }} v{{ AppVersion }} </span>
        </div>
        <div class="flex items-center gap-2">
          <UTooltip text="View on GitHub">
            <UButton color="neutral" variant="ghost" icon="i-lucide-github" square target="_blank" href="https://github.com/yourusername/ludolingua" />
          </UTooltip>
          <UTooltip text="Toggle Color Mode">
            <ColorModeButton />
          </UTooltip>
        </div>
      </div>
    </UContainer>
  </footer>
</template>

<script setup lang="ts">
import ColorModeButton from './ColorModeButton.vue';
import { useAppInfo } from '~/composables/useAppInfo';

const AppName = ref();
const AppVersion = ref();

useAppInfo().getAppInfo().then((info) => {
  AppName.value = info.name;
  AppVersion.value = info.version;
});

</script> 