/**
 * Status of a text unit's translation
 */
export enum TranslationStatus {
  NotTranslated = 'NotTranslated',
  MachineTranslated = 'MachineTranslated',
  HumanReviewed = 'HumanReviewed',
  Ignored = 'Ignored',
}

/**
 * Type of prompt to be used for translation
 */
export enum PromptType {
  Character = 'Character',
  State = 'State',
  System = 'System',
  Dialogue = 'Dialogue',
  Equipment = 'Equipment',
  Skill = 'Skill',
  Class = 'Class',
  Other = 'Other',
}

/**
 * Represents a single unit of text that can be translated
 */
export interface TextUnit {
  id: string;
  source_text: string;
  translated_text: string;
  field_type: string;
  status: TranslationStatus;
  prompt_type: PromptType;
}

/**
 * Actual token usage from completed translation
 */
export interface ActualTokenUsage {
  input_tokens: number;
  output_tokens: number;
  total_tokens: number;
  text_unit_id: string;
  model_name: string;
}

/**
 * Translation result including both the text unit and token usage
 */
export interface TranslationResult {
  text_unit: TextUnit;
  token_usage?: ActualTokenUsage;
} 