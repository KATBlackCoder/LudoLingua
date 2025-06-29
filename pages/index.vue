<template>
  <div>
    <UCard class="max-w-3xl mx-auto">
      <template #header>
        <div class="flex items-center justify-between">
          <h2 class="text-xl font-semibold">Welcome to LudoLingua</h2>
        </div>
      </template>
      
      <div class="space-y-4">
        <p>
          LudoLingua is a desktop application for translating RPG Maker game files.
          It helps you manage and translate game text using AI assistance.
        </p>
        
        <div class="space-y-4">
          <UButton
            label="Load Project"
            icon="i-heroicons-folder-open"
            color="primary"
            size="lg"
            class="mt-4"
            disabled
          />
          
          <UDivider />
          
          <div>
            <p class="mb-2">Test Rust Command:</p>
            <UInput v-model="name" placeholder="Enter your name" class="mb-2" />
            <UButton
              label="Say Hello"
              icon="i-heroicons-hand-raised"
              color="secondary"
              @click="sayHello"
            />
            <p v-if="response" class="mt-2 text-green-600">{{ response }}</p>
          </div>
        </div>
      </div>
      
      <template #footer>
        <p class="text-sm text-gray-500">
          To get started, click the "Load Project" button to open an RPG Maker project.
        </p>
      </template>
    </UCard>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const name = ref('');
const response = ref('');

async function sayHello() {
  try {
    response.value = await invoke('hello', { name: name.value || 'Guest' });
  } catch (error) {
    console.error('Error calling Rust function:', error);
    response.value = `Error: ${error}`;
  }
}
</script> 