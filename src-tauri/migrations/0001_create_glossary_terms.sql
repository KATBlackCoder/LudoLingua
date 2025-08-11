CREATE TABLE IF NOT EXISTS glossary_terms (
  id          INTEGER PRIMARY KEY,
  category    TEXT NOT NULL,
  source_lang TEXT NOT NULL,
  target_lang TEXT NOT NULL,
  input       TEXT NOT NULL,
  output      TEXT NOT NULL,
  enabled     INTEGER NOT NULL DEFAULT 1,
  created_at  TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_gls_main
  ON glossary_terms (enabled, source_lang, target_lang, category);


