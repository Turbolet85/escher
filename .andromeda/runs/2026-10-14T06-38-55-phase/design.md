# design extract

## Relevance
partial — the chunk's colour work (CARRY 7 re-colour of the accent, CARRY 8–10 unmeasured colour states, rows and text outside the controls) is in design-system's domain. Keyboard reach, focus order, the platform adapter and the CI leg are not.

## Constraints
- design-system §Color Palette (Core Colors) records the seven_guis accent `#4a6cf7` (hover `#3a5ce5`, active `#2a4cd3`) on page `#f5f5f5` with text `#1a1a1a`. It also records white `#ffffff` text on that accent across the five accent controls (`counter-increment`, `flight-one-way`, `flight-book`, `timer-reset`, `crud-create`; 14–16 px, weight 500–600, at boot) at 4.39 against 4.5, a fail. Any re-colour from CARRY 7 must amend this row: the new value, its hover and active values, and the measured ratio. The row cites `tests/blitz-tests/tests/stand_contrast.rs:41-129`, so it moves with that table.
- design-system §Color Palette (Core Colors) records the accent's hover and active shades, `#3a5ce5` and `#2a4cd3`. These are the colour-changing states that CARRY 8 names. A re-colour has to cover all three shades (rest, hover, active), not only the rest colour. Whether each shade passes 4.5 against white is research's measurement; the plan does not record it.
- design-system §Color Palette (Semantic Colors) records the seven_guis invalid state as `#e53e3e` / `#fff5f5` / `#c53030` and the success message as `#ebf8ee` / `#68d391` / `#276749`. CARRY 8's invalid date (`#c53030` on `#fff5f5`) and CARRY 10's message text are these declared pairs. Any change to them amends this table.
- design-system §Color Palette (Surface Scale, Text Hierarchy) records the stand's page as `#f5f5f5` and its text as `#1a1a1a`. These are the backgrounds and foregrounds that CARRY 10 (task titles, labels, value displays) composites over.
- design-system §Color Palette ends `NOT YET MEASURED` on a project-wide UI colour token set. It records as observed absent any colour tokens or CSS custom properties in the engine and integration crates. The stand's colours are hex literals repeated per task file, so a replacement for `#4a6cf7` has to be applied at every place it is declared. Which files declare it (lean and non-lean tasks, and backgrounds against borders, outlines and `accent-color`) is research's question.
- design-system §Typography (seven_guis row) records the stand's type as sans-serif, base 14px, home title 36px weight 700. The accent controls' 14–16 px at weight 500–600 (per §Color Palette, Core Colors) are below the large-text threshold, so the 4.5 ratio applies to them. Which stand text, if any, qualifies as large text under `TextContrast::required_ratio` is research's to read.
- design-system §Typography (Loading) records that the headless stand registers bundled DejaVu Sans for every generic, with system fonts off. The contrast and screenshot readings are taken on that font. Any colour assertion holds for the stand as booted headlessly, not for the windowed build's system fonts.

## Patterns to follow
- Per design-system §Color Palette (Core Colors): record a measured stand colour as its declared value, with its ratio and a pointer to the measuring check and the chunk report ("every measured stand colour equals its declared value"). Keep that shape for the re-coloured accent and for each newly measured state or row.
- Per design-system §Color Palette (Semantic Colors): stand state colours are recorded as declared triples (background / border / text). Record a measured invalid-date or success pair against its triple rather than as a new free-standing row.
- Per design-system §Motion (This project's values): seven_guis styles record transitions and animations as observed absent. A hover or active colour therefore has no transition to wait on, which suits reading it headlessly after a pointer move or press (CARRY 8). Whether `:hover` / `:active` resolve in the harness's computed styles is research's question.
- Per design-system §Border Radius (seven_guis row) and §Depth Strategy (observed seven_guis card shadows): buttons are 6px, inputs 4px and cards 8px, with a `0 2px 8px rgba(0, 0, 0, 0.08)` card shadow. A re-colour changes colour declarations only and leaves these values as recorded.

## Anti-patterns to avoid
- Do not introduce a project-wide colour token set or CSS custom properties as a side effect of the re-colour. design-system §Color Palette marks that token set `NOT YET MEASURED` and records it as observed absent. Adding one would be a new design surface, not a re-colour.
- Do not re-colour only the accent's rest state. design-system §Color Palette (Core Colors) records hover and active shades, and leaving them at the old values would move CARRY 8's fail rather than close it.

## Contract bindings
- design ↔ a11y: the token-contrast binding. The accent and state pairs in design-system §Color Palette (Core Colors, Semantic Colors) are SC 1.4.3 pairs (4.5:1, or 3.0 for large text per `TextContrast::required_ratio`). A re-colour moves the a11y-plan §6/§9 readings and the `stand_contrast` record together with the design-system row.
- design ↔ tests: the design-system §Color Palette (Core Colors) row cites `tests/blitz-tests/tests/stand_contrast.rs:41-129` as its measurement. Turning that record into an assertion makes the design row's ratios the asserted values, so a colour change moves both in the same commit.
- design ↔ layouts: the accent is used beyond the five lean-task buttons (non-lean tasks per scope CARRY 7). Per-screen placement belongs to layout-templates, and the token change reaches the screens the "Stand requirement sweep" entry owns.

## Acceptance criteria contributions
- (design) Every stand control's declared text/background pair recorded in design-system §Color Palette (Core Colors, Semantic Colors) has a measured ratio at or above its required ratio, the five accent controls included. After the re-colour, no row reads `fail` at 4.39 (per design-system §Color Palette).
- (design) The accent's rest, hover and active shades are all replaced or all retained together, and each white-text pair on them is measured. No state is left at a value whose ratio is not recorded (per design-system §Color Palette).
- (design) The re-colour adds no CSS custom property or token layer, and no task file still declares `#4a6cf7` as a background behind white text (per design-system §Color Palette).
- (design) The re-colour changes no radius, shadow or type size or weight recorded for seven_guis (per design-system §Typography, §Border Radius).
