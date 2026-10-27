proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 67
    section: "## Established Decisions"
    change: "In line 67 the clause «at upstream `main` `f9cbb2756` upstream's taken hunks reach `node/element.rs`, `node/mod.rs`, `layout/construct.rs` and `mutator.rs`, the one conflict `construct.rs`'s import list, resolved to upstream's list plus `RangeInputData` (as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/take.md)» reads: at upstream `main` `ebacd68bd` this sync's eleven taken commits reach `node/element.rs`, `layout/construct.rs` and `mutator.rs`, and the one conflict is `mutator.rs`'s `set_attribute`, resolved by placing upstream's `open` arm after the range re-sanitize arm (`min` · `max` · `step` → `resanitize_range_input`) as a second independent `if` (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/take.md)"
    sidecar: "established-decisions [Scrolling and input]: the range merge surface's upstream take reading moves from `f9cbb2756` to `ebacd68bd`, the one conflict now `mutator.rs`'s `set_attribute` (2026-10-27-upstream-sync-engine-debts)"
    rationale: "The report's Counts `upstream` names `established-decisions.md:67` changed, a take-read-at reading; Symbols put the `open` arm after the range re-sanitize arm as the take's resolution 3, and the reached files list `mutator.rs` as a conflict surface and `node/element.rs` and `layout/construct.rs` as merge surfaces"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:34
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 69
    section: "## Established Decisions"
    change: "In line 69 the clause «at upstream `main` `f9cbb2756` upstream's taken hunks reach `node/element.rs`, with no conflict (as measured at escher-0.2.0/chunks/2026-10-24-upstream-sync-polish-and-ship/evidence/take.md)» reads: at upstream `main` `ebacd68bd` this sync's taken hunks reach `node/element.rs` (`ElementData::flush_open_state` and its call from `ElementData::new`) with no conflict, and leave `events/keyboard.rs`, `events/mod.rs` and `events/pointer.rs` alone (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/take.md)"
    sidecar: "established-decisions [Scrolling and input]: the keyboard-activation merge surface's upstream take reading moves from `f9cbb2756` to `ebacd68bd`, `node/element.rs` reached with no conflict (2026-10-27-upstream-sync-engine-debts)"
    rationale: "The report's Counts `upstream` names `established-decisions.md:69` changed; Changes list `node/element.rs` as modified (flush_open_state at 543-549 and the call at 488-489) and no `events/*` file among the 41"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:13
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 75
    section: "## Established Decisions"
    change: "Line 75's closing clause «is owned by a `.andromeda/residuals.md` line (target: next), re-read at the next upstream sync.» reads: is owned by a `.andromeda/residuals.md` line (target: next). The 2026-10-27 upstream sync re-read it row by row with its owner quoted (escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/evidence/readings.md), and the following sync re-reads it again: Epoch 3's «Upstream sync ahead of the UI graph»."
    sidecar: "established-decisions [Scrolling and input]: the engine-debts merge surface's residual was re-read at the 2026-10-27 sync and its owner moved to Epoch 3's «Upstream sync ahead of the UI graph» (inputs#I5)"
    rationale: "The report's Counts `res-35-36` name `established-decisions.md:75` read for an owner; the operator's P5 «yes» (inputs#I5) moves residual line 36's owner to the Epoch 3 entry, and the Outcome records evidence/readings.md re-reading every row with its owner quoted, so «re-read at the next upstream sync» was done by this sync and now points at the following one"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:46
  - detector: D-cascade
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: "In line 78 the merge-surface entry «blitz-vibey-script `runtime.rs`» reads blitz-vibey-script `runtime/events.rs`, where the 2026-10-27 take's runtime split moved the select's event dispatch and escher's select-by-id comment. The rest of the list is unchanged."
    sidecar: "established-decisions [Scrolling and input]: the select model's blitz-vibey-script merge surface moves from `runtime.rs` to `runtime/events.rs` with the runtime split (2026-10-27-upstream-sync-engine-debts)"
    rationale: "The report's Symbols carry escher's select-by-id comment to `runtime/events.rs:171-173` in the `e6f4615ec` split, where `runtime.rs` keeps 151 lines; Reverted facts record upstream's comment in `runtime/events.rs` replaced by escher's (residual line 36's select model), and the reached files list `runtime/events.rs` as a conflict surface"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:16
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 78
    section: "## Established Decisions"
    change: "Add after line 78, as a record of its own: - Six behaviour changes for every document, taken at the 2026-10-27 upstream sync, are upstream's and are not counted among escher's direct engine changes above. They are `:open` ([DOM semantics]); the `::before`/`::after` flow vote and IME at a focused input's editor construction ([Layout]); the windowed shell's redraw route ([Shell and windowing]); and the `file:` script-load refusal and the CSS property accessors on `CSSStyleDeclaration` (§Standard Contracts → JavaScript). The founder ratified them for this take with his word «Одиннадцать коммитов (Рекомендую)» (2026-10-27, relayed verbatim by the overseer), under the playbook pattern for an engine behaviour change for every document. The `file:` refusal is recorded as a narrowing and the accessors as a realization on the registered script-to-DOM crossing (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)."
    sidecar: "established-decisions [Scrolling and input]: the 2026-10-27 take's six every-document behaviour changes recorded as upstream's, ratified by the founder for this take and not counted as escher's direct changes (F8)"
    rationale: "The report's Decisions & corrections record the take as the founder's own word (inputs#I4), ratifying the six behaviour changes for this take under the playbook pattern, with `cee6b8731` a narrowing and `efdc5472a` a realization on the registered crossing. The task asks that the series counting escher's direct changes not absorb them"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:98
    fact: F8
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 104
    section: "## Established Decisions"
    change: "Add after line 104, as a continuation line of the [DOM semantics] record: `:open` matches a `details` or `dialog` that carries `open`. The public `ElementData::flush_open_state` sets `ElementState::OPEN`, called from `ElementData::new` and from `DocumentMutator::set_attribute` / `remove_attribute` on an `open` write, and `NonTSPseudoClass::Open` reads that state where before it always read false (packages/blitz-dom/src/node/element.rs:543-549; packages/blitz-dom/src/node/element.rs:488-489; packages/blitz-dom/src/mutator.rs:368-371; packages/blitz-dom/src/mutator.rs:520-523; packages/blitz-dom/src/stylo.rs:510). It is upstream's, taken at the 2026-10-27 sync, and has no test of its own: `details_element` and the a11y leg are green, and `summary` activation is unchanged (as measured at escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md)."
    sidecar: "established-decisions [DOM semantics]: `:open` matches a details or dialog carrying `open` through `ElementData::flush_open_state` (F1, upstream's, 2026-10-27 sync)"
    rationale: "The report's Symbols state `flush_open_state` (new, pub), its callers and the `NonTSPseudoClass::Open` read, and Counts `open` say no master states `:open`, so it is a new fact for its owner"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:13
    fact: F1
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 91
    section: "## Established Decisions"
    change: "Add after line 91, as a continuation line of the [Layout] record: every box-generating `::before` / `::after` votes in flow classification (`classify_flow_children`), now that the `display: contents` filter is dropped (packages/blitz-dom/src/layout/construct.rs:860; packages/blitz-dom/src/layout/construct.rs:867-872). The vote is held by blitz-tests `pseudo_element_flow_classification`: an inline-block whose only child is a pseudo exports its line baseline and bottom-aligns, and a grid row grows by the baseline shim of pseudo-only items (tests/blitz-tests/tests/pseudo_element_flow_classification.rs:37-48; tests/blitz-tests/tests/pseudo_element_flow_classification.rs:50-60; tests/blitz-tests/tests/pseudo_element_flow_classification.rs:62-78). That file does not assert both layout modes, and `incremental_oracle` is green. It is upstream's `2963b65a2`, taken at the 2026-10-27 sync."
    sidecar: "established-decisions [Layout]: box-generating `::before`/`::after` vote in flow classification, the `display: contents` filter dropped (F2, upstream's `2963b65a2`)"
    rationale: "The report's Symbols state the `classify_flow_children` change at construct.rs:860 and :867-872, Coverage records the new target's three tests with both layout modes not asserted by the file, and Counts `pseudo` say no master states the vote"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:14
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 91
    section: "## Established Decisions"
    change: "Add after line 91, as a continuation line of the [Layout] record, beside its text-input editor rule: a text input that is already focused when its editor is constructed enables IME through `shell_provider.set_ime_enabled(true)` in `create_text_editor` (packages/blitz-dom/src/layout/construct.rs:313-317). This is held by dioxus-native-dom's `input_focused_on_mount_enables_ime` (packages/dioxus-native-dom/src/dioxus_document.rs:725-759) and is upstream's `7fc9ee3af`, taken at the 2026-10-27 sync."
    sidecar: "established-decisions [Layout]: a text input focused before its editor exists enables IME at the editor's construction (F3, upstream's `7fc9ee3af`)"
    rationale: "The report's Symbols state the `create_text_editor` call at construct.rs:313-317, and Coverage names the unit test `input_focused_on_mount_enables_ime`. The Layout record already holds the rule that text inputs get a Parley editor at construction"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:14
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 131
    section: "## Established Decisions"
    change: "Add after line 131, as a continuation line of the [Shell and windowing] record: the windowed shell's redraw requests travel through its event loop (packages/blitz-shell/src/lib.rs:90; packages/blitz-shell/src/lib.rs:93-98; packages/blitz-shell/src/lib.rs:104-106; packages/blitz-shell/src/window.rs:193-194). This is upstream's `6587e7e79`, taken at the 2026-10-27 sync, held by the windowed `seven_guis_native` smoke boot and `focus_redraw`."
    sidecar: "established-decisions [Shell and windowing]: the windowed shell routes redraw requests through its event loop (F4, upstream's `6587e7e79`)"
    rationale: "The report's Symbols state the blitz-shell redraw route at lib.rs:90, :93-98, :104-106 and window.rs:193-194, and Coverage names the smoke boot and `focus_redraw`. Counts `redraw` read `established-decisions.md:131` at P2"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:15
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/established-decisions.md
    near: 155
    section: "## Established Decisions"
    change: "Add after line 155, as a continuation line of the [Browser app] record: a tab's lenses survive a replacement of its webview props (apps/browser/src/tab.rs:130-136; apps/browser/src/tab.rs:177-182), held by `tab_lens_survives_webview_props_replacement` (apps/browser/src/tab.rs:301-342). This is upstream's `d63ed2c8c`, taken at the 2026-10-27 sync, in the upstream-only app."
    sidecar: "established-decisions [Browser app]: tab lenses survive a webview props replacement (F9, upstream's `d63ed2c8c`)"
    rationale: "The report's Symbols state the apps/browser tab-lens change at tab.rs:130-136 and :177-182 with its test module, and Coverage names `tab_lens_survives_webview_props_replacement`"
    basis: escher-0.3.0/chunks/2026-10-27-upstream-sync-engine-debts/report.md:17
    fact: F9
note: "[JavaScript] (dec-javascript, line 143; body architecture.md:84) holds after the move. The `file:` narrowing (F5), the CSS accessors (F6) and the runtime and element module trees (F7) are standard-contracts' and existing-scopes' facts, so the new-detail pass places them; if the plan's expected [JavaScript] amendment is wanted, it is a pointer clause there. The new pub APIs (`flush_open_state`, `document_may_load`, the `FetchError` variant) and the second test-only `TMPDIR` read are D-arch-resources items for standard-contracts and occupied-resources. Lines 76 and 78 say «owned by the same `.andromeda/residuals.md` line», which still holds after line 75's edit."
