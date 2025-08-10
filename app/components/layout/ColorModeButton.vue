<script setup lang="ts">
const colorMode = useColorMode()

const isDark = computed({
  get() {
    return colorMode.value === 'dark'
  },
  set(_isDark) {
    colorMode.preference = _isDark ? 'dark' : 'light'
  }
})
</script>

<template>
  <ClientOnly v-if="!colorMode?.forced">
    <UTooltip :text="isDark ? 'Switch to Light' : 'Switch to Dark'">
      <UButton
        :aria-label="isDark ? 'Light mode' : 'Dark mode'"
        :icon="isDark ? 'i-heroicons-moon' : 'i-heroicons-sun'"
        color="neutral"
        variant="ghost"
        square
        @click="isDark = !isDark"
      />
    </UTooltip>

    <template #fallback>
      <div class="size-8" />
    </template>
  </ClientOnly>
</template>
