-- Remove the overly restrictive unique constraint that prevents legitimate text duplicates
-- In RPG games, the same text can appear multiple times in different contexts
-- and each instance needs separate translation opportunities

-- Drop the existing unique constraint that was preventing duplicate text
DROP INDEX IF EXISTS ux_text_units_project_file_field_text;

-- For RPG games, we need to allow the same text to appear multiple times because:
-- 1. Multiple NPCs can say the same thing but need different translations
-- 2. Same dialogue can appear in different story contexts  
-- 3. Character-specific variations may be needed
-- 4. Context-dependent translations are important

-- We'll rely on application logic to handle duplicates appropriately
-- rather than database constraints that are too restrictive for this use case
