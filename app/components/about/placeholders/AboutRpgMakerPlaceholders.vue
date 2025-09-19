<template>
  <div class="space-y-4">
    <UCard>
      <template #header>
        <span class="font-medium">Formatting & Names</span>
      </template>
      <UTable :data="formattingData" :columns="columns" />
    </UCard>

    <UCard>
      <template #header>
        <span class="font-medium">Variables, Items & Entities</span>
      </template>
      <UTable :data="entitiesData" :columns="columns" />
    </UCard>

    <UCard>
      <template #header>
        <span class="font-medium">Conditional Expressions</span>
      </template>
      <UTable :data="conditionalData" :columns="columns" />
    </UCard>
  </div>
</template>

<script setup lang="ts">
const columns = [
  {
    accessorKey: 'placeholder',
    header: 'Placeholder'
  },
  {
    accessorKey: 'function',
    header: 'Function'
  },
  {
    accessorKey: 'example',
    header: 'Example'
  }
]

const formattingData = [
  {
    placeholder: '[COLOR_n]',
    function: 'Color index',
    example: '[COLOR_5] → \\C[5]'
  },
  {
    placeholder: '[NAME_n]',
    function: 'Name by index',
    example: '[NAME_1] → \\N[1]'
  },
  {
    placeholder: '[NEWLINE_n]',
    function: 'Insert n newlines',
    example: '[NEWLINE_2] → \\n[2]'
  }
]

const entitiesData = [
  {
    placeholder: '[VARIABLE_...]',
    function: 'Variables (uppercase)',
    example: '[VARIABLE_1] → \\V[1]'
  },
  {
    placeholder: '[variable_...]',
    function: 'Variables (lowercase)',
    example: '[variable_1] → \\v[1]'
  },
  {
    placeholder: '[SWITCH_...]',
    function: 'Switches',
    example: '[SWITCH_1] → \\S[1]'
  },
  {
    placeholder: '[ITEM_...]',
    function: 'Items',
    example: '[ITEM_1] → \\I[1]'
  },
  {
    placeholder: '[WEAPON_...]',
    function: 'Weapons',
    example: '[WEAPON_1] → \\W[1]'
  },
  {
    placeholder: '[ARMOR_...]',
    function: 'Armor',
    example: '[ARMOR_1] → \\A[1]'
  },
  {
    placeholder: '[ACTOR_...]',
    function: 'Actors',
    example: '[ACTOR_1] → \\P[1]'
  },
  {
    placeholder: '[GOLD]',
    function: 'Gold',
    example: '[GOLD] → \\G'
  },
  {
    placeholder: '[CURRENCY]',
    function: 'Currency',
    example: '[CURRENCY] → \\$'
  }
]

const conditionalData = [
  {
    placeholder: '[CONDITIONAL_v$1>$2]',
    function: 'Conditional expressions',
    example: '[CONDITIONAL_v1>5] → en(v[1]>5)'
  }
]
</script>