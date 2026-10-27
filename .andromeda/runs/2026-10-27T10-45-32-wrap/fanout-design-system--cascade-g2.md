proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 10
    section: ## Surface: desktop-native › ### Tokens (platform-specific)
    change: Remove the line on the stand flight booker's `select#flight-type` overrides (15px, `#1a1a1a` on `#ffffff`, `1px solid #c0c0c0`, radius 6, padding 8/10, `:focus` `#3a5ce5` border, its two options). Its values are unchanged and fold into layout-templates' desktop-native mode-region bullet :15. The engine's select-default line above it stays.
    sidecar: surface.md:10 — the stand's flight-type select overrides leave design-system for layout-templates :15, values unchanged, by the founder's «Все десять, общие разделить (Рекомендую)» (inputs#I7)
    rationale: report Decisions & corrections (the founder's word inputs#I7: the eight stand-only lines go to layout-templates' desktop-native screens with values unchanged) and Counts stand-ten, which lists `surface.md:10` as changed and the select facts as folding into `layout-templates.md:15`. The surface category holds no test stand's own colours or type (the founder's 2026-10-25 ruling).
    basis: .andromeda/design-system-detail/surface.md:10
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system-detail/surface.md
    near: 40
    section: ## Surface: desktop-native › ### Component Patterns
    change: Remove the stand load page's line (`seven_guis::stand::boot_load`, TaskShell "Load", keyed `load-fetch` and `load-done`, classes `load-root` · `load-body` · `load-btn` · `load-done`). Its facts fold into layout-templates' desktop-native served-screens bullet :14. The group `desktop-native-stand-load-page` (lines 38–40) is left empty, so remove its head lines too.
    sidecar: surface.md:38–40 — the desktop-native-stand-load-page group leaves design-system, its facts folded into layout-templates :14, values unchanged, by the founder's «Все десять, общие разделить (Рекомендую)» (inputs#I7)
    rationale: report Counts stand-ten lists `surface.md:38` and `:40` as changed, and Decisions & corrections places the load-page facts (`surface.md:38-40`, `motion.md:34`) in the served-screens bullet `:14`. The founder's word inputs#I7 moves the stand-only lines with their values unchanged.
    basis: .andromeda/design-system-detail/surface.md:40
  - detector: D-cascade
    severity: warning
    file: .andromeda/design-system.md
    near: 247
    section: ## Self-Validation Protocol
    change: In the subjects comment, drop `desktop-native-stand-load-page` (between `desktop-native-devtools-overlay` and `desktop-native-window`) and keep every other subject in its order.
    sidecar: design-system.md:247 — the subjects comment drops desktop-native-stand-load-page, whose surface group has emptied, by the founder's word inputs#I7
    rationale: report Decisions & corrections («the subjects line drops `desktop-native-stand-load-page`») and Counts stand-ten (`design-system.md:247` changed). It follows from the surface group emptied at surface.md:38–40.
    basis: .andromeda/design-system.md:247
note: The fold targets (layout-templates.md:14 for the load page, :15 for the flight-type select) are layout-templates' to edit, not this group's. No coverage row reads hardcoded✗, so D-design-tokens does not fire. The task lists no new facts. surface#1.6's runtime.rs citations and the window.rs and lib.rs line moves are the citation sweep's.
