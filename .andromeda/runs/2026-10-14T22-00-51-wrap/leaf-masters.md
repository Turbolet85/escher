group: masters
read: CLAUDE.md · 147 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:37   class: stale
  leaf-text: «One reading is measured off and said by the schema's own text: a hit reaches content clipped by `contain: paint`, so a control lying where such content extends can read `covered`»
  source: .andromeda/architecture.md:133 «since 2026-10-14-stand-requirement-sweep, a target lying beside a box clipped on one axis, along the axis it does not clip, which reads in view and is refused `covered`»
  replace-with: «Two readings are measured off and said by the schema's own text, `covered`'s meaning: a hit reaches content clipped by `contain: paint`, so a control lying where such content extends can read `covered`, and a box clipped on one axis stops a hit at its whole padding box, so a control lying outside it along the axis it does not clip can read `covered` though it shows»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «one reading is still off and said by the tool itself — a hit reaches content clipped by `contain: paint` —»
  source: .andromeda/architecture.md:133 «two readings are still off — content clipped by `contain: paint` beside a `visible` `overflow`, which a hit still reaches, and, since 2026-10-14-stand-requirement-sweep»
  replace-with: «two readings are still off and said by the tool itself — a hit reaches content clipped by `contain: paint`, and a box clipped on one axis stops a hit at its whole padding box, so a target beside it along the axis it does not clip is refused `covered` though it shows —»

- leaf: CLAUDE.md:110   class: stale
  leaf-text: «is measured and left as it reads, each owned by a route entry)»
  source: .andromeda/architecture.md:124 «of these readings two stand as measured, each owned by a `.andromeda/residuals.md` line (target: next) — widening the stop to a `contain: paint` box»
  replace-with: «is measured and left as it reads, the `contain: paint` stop and `Node::absolute_position` each owned by a `.andromeda/residuals.md` line (target: next); a box both transformed and scrolled is hit where its rows show, while `BaseDocument::get_client_bounding_rect` reads it at its layout position, not where its transform moves it — a divergence from CSSOM View, measured and pinned, owned by a `.andromeda/residuals.md` line (target: next))»

- leaf: CLAUDE.md:26   class: omission
  leaf-text: «equal-length values paint identical frames, the caret at its bullet (an engine change ratified by the founder, 2026-10-12)»
  source: .andromeda/architecture.md:129 «a known 0.1.0 limit whose fix — placing a click's caret by the bullet layout, an engine change for every document — is owned by a `.andromeda/residuals.md` line (target: next)»
  replace-with: «equal-length values paint identical frames, the caret at its bullet (an engine change ratified by the founder, 2026-10-12); a click inside a field holding a value still places the caret by the real characters' widths (`password_caret_click`: index 5 after eight narrow characters, 2 after eight wide), a known 0.1.0 limit whose fix is owned by a `.andromeda/residuals.md` line (target: next)»

- leaf: CLAUDE.md:77   class: omission
  leaf-text: «checks `tests/blitz-tests/tests/stand_{element_ids,id_persistence,id_edits,actionable_keys}.rs`»
  source: .andromeda/architecture.md:136 «on Temperature Converter, Circle Drawer and Cells too (2 · 3 · 676 until then), Circle Drawer's dialog open included — keyed by nine author ids added as `id:` lines alone»
  replace-with: «checks `tests/blitz-tests/tests/stand_{element_ids,id_persistence,id_edits,actionable_keys,non_lean_tasks}.rs`»
