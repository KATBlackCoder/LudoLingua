<template>
  <div class="min-h-screen flex flex-col bg-gray-50 dark:bg-gray-900">
    <!-- Use the AppHeader component -->
    <AppHeader />
    
    <main class="flex-grow py-6">
      <UContainer>
        <!-- Use slot to render the page content -->
        <slot />
      </UContainer>
    </main>
    
    <!-- Use the AppFooter component -->
    <AppFooter />
  </div>
</template>

<script setup lang="ts">
import AppHeader from '~/components/layout/AppHeader.vue';
import AppFooter from '~/components/layout/AppFooter.vue';

</script> 