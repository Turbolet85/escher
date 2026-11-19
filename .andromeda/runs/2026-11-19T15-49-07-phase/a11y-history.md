@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — (a named a11y leg on fork CI) wired `ci.yml`'s `a11y` job "Accessibility (a11y) tests" running `bash .github/scripts/ci-leg.sh a11y` in the slow tier with no required status check; this is the a11y leg the chunk must keep green after the merge (decision/a11y-ci-leg, decision/a11y-ci-gate).

@history_item n=2 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — (both entries) the first upstream merge re-pointed 16 `file:line` citations by the merge's measured line map and fixed root-`Cargo.toml` citations an earlier insert left one line low; precedent that a merge moves cited lines and the wrap re-points them by measured map with no claim text changed, and that root-manifest inserts (escher crates, seven_guis members) shift manifest citations.

@history_item n=3 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — set the Dioxus feature wiring (dioxus-native-dom's `accessibility` forwards to blitz-dom and gates its `accessibility_tree` override carrying stable ids as `author_id`; the workspace takes Dioxus crates with `default-features = false`, so a crate gets the feature only by naming it), and the shell building the tree through `Document::accessibility_tree`; the Dioxus 0.7.3 → `0.8.0-alpha.3` move the merge carries must keep this wiring (decision/dioxus-accessibility-feature-by-name, decision/accessibility-tree-through-document-method, contract/accessibility-tree/author-id).

@history_item n=4 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — made the snapshot consume the accessibility tree (role, name, focus) with no second role or name mapping; a merge resolution in dioxus-native-dom or the builder must keep the snapshot reading from the tree (decision/snapshot-reads-accessibility-tree, decision/snapshot-reads-the-tree).

@history_item n=5 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the actionable-key rule and `unkeyed_actionable`, with the trap: never satisfy the check by adding a `tabindex` or a role, which moves the pinned Tab order; stable ids are named among what the merge's resolutions keep (decision/actionable-key-rule).

@history_item n=6 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the bridge's falsy boolean clear (then 27 names, Dioxus 0.7.10's web-interpreter `isBoolAttr` list) so a `hidden: false` node stays in the tree; the Dioxus move must keep a falsy boolean attribute removed on both attribute paths (decision/falsy-boolean-attribute-removed, decision/falsy-boolean-attributes-removed).

@history_item n=7 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — made the engine's changed set truthful and the shell drain it, rebuilding the tree on a non-empty set, and defined what the set marks; "the changed set's contract" is the first item the scope says every resolution keeps (decision/a11y-tree-refresh-on-change, decision/changed-set-marks).

@history_item n=8 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the founder ratified the refresh on change and the changed-set contract, and the 27-name falsy clear; a merge that alters either reopens a founder ruling, not a planner choice.

@history_item n=9 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — §3's note restates the obs sink's target allowlist (safe fields only for `blitz*` and `accesskit_xplat`, `escher_*` redacted, any other target dropped whole); the scope re-checks the sink allowlist over every new `tracing`/`log` call site and target the merged delta brings (`blitz-vibey-cdp-server` among them).

@history_item n=10 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set and a fresh harness document holds a non-empty set; a resolution touching `settle.rs` or the changed set must keep this (decision/harness-settle-leaves-changed-set).

@history_item n=11 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` names `accessibility` on dioxus-native-dom unconditionally so both seven_guis binaries compile the tree override, neither builds a platform adapter, measured by dependency-graph reads with the lockfile byte-identical; the lock re-read after this merge must re-measure that reach (decision/driver-requires-accessibility, decision/stand-builds-no-platform-adapter).

@history_item n=12 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — restated the sink as two record classes and re-read the driver's added `tracing` dependency against the lock and the stand graph (no adapter joining); precedent for reading each new dependency line and the sink format after a lock change.

@history_item n=13 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — an upstream sync that changed engine behaviour (autofocus read `!= "false"`) and moved the accesskit family on the merged lock, after which the stand's graph was re-read for `accesskit_xplat`/`accesskit_winit`; direct precedent for this merge's lock re-read and for recording an upstream engine change in the plan.

@history_item n=14 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — re-read the stand graph on a new driver dependency (`serde_json`), `accesskit_(xplat|winit)` 0 rows; one of the series of adapter-absence re-reads each lock change owes.

@history_item n=15 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — (the builder's state reads) the builder writes `selected`, `toggled`, `invalid` from ARIA attributes trimmed and lower-cased, one parser per attribute, ratified by the founder; "the accessibility state reads" are named among what the merge's resolutions keep (decision/aria-states-on-every-element, decision/aria-state-lower-case-closed-values).

@history_item n=16 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a password paints as bullets in blitz-paint in every renderer (`layout_as_text_input`), the founder's fork 2; a merge resolution in blitz-paint or text-input layout must keep the mask (decision/password-value-masked, decision/password-never-revealed).

@history_item n=17 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — an earlier sync replaced the bridge's own list with upstream's 28-name `is_bool_attr` predicate, `mutation_writer.rs` byte-identical to upstream's, so a falsy `inert` is removed; precedent for taking upstream's code where it holds escher's intent, and the Dioxus 0.8 move must keep that predicate's behaviour (decision/falsy-boolean-attributes-removed).

@history_item n=18 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — the range input's keys, click focus and pointerdown value, founder-ratified engine changes for every document; "the range and keyboard models" are named among what the merge keeps (decision/range-input-keys, decision/range-click-focuses-not-value, decision/range-pointerdown-sets-value).

@history_item n=19 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — (both entries) Enter/Space activation and click-focuses-button, and `disabled` keyed by presence, all engine changes for every document ratified by the founder; resolutions in blitz-dom's event handling must keep them (decision/keyboard-activation-enter-space, decision/click-focuses-buttons, decision/disabled-by-presence).

@history_item n=20 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — SC 1.4.3 met on the stand (accent `#3a5ce5`), contrast tables, and the a11y leg widened by the founder with no new job or required check; a moved contrast or layout reading after the merge is re-measured, never loosened (decision/stand-contrast-asserted, decision/stand-accent-colour).

@history_item n=21 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the stand's a11y files also run three rounds in `stand-repeat` under the zero-failure budget; the post-merge proof runs those rounds (decision/stand-checks-repeated-three-rounds).

@history_item n=22 marker=2026-10-16-handler-splits-and-readme
**2026-10-16-handler-splits-and-readme** — moved the click and range bodies into private `click_button` and `press_range_input`; upstream edits to the same event-handling code meet these splits in the merge, and the cited functions are where §5's rows point.

@history_item n=23 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — a merge took upstream's six builder reads (`aria-labelledby`, `aria-describedby`, `aria-description`, `aria-expanded`, `aria-disabled`, `aria-level`) as written while keeping escher's three closed-set state reads and trimmed `aria-label`, one read per attribute, upstream's boolean `aria-selected` and unfiltered `aria-label` reads dropped; the closest precedent for resolving a builder conflict in this merge (decision/one-read-per-aria-attribute, decision/aria-labelledby-outranks-label, decision/aria-description-to-adapter).

@history_item n=24 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (both entries and the ratification) the toggle read per role and `role="switch"`, `disabled` narrowed to form controls, `inert` taking focus, hits and the tree, ratified by the founder at the Epoch 2 close; "`inert`" and the state reads are named among what the merge keeps (decision/toggle-read-by-role, decision/inert-subtree-excluded, decision/focusable-elements, decision/disabled-control-stops-click-walk).

@history_item n=25 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the double click read on the document's clock (the harness's virtual clock), ratified; "the clock invariant" is named among what the merge keeps (decision/double-click-on-document-clock).

@history_item n=26 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — an upstream sync took upstream #1226's `:focus-within` for every document, `:focus-visible` still never matching; precedent that an upstream selector-state change is recorded and gated by the founder's word (decision/focus-visible-never-matches).

@history_item n=27 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select model: a changed choice marks the changed set, the option excepted from `aria-selected` and from `display: none`, keys and typeahead on the document's clock; "the select model" is named among what the merge keeps (contract/accessibility-tree-states/select-option, decision/hidden-content-excluded).

@history_item n=28 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — an upstream take brought the dropped-node hit skip (an engine change ratified by the founder) and re-read the stand graph for the adapter; precedent for recording an upstream hit-test change and the per-take adapter re-read.

@history_item n=29 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — named a11y witnesses outside the `stand_` prefix join `stand-repeat` by its named `--test` list, held by `stand_requirement_sweep`; new upstream test files meet this list and the blitz-tests target rule.

@history_item n=30 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the a11y leg runs fourteen files (the founder's word), the pin one `assertIn` per target word; the leg's membership and count are what the chunk proves green after the merge (decision/a11y-test-leg).

@history_item n=31 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the windowed AT witness owned by the `re-carried` residual, and `docs/capabilities.md`'s accessibility reach edited by hand; a merge that moves any named limit or the reach changes that document by hand (absence/assistive-technology-reach).

@history_item n=32 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (the served pages and the scrollbar's band) a driver click right after a `scroll` in the shown overlay scrollbar's band is a thumb drag; an upstream scroll or scrollbar change could move it (decision/driver-click-in-shown-scrollbar-drags).

@history_item n=33 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** — the `stand-repeat` list's membership is held jointly by `stand_requirement_sweep` and `test_stand_repeat_leg_runs_three_rounds`, a name dropped from one list alone reading green; trap when `ci-leg.sh` lists are touched by the merge.

@history_item n=34 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the stand graph re-read on the take at `ebacd68bd`, citations re-pointed, `:open`, `CSSStyleDeclaration` accessors, IME at editor construction) the previous same-base sync (merge base `23b9b26a9`): manifests and lock unchanged and the adapter probe at 0 rows (`cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) `), citations re-pointed by the take's line map with blitz-vibey-script's split re-pointed by hand, and three upstream engine changes for every document taken on the founder's word on the take; the direct template for this merge's re-reads and engine-change rows (decision/open-pseudo-class-on-details-dialog, decision/script-style-through-inline-attribute, decision/ime-enabled-for-prefocused-input).

@history_item n=35 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the stand graph re-read on the lock's move) `pulldown-cmark` moved in the lock alone, the adapter probe re-read at 0 rows; a lock move of any size owes the re-read.

@history_item n=36 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (native checked state, `multiple` select, `optgroup`, focus fixup, tree focus fallback, `change` on checkables, non-invertible transform hit, the a11y leg's tally) engine changes for every document on the founder's F2–F11: the native toggle write, `set_multiselectable`, `optgroup` → `Group`, `clear_focus` at the flush, tree focus `u64::MAX` fallback, one engine `change`, no hit through a non-invertible transform, the leg at 14 files; "the hit stops", "`inert`" and the select model the merge keeps rest on these (contract/accessibility-tree-states/native-checkbox-or-radio, contract/accessibility-tree/multiple-select, contract/accessibility-tree/optgroup, contract/accessibility-tree/focus, decision/one-engine-change-per-checkable-click).

@history_item n=37 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the select list as a scope entity, the changed set where a list stands, `expanded`, opening, keys, click and focus, snapshot reading, colours, `:open`/clips/`opacity`, the legs) the engine overlay list on every document (`select_list`, answered first by `hit_with_scrollbar`, painted by `paint_scene` after the root), a single select's `expanded` from engine state, the list's changed-set marks, its colours as engine constants; upstream edits to `paint_scene`, hit testing, `handle_click` or the default sheet meet these in the merge (decision/single-select-engine-overlay-list, decision/single-select-expanded-from-list, decision/select-list-colours-engine-constants, decision/open-select-list-not-open-state).

@history_item n=38 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (citations re-pointed, the adapter re-read at `e42ee1f25`, `select_popup_painted` as `embolden`, the focus limit, the dropped anonymous block, `text_contrast` past a blend, blending with no frame test) the previous sync: thirteen files' citations re-pointed, the adapter probe at 0 rows, the `embolden` leg added, upstream blend paint taken with `text_contrast` reading `None` past a `mix-blend-mode` and its frame-test absence carried to «Upstream sync ahead of determinism and exploration»; the `embolden` witness and the contrast reader are proofs this chunk re-runs (decision/contrast-unread-past-non-colour-paint, decision/focus-fixup-reads-inert-alone).

@history_item n=39 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** — (the listener mark's over-report, the stand graph re-read on pulldown-cmark 0.13.6) the vdom walk's dynamic attributes read each element's current listeners through public dioxus-core 0.7.10 API, and the adapter re-read noted Dioxus at the `"0.7.3"` requirement; the Dioxus `0.8.0-alpha.3` move puts that API and the residuals `res-xpkmxnvl`/`res-xchmbfjr` to re-measurement (decision/listeners-from-vdom-walk).

@history_item n=40 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (the hazard closed by the buffered render, its same-render limit, the owner clauses) the founder's fix ported onto Dioxus 0.7: `initial_build` and `poll` render into a `MutationBuffer` unborrowed then replay through one `MutationWriter`, pinned by `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}`; "the buffered render" is named among what the merge keeps, and the Dioxus 0.8 move must re-port it, its pins discriminating (decision/buffered-dioxus-render).

@history_item n=41 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (`NodeState.clickable`, `DioxusDocument::graph`, graph state registration) the snapshot's `clickable`, the read-only UI graph and the `SignalRegistry` wrapped around `runtime.handle_event`, all built on Dioxus 0.7's runtime and vdom walk; the merge must keep them under Dioxus 0.8 (contract/snapshot/clickable, decision/ui-graph-one-node-per-snapshot-node, contract/ui-graph-state/use-graph-state).

@history_item n=42 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the one actionable rule, the actionable-node reader) `Readings`, the crate-private actionable rule over `element_facts`' current listeners, shared by `unkeyed_actionable` and `actionable_nodes`; a resolution in dioxus-native-dom keeps one rule (decision/one-actionable-rule, contract/actionable-nodes/listing).

@history_item n=43 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the ungated reached record) the reached record written on `DioxusEventHandler::handle_event`'s path before `runtime.handle_event`, ungated because the wasm build compiles dioxus-native-dom without `accessibility`; an upstream change to the event handler or the Dioxus runtime meets it (contract/dioxus-document/reached-handlers, contract/dioxus-document/record-reached, decision/agent-readers-behind-accessibility).

@history_item n=44 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows, each verified against the code; this chunk's amendments name changed rows by handle (`+`/`~`/`−`), as the later upstream sync did.

@history_item n=45 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (the three row entries) the last upstream sync re-stated `decision/changed-set-marks` (a scroll closing a list included) and added `decision/empty-alt-image-decorative` and `decision/hidden-checkable-label-toggle` as upstream behaviours the take brought, ratified by «Утвердить все 19 (Рекомендую)»; the immediate precedent for adding rows for upstream behaviours this merge brings and putting them to the founder.

@history_item n=46 marker=2026-11-18-a11y-re-read
**2026-11-18-a11y-re-read** — re-stated `decision/focus-fixup-reads-inert-alone` (a named SC 2.4.3 gap, the tree's focus naming the window) and `decision/removed-focused-node-drops-focus` (focus to the root element, measured); upstream focus or mutator changes in the merge re-read against these measured rules.

@history_item n=47 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — the capability statement's pin now reads the a11y leg's file count; a merge that changes the leg's files moves a pinned claim in `docs/capabilities.md`.

@history_item n=48 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `decision/stand-shared-tab-order-table` gained its exception for a Tab order reached only after a state change; the Tab-order walks re-run after the merge read against the shared table and that exception.
