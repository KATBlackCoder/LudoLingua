<template>
  <div>
    <UContainer>
      <UCard class="max-w-4xl mx-auto">
        <template #header>
          <div class="flex items-center justify-between">
            <h2 class="text-xl font-semibold">About LudoLingua</h2>
          </div>
        </template>

        <div class="space-y-6">
          <section>
            <h3 class="text-lg font-medium mb-2">What is LudoLingua?</h3>
            <p class="text-muted">LudoLingua is a desktop application designed for translating RPG Maker game files. It helps game developers and translators manage and translate game text using AI assistance.</p>
          </section>

          <USeparator />

          <section>
            <h3 class="text-lg font-medium mb-2">Features</h3>
            <div class="grid grid-cols-1 sm:grid-cols-2 gap-2 text-sm">
              <UBadge color="primary" variant="soft">Load/parse RPG Maker MV/MZ</UBadge>
              <UBadge color="primary" variant="soft">Extract translatable text</UBadge>
              <UBadge color="primary" variant="soft">Organize by file/type</UBadge>
              <UBadge color="primary" variant="soft">AI-assisted translation</UBadge>
              <UBadge color="primary" variant="soft">Glossary/termbase</UBadge>
              <UBadge color="primary" variant="soft">Save back to files</UBadge>
            </div>
          </section>

          <USeparator />

          <section>
            <h3 class="text-lg font-medium mb-2">Technology</h3>
            <div class="flex flex-wrap gap-2 mt-2">
              <UBadge color="primary">Tauri</UBadge>
              <UBadge color="primary">Rust</UBadge>
              <UBadge color="primary">Nuxt.js</UBadge>
              <UBadge color="primary">TypeScript</UBadge>
              <UBadge color="primary">Nuxt UI</UBadge>
              <UBadge color="primary">Tailwind CSS</UBadge>
            </div>
          </section>
        </div>

        <template #footer>
          <p class="text-sm text-gray-500">LudoLingua is an open-source project. Contributions are welcome!</p>
        </template>
      </UCard>
    </UContainer>
  </div>
</template>

<script setup lang="ts">
// About page setup code can go here
</script> 