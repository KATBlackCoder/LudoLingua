export interface GlossaryTerm {
  id: number;
  category: string;
  source_lang: string;
  target_lang: string;
  input: string;
  output: string;
  enabled: boolean;
}

export interface GlossaryQuery {
  source_lang: string;
  target_lang: string;
  categories: string[];
  prompt_types: string[]; // kept for future but unused server-side
  limit?: number;
  only_enabled: boolean;
}


