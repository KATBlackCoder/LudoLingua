<template>
  <div class="space-y-6 w-full">

    <!-- DataTable Component -->
    <DataTable
      ref="dataTableRef"
      :data="rows"
      :columns="columns as any"
      :loading="isBusy"
      title="Translation Results"
      icon="i-lucide-table"
      :show-filters="true"
      :show-search="true"
      :show-selection="true"
      :show-bulk-actions="true"
      :show-pagination="true"
      :show-row-count="true"
      :show-selected-count="true"
      :show-placeholder-filter="true"
      :show-text-length-filter="true"
      :text-length-min="0"
      :text-length-max="maxTextLength"
      :text-length-step="15"
      :placeholder-filter-options="placeholderOptions"
      :bulk-actions="bulkActions"
      :bulk-alert-color="'info'"
      :search-fields="['source_text', 'translated_text']"
      @selection-change="onSelectionChange as any"
      @bulk-action="onBulkAction"
    />

    <TranslationEditor
      v-model:open="editorOpen"
      :item="editingItem"
      @save="onSave"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, ref, h, resolveComponent, onMounted, onUnmounted } from "vue";
import type { Component } from "vue";
import type { TextUnit } from "~/types/translation";
import TranslationEditor from "~/components/translator/TranslationEditor.vue";
import DataTable from "~/components/shared/DataTable.vue";
import { useTranslator } from "~/composables/useTranslator";
import type { TableColumn } from "#ui/types";
import { useGlossary } from "~/composables/useGlossary";
import type { GlossaryTerm } from "~/types/glossary";
import { useLanguageStore } from "~/stores/language";
import { useAppToast } from "~/composables/useAppToast";
import { useEngineStore } from "~/stores/engine";
import { useNotifications } from "~/composables/useNotifications";
import { getPlaceholderFilterOptions, extractPlaceholderTypes } from "~/utils/translation";

const props = defineProps<{ items: TextUnit[] }>();
const emit = defineEmits<{
  (
    e: "save",
    payload: { id: string; translated_text: string; prompt_type?: string }
  ): void;
  (e: "remove", id: string): void;
  (e: "retranslate-selected", selectedRows: Row[]): void;
}>();
const { isBusy, retranslate, saveEdit } = useTranslator();
const engineStore = useEngineStore();
const glossary = useGlossary();
const languageStore = useLanguageStore();
const { showToast } = useAppToast();
const { notify } = useNotifications();

// DataTable ref
const dataTableRef = ref();

// Bulk action states
const isBulkRetranslating = ref(false);
const isBulkReverting = ref(false);

const promptTypeToCategory: Record<string, string> = {
  Character: "Characters",
  Class: "Mechanics",
  Skill: "Mechanics",
  Equipment: "Mechanics",
  State: "Status Effects",
  System: "Essential Terms",
  Dialogue: "Essential Terms",
  Other: "Essential Terms",
};

type Row = {
  id: string;
  source_text: string;
  translated_text: string;
  prompt_type: string;
  field_type: string;
};

const rows = computed<Row[]>(() =>
  props.items.map((u) => ({
    id: u.id,
    prompt_type: u.prompt_type,
    source_text: u.source_text,
    translated_text: u.translated_text ?? "",
    field_type: u.field_type,
  }))
);

// Calculate max text length dynamically for DataTable
const maxTextLength = computed(() => {
  if (rows.value.length === 0) return 200;
  const maxSourceLength = Math.max(
    ...rows.value.map((r) => r.source_text.length)
  );
  const maxTranslatedLength = Math.max(
    ...rows.value.map((r) => r.translated_text.length)
  );
  // Cap at reasonable maximum to avoid AI error outliers
  return Math.min(Math.max(maxSourceLength, maxTranslatedLength, 200), 1000);
});

// Predefined placeholder types based on documentation
const placeholderOptions = computed(() => {
  // Get all unique placeholder types that actually exist in the current data
  const existingPlaceholders = new Set<string>();

  // Scan all text units for placeholder patterns
  rows.value.forEach((row) => {
    const text = `${row.source_text} ${row.translated_text}`;
    const placeholders = extractPlaceholderTypes(text);
    placeholders.forEach(type => existingPlaceholders.add(type));
  });

  return getPlaceholderFilterOptions(existingPlaceholders);
});


// Fullscreen detection
const isFullscreen = ref(false);

// Window resize handler
const handleResize = () => {
  isFullscreen.value = window.innerWidth >= 1920; // Consider 1920px+ as fullscreen
};

// Lifecycle hooks
onMounted(() => {
  handleResize();
  window.addEventListener('resize', handleResize);
});

onUnmounted(() => {
  window.removeEventListener('resize', handleResize);
});


// Bulk actions for DataTable
const bulkActions = computed(() => [
  {
    label: 'Re-translate Selected',
    icon: 'i-lucide-refresh-cw',
    color: 'primary' as const,
    variant: 'soft' as const,
    loading: isBulkRetranslating.value,
    disabled: isBusy.value,
    onClick: onBulkRetranslate
  },
  {
    label: 'Revert to Raw',
    icon: 'i-lucide-undo',
    color: 'warning' as const,
    variant: 'soft' as const,
    loading: isBulkReverting.value,
    disabled: isBusy.value,
    onClick: onBulkRevert
  }
]);

const columns: TableColumn<Row>[] = [
  {
    accessorKey: "prompt_type",
    header: ({ column }) => {
      const isSorted = column.getIsSorted();
      const UButton = resolveComponent("UButton") as Component;
      return h(UButton, {
        color: "neutral",
        variant: "ghost",
        label: "Type",
        icon: isSorted
          ? isSorted === "asc"
            ? "i-lucide-arrow-up-narrow-wide"
            : "i-lucide-arrow-down-wide-narrow"
          : "i-lucide-arrow-up-down",
        class: "-mx-2.5",
        onClick: () => column.toggleSorting(column.getIsSorted() === "asc"),
      });
    },
    enableSorting: true,
  },
  {
    accessorKey: "field_type",
    header: ({ column }) => {
      const isSorted = column.getIsSorted();
      const UButton = resolveComponent("UButton") as Component;
      return h(UButton, {
        color: "neutral",
        variant: "ghost",
        label: "Field Type",
        icon: isSorted
          ? isSorted === "asc"
            ? "i-lucide-arrow-up-narrow-wide"
            : "i-lucide-arrow-down-wide-narrow"
          : "i-lucide-arrow-up-down",
        class: "-mx-2.5",
        onClick: () => column.toggleSorting(column.getIsSorted() === "asc"),
      });
    },
    enableSorting: true,
  },
  { 
    accessorKey: "source_text", 
    header: "Source", 
    enableSorting: false,
    size: isFullscreen.value ? 300 : 200,
    cell: ({ row }) => {
      const text = row.getValue("source_text") as string;
      const isLong = text.length > (isFullscreen.value ? 150 : 100);
      const maxWidth = isFullscreen.value ? "max-w-md" : "max-w-xs";
      
      if (isLong) {
        const UTooltip = resolveComponent("UTooltip") as Component;
        return h(UTooltip, {
          text: text,
          popper: { placement: 'top' }
        }, {
          default: () => h("div", {
            class: `${maxWidth} truncate cursor-help`
          }, text.substring(0, isFullscreen.value ? 150 : 100) + " [NEXT_LIGNE]")
        });
      }
      
      return h("div", {
        class: `${maxWidth} whitespace-normal break-words`
      }, text);
    }
  },
  {
    accessorKey: "translated_text",
    header: "Translated",
    enableSorting: false,
    size: isFullscreen.value ? 300 : 200,
    cell: ({ row }) => {
      const text = row.getValue("translated_text") as string;
      const isLong = text.length > (isFullscreen.value ? 150 : 100);
      const maxWidth = isFullscreen.value ? "max-w-md" : "max-w-xs";
      
      if (isLong) {
        const UTooltip = resolveComponent("UTooltip") as Component;
        return h(UTooltip, {
          text: text,
          popper: { placement: 'top' }
        }, {
          default: () => h("div", {
            class: `${maxWidth} truncate cursor-help`
          }, text.substring(0, isFullscreen.value ? 150 : 100) + " [NEXT_LIGNE]")
        });
      }
      
      return h("div", {
        class: `${maxWidth} whitespace-normal break-words`
      }, text);
    }
  },
  {
    accessorKey: "raw_text",
    header: "Raw Text",
    enableSorting: false,
    size: 100,
    cell: ({ row }) => {
      const UButton = resolveComponent("UButton") as Component;
      return h(
        UButton,
        {
          size: "xs",
          color: "neutral",
          variant: "soft",
          icon: "i-lucide-undo",
          disabled: isBusy.value,
          onClick: () => onRevertToRaw(row.original.id),
        },
        { default: () => "Revert" }
      );
    },
  },
  {
    id: "actions",
    header: "Actions",
    enableSorting: false,
    size: isFullscreen.value ? 200 : 150,
    cell: ({ row }) => {
      const UButton = resolveComponent("UButton") as Component;
      const buttonSize = isFullscreen.value ? "sm" : "xs";
      const buttonClass = isFullscreen.value ? "flex gap-2" : "flex gap-1";
      
      return h("div", { class: buttonClass }, [
        h(
          UButton,
          {
            size: buttonSize,
            color: "primary",
            variant: "soft",
            icon: "i-lucide-refresh-cw",
            disabled: isBusy.value,
            onClick: async () => {
              await onRetranslate(row.original.id);
            },
          },
          { default: () => isFullscreen.value ? "Re-translate" : "Re-translate" }
        ),
        h(
          UButton,
          {
            size: buttonSize,
            color: "warning",
            variant: "soft",
            icon: "i-lucide-plus",
            disabled: isBusy.value,
            onClick: async () => {
              await onAddToGlossary(row.original.id);
            },
          },
          { default: () => isFullscreen.value ? "Add to glossary" : "Add" }
        ),
        h(
          UButton,
          {
            size: buttonSize,
            color: "error",
            variant: "soft",
            icon: "i-lucide-trash-2",
            disabled: isBusy.value,
            onClick: () => {
              onRemove(row.original.id);
            },
          },
          { default: () => "Remove" }
        ),
        h(
          UButton,
          {
            size: buttonSize,
            color: "neutral",
            icon: "i-lucide-pencil",
            disabled: isBusy.value || editorOpen.value,
            onClick: () => openEditor(row.original.id),
          },
          { default: () => "Edit" }
        ),
      ]);
    },
  },
];

const editorOpen = ref(false);
const editingItem = ref<TextUnit | null>(null);

const openEditor = (id: string) => {
  const unit = props.items.find((u) => u.id === id) || null;
  editingItem.value = unit;
  editorOpen.value = !!unit;
};

async function onSave(payload: { id: string; translated_text: string }) {
  // Forward to store immediately and also emit for parent listeners
  await saveEdit({ id: payload.id, translated_text: payload.translated_text });
  emit("save", payload);
  editorOpen.value = false;
}

async function onRetranslate(id: string) {
  await retranslate(id);
  // Notify when single retranslation is complete
  await notify(
    "Translation Complete",
    "Single translation completed successfully"
  );
}

async function onAddToGlossary(id: string) {
  const unit = props.items.find((u) => u.id === id);
  if (!unit) return;
  const src = languageStore.getLanguage.source_language?.id || "en";
  const tgt = languageStore.getLanguage.target_language?.id || "en";
  const category = promptTypeToCategory[unit.prompt_type] || "Essential Terms";
  const term: GlossaryTerm = {
    id: 0,
    category,
    source_lang: src,
    target_lang: tgt,
    input: unit.source_text,
    output: unit.translated_text || "",
    enabled: true,
  };
  await glossary.save(term);
  showToast(
    "Added to glossary",
    `${category}: "${term.input}" → "${term.output || "…"}"`,
    "success",
    2500,
    "i-lucide-check-circle"
  );
}

function onRemove(id: string) {
  // Optimistically remove from store; also notify parent
  const idx = engineStore.textUnits.findIndex((u) => u.id === id);
  if (idx !== -1) engineStore.textUnits.splice(idx, 1);
  emit("remove", id);
}

// Row selection handlers - now handled by DataTable

// Bulk retranslation - send to process view
async function onBulkRetranslate() {
  const selectedRows = dataTableRef.value?.tableConfig?.selectedRows || [];
  if (selectedRows.length < 2) return;

  try {
    isBulkRetranslating.value = true;

    // Emit selected rows to parent (translator.vue) to pass to TranslationProcess
    emit("retranslate-selected", selectedRows);
  } finally {
    isBulkRetranslating.value = false;
  }
}

// Revert single text unit to raw (source text)
async function onRevertToRaw(id: string) {
  const unit = props.items.find((u) => u.id === id);
  if (!unit) return;

  // Update the translated text to be the same as source text
  const payload = { id, translated_text: unit.source_text };
  await saveEdit(payload);
  emit("save", payload);

  showToast(
    "Reverted to raw",
    `Text reverted to source: "${unit.source_text}"`,
    "warning",
    2000,
    "i-lucide-undo"
  );
}

// DataTable event handlers
const onSelectionChange = (selectedRows: Row[]) => {
  // Handle selection change if needed
  console.log('Selection changed:', selectedRows.length, 'items selected');
};

const onBulkAction = (action: { label: string }) => {
  if (action.label === 'Re-translate Selected') {
    onBulkRetranslate();
  } else if (action.label === 'Revert to Raw') {
    onBulkRevert();
  }
};

// Bulk revert selected rows to raw (source text)
async function onBulkRevert() {
  const selectedRows = dataTableRef.value?.tableConfig?.selectedRows || [];
  if (selectedRows.length === 0) return;

  try {
    isBulkReverting.value = true;

    // Revert each selected row to its source text
    for (const row of selectedRows) {
      const payload = { id: row.id, translated_text: row.source_text };
      await saveEdit(payload);
      emit("save", payload);
    }

    showToast(
      "Bulk revert complete",
      `Reverted ${selectedRows.length} items to raw text`,
      "warning",
      3000,
      "i-lucide-undo"
    );
  } finally {
    isBulkReverting.value = false;
  }
}
</script>
