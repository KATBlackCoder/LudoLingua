-- Create text_units table for storing translation data
CREATE TABLE IF NOT EXISTS text_units (
  id INTEGER PRIMARY KEY,
  project_path TEXT NOT NULL,
  file_path TEXT NOT NULL,
  field_type TEXT NOT NULL,
  source_text TEXT NOT NULL,
  translated_text TEXT,
  status TEXT NOT NULL DEFAULT 'NotTranslated',
  prompt_type TEXT NOT NULL,
  source_lang TEXT NOT NULL,
  target_lang TEXT NOT NULL,
  manifest_hash TEXT, -- For project identification via .ludolingua.json
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Unique constraint prevents duplicate entries per project/file/field/source_text
-- This allows the same source_text to appear multiple times in different contexts
CREATE UNIQUE INDEX IF NOT EXISTS ux_text_units_project_file_field_text
  ON text_units (project_path, file_path, field_type, source_text);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_text_units_status ON text_units (status);
CREATE INDEX IF NOT EXISTS idx_text_units_project_path ON text_units (project_path);
CREATE INDEX IF NOT EXISTS idx_text_units_updated_at ON text_units (updated_at);
CREATE INDEX IF NOT EXISTS idx_text_units_manifest_hash ON text_units (manifest_hash);

-- Trigger to update updated_at timestamp
CREATE TRIGGER IF NOT EXISTS update_text_units_updated_at
  AFTER UPDATE ON text_units
  FOR EACH ROW
  BEGIN
    UPDATE text_units SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
  END;
