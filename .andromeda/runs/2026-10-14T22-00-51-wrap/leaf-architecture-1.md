group: architecture-1
read: .claude/docs/conventions.md · 61 · «- Architecture: `.andromeda/architecture.md` · directory lay»
read: .claude/docs/gotchas.md · 68 · «- Decisions and rationale: `.andromeda/architecture.md` §Est»
read: .claude/docs/services/blitz-dom.md · 43 · «- `.andromeda/architecture.md` · `.claude/docs/stack.md` · `»
read: .claude/docs/services/blitz-paint.md · 35 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-shell.md · 34 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/blitz-traits.md · 33 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»
read: .claude/docs/services/dioxus-native-dom.md · 44 · «- `.andromeda/architecture.md` · `.claude/docs/services/blit»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «a known limit pinned by `stand_keyboard`.»
  source: .andromeda/architecture.md:91 «its narrowing to `button`, `input`, `select` and `textarea` owned by a `.andromeda/residuals.md` line (target: next)»
  replace-with: «a known limit pinned by `stand_keyboard`, its narrowing to `button`, `input`, `select` and `textarea` owned by a `.andromeda/residuals.md` line (target: next).»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «it still reaches content clipped by `contain: paint`)»
  source: .andromeda/architecture.md:124 «a box clipped on one axis (`overflow-x: clip` beside `overflow-y: visible`) stops the hit at its whole padding box, as paint clips it, while `visible_region` narrows such a box on its clipped axis only»
  replace-with: «it still reaches content clipped by `contain: paint`, its widening owned by a `.andromeda/residuals.md` line (target: next); a box clipped on one axis stops it at its whole padding box while `visible_region` narrows that box on its clipped axis only, so a target beside it along its unclipped axis reads in view and is not reached by a hit — measured since 2026-10-14-stand-requirement-sweep and left as it reads, as are a hit at an element's own box and at a box both transformed and scrolled)»

- leaf: .claude/docs/services/blitz-dom.md:15   class: omission
  leaf-text: «while `get_client_bounding_rect` reads where the box stands)»
  source: .andromeda/architecture.md:124 «`BaseDocument::get_client_bounding_rect` reads that box at its layout position (0, 40, 300, 140), not where its transform moves it — a divergence from CSSOM View's `getBoundingClientRect`»
  replace-with: «while `get_client_bounding_rect` reads where the box stands — its fix owned by a `.andromeda/residuals.md` line (target: next) — though it reads a transformed box at its layout position, not where its transform moves it, a divergence from CSSOM View's `getBoundingClientRect` measured and pinned since 2026-10-14-stand-requirement-sweep, not fixed, owned by another such line)»

- leaf: .claude/docs/services/blitz-paint.md:18   class: stale
  leaf-text: «unchanged, not measured by any check)»
  source: .andromeda/architecture.md:129 «so which bullet it lands on depends on them — measured since 2026-10-14-stand-requirement-sweep by `password_caret_click` (both layout modes, the bundled font at 20 px)»
  replace-with: «unchanged), so which bullet it lands on depends on them — measured since 2026-10-14-stand-requirement-sweep by `password_caret_click` (both layout modes, the bundled font at 20 px): a click 30 px into the content box places the caret at index 5 after eight narrow characters and at 2 after eight wide ones, a known 0.1.0 limit whose fix, placing a click's caret by the bullet layout, is owned by a `.andromeda/residuals.md` line (target: next)»

- leaf: .claude/docs/services/dioxus-native-dom.md:16   class: stale
  leaf-text: «The stand reads it empty on the four lean tasks and Home; the three other tasks read 2 · 3 · 676 (`stand_actionable_keys`).»
  source: .andromeda/architecture.md:136 «since 2026-10-14-stand-requirement-sweep, on Temperature Converter, Circle Drawer and Cells too (2 · 3 · 676 until then), Circle Drawer's dialog open included»
  replace-with: «The stand reads it empty on the four lean tasks and Home and, since 2026-10-14-stand-requirement-sweep, on Temperature Converter, Circle Drawer (its dialog open included) and Cells too (2 · 3 · 676 until then), keyed by author ids added as `id:` lines alone (`stand_actionable_keys`, `stand_non_lean_tasks`).»
