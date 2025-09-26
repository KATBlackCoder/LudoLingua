<template>
  <div class="min-h-dvh flex flex-col bg-gray-50 dark:bg-gray-900">
    <AppHeader />

    <UMain class="flex-1">
      <UContainer class="py-8 max-w-none">
        <slot />
      </UContainer>
    </UMain>

    <AppFooter />
  </div>
</template>

<script setup lang="ts">
import AppHeader from '~/components/layout/AppHeader.vue';
import AppFooter from '~/components/layout/AppFooter.vue';

</script> 