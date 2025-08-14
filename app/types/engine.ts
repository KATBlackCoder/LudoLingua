import type { Language } from './language';
import type { TextUnit } from './translation';

/**
 * Represents the type of game engine detected
 */
export enum EngineType {
  RpgMakerMv = 'RpgMakerMv',
  RpgMakerMz = 'RpgMakerMz',
  WolfRpg = 'WolfRpg',
  Unknown = 'Unknown',
}

/**
 * Criteria used to detect a specific game engine
 */
export interface EngineCriteria {
  required_files: string[];
  required_folders: string[];
  extra_files: string[];
  export_data_roots: string[];
}

/**
 * Information about the detected game engine and the game project itself
 */
export interface EngineInfo {
  name: string;
  path: string;
  engine_type: EngineType;
  source_language: Language;
  target_language: Language;
  version?: string;
  detection_criteria: EngineCriteria;
}

/**
 * Alias for EngineInfo to maintain compatibility with backend naming
 */
export type ProjectInfo = EngineInfo;

/**
 * Represents a game data file containing text that needs translation
 */
export interface GameDataFile {
  name: string;
  path: string;
  text_units: TextUnit[];
  text_unit_count: number;
} 