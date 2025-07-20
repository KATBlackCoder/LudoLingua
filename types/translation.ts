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
  Name = 'Name',
  Description = 'Description',
  Dialogue = 'Dialogue',
  Item = 'Item',
  Skill = 'Skill',
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