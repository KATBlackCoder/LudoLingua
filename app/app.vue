<template>
  <UApp>
    <NuxtLayout name="default">
      <NuxtPage :transition="{ name: 'page', mode: 'out-in' }" />
    </NuxtLayout>
  </UApp>
</template>

<script setup lang="ts">
// App-wide setup code can go here
</script>

<style>
.page-enter-active,
.page-leave-active {
  transition: all 0.3s ease;
}
.page-enter-from,
.page-leave-to {
  opacity: 0;
  filter: blur(6px);
}
</style>
