RPG Maker MZ Engine — Implementation Tasks

Overall approach

- Follow the same flow as MV: detection → extract → translate → inject → minimal export
- Reuse MV's extraction/injection patterns (helpers in `engines/common.rs` and file-structure helpers in `rpg_maker_mv/files/common.rs`),
  differing only by project root paths (`data/*`) and detection (`js/rmmz_core.js`).

Phase 1 — MVP: Actors.json only

- Backend
  - [x] Add `rpg_maker_mz/engine.rs` implementing `Engine` with detection (`data/` + `js/rmmz_core.js`)
  - [x] Add MZ branch in `engines/factory.rs` with detection criteria
  - [x] Expose module in `engines/mod.rs`
  - [x] Extract Actors only via MV helpers using `data/Actors.json`
  - [x] Inject Actors only via MV helpers back to `data/Actors.json`
  - [x] Unit test/quick check: load sample `GAME/data/Actors.json`, verify extract/inject IDs and fields
  - [x] Optional: minimal export for `data/` + inject (Actors only)

- Frontend
  - [x] No changes required; validate `EngineType.RpgMakerMz` shows and Actors units render

- Docs
  - [x] Update `CHANGELOG.md` when merged
  - [x] Update `PROGRESS.MD` noting MVP scope (Actors only)

Phase 2 — Core files parity with MV

- Backend
  - [x] Classes.json: reuse MV `classes` with `data/Classes.json` (prefix `class_`), extract/inject
  - [x] Items.json: reuse MV `items` with `data/Items.json` (prefix `item_`), extract/inject
  - [x] Weapons.json: reuse MV `weapons` with `data/Weapons.json` (prefix `weapon_`), extract/inject
  - [x] Armors.json: reuse MV `armors` with `data/Armors.json` (prefix `armor_`), extract/inject
  - [x] Skills.json: reuse MV `skills` with `data/Skills.json` (prefix `skill_`), extract/inject
  - [x] System.json: reuse MV `system` with `data/System.json` (prefix `system_`), extract/inject; verify field deltas if any
  - [x] States.json: reuse MV `states` with `data/States.json` (prefix `state_`), extract/inject
  - [x] Enemies.json: reuse MV `enemies` with `data/Enemies.json` (prefix `enemy_`), extract/inject
  - [x] CommonEvents.json: reuse MV `common_events` with `data/CommonEvents.json` (prefix `common_event_`), extract/inject
  - [x] Troops.json: reuse MV `troops` with `data/Troops.json` (prefix `troop_`), extract/inject
  - [x] MapInfos.json: reuse MV `maps_infos` with `data/MapInfos.json` (prefix `map_info_`), extract/inject
  - [x] Implement MZ map discovery scanning `data/` for `MapXXX.json`; reuse MV `maps` extract/inject
  - [ ] Manual QA: extract/inject roundtrip on Items/System/MapXXX across 1–2 MZ projects

- Export flow
  - [ ] Ensure `export_data_roots: ["data"]` in MZ criteria
  - [ ] Verify minimal export contains `data/` and `ludollingua.json`

Per-file reuse map (MV → MZ)

- [x] Classes.json → `mv::classes` | path: `data/Classes.json` | prefix: `class_`
- [x] Items.json → `mv::items` | path: `data/Items.json` | prefix: `item_`
- [x] Weapons.json → `mv::weapons` | path: `data/Weapons.json` | prefix: `weapon_`
- [x] Armors.json → `mv::armors` | path: `data/Armors.json` | prefix: `armor_`
- [x] Skills.json → `mv::skills` | path: `data/Skills.json` | prefix: `skill_`
- [x] States.json → `mv::states` | path: `data/States.json` | prefix: `state_`
- [x] Enemies.json → `mv::enemies` | path: `data/Enemies.json` | prefix: `enemy_`
- [x] CommonEvents.json → `mv::common_events` | path: `data/CommonEvents.json` | prefix: `common_event_`
- [x] Troops.json → `mv::troops` | path: `data/Troops.json` | prefix: `troop_`
- [x] MapInfos.json → `mv::maps_infos` | path: `data/MapInfos.json` | prefix: `map_info_`
- [x] MapXXX.json → `mv::maps` | path: `data/MapNNN.json` | prefix: `map_<id>_event_...`

Notes for quick wiring

- Import the MV module in `rpg_maker_mz/engine.rs` and call its `extract_text` / `inject_translations` with `data/*.json` paths.
- Keep prefixes identical to MV to reuse existing IDs and UI filters.
- Event commands: reuse MV helpers (`extract_text_units_from_event_commands` / `inject_text_units_into_event_commands`). Codes 401 (text) and 102 (choices) are the same in MZ.

Excluded (non‑translatable)

- [x] Animations.json (MZ Effekseer metadata; not shown to players)
- [x] Tilesets.json (no player‑visible strings)
