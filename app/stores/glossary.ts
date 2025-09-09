import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { GlossaryTerm, GlossaryQuery } from '~/types/glossary'

export const useGlossaryStore = defineStore('glossary', () => {
  const terms = ref<GlossaryTerm[]>([])
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  async function fetchTerms(query: GlossaryQuery) {
    try {
      isLoading.value = true
      error.value = null
      const data = await invoke<GlossaryTerm[]>('glossary_list_terms', { q: query })
      terms.value = data
    } catch (e) {
      error.value = e instanceof Error ? e.message : 'Failed to load terms'
    } finally {
      isLoading.value = false
    }
  }

  async function upsertTerm(term: GlossaryTerm): Promise<number> {
    const id = await invoke<number>('glossary_upsert_term', { term })
    return id
  }

  async function deleteTerm(id: number) {
    await invoke('glossary_delete_term', { id })
  }

  return { terms, isLoading, error, fetchTerms, upsertTerm, deleteTerm }
})
