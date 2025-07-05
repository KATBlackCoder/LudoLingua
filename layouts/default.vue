<template>
  <div class="min-h-screen flex flex-col bg-gray-50 dark:bg-gray-900">
    <!-- Use the AppHeader component -->
    <AppHeader />
    
    <!-- Project info alert shown when a project is loaded -->
    <UContainer class="mt-2">
      <ProjectInfoAlert />
    </UContainer>
    
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
import ProjectInfoAlert from '~/components/layout/ProjectInfoAlert.vue';
</script> 