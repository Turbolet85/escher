# layouts extract

## Relevance
partial — the README and the capabilities statement are no layout surface; the layouts domain is touched only as one landing site of v030-16's move of the design-system stand rows (accent, success, cards, the stand's DejaVu Sans) into `.andromeda/layout-templates.md`, an expected wrap amendment, and through the README's naming of the `escher-session` command line.

## Constraints
- The seven_guis stand's layout facts stand under one entry: the stand rows that move into layout-templates land with the seven_guis Home and TaskShell entry or the served-screens entries beside it, not in a new surface or section (per layout-templates §Surface: desktop-native → Primary screens).
- Value-level stand facts (colours, select styling, the `load-fetch` accent with its hover and active) sit inside the `<!-- detail: values · desktop-native-screens -->` block of that section, under the subject `desktop-native-screens`; a moved accent, success or cards row is a value-level fact of the same kind and takes that block and subject (per layout-templates §Surface: desktop-native → Primary screens).
- Every fact in that section carries its source citation (`path:line` or a chunk's measured record); a moved row keeps the source citations it holds in the design-system detail files, re-read where they stand (per layout-templates §Surface: desktop-native → Primary screens).
- The stand's pinned viewport (800 × 600, scale 1.0, Light) with the bundled DejaVu Sans is already part of the TaskShell entry; whether the stand's DejaVu Sans item adds anything to it or duplicates it is research's question (per layout-templates §Surface: desktop-native → Primary screens).
- The harness's default 800x600 viewport at scale 1 in light mode is recorded apart from the stand's pinned viewport; a moved row does not restate it (per layout-templates §Surface: desktop-native → IA notes).
- The README's description of the driver's command line follows the `escher-session` primary screen: lowercase binary name, `escher-session <verb> [arguments] --session <dir>`, the twelve verbs in three levels, `start <task>` over the six served tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), `help` reaching no session, and the `mcp` role (per layout-templates §Surface: cli → Primary screens).
- The README's and the statement's cold-agent description of the run's six apps matches the `scripts/cold-agent.sh` primary screen's one task `stand` over counter, flight booker, timer, CRUD, cells and the load page (per layout-templates §Surface: cli → Primary screens).

## Patterns to follow
- A stand fact added to layout-templates is phrased in the section's register: what the screen holds, then "adding no element, class, style, wrapper, id or order" where markup is concerned, then its citations in parentheses, as the seven_guis entries of layout-templates §Surface: desktop-native → Primary screens do.
- A value-level row is placed inside the existing `<!-- detail: values · desktop-native-screens -->` … `<!-- /detail -->` block and does not open another, keeping `<!-- subjects: desktop-native-screens -->` the section's one subject (layout-templates §Surface: desktop-native → Primary screens).
- The master is changed only through its amendment record (`.andromeda/layout-templates-amendments.md`) at a wrap, never by phase. Phase names the landing as an expected wrap amendment, matching the scope's own boundary.

## Anti-patterns to avoid
- Duplicating a stand value the section already states (the accent `#3a5ce5` / `#2a4cd3` / `#1f3fc1` on `load-fetch`, the bundled DejaVu Sans on the TaskShell mount) under a second line, so that two lines hold one fact (layout-templates §Surface: desktop-native → Primary screens).
- Citing or writing into layout-templates §Decisions Log, which reads NO RECORDED INTENT. The founder's «В 0.3.0, через инкубатор (Рекомендую)» is the scope's record of the move and is not a layout decision to log there.

## Contract bindings
- design-system ↔ layouts: the four stand rows leave `.andromeda/design-system.md` (§Color Palette → Core Colors, and its detail files `color-palette.md`, `border-radius.md`, `depth-strategy.md`, `spacing.md`, `typography.md` where the rows stand) and land in layout-templates §Surface: desktop-native → Primary screens and in architecture. Which master takes each row is P3's reading.
- layouts ↔ a11y: the accent row's SC 1.4.3 claim (white on `#3a5ce5`) binds to the a11y plan and `stand_contrast`'s rows. The move relocates the row's record and changes no colour or contrast row. The stand's Tab sequences stated in layout-templates §Surface: desktop-native → Primary screens (flight booker five stops when enabled, CRUD six) bind to a11y SC 2.4.3 and stay unchanged, since the chunk adds no focusable element.

## Acceptance criteria contributions
- (layouts) After the wrap amendment, each stand row that is assigned to layout-templates stands once in §Surface: desktop-native → Primary screens. A value-level row stands inside the `values · desktop-native-screens` detail block and carries its source citations. No line in that section repeats a value another line already states (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) The amendment changes no layout-templates section other than §Surface: desktop-native → Primary screens, and it records no stand markup, id, class, style or Tab-order change, because the chunk changes no code (per layout-templates §Surface: desktop-native → Primary screens).
- (layouts) Every `escher-session` command that the README shows reads as the cli primary screen states it: the verb and flag spellings, the six task words, `--session <dir>`, and `help` and `mcp` (per layout-templates §Surface: cli → Primary screens).
