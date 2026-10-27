proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 27
    section: '## Color Palette › ### Core Colors'
    change: 'Remove the «seven_guis accent» row (its value, hover and active shades, non-text uses and SC 1.4.3 ratios) — values unchanged, it lands in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), PROVISIONAL on F2.'
    sidecar: 'color-palette: the seven_guis accent row moved out to layout-templates `values · desktop-native-screens`, values unchanged (move-set row 2; F2 «Every stand row», the founder''s delegate answer, PROVISIONAL).'
    rationale: 'Report Expected amendments 2: every seven_guis stand row leaves the product surface, move set rows 1–9 of evidence/records.md (b), `color-palette.md:27` row 2; F2 PROVISIONAL (inputs#I3, I5); category focus Not here: a stand page''s own colours are architecture''s and layout-templates''.'
    basis: .andromeda/design-system-detail/color-palette.md:27
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system.md
    near: 16
    section: '## Color Palette › ### Core Colors'
    change: 'Remove the body line «seven_guis accent — … passes SC 1.4.3''s 4.5 at rest, on hover and when active.» — the Core Colors broad line of the moved row; the subsection keeps its Detail pointer alone.'
    sidecar: 'design-system body: the seven_guis accent line under Core Colors removed with its detail row (move-set row 1; F2, PROVISIONAL).'
    rationale: 'Report Expected amendments 2, `ds-accent` site `design-system.md:16` row 1; v030-16''s flip reads that design-system''s body and detail files hold none of the nine rows.'
    basis: .andromeda/design-system.md:16
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 28
    section: '## Color Palette › ### Core Colors'
    change: 'Remove the «seven_guis load page (stand)» row (accent shades on `load-fetch`, text and page colours) — values unchanged, folded into layout-templates `values · desktop-native-screens`, PROVISIONAL on F2.'
    sidecar: 'color-palette: the stand load page''s colour row moved out to layout-templates `values · desktop-native-screens`, values unchanged (move-set row 3; F2, PROVISIONAL).'
    rationale: 'Report Expected amendments 2 and 3: `color-palette.md:28` is move-set row 3; layout-templates already states the three accent shades on `load-fetch` (fold).'
    basis: .andromeda/design-system-detail/color-palette.md:28
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 73
    section: '## Color Palette › ### Semantic Colors'
    change: 'Remove the «seven_guis success» row (the flight booker''s success message colours) — values unchanged, it lands in layout-templates `values · desktop-native-screens`, PROVISIONAL on F2.'
    sidecar: 'color-palette: the seven_guis success row moved out to layout-templates `values · desktop-native-screens`, values unchanged (move-set row 4; F2, PROVISIONAL).'
    rationale: 'Report Expected amendments 2, pattern `ds-success`: 1 standing, `color-palette.md:73` (row 4).'
    basis: .andromeda/design-system-detail/color-palette.md:73
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 101
    section: '## Color Palette › ### Border Progression'
    change: 'Remove the «Field — seven_guis flight-type select» row (its border and focused border) — values unchanged, folded into layout-templates `values · desktop-native-screens`, PROVISIONAL on F2.'
    sidecar: 'color-palette: the stand''s flight-type select border row moved out to layout-templates `values · desktop-native-screens`, values unchanged (move-set row 5; F2, PROVISIONAL).'
    rationale: 'Report Expected amendments 2 and 3: `color-palette.md:101` is move-set row 5 (`ds-accent`, `ds-select`); layout-templates:15 already states the select''s border (fold).'
    basis: .andromeda/design-system-detail/color-palette.md:101
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 27
    section: '## Color Palette › ### Core Colors'
    change: '| seven_guis stand (not tokens) | — | The stand''s accent, success and select-border colours are product-surface values held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not design-system tokens — F2, PROVISIONAL |'
    sidecar: 'color-palette: a pointer row records that the stand''s accent, success and select-border colours are held in layout-templates `values · desktop-native-screens`, no value kept here (F2, PROVISIONAL).'
    rationale: 'Task New facts F1, carried by the accent row''s place once the move-set rows 2–5 leave (report Expected amendments 2–3); the row names no hex so the `ds-*` patterns read no move-set value here.'
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 31
    section: '## Typography'
    change: 'Remove the item «The seven_guis headless stand registers the same bundled DejaVu Sans for every generic …» — unchanged, it lands in architecture §Standard Contracts → Headless stand beside `font_ctx()`, PROVISIONAL on F2.'
    sidecar: 'typography: the stand''s bundled-font registration moved out to architecture §Standard Contracts → Headless stand (move-set row 6; F2, PROVISIONAL).'
    rationale: 'Report Expected amendments 2 and 4, pattern `ds-dejavu`: `typography.md:31` row 6; `typography.md:30`, `:32` are not stand rows, no-change.'
    basis: .andromeda/design-system-detail/typography.md:31
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 31
    section: '## Typography'
    change: '- The seven_guis headless stand''s font registration is held as architecture §Standard Contracts → Headless stand''s contract beside `font_ctx()`, not as a design-system typography row — F2, PROVISIONAL.'
    sidecar: 'typography: a pointer item records that the stand''s font registration is architecture''s Headless stand contract (F2, PROVISIONAL).'
    rationale: 'Task New facts F2, beside the removed row 6 (report Expected amendments 4); it names no font or builder so `ds-dejavu` reads no stand row here.'
    fact: F2
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 19
    section: '## Spacing'
    change: 'Remove the item «seven_guis cards pad 24px 32px with 16px gap; home pads 48px 32px 64px» — values unchanged, it lands in layout-templates `values · desktop-native-screens`, PROVISIONAL on F2.'
    sidecar: 'spacing: the stand''s card and home padding moved out to layout-templates `values · desktop-native-screens`, values unchanged (move-set row 8; F2, PROVISIONAL).'
    rationale: 'Report Expected amendments 2, pattern `ds-cards`: `spacing.md:19` (row 8).'
    basis: .andromeda/design-system-detail/spacing.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 19
    section: '## Spacing'
    change: '- The seven_guis stand''s card padding is held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not design-system — F2, PROVISIONAL.'
    sidecar: 'spacing: a pointer item records that the stand''s card padding is held in layout-templates `values · desktop-native-screens` (F2, PROVISIONAL).'
    rationale: 'Task New facts F4, beside the removed row 8 (report Expected amendments 2–3).'
    fact: F4
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 10
    section: '## Depth Strategy'
    change: 'Remove the item «seven_guis cards use `0 2px 8px rgba(0, 0, 0, 0.08)`; the circle dialog uses 0.10» — values unchanged, it lands in layout-templates `values · desktop-native-screens`, PROVISIONAL on F2.'
    sidecar: 'depth-strategy: the stand''s card shadow moved out to layout-templates `values · desktop-native-screens`, values unchanged (move-set row 7; F2, PROVISIONAL).'
    rationale: 'Report Expected amendments 2, pattern `ds-cards`: `depth-strategy.md:10` (row 7); `depth-strategy.md:9`, the browser app''s menus, no-change.'
    basis: .andromeda/design-system-detail/depth-strategy.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/depth-strategy.md
    near: 10
    section: '## Depth Strategy'
    change: '- The seven_guis stand''s card shadow is held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not design-system — F2, PROVISIONAL.'
    sidecar: 'depth-strategy: a pointer item records that the stand''s card shadow is held in layout-templates `values · desktop-native-screens` (F2, PROVISIONAL).'
    rationale: 'Task New facts F3, beside the removed row 7 (report Expected amendments 2–3).'
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/border-radius.md
    near: 12
    section: '## Border Radius'
    change: 'Remove the «seven_guis (untokenized)» row (cards, buttons, inputs, task cards, tags and the flight-type select radii) — values unchanged, it lands in layout-templates `values · desktop-native-screens`, PROVISIONAL on F2.'
    sidecar: 'border-radius: the stand''s radius row moved out to layout-templates `values · desktop-native-screens`, values unchanged (move-set row 9; F2, PROVISIONAL).'
    rationale: 'Report Expected amendments 2, pattern `ds-cards`: `border-radius.md:12` (row 9).'
    basis: .andromeda/design-system-detail/border-radius.md:12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/design-system-detail/border-radius.md
    near: 12
    section: '## Border Radius'
    change: '| seven_guis stand (not tokens) | — | The stand''s card radius is held in layout-templates §Surface: desktop-native → Primary screens (`values · desktop-native-screens`), not design-system — F2, PROVISIONAL |'
    sidecar: 'border-radius: a pointer row records that the stand''s card radius is held in layout-templates `values · desktop-native-screens` (F2, PROVISIONAL).'
    rationale: 'Task New facts F5, beside the removed row 9 (report Expected amendments 2–3).'
    fact: F5
  - detector: D-cascade
    severity: escalate
    file: .andromeda/design-system-detail/surface.md
    near: 10
    section: '## Surface: desktop-native › ### Tokens (platform-specific)'
    change: 'IF the operator moves it at the P2 dialog: remove the item «The stand''s flight booker overrides those select defaults on its one select, `select#flight-type` …» — values unchanged, to layout-templates `values · desktop-native-screens` (already stated there, fold); else leave as it stands.'
    sidecar: 'surface: the stand select''s overrides moved out to layout-templates `values · desktop-native-screens`, values unchanged (one of the ten further stand lines, on the operator''s dialog answer; F2, PROVISIONAL).'
    rationale: 'Report Expected amendments, closing item: `surface.md:10` is one of the ten further stand lines «Listed, not decided», whether it moves under F2 brought to the operator as a dialog at P2 (inputs#I5, Decisions); `ds-accent` and `ds-select` read it standing.'
    basis: .andromeda/design-system-detail/surface.md:10
  - detector: D-cascade
    severity: escalate
    file: .andromeda/design-system-detail/surface.md
    near: 40
    section: '## Surface: desktop-native › ### Component Patterns'
    change: 'IF the operator moves it at the P2 dialog: remove the `values · desktop-native-stand-load-page` group (the load page''s keyed elements and classes) — unchanged, to layout-templates §Surface: desktop-native → Primary screens; else leave as it stands.'
    sidecar: 'surface: the stand load page''s component group moved out to layout-templates §Surface: desktop-native, unchanged (one of the ten further stand lines, on the operator''s dialog answer; F2, PROVISIONAL).'
    rationale: 'Report Expected amendments, closing item: `surface.md:38-40` (the `values · desktop-native-stand-load-page` block) is listed, not decided, its move the operator''s P2 dialog (inputs#I5); pattern `ds-extra` reads `surface.md:38`.'
    basis: .andromeda/design-system-detail/surface.md:38
note: Not mine — the landings (layout-templates `values · desktop-native-screens`, architecture Headless stand), a11y-plan visual-design-verification.md:12 citation re-point; the body subjects line design-system.md:249 drops `desktop-native-stand-load-page` only if the operator moves that group, and body design-system.md:220 (web-spa `#f5f5f5`) plus the other undecided lines (color-palette.md:45, :60, :72, typography.md:18, :19, spacing.md:20, motion.md:34) were not picked and await the same dialog; D-design-tokens unfired (no new UI element).
