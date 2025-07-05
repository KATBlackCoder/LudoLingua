<template>
  <footer class="border-t border-gray-200 dark:border-gray-800 py-4 mt-auto">
    <UContainer>
      <div class="flex justify-between items-center">
        <div class="text-sm text-gray-500 dark:text-gray-400">
          LudoLingua v0.1.0
        </div>
        <div class="flex items-center space-x-4">
          <UTooltip text="View on GitHub">
            <UButton
              color="neutral"
              variant="ghost"
              icon="i-heroicons-code-bracket"
              square
              target="_blank"
              href="https://github.com/yourusername/ludolingua"
            />
          </UTooltip>
          <UTooltip text="Toggle dark mode">
            <UButton
              color="neutral"
              variant="ghost"
              :icon="$colorMode.value === 'dark' ? 'i-heroicons-sun' : 'i-heroicons-moon'"
              square
              @click="$colorMode.preference = $colorMode.value === 'dark' ? 'light' : 'dark'"
            />
          </UTooltip>
        </div>
      </div>
    </UContainer>
  </footer>
</template>

<script setup lang="ts">
// Footer component logic can go here
</script> 