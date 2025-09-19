<template>
  <div class="space-y-4">
    <UCard>
      <template #header>
        <span class="font-medium">Arguments & Parameters</span>
      </template>
      <UTable :data="argumentData" :columns="columns" />
    </UCard>

    <UCard>
      <template #header>
        <span class="font-medium">Whitespace & Formatting</span>
      </template>
      <UTable :data="whitespaceData" :columns="columns" />
    </UCard>

    <UCard>
      <template #header>
        <span class="font-medium">Control & Flow</span>
      </template>
      <UTable :data="controlData" :columns="columns" />
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

const argumentData = [
  {
    placeholder: '[ARG_n]',
    function: 'Argument placeholder',
    example: '[ARG_1] → %1'
  },
  {
    placeholder: '[NUM_PREFIX_ddd]',
    function: 'Numeric prefix token',
    example: '[NUM_PREFIX_100] → 100＿'
  }
]

const whitespaceData = [
  {
    placeholder: '[FWSPC_n]',
    function: 'n full-width spaces (U+3000)',
    example: '[FWSPC_2] →   (2 full-width spaces)'
  },
  {
    placeholder: '[SPC_n]',
    function: 'n ASCII spaces',
    example: '[SPC_3] →    (3 ASCII spaces)'
  },
  {
    placeholder: '[TAB_n]',
    function: 'n tabs',
    example: '[TAB_1] → \t'
  },
  {
    placeholder: '[NEWLINE]',
    function: 'Actual newline',
    example: '[NEWLINE] → \n'
  },
  {
    placeholder: '[CARRIAGE_RETURN]',
    function: 'Carriage return',
    example: '[CARRIAGE_RETURN] → \r'
  }
]

const controlData = [
  {
    placeholder: '[CTRL_DOT]',
    function: 'Pause',
    example: '[CTRL_DOT] → \\.'
  },
  {
    placeholder: '[CTRL_WAIT]',
    function: 'Wait for input',
    example: '[CTRL_WAIT] → \\|'
  },
  {
    placeholder: '[CTRL_INSTANT]',
    function: 'Instant display',
    example: '[CTRL_INSTANT] → \\^'
  },
  {
    placeholder: '[CTRL_INPUT]',
    function: 'Input prompt',
    example: '[CTRL_INPUT] → \\!'
  }
]
</script>
