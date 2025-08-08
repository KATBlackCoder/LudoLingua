// Language definition for translation context.
// Mirrors the backend Language struct.

/**
 * Language definition for translation context.
 * Mirrors the backend Language struct.
 */
export interface Language {
    id: string;
    label: string;
    native_name: string;
    dir: 'ltr' | 'rtl';
    enabled: boolean;
  }