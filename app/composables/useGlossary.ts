import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useGlossaryStore } from '~/stores/glossary'
import { useLanguageStore } from '~/stores/language'
import type { GlossaryQuery, GlossaryTerm } from '~/types/glossary'

export function useGlossary() {
  const glossary = useGlossaryStore()
  const language = useLanguageStore()

  // Filters
  const search = ref('')
  const categoryItems = ['All', 'Characters', 'Essential Terms', 'Status Effects', 'Mechanics', 'Translation Rules', 'Locations', 'Time & Weather']
  const categoryFilter = ref<string>('All')
  const languageItems = computed(() => language.languageOptions.map(l => ({ label: l.label, value: l.id })))
  const sourceFilter = ref<string>(language.getLanguage.source_language?.id ?? 'en')
  const targetFilter = ref<string>(language.getLanguage.target_language?.id ?? 'fr')
  const onlyEnabled = ref<boolean>(false)

  // Rows/projection
  type Row = { _id: number; enabled: boolean; category: string; input: string; output: string; source_lang: string; target_lang: string }
  const filteredTerms = computed(() => {
    const q = search.value.trim().toLowerCase()
    return glossary.terms.filter(t => {
      if (onlyEnabled.value && !t.enabled) return false
      if (categoryFilter.value && categoryFilter.value !== 'All' && t.category !== categoryFilter.value) return false
      if (t.source_lang !== sourceFilter.value) return false
      if (t.target_lang !== targetFilter.value) return false
      if (!q) return true
      const hay = `${t.input}\u0000${t.output}\u0000${t.category}`.toLowerCase()
      return hay.includes(q)
    })
  })
  const rows = computed<Row[]>(() => filteredTerms.value.map(t => ({ 
    _id: t.id, 
    enabled: !!t.enabled, 
    category: t.category, 
    input: t.input, 
    output: t.output,
    source_lang: t.source_lang,
    target_lang: t.target_lang
  })))

  // Pagination
  const page = ref(1)
  const pageSize = ref(25)
  const pageCount = computed(() => Math.max(1, Math.ceil(rows.value.length / pageSize.value)))
  const pagedRows = computed<Row[]>(() => {
    const start = (page.value - 1) * pageSize.value
    return rows.value.slice(start, start + pageSize.value)
  })

  // Queries
  function buildQuery(limit?: number): GlossaryQuery {
    return {
      source_lang: sourceFilter.value,
      target_lang: targetFilter.value,
      categories: categoryFilter.value && categoryFilter.value !== 'All' ? [categoryFilter.value] : [],
      prompt_types: [],
      only_enabled: onlyEnabled.value,
      limit,
    }
  }

  async function reload() {
    await glossary.fetchTerms(buildQuery(500))
    page.value = 1
  }

  // CRUD helpers
  async function save(term: GlossaryTerm) {
    await glossary.upsertTerm(term)
    await reload()
  }
  async function remove(id: number) {
    // Use native Tauri dialog for delete confirmation
    const { ask } = await import('@tauri-apps/plugin-dialog')
    
    const confirmed = await ask('Are you sure you want to delete this glossary term?', {
      title: 'Delete Glossary Term',
      kind: 'warning'
    })
    
    if (confirmed) {
      await glossary.deleteTerm(id)
      await reload()
    }
  }
  async function toggleEnabled(id: number, v: boolean) {
    const t = glossary.terms.find(x => x.id === id)
    if (!t) return
    await glossary.upsertTerm({ ...t, enabled: v })
    await reload()
  }

  // Export/Import
  async function exportJson() {
    const q = buildQuery(undefined)
    const json = await invoke<string>('glossary_export_terms', { q })
    const blob = new Blob([json], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `glossary_${q.source_lang}_${q.target_lang}.json`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }

  async function importJson() {
    const input = document.createElement('input')
    input.type = 'file'
    input.accept = 'application/json'
    input.onchange = async () => {
      const file = input.files?.[0]
      if (!file) return
      const text = await file.text()
      let payload = text
      try {
        const parsed = JSON.parse(text)
        if (parsed && typeof parsed === 'object' && Array.isArray(parsed.terms)) {
          payload = JSON.stringify(parsed.terms)
        }
      } catch { /* keep raw */ }
      await invoke<number>('glossary_import_terms', { json: payload })
      await reload()
    }
    input.click()
  }

  function getTermById(id: number): GlossaryTerm | undefined {
    return glossary.terms.find(t => t.id === id)
  }

  return {
    // state
    terms: glossary.terms,
    isLoading: glossary.isLoading,
    error: glossary.error,

    // filters
    search,
    categoryItems,
    categoryFilter,
    languageItems,
    sourceFilter,
    targetFilter,
    onlyEnabled,

    // table
    rows,
    pagedRows,
    page,
    pageSize,
    pageCount,

    // actions
    reload,
    save,
    remove,
    toggleEnabled,
    exportJson,
    importJson,
    getTermById,
  }
}


