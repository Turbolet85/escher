# Apply — architecture · part 1 of 2

## Verdicts
- fanout-architecture.md 1 · applied · .andromeda/architecture.md:216#1
- fanout-architecture--cascade-g6.md 2 · folded into fanout-architecture.md 1
- fanout-architecture.md 2 · applied · .andromeda/architecture.md:173#1
- fanout-architecture.md 4 · applied · .andromeda/architecture.md:287#1
- fanout-architecture--cascade-g6.md 6 · folded into fanout-architecture.md 4
- fanout-architecture.md 5 · applied · .andromeda/architecture.md:205#1, .andromeda/architecture.md:205#2, .andromeda/architecture.md:254#1, .andromeda/architecture.md:269#1, .andromeda/architecture.md:298#1
- fanout-architecture--cascade-g2.md 1 · applied · .andromeda/architecture-detail/established-decisions.md:67#1
- fanout-architecture--cascade-g2.md 2 · applied · .andromeda/architecture-detail/established-decisions.md:69#1
- fanout-architecture--cascade-g2.md 3 · applied · .andromeda/architecture-detail/established-decisions.md:75#1
- fanout-architecture--cascade-g2.md 4 · applied · .andromeda/architecture-detail/established-decisions.md:78#1
- fanout-architecture--cascade-g2.md 5 · applied · .andromeda/architecture-detail/established-decisions.md:78+#1
- fanout-architecture--cascade-g2.md 6 · applied · .andromeda/architecture-detail/established-decisions.md:119+#1
- fanout-architecture--cascade-g2.md 7 · applied · .andromeda/architecture-detail/established-decisions.md:97+#1
- fanout-architecture--cascade-g2.md 8 · applied · .andromeda/architecture-detail/established-decisions.md:97+#2
- fanout-architecture--cascade-g2.md 9 · applied · .andromeda/architecture-detail/established-decisions.md:133+#1
- fanout-architecture--cascade-g2.md 10 · applied · .andromeda/architecture-detail/established-decisions.md:157+#1
- fanout-architecture.md 8 · applied · .andromeda/architecture-detail/established-decisions.md:39#1, .andromeda/architecture-detail/established-decisions.md:143#1, .andromeda/architecture-detail/established-decisions.md:143#2, .andromeda/architecture-detail/established-decisions.md:143#3, .andromeda/architecture-detail/established-decisions.md:143#4, .andromeda/architecture-detail/established-decisions.md:143#5
- fanout-architecture.md 9 · applied · .andromeda/architecture-detail/established-decisions.md:76#1
- fanout-architecture--cascade-g3.md 1 · applied · .andromeda/architecture-detail/conventions.md:33#1
- fanout-architecture--cascade-g3.md 2 · applied · .andromeda/architecture-detail/conventions.md:53#1
- fanout-architecture.md 6 · applied · .andromeda/architecture-detail/conventions.md:27#1, .andromeda/architecture-detail/conventions.md:33#2, .andromeda/architecture-detail/conventions.md:46#1

## Edits
- line: .andromeda/architecture.md:216
  old: «`TMPDIR`, in one unit test only;»
  new: «`TMPDIR`, at two test-only sites (escher-driver's client unit test and blitz-vibey-script's `tests/dom.rs`);»
- line: .andromeda/architecture.md:173
  old: «run a document's scripts on Boa, JS listeners before Blitz default actions;»
  new: «run a document's scripts on Boa, JS listeners before Blitz default actions; the public `document_may_load` lets a `file:` URL load only for a `file:` document, its refusal a `FetchError` variant, checked by `fetch()`, a module `import` and `<script src>`;»
- line: .andromeda/architecture.md:287
  old: «upstream `main` last read at `f9cbb2756`, thirteen of its commits taken by no-commit cherry-picks with the merge base unmoved;»
  new: «upstream `main` last read at `ebacd68bd`, twenty-four of the forty commits past the merge base taken by no-commit cherry-picks with the merge base unmoved and sixteen left out (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/lock.md);»
- line: .andromeda/architecture.md:205
  old: «packages/blitz-vibey-script/src/runtime.rs:1352-1358»
  new: «packages/blitz-vibey-script/src/runtime/bootstrap.rs:59-65»
- line: .andromeda/architecture.md:205
  old: «packages/blitz-vibey-script/src/runtime.rs:2009-2017»
  new: «packages/blitz-vibey-script/src/runtime/window.rs:76-84»
- line: .andromeda/architecture.md:254
  old: «the ES module loader fetches through the ScriptFetcher and caches modules by URL (packages/blitz-vibey-script/src/runtime.rs:1183-1246)»
  new: «the ES module loader fetches through the ScriptFetcher, gated by `document_may_load`, and caches modules by URL (packages/blitz-vibey-script/src/runtime/modules.rs:16-92)»
- line: .andromeda/architecture.md:269
  old: «packages/blitz-vibey-script/src/runtime.rs:1097-1116»
  new: «packages/blitz-vibey-script/src/runtime/events.rs:13-32»
- line: .andromeda/architecture.md:298
  old: «packages/blitz-vibey-script/src/runtime.rs:38-41»
  new: «packages/blitz-vibey-script/src/runtime/bootstrap/window.js:1-4»
- line: .andromeda/architecture-detail/established-decisions.md:67
  old: «at upstream `main` `f9cbb2756` upstream's taken hunks reach `node/element.rs`, `node/mod.rs`, `layout/construct.rs` and `mutator.rs`, the one conflict `construct.rs`'s import list, resolved to upstream's list plus `RangeInputData` (as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/take.md)»
  new: «at upstream `main` `ebacd68bd` the taken commits reach `node/element.rs`, `layout/construct.rs` and `mutator.rs`, the one conflict `mutator.rs`'s `set_attribute`, resolved by placing upstream's `open` arm after the range re-sanitize arm (`min` · `max` · `step` → `resanitize_range_input`) as a second independent `if` (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/take.md)»
- line: .andromeda/architecture-detail/established-decisions.md:69
  old: «at upstream `main` `f9cbb2756` upstream's taken hunks reach `node/element.rs`, with no conflict (as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/take.md)»
  new: «at upstream `main` `ebacd68bd` the taken commits reach `node/element.rs` (`ElementData::flush_open_state` and its call from `ElementData::new`), with no conflict, and leave `events/keyboard.rs`, `events/mod.rs` and `events/pointer.rs` untouched (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/take.md)»
- line: .andromeda/architecture-detail/established-decisions.md:75
  old: «is owned by a `.andromeda/residuals.md` line (target: next), re-read at the next upstream sync.»
  new: «is owned by a `.andromeda/residuals.md` line, re-read row by row with its owner quoted at the 2026-10-27 upstream sync (escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/readings.md), its owner Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5).»
- line: .andromeda/architecture-detail/established-decisions.md:78
  old: «blitz-vibey-script `runtime.rs` — is owned by the same `.andromeda/residuals.md` line (target: next).»
  new: «blitz-vibey-script `runtime/events.rs` — is owned by the same `.andromeda/residuals.md` line, whose owner is Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5).»
- after: .andromeda/architecture-detail/established-decisions.md:78
  anchor: « Flagged upstreamable, with no upstream pull request open; its merge surface, by file — blitz-dom `node/select.rs`, `document.rs`, `events/{keyboard,pointer,mod}.rs`, `mutator.rs`, `accessibility.rs`, `form.rs`, `node/element.rs` and `assets/default.css`, blitz-traits `events.rs` and blitz-vibey-script `runtime.rs` — is owned by the same `.andromeda/residuals.md` line (target: next).»
  add: «- Six behaviour changes for every document are upstream's and are not counted among escher's direct engine changes above: `:open` ([DOM semantics]); the `::before` / `::after` flow vote and IME at a focused input's editor construction ([Layout]); the windowed shell's redraw route ([Shell and windowing]); the `file:` script-load refusal (§Standard Contracts → JavaScript); and the CSS property accessors on `CSSStyleDeclaration`. Each stands ratified for its take under the playbook pattern for an engine behaviour change for every document; the `file:` refusal is a narrowing and the accessors a realization on the registered script-to-DOM crossing (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md).»
- after: .andromeda/architecture-detail/established-decisions.md:119
  anchor: «- A hit test skips a node dropped since the last resolve, and the paint tree's walk does the same — an upstream engine change for every document (§Standard Contracts → Scrolling, selection, tree).»
  add: «- `:open` matches a `details` or `dialog` that carries `open`: the public `ElementData::flush_open_state` sets the `OPEN` element state, called from `ElementData::new` and from `DocumentMutator::set_attribute` / `remove_attribute` on an `open` write, and `NonTSPseudoClass::Open` reads that state (packages/blitz-dom/src/node/element.rs:543-549; packages/blitz-dom/src/node/element.rs:488-489; packages/blitz-dom/src/mutator.rs:368-371; packages/blitz-dom/src/mutator.rs:520-523; packages/blitz-dom/src/stylo.rs:510) — an upstream engine change for every document with no test of its own, `details_element` and the a11y leg green and `summary` activation unchanged (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md).»
- after: .andromeda/architecture-detail/established-decisions.md:97
  anchor: «- A replaced element's natural height follows its usable `aspect-ratio` in `compute_replaced_layout` (`is_usable_ratio`: finite and > 0), clamped at 0, an upstream engine change for every document (packages/blitz-dom/src/layout/replaced.rs:144-145; packages/blitz-dom/src/layout/replaced.rs:201-211).»
  add: «- Every box-generating `::before` / `::after` votes in flow classification (`classify_flow_children`) (packages/blitz-dom/src/layout/construct.rs:860; packages/blitz-dom/src/layout/construct.rs:867-872): an inline-block whose only child is a pseudo exports its line baseline and bottom-aligns, and a grid row grows by the baseline shim of pseudo-only items (tests/blitz-tests/tests/pseudo_element_flow_classification.rs:37-48; tests/blitz-tests/tests/pseudo_element_flow_classification.rs:50-60; tests/blitz-tests/tests/pseudo_element_flow_classification.rs:62-78) — an upstream engine change for every document, its test file not asserting both layout modes (§Conventions → Tests).»
- after: .andromeda/architecture-detail/established-decisions.md:97
  anchor: «- A replaced element's natural height follows its usable `aspect-ratio` in `compute_replaced_layout` (`is_usable_ratio`: finite and > 0), clamped at 0, an upstream engine change for every document (packages/blitz-dom/src/layout/replaced.rs:144-145; packages/blitz-dom/src/layout/replaced.rs:201-211).»
  add: «- A text input already focused when its editor is constructed enables IME through `shell_provider.set_ime_enabled(true)` in `create_text_editor` (packages/blitz-dom/src/layout/construct.rs:313-317), held by dioxus-native-dom's `input_focused_on_mount_enables_ime` (packages/dioxus-native-dom/src/dioxus_document.rs:725-759) — an upstream engine change for every document.»
- after: .andromeda/architecture-detail/established-decisions.md:133
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-shell-and-windowing -->»
  add: «»
  add: «- The windowed shell's redraw requests travel through its event loop (packages/blitz-shell/src/lib.rs:90; packages/blitz-shell/src/lib.rs:93-98; packages/blitz-shell/src/lib.rs:104-106; packages/blitz-shell/src/window.rs:193-194), held by the windowed `seven_guis_native` smoke boot and `focus_redraw` — an upstream engine change for every document.»
  add: «»
  add: «<!-- /detail -->»
- after: .andromeda/architecture-detail/established-decisions.md:157
  anchor: «<!-- /detail -->»
  add: «»
  add: «<!-- detail: values · dec-browser-app -->»
  add: «»
  add: «- A tab's lenses survive a replacement of its webview props (apps/browser/src/tab.rs:130-136; apps/browser/src/tab.rs:177-182), held by `tab_lens_survives_webview_props_replacement` (apps/browser/src/tab.rs:301-342) — upstream's, in the upstream-only app.»
  add: «»
  add: «<!-- /detail -->»
- line: .andromeda/architecture-detail/established-decisions.md:39
  old: «packages/blitz-vibey-script/src/dom/element.rs:1072-1073»
  new: «packages/blitz-vibey-script/src/dom/element/geometry.rs:78-79»
- line: .andromeda/architecture-detail/established-decisions.md:143
  old: «packages/blitz-vibey-script/src/dom/element.rs:964-968»
  new: «packages/blitz-vibey-script/src/dom/element/markup.rs:82-86»
- line: .andromeda/architecture-detail/established-decisions.md:143
  old: «packages/blitz-vibey-script/src/runtime.rs:360-385»
  new: «packages/blitz-vibey-script/src/runtime/bootstrap/interfaces.js:23-48»
- line: .andromeda/architecture-detail/established-decisions.md:143
  old: «packages/blitz-vibey-script/src/runtime.rs:38-76»
  new: «packages/blitz-vibey-script/src/runtime/bootstrap/window.js:1-39»
- line: .andromeda/architecture-detail/established-decisions.md:143
  old: «packages/blitz-vibey-script/src/runtime.rs:1341-1345»
  new: «packages/blitz-vibey-script/src/runtime/bootstrap.rs:48-52»
- line: .andromeda/architecture-detail/established-decisions.md:143
  old: «packages/blitz-vibey-script/src/runtime.rs:1671-1688»
  new: «packages/blitz-vibey-script/src/runtime/events.rs:171-188»
- line: .andromeda/architecture-detail/established-decisions.md:76
  old: «`packages/blitz-dom/src/events/pointer.rs` — owned by the same `.andromeda/residuals.md` line (target: next).»
  new: «`packages/blitz-dom/src/events/pointer.rs` — owned by the same `.andromeda/residuals.md` line, whose owner is Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5).»
- line: .andromeda/architecture-detail/conventions.md:33
  old: «blitz-vibey-script re-exports only `ScriptDocument`, `DefaultScriptFetcher`, `FetchError`, `ScriptFetcher` (packages/blitz-vibey-script/src/lib.rs:32-43)»
  new: «blitz-vibey-script re-exports only `ScriptDocument`, `DefaultScriptFetcher`, `FetchError`, `ScriptFetcher` and `document_may_load`, the check that lets a `file:` URL load only for a `file:` document (packages/blitz-vibey-script/src/lib.rs:32-43)»
- line: .andromeda/architecture-detail/conventions.md:53
  old: «scenarios run in both layout modes `for incremental in [false, true]` (tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:92-101; tests/blitz-tests/tests/flex_grid_order.rs:85-99);»
  new: «scenarios run in both layout modes `for incremental in [false, true]` (tests/blitz-tests/tests/anonymous_block_cache_invalidation.rs:92-101; tests/blitz-tests/tests/flex_grid_order.rs:85-99), except upstream's `pseudo_element_flow_classification`, which does not assert both layout modes, `incremental_oracle` green beside it;»
- line: .andromeda/architecture-detail/conventions.md:27
  old: «packages/blitz-vibey-script/src/runtime.rs:2056; packages/blitz-vibey-script/src/dom/element.rs:224»
  new: «packages/blitz-vibey-script/src/runtime/window.rs:123; packages/blitz-vibey-script/src/dom/element/attributes.rs:15»
- line: .andromeda/architecture-detail/conventions.md:33
  old: «packages/blitz-vibey-script/src/runtime.rs:1386-1408»
  new: «packages/blitz-vibey-script/src/runtime/bootstrap.rs:93-121»
- line: .andromeda/architecture-detail/conventions.md:46
  old: «packages/blitz-vibey-script/src/runtime.rs:1498-1500»
  new: «packages/blitz-vibey-script/src/runtime/modules.rs:115-117»

## Patterns
[[pattern]]
id = "arch1-01"
fixed = 'in one unit test only'

[[pattern]]
id = "arch1-02"
fixed = 'last read at `f9cbb2756`'

[[pattern]]
id = "arch1-03"
fixed = "`f9cbb2756` upstream's taken hunks reach"

[[pattern]]
id = "arch1-04"
fixed = 're-read at the next upstream sync'

[[pattern]]
id = "arch1-05"
fixed = 'owned by the same `.andromeda/residuals.md` line (target: next)'

[[pattern]]
id = "arch1-06"
fixed = 'blitz-vibey-script `runtime.rs`'

[[pattern]]
id = "arch1-07"
regex = 're-exports only `ScriptDocument`, `DefaultScriptFetcher`, `FetchError`,? (and )?`ScriptFetcher`(?! and)'
