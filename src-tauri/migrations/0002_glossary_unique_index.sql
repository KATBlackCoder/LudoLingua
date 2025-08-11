-- Deduplicate before enforcing uniqueness (keeps the latest row per term key)
DELETE FROM glossary_terms
WHERE rowid NOT IN (
  SELECT MAX(rowid)
  FROM glossary_terms
  GROUP BY source_lang, target_lang, category, input
);

CREATE UNIQUE INDEX IF NOT EXISTS ux_gls_lang_cat_input
  ON glossary_terms (source_lang, target_lang, category, input);


