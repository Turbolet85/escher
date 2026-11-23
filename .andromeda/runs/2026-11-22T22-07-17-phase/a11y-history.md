@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — created ci.yml's `a11y` job ("Accessibility (a11y) tests", `bash .github/scripts/ci-leg.sh a11y`, slow tier, no required status check); this sync must keep that leg green (scope: "the a11y leg and the dependency audit") and keeps decision/a11y-ci-gate · decision/a11y-ci-leg · decision/a11y-test-leg intact.

@history_item n=2 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** (both entries) — first upstream merge's precedent: `file:line` citations into merged upstream files re-pointed by the merge's measured line map with no claim text changed, the a11y files left unnarrowed, and root `Cargo.toml` citations re-pointed after an insert; the same citation sweep falls to this merge's 21 touched files.

@history_item n=3 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — set the Dioxus `accessibility` feature wiring (workspace takes Dioxus crates `default-features = false`, a crate gets the feature only by naming it; the stand binary builds no platform adapter) and the tree through `Document::accessibility_tree`; rests under decision/dioxus-accessibility-feature-by-name and decision/a11y-tree-through-document-trait, which a Dioxus 0.8 fork option or a manifest conflict must not disturb.

@history_item n=4 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the bridge's falsy `disabled` clear and disabled-keyed-two-ways trap (focusability vs state/click); history of decision/falsy-boolean-attribute(s)-removed and the disabled-by-presence rows that a Dioxus move or a mutator conflict would touch.

@history_item n=5 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — ratified the falsy-`disabled` engine fix; part of the ratified Dioxus-bridge behaviour this sync's Dioxus hold fork must keep.

@history_item n=6 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot consumes the accessibility tree with no second role/name mapping (decision/snapshot-reads-accessibility-tree, decision/snapshot-model-single-source); the stable ids and the snapshot are on the scope's keep-both-sides conflict list.

@history_item n=7 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the actionable-key rule and `unkeyed_actionable` (decision/actionable-key-rule); trap: never satisfy the check by adding a `tabindex` or role, which moves the pinned Tab order — applies to any conflict resolution in the stand or bridge.

@history_item n=8 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — falsy boolean attributes removed on both attribute paths so `hidden: false` stays in the tree; key-press focus checks; the bridge's boolean predicate is tied to the Dioxus version, so the Dioxus 0.7.3 hold / 0.8 fork re-reads it.

@history_item n=9 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — measured the obs log format behind the allowlist scrub (§3 NOT YET MEASURED note); scope's sink allowlist re-check over the merged delta's new `tracing`/`log` call sites reads against it.

@history_item n=10 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — made the engine's changed set truthful and the tree refreshed on change (decision/a11y-tree-refresh-on-change, decision/changed-set-marks); "the changed set's contract" heads the scope's keep-both-sides conflict list, so an upstream hunk in `document.rs`/`mutator.rs` must keep every marking site.

@history_item n=11 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — founder ratified the refresh on change and the changed-set contract (and the falsy clear); ratified behaviour a merge may not regress by a default.

@history_item n=12 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink drops any non-allowlisted target whole and prints only safe fields for `blitz*`/`accesskit_xplat`; the scope's sink re-check (new call sites, targets, prints, env reads; capture checks at 0 needles) rests on it.

@history_item n=13 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — harness settle neither reads nor drains the changed set, and a fresh harness document holds a non-empty set (decision/harness-settle-leaves-changed-set); a merge touching settle or the redraw must keep it.

@history_item n=14 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the driver's `disabled` cause stated on the presence reading of the snapshot's `enabled`; rule for later chunks: read `enabled`, never the attribute value (contract/driver-actions/disabled, contract/driver-refusal/disabled).

@history_item n=15 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — escher-driver names `accessibility` on dioxus-native-dom unconditionally (decision/driver-requires-accessibility); neither stand binary builds `accesskit_xplat`/`accesskit_winit`; the lockfile byte-identical — the measurement each later sync repeats on its lock (scope: lock moves measured by `cargo build --workspace --locked`).

@history_item n=16 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the obs format's two record classes (event lines, closed-span lines) behind the allowlist, and the driver's `tracing` dependency adding one lock line and no adapter; the sink re-check and the adapter re-read both read against it.

@history_item n=17 marker=2026-10-09-audit-corrections-agent-surfaces
**2026-10-09-audit-corrections-agent-surfaces** — the snapshot's `value` proven on a textarea (role `MultilineTextInput`, typed text read back); upstream `6636442d3` (a textarea's text laid out once its width is known) touches that surface, so `stand_snapshot_state`'s textarea reading must stay green.

@history_item n=18 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — precedent of an upstream sync changing an engine read for every document (autofocus `== "true"` → `!= "false"`) recorded in the plan, and the stand's adapter absence re-read on the merged lock; this sync repeats both disciplines.

@history_item n=19 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — measured that a plain button click left nothing focused (later superseded); part of the hit stops and scrolled-box bounds history the scope's conflict list keeps.

@history_item n=20 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (both entries) — the builder's closed-set lower-case `aria-selected`/`aria-pressed`/`aria-invalid` reads on every element (decision/aria-state-lower-case-closed-values, decision/aria-states-on-every-element); "the accessibility state reads" are on the scope's keep-both-sides list.

@history_item n=21 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — stand graph re-read with the driver's `serde_json` (one lock line, `accesskit_(xplat|winit)` 0 rows); the probe this sync re-runs on any lock move.

@history_item n=22 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a password painted masked through `layout_as_text_input` in blitz-paint, its caret at the bullet; upstream `7beb59176` (IME preedit underline, blitz-paint) and `6636442d3` (textarea layout) touch the text-input paint path, and the screenshot readings are re-measured, never loosened.

@history_item n=23 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — bridge took upstream's private `is_bool_attr` (28 names), `mutation_writer.rs` byte-identical to upstream, falsy `inert` cleared (decision/falsy-boolean-attribute(s)-removed); a Dioxus 0.8 option or a conflict in `mutation_writer.rs` re-reads this identity.

@history_item n=24 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — built `Harness::text_contrast` and `stand_contrast` (decision/stand-contrast-measured, decision/wcag-contrast-thresholds); the scope's stand checks in both modes include it.

@history_item n=25 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the range input's keys, click focus and pointerdown value (contract/range-input/*, decision/range-*); "the range and keyboard models" are on the scope's keep-both-sides conflict list.

@history_item n=26 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (both entries) — Enter/Space activation, a click focusing buttons, `disabled` by presence, the CRUD list's keys, `stand_keyboard` (decision/keyboard-activation-enter-space, decision/click-focuses-buttons, decision/disabled-by-presence, contract/crud-list/*); the keyboard model a merge in `event.rs`/`document.rs` must keep.

@history_item n=27 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** (both entries) — SC 1.4.3 met by the stand accent change, the a11y leg widened (founder's F4: no new job, no required check, no repository setting — decision/a11y-ci-leg), the adapter and its windowed AT witness re-homed to a residual (absence/assistive-technology-reach); a CI edit at this sync keeps that leg shape.

@history_item n=28 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the stand's a11y files also run three rounds in `stand-repeat` under the zero-failure budget (decision/stand-checks-repeated-three-rounds); upstream's new test targets must not break the leg lists.

@history_item n=29 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — an upstream sync took upstream's six builder reads as upstream wrote them (`aria-labelledby`, `aria-describedby`, `aria-description`, `aria-expanded`, `aria-disabled`, `aria-level`) while keeping escher's closed-set state reads and trimmed `aria-label`, one read per attribute (decision/one-read-per-aria-attribute, decision/aria-labelledby-outranks-label, decision/aria-description-to-adapter); the precedent for any builder conflict here: keep both sides' intent, never two reads of one attribute.

@history_item n=30 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (three entries: the toggle per role and snapshot sources; `inert` and `disabled` in focus order, keys and hit; the PROVISIONAL batch ratified) — the per-role toggle read, `role="switch"`, `disabled` narrowed to form controls, `inert` taking focus, hits and the tree, all ratified (decision/toggle-read-by-role, decision/inert-subtree-excluded, decision/focusable-elements); `inert` is on the scope's keep-both-sides list.

@history_item n=31 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — double click read on the document's clock, a harness document on the virtual clock (decision/double-click-on-document-clock); "the clock invariant" is on the scope's keep-both-sides conflict list.

@history_item n=32 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — an upstream sync's selector change (`:focus-within` matching) recorded with focus order unmoved and the a11y leg green; precedent for recording an upstream engine behaviour change for every document and verifying focus order after the merge.

@history_item n=33 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select model in the tree, keys and label: an option under a `select` kept in the tree under `display: none`, its `selected` its chosen state (decision/hidden-content-excluded except, contract/accessibility-tree-states/select-option); "the select model" is on the scope's conflict list, and upstream's `display: contents` commits touch the display-none option exception's layout path.

@history_item n=34 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — an upstream hit change (dropped-node hit skip) recorded for every document, and the stand graph re-read on the take (no package in/out of `Cargo.lock`, neither adapter) — the re-read this sync repeats (decision/stand-builds-no-platform-adapter).

@history_item n=35 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — three a11y witnesses and `select_interaction` joined `stand-repeat` by its named `--test` list, held by `stand_requirement_sweep`'s leg check (guard of decision/stand-checks-repeated-three-rounds).

@history_item n=36 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg runs fourteen files on one `cargo test --workspace --locked` line (founder's word); upstream's new test files (`display_contents.rs`, `ime_preedit_underline.rs`) join neither the a11y leg nor stand-repeat unless named, and the leg's file count is pinned.

@history_item n=37 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the AT witness owned by a residual re-carried, and `docs/capabilities.md`'s `## Accessibility reach` restating the a11y leg; a leg or reach change from this sync is edited there by hand.

@history_item n=38 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (both entries) — the flight booker's select in the stand tables, the lint at boot, a click under an open list refused `covered`, the scrollbar band drag after a scroll (decision/driver-click-in-shown-scrollbar-drags, contract/driver-click-refusals/covered); stand readings this sync re-measures in both layout modes.

@history_item n=39 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the stand-repeat list's membership held jointly by the sweep's leg check and `test_stand_repeat_leg_runs_three_rounds`; trap: the sweep reads `ci-leg.sh` as one text, so a name dropped from one leg alone reads green.

@history_item n=40 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (six entries) — the last epoch-style upstream take's a11y footprint: stand graph re-read on the take (manifests, lock, `deny.toml` unchanged, adapters 0 rows); citations into moved upstream code re-pointed, blitz-vibey-script's element bindings split into modules; upstream's `:open`, CSS-property accessors (decision/script-style-through-inline-attribute) and IME at editor construction (decision/ime-enabled-for-prefocused-input) recorded on the founder's take word. This sync's `6fd5a3ade` (`DOMTokenList.replace` in blitz-vibey-script) and `7beb59176` (IME preedit) land on those same surfaces.

@history_item n=41 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — focus fixup built, native checked in the tree, `multiple` select and `optgroup`, Cells by keyboard, the `MountedData` hazard, the tree's focus window fallback, the stand graph re-read on a lock-only move, the a11y leg tally (14 files) — the rows of §5/§7 this merge's conflicts must keep, and the lock-move re-read precedent.

@history_item n=42 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — the engine overlay select list (decision/single-select-engine-overlay-list), its keys, clicks, focus staying on the select, the changed set marking a list's open/close, `expanded` from engine state, engine-constant colours, no motion, no `:open`; "the select model and its list" is on the scope's keep-both-sides list, and the list paints after the root past blend layers in blitz-paint, where upstream's preedit-underline commit lands.

@history_item n=43 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (six entries) — prior upstream sync: citations re-pointed into thirteen files; adapter re-read with manifests/lock unchanged; `select_popup_painted` also in the `embolden` leg (scope: the `embolden` witness); the panic fix in `deallocate_anonymous_block` for a key press past a dropped anonymous block (`key_press_after_container_lost_its_box`), directly beside upstream's `1fa2034d0`/`719268a9e` `display: contents` and anonymous-block changes; blending paints with no frame test.

@history_item n=44 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — listener reading via the vdom walk on dioxus-core 0.7.10's public API; the stand graph re-read on a lock-only move with Dioxus at `"0.7.3"`; the MountedData mechanism measured on dioxus-core 0.7.10 — the Dioxus-version-bound facts the hold fork (`cry-ikyyzjzj`) weighs.

@history_item n=45 marker=2026-11-05-residuals-repoint
**2026-11-05-residuals-repoint** — owner sentences re-pointed to residual rows, the adapter and its AT witness to `res-llkgqygv` (absence/assistive-technology-reach); the scope's residual re-reads name rows the same way.

@history_item n=46 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — the buffered Dioxus render ported onto Dioxus 0.7 (decision/buffered-dioxus-render), its same-render limit an open residual; "the buffered render" is on the scope's conflict list and a Dioxus 0.8 option must re-measure it.

@history_item n=47 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — `NodeState.clickable` from the vdom walk's current listeners and the read-only UI graph (decision/listeners-from-vdom-walk, decision/ui-graph-one-node-per-snapshot-node, contract/snapshot/clickable); readers bound to the Dioxus bridge's API that the hold fork weighs.

@history_item n=48 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the one crate-private actionable rule shared by `unkeyed_actionable` and `actionable_nodes`, the `data-dioxus-id` mark for dispatch alone, the `actions` verb read-only (decision/one-actionable-rule, decision/actions-verb-changes-nothing).

@history_item n=49 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the reached record ungated because the wasm build compiles dioxus-native-dom without `accessibility` (decision/agent-readers-behind-accessibility); a feature-gate conflict must keep that split.

@history_item n=50 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows verified against the code; every row this chunk's picked set holds dates from it, and an amendment from this sync names rows by handle.

@history_item n=51 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (three entries) — an upstream sync's a11y amendments in row form: `~ decision/changed-set-marks` (a closing scroll), `+ decision/empty-alt-image-decorative` and `+ decision/hidden-checkable-label-toggle`, each upstream behaviour ratified as upstream's; the form this sync's own taken behaviours follow.

@history_item n=52 marker=2026-11-18-a11y-re-read
**2026-11-18-a11y-re-read** — `~ decision/removed-focused-node-drops-focus` (focus falls to the root element, tree focus the window) and the focus-fixup SC 2.4.3 gap (later retired); focus behaviour a merge in focus code must keep.

@history_item n=53 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — the capability statement's pin reads the a11y leg's file count; a leg change from this sync moves that pin.

@history_item n=54 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (five entries) — the last upstream sync (merge base `547d7514b`): `~ decision/a11y-tree-refresh-on-change` and `~ decision/accessibility-tree-through-document-method` (refresh moved from the poll to the redraw after resolve), `~ decision/hidden-content-excluded`, `~ contract/accessibility-tree/hidden-and-inert-nodes`, `~ decision/hidden-content-excluded-from-a11y-tree` (visibility collapse excluded, W9); its left-out table and Dioxus hold are what this chunk re-reads, and upstream's `display: contents` commits touch the same hidden-content/layout path.

@history_item n=55 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — `~ decision/changed-set-marks` (highlight move), `~ decision/wcag-contrast-thresholds` (painted highlight pair), `~ decision/select-list-colours-engine-constants`, `~ decision/select-list-rows-one-height`, the snapshot's `highlighted` reading and the active descendant on an open list (contract/snapshot/open-list-highlighted-row); select-list rows the merge must keep.

@history_item n=56 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `~ contract/dioxus-document/reached-handlers` reads the dispatch's stop; bound to the Dioxus event handler path the hold fork weighs.

@history_item n=57 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — `~ decision/changed-set-marks` (post-layout focus loss), `~ decision/tab-and-shift-tab-move-focus` (skip unshown: `display: none`, not visible), `+ decision/focus-fixup-gives-up-unshown-focus` (a `display: contents` control keeps focus), `+ decision/focus-within-hidden-holder-limit`; upstream's `1fa2034d0`/`719268a9e` change `display: contents` handling, so the "shows" reading and these rows' guards (`focus_fixup_unshown`, `stand_keyboard`) must be re-measured after the merge.

@history_item n=58 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** (eight entries) — `~ decision/single-select-engine-overlay-list`, `~ contract/aria-property-reads/aria-expanded`, `~ decision/single-select-expanded-from-list`, `~ decision/hidden-content-excluded` (why), `~ contract/accessibility-tree/single-select`, `~ contract/accessibility-tree/select-expanded`, `~ contract/accessibility-tree/select-role`, `~ contract/snapshot/closed-select-option-bounds`, `~ contract/select-closed-list/enter-space`, `~ decision/multiple-select-chosen-row-pair`, `+ contract/single-list-box/keys`, `+ contract/single-list-box/row-click`; the newest select model the conflicts keep.

@history_item n=59 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** (three entries) — the founder ratified the last upstream sync's card operations (decision/hidden-content-excluded, contract/accessibility-tree/hidden-and-inert-nodes, decision/hidden-content-excluded-from-a11y-tree), no row changing; precedent that a sync's card operations confirmed in the founder's stead stand PROVISIONAL until his own word.
