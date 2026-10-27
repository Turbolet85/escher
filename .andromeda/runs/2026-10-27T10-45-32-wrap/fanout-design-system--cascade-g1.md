proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 44
    section: "## Color Palette › ### Surface Scale (elevation hierarchy)"
    change: "The Surface Scale row's label cell becomes `todomvc page`, with seven_guis dropped; its `#f5f5f5` value and its Page background usage stay as they are. The seven_guis page background now lives in layout-templates' desktop-native screens."
    sidecar: "color-palette: the `#f5f5f5` page row, which seven_guis shared with todomvc, keeps todomvc and drops seven_guis, on the founder's «Все десять, общие разделить (Рекомендую)» (inputs#I7)."
    rationale: "report.md:103 records the founder's word that the two rows shared with todomvc keep todomvc and drop seven_guis; report.md:48 (`stand-ten`) lists color-palette.md:44 among the ten lines that change."
    basis: .andromeda/design-system-detail/color-palette.md:44
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 59
    section: "## Color Palette › ### Text Hierarchy"
    change: "Remove the Text Hierarchy row `seven_guis text` `#1a1a1a`. The stand's text colour now lives in layout-templates' desktop-native screens (`values · desktop-native-screens`), its value unchanged."
    sidecar: "color-palette: the seven_guis text `#1a1a1a` row moves to layout-templates' desktop-native screens, value unchanged, on the founder's «Все десять, общие разделить (Рекомендую)» (inputs#I7)."
    rationale: "report.md:103 says the eight stand-only lines go to layout-templates with their values unchanged; report.md:48 lists color-palette.md:59. report.md:109 notes that layout-templates :15 and :16 already state `#1a1a1a`, so the landing restates a value that is already there. That is for Validate, not for this file."
    basis: .andromeda/design-system-detail/color-palette.md:59
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/color-palette.md
    near: 71
    section: "## Color Palette › ### Semantic Colors"
    change: "Remove the Semantic Colors row `seven_guis invalid` `#e53e3e` / `#fff5f5` / `#c53030`. The stand's invalid state now lives in layout-templates' desktop-native screens, its values unchanged. The table keeps its header and any other rows."
    sidecar: "color-palette: the seven_guis invalid-state row (`#e53e3e` / `#fff5f5` / `#c53030`) moves to layout-templates' desktop-native screens, values unchanged, on the founder's «Все десять, общие разделить (Рекомендую)» (inputs#I7)."
    rationale: "report.md:103 records the founder's word on the eight stand-only lines; report.md:48 lists color-palette.md:71; report.md:109 says no line of layout-templates states these values yet, so this is one of the three new lines there."
    basis: .andromeda/design-system-detail/color-palette.md:71
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 18
    section: "## Typography"
    change: "Remove the Typography row `seven_guis` | sans-serif | home title 700 | base 14px; home title 36px. The stand's type now lives in layout-templates' desktop-native screens, its values unchanged."
    sidecar: "typography: the seven_guis type row (sans-serif, home title 700, base 14px, home title 36px) moves to layout-templates' desktop-native screens, values unchanged, on the founder's «Все десять, общие разделить (Рекомендую)» (inputs#I7)."
    rationale: "report.md:103 records the founder's word on the eight stand-only lines; report.md:48 locates typography.md:18 as one of the directive's lines that the listing missed because of its wording; report.md:109 says layout-templates does not state it yet, so it is new there."
    basis: .andromeda/design-system-detail/typography.md:18
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/typography.md
    near: 19
    section: "## Typography"
    change: "Remove the Typography row for the seven_guis flight booker's `flight-type` select (`.type-select`, 400, 15px). That fact folds into layout-templates' mode-region bullet :15, its values unchanged."
    sidecar: "typography: the seven_guis `flight-type` select row (400, 15px) folds into layout-templates' desktop-native mode-region bullet, values unchanged, on the founder's «Все десять, общие разделить (Рекомендую)» (inputs#I7)."
    rationale: "report.md:103 records the founder's word; report.md:108 says the select facts fold into the mode-region bullet `layout-templates.md:15`; report.md:48 lists typography.md:19."
    basis: .andromeda/design-system-detail/typography.md:19
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/spacing.md
    near: 20
    section: "## Spacing"
    change: "Remove the Spacing item for the seven_guis flight booker's `flight-type` select (8px vertical, 10px horizontal padding). That fact folds into layout-templates' mode-region bullet :15, its values unchanged."
    sidecar: "spacing: the seven_guis `flight-type` select padding (8px vertical, 10px horizontal) folds into layout-templates' desktop-native mode-region bullet, values unchanged, on the founder's «Все десять, общие разделить (Рекомендую)» (inputs#I7)."
    rationale: "report.md:103 records the founder's word; report.md:108 says the select facts fold into `layout-templates.md:15`; report.md:48 locates spacing.md:20 as one of the directive's lines that the listing missed because of its wording."
    basis: .andromeda/design-system-detail/spacing.md:20
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/motion.md
    near: 34
    section: "## Motion (calibrated to expression level `unrecorded`)"
    change: "Remove the item on the stand's served load page (`LoadNet` answering after `LOAD_ANSWER_DELAY`, 200 ms; `load-done` shown once the sheet lands). That fact folds into layout-templates' served-screens bullet :14. The driver's load-wait line at :33 above it stays as it reads."
    sidecar: "motion: the stand load page's `LoadNet` wait (200 ms, `load-done`) folds into layout-templates' desktop-native served-screens bullet, values unchanged, on the founder's «Все десять, общие разделить (Рекомендую)» (inputs#I7); the load-wait rule beside it stays."
    rationale: "report.md:103 records the founder's word; report.md:108 says the load-page facts (`motion.md:34`) fold into the served-screens bullet `layout-templates.md:14`; report.md:110 says motion.md:33 is outside the ten and stays."
    basis: .andromeda/design-system-detail/motion.md:34
note: "The other three of the ten lines in design-system are in group 2's categories: design-system.md:218 (the web-spa platform line, which keeps todomvc and drops seven_guis), :247 (the subjects line, which drops `desktop-native-stand-load-page`) and surface.md:10 / :38-40. The layout-templates landings, with five folds and three new lines, belong to layout-templates' pass."
