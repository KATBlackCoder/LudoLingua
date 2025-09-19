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
        <span class="font-medium">Special Wolf RPG Codes</span>
      </template>
      <UTable :data="specialData" :columns="columns" />
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
    example: '[COLOR_5] → \\c[5]'
  },
  {
    placeholder: '[ICON_n]',
    function: 'Icon index',
    example: '[ICON_1] → \\\\i[1]'
  },
  {
    placeholder: '[FONT_n]',
    function: 'Font/face index',
    example: '[FONT_1] → \\\\f[1]'
  },
  {
    placeholder: '[WOLF_END]',
    function: 'End/reset formatting',
    example: '[WOLF_END] → \\E'
  },
  {
    placeholder: '[RUBY_START]',
    function: 'Ruby marker start',
    example: '[RUBY_START] → \\\\r'
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

const specialData = [
  {
    placeholder: '[AT_n]',
    function: 'Event/command markers',
    example: '[AT_1] → @1'
  },
  {
    placeholder: '[SLOT_n]',
    function: 'Character/slot references',
    example: '[SLOT_1] → \\\\s[1]'
  },
  {
    placeholder: '[CSELF_n]',
    function: 'Self-referencing color codes',
    example: '[CSELF_1] → \\\\cself[1]'
  }
]
</script>