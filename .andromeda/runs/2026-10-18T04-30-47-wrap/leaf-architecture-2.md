group: architecture-2
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»
read: .claude/docs/services/seven_guis.md · 44 · «- `.claude/rules/a11y.md` (the stand is in its paths) · `.cl»

- leaf: .claude/docs/services/escher-driver.md:35   class: retired
  leaf-text: «two of the tool's own texts say measured limits — `covered`'s meaning, two readings — the `contain: paint` one and, since 2026-10-14-stand-requirement-sweep, a box clipped on one axis stopping a hit at its whole padding box, so an element lying outside it along the axis it does not clip can read `covered` though it shows,»
  source: .andromeda/architecture.md:160 «`covered`'s meaning reads "another element is hit at the point the action would land; a box clipped on one axis stops a hit at its whole padding box, so an element lying outside it along the axis»
  replace-with: «three of the tool's own texts say measured limits — `covered`'s meaning, two readings — a box clipped on one axis stopping a hit at its whole padding box, so an element lying outside it along the axis it does not clip can read `covered` though it shows, and a box with no width or no height hit at its centre on the element that holds it, so it can read `covered` though nothing covers it; `off-screen`'s meaning, one — a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows;»

- leaf: .claude/docs/services/escher-driver.md:53   class: retired
  leaf-text: «Measured limit, unfixed: the engine's hit reaches content clipped by `contain: paint` beside a `visible` `overflow`, so a control lying where such content extends reads `covered` though nothing shows over it — `covered`'s meaning says so.»
  source: .andromeda/architecture.md:160 «because the engine's hit stops at the padding box of a box paint clips (the Scrolling, selection, tree contract above); two readings are off — a box with no width or no height, hit at its centre»
  replace-with: «Fixed in the engine (PROVISIONAL): a hit stops at the padding box of a box paint clips — by `overflow` or by a computed `contain` that includes `paint` — so content clipped by `contain: paint` no longer reads `covered`. Measured limit, unfixed: a box with no width or no height is hit at its centre on the element that holds it, so it reads `covered` though nothing covers it (the stand's `task-header-spacer`, 0 high) — `covered`'s meaning says so.»

- leaf: .claude/docs/services/escher-driver.md:18   class: omission
  leaf-text: «`off-screen` (the centre of its `bounds` is outside `BaseDocument::visible_region` — the viewport and every scrolling box that holds it)»
  source: .andromeda/architecture.md:160 «its meaning reads "the element lies outside the viewport, or outside the visible part of a scrolling box that holds it; a scrolling box under a transform is read where it stands untransformed, so»
  replace-with: «`off-screen` (the centre of its `bounds` is outside `BaseDocument::visible_region` — the viewport and every scrolling box that holds it; a scrolling box under a transform is read where it stands untransformed, so an element inside it can read `off-screen` though it shows, a measured limit `off-screen`'s meaning states)»

- leaf: .claude/docs/services/escher-driver.md:24   class: omission
  leaf-text: «whose help states the key's reading — the two limits among them —»
  source: .andromeda/architecture.md:160 «`checked` a native checkbox's or radio's own state, else the toggle of the five checkable roles (`aria-checked`), a mixed one reading false, `selected` · `pressed` · `invalid` read by `aria-selected`»
  replace-with: «whose help states the key's reading — `checked` a native checkbox's or radio's own state, else the toggle of the five checkable roles (`aria-checked`), a mixed one reading false, `selected` · `pressed` · `invalid` read by `aria-selected` · `aria-pressed` (left out for those five roles) · `aria-invalid`, the two limits among them —»

- leaf: .claude/docs/services/escher-driver.md:29   class: stale
  leaf-text: «whose thirteen `stand_act_*` checks»
  source: .andromeda/architecture.md:128 «the settle check `stand_settle`, the fourteen driver-action checks `stand_act_*`»
  replace-with: «whose fourteen `stand_act_*` checks»

- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «in blitz-tests — thirteen files —»
  source: .andromeda/architecture.md:290 «— and a fourteenth, `stand_act_transformed`, a `click` on a translated button accepted and landing and its snapshot `bounds` read where it shows, 2 tests in both layout modes»
  replace-with: «in blitz-tests — fourteen files —»

- leaf: .claude/docs/services/escher-driver.md:61   class: omission
  leaf-text: «and a shot changing nothing); beside them»
  source: .andromeda/architecture.md:290 «— and a fourteenth, `stand_act_transformed`, a `click` on a translated button accepted and landing and its snapshot `bounds` read where it shows, 2 tests in both layout modes»
  replace-with: «and a shot changing nothing) and a fourteenth, `stand_act_transformed` (a `click` on a translated button accepted and landing and its snapshot `bounds` read where it shows, 2 tests in both layout modes); beside them»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «The thirteen driver-action checks `stand_act_{ids,diff,timer,refused,keys,range,disabled,obstructed,scroll,spans,filled,states,screenshot}.rs`»
  source: .andromeda/architecture.md:290 «— and a fourteenth, `stand_act_transformed`, a `click` on a translated button accepted and landing and its snapshot `bounds` read where it shows, 2 tests in both layout modes»
  replace-with: «The fourteen driver-action checks `stand_act_{ids,diff,timer,refused,keys,range,disabled,obstructed,scroll,spans,filled,states,screenshot,transformed}.rs`»

- leaf: .claude/docs/services/seven_guis.md:38   class: stale
  leaf-text: «skips a `disabled` attribute of any value on Tab (the `<div disabled tabindex=0>` limit pinned)»
  source: .andromeda/architecture.md:290 «a `disabled` attribute skipping a form control by Tab and not a `<div disabled tabindex=0>`»
  replace-with: «reads a `disabled` attribute skipping a form control on Tab and not a `<div disabled tabindex=0>`»

- leaf: .claude/docs/services/seven_guis.md:38   class: retired
  leaf-text: «and pins that the engine honours no `inert`»
  source: .andromeda/architecture.md:290 «the Apple delete binding on every OS; an inert subtree skipped by Tab;»
  replace-with: «and reads an inert subtree skipped by Tab»
