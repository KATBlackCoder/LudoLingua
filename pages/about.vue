<template>
  <div>
    <UCard>
      <template #header>
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-semibold">About LudoLingua</h2>
        </div>
      </template>
      
      <div class="space-y-6">
        <section>
          <h3 class="text-lg font-medium mb-2">What is LudoLingua?</h3>
          <p>
            LudoLingua is a desktop application designed for translating RPG Maker game files.
            It helps game developers and translators manage and translate game text using AI assistance.
          </p>
        </section>
        
        <USeparator />
        
        <section>
          <h3 class="text-lg font-medium mb-2">Features</h3>
          <ul class="list-disc pl-5 space-y-2">
            <li>Load and parse RPG Maker MV/MZ projects</li>
            <li>Extract translatable text from game files</li>
            <li>Organize text by file and type</li>
            <li>AI-assisted translation (coming soon)</li>
            <li>Glossary/termbase management (coming soon)</li>
            <li>Save translated text back to game files (coming soon)</li>
          </ul>
        </section>
        
        <USeparator />
        
        <section>
          <h3 class="text-lg font-medium mb-2">Technology</h3>
          <p>
            LudoLingua is built with:
          </p>
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
        <p class="text-sm text-gray-500">
          LudoLingua is an open-source project. Contributions are welcome!
        </p>
      </template>
    </UCard>
  </div>
</template>

<script setup lang="ts">
// About page setup code can go here
</script> 