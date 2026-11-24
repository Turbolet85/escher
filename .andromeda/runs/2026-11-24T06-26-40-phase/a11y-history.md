@history_item n=1 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — a named a11y leg on fork CI: ci.yml's `a11y` job runs `bash .github/scripts/ci-leg.sh a11y` in the slow tier, no required status check (decision/a11y-ci-leg, decision/a11y-ci-gate). This chunk's acceptance names "a11y leg green" on the merged tree; the leg's shape is the thing to keep, not widen.

@history_item n=2 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — the first upstream merge: 16 `file:line` citations into merged upstream files re-pointed by the merge's measured line map, no claim moved, and the `ci-leg.sh a11y` files kept unnarrowed; a second entry re-pointed root `Cargo.toml` citations a manifest insert had shifted. Precedent: this merge touches `Cargo.toml`, `Cargo.lock`, `blitz-dom` layout and `dioxus-native-dom/src/events.rs`, so cited lines will move and the wrap re-points them.

@history_item n=3 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — Dioxus feature wiring: the workspace takes the Dioxus crates with `default-features = false`, so a crate gets `accessibility` only by naming it; the shell builds and refreshes the tree through `Document::accessibility_tree`; `author_id` on every Dioxus element node. Rests decision/dioxus-accessibility-feature-by-name, decision/accessibility-tree-through-document-method and contract/accessibility-tree/author-id, which the Dioxus hold (D1 A) and the merged manifests must leave intact.

@history_item n=4 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — escher-driver names `accessibility` on dioxus-native-dom unconditionally; both seven_guis binaries compile the tree override but neither builds a platform adapter (`accesskit_xplat`/`accesskit_winit` in neither graph), measured by dependency-graph reads before and after. Origin of decision/driver-requires-accessibility and the adapter-absence probe this merge's lock move (taffy 0.16.0 registry, `url` into blitz-shell) must re-run.

@history_item n=5 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — §3's NOT YET MEASURED note: the obs sink prints only safe fields for `blitz*` and `accesskit_xplat` targets, redacts four fields for `escher_*`, and drops any other target whole; with 2026-10-07-driver-command-spans and 2026-10-09-founder-rulings it reads two record classes (event lines and closed-span lines). This chunk re-checks the sink allowlist against every new `tracing`/`log` call site the merged delta brings (the drop source in blitz-shell `dnd.rs`, the file reader in dioxus-native-dom); a moved target changes what this note states.

@history_item n=6 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** — an upstream take changed an engine reading (`autofocus` `== "true"` → `!= "false"`) and the plan recorded it, the a11y leg unmoved; on the merged lock where the accesskit family moved, the stand graph was re-read holding neither adapter. Precedent for recording an upstream engine behaviour change and for re-reading the adapter absence on a merged lock.

@history_item n=7 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the take retired the bridge's own `BOOLEAN_ATTRIBUTES` for upstream's 28-name `is_bool_attr`, `mutation_writer.rs` byte-identical to upstream's, a falsy `inert` now removed (follows 2026-10-06-snapshot-state-fidelity's falsy-`hidden` fix). Trap: this merge's one conflict sits in `dioxus-native-dom/src/events.rs`; a resolution must keep the bridge's boolean clearing and its `inert`/`hidden` consequences for the tree.

@history_item n=8 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — upstream's six builder reads (`aria-labelledby`, `aria-describedby`, `aria-description`, `aria-expanded`, `aria-disabled`, `aria-level`) taken as upstream wrote them while the three closed-set state reads and the trimmed `aria-label` read were kept, one read per attribute. Origin of contract/aria-property-reads/*, decision/one-read-per-aria-attribute, decision/aria-labelledby-outranks-label, decision/aria-description-to-adapter: the precedent for "conflicts keep both sides' intent" on the accessibility state reads.

@history_item n=9 marker=2026-10-19-upstream-sync-visual-lint-and-real-app-readiness
**2026-10-19-upstream-sync-visual-lint-and-real-app-readiness** — an upstream engine change for every document (`:focus-within` matching) landed PROVISIONAL on a delegate's answer, focus order measured unmoved and the a11y leg read green. Precedent: an upstream behaviour change crossing an a11y row is named and ratified, not taken silently; this chunk's drop events reach every windowed document.

@history_item n=10 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — the take's hit skip of a dropped node recorded under visibility and hit-testing, and the stand graph re-read at `f9cbb2756` holding neither adapter. Precedent for re-reading decision/stand-builds-no-platform-adapter at each take, and for an upstream hit change landing on a §6 row (this merge's `f65baeca5` one-axis clip moves paint beside the hit stop).

@history_item n=11 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg widened to fourteen files on the founder's word, eight also in `stand-repeat`; the leg's membership pinned per target word. The leg's file count is what this merge must leave unmoved; upstream's new `flex_br.rs` and `drag_and_drop.rs` join no leg list.

@history_item n=12 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes the accessibility reach (tier 0, SC 2.1.1 · 1.4.3 · 2.4.3, the a11y leg's files, the AT reading unmeasured) and the windowed AT witness is owned by residual `res-llkgqygv`; 2026-11-18-capability-statement made its pin read the a11y leg's file count. A take that moves the leg or a named limit edits that document by hand.

@history_item n=13 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the stand graph re-read; citations; `:open`; IME) — on the take at `ebacd68bd` the manifests, lock and `deny.toml` were read unchanged and the adapter probe (`cargo tree --locked -p seven_guis -e normal --prefix none`, `^accesskit_(xplat|winit) ` 0 rows) run; citations into code the take moved re-pointed; upstream engine changes (`:open`, IME at editor construction → decision/ime-enabled-for-prefocused-input) recorded on the founder's word on the take. The probe command and the "engine change for every document" recording are the pattern this sync follows — this time with a lock that does move.

@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the stand's graph re-read on the lock's move; the a11y leg's tally) — a lock-only move (`pulldown-cmark` 0.13.5) re-ran the adapter probe at 0 rows; the leg's tally recorded 14 files · 112 tests on CI. Precedent: this chunk's lock move (taffy git → registry, `url`) gets the same re-read, and any leg tally change is recorded with its CI run.

@history_item n=15 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (focus fixup; `inert`/native toggle/`multiple`/`optgroup`; `change` on a checkbox; non-invertible transform hit) — built the focus fixup with no `blur`, the native checkbox/radio `toggled` write, `multiple` select keys and rows, `optgroup` as Group, one engine `change` per checkable click, and a non-invertible transform taking no hit. These are the select model, `inert`, accessibility state reads and hit stops the scope lists under "conflicts keep both sides' intent"; the merge's `events/driver.rs` and `construct.rs` hunks must not undo them.

@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the select list as a scope entity; the changed set; clips and `opacity`; leg counts) — the overlay list on every document, answered first by hit testing and painted after the root (decision/single-select-engine-overlay-list), painting whole past a clipping box and `opacity`, a list's open/close marking the changed set; `select_popup`, `select_popup_painted`, `stand_act_popup` outside the a11y leg. `f65baeca5` changes one-axis `overflow: clip` in `blitz-paint/src/render.rs`, the same paint path the list's clip-escape rests on.

@history_item n=17 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (citations; adapter re-read; `embolden`; the focus limit; blend) — the last sync of this kind before 0.4.0: citations re-pointed, adapter probe 0 rows, `select_popup_painted` added to the `embolden` leg, a panic past a dropped anonymous block fixed in `deallocate_anonymous_block` with `key_press_after_container_lost_its_box`, and upstream's blend painting recorded with `text_contrast` reading `None` past a blend. This merge's `f559f3fe2` edits anonymous flex items in `layout/construct.rs` (that panic's ground) and `f65baeca5` edits paint, and the `embolden` witness is in this chunk's proofs.

@history_item n=18 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the stand's graph on `pulldown-cmark` 0.13.6; the listener mark; the probe target outside the a11y leg) — adapter re-read with Dioxus at the `"0.7.3"` requirement, the adapter owned by residual `res-llkgqygv`; the bridge's listener attributes over-report while the vdom walk reads exact listeners. Rests the Dioxus hold CARRY `cry-twrvkwp6`, and new drag/drop listeners from `82a6573ee` reach the listener readers.

@history_item n=19 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — `DioxusDocument::poll` and `initial_build` render into a mutation buffer then replay it under one borrow (decision/buffered-dioxus-render), pinned by `mounted_data_in_a_task_a_key_handler_spawned_acts_{non_incremental,incremental}`; owner clauses for `res-llkgqygv` re-said at 0.3.0. The merge's dioxus-native-dom side (events, the drop file reader) must keep this render path.

@history_item n=20 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (`NodeState.clickable`; the graph reader; registry dispatch) — `DioxusEventHandler::handle_event` sets the `SignalRegistry` dispatch around `runtime.handle_event`; `clickable` read from the vdom walk's current listeners. The conflicting file `packages/dioxus-native-dom/src/events.rs` holds this dispatch path, so the import-block conflict and the drag event's `convert_modifiers` adaptation sit beside it.

@history_item n=21 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — one actionable rule (`Readings`) read by `unkeyed_actionable` and `actionable_nodes`, its listener leg the vdom walk's listeners holding any event other than `mounted` (decision/one-actionable-rule, decision/listeners-from-vdom-walk); `ACTIONABLE_AT_BOOT` pinned. A drag/drop listener counts as actionable under this rule; the stand's counts must read unmoved after the merge (no stand task adds drop handlers).

@history_item n=22 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the ungated reached record written on `DioxusEventHandler::handle_event`'s path before `runtime.handle_event` (contract/dioxus-document/reached-handlers, record-reached), refined by 2026-11-20-driver-debts to read the stop. Same file as the merge's conflict; a non-cancelable `preventDefault` change (`22c18e38f`) and new drag events pass through this dispatch.

@history_item n=23 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — Enter/Space activation as a synthetic click with app handlers first then `handle_click`, a pointer click focusing buttons, `disabled` by presence (contract/keyboard-activation/*, decision/click-focuses-buttons, decision/disabled-by-presence). `22c18e38f` edits `blitz-dom/src/events/driver.rs` and `82a6573ee` adds drag events there: the "pointer frame and click synthesis" the scope's conflict rule protects.

@history_item n=24 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6: `inert` and `disabled`; the PROVISIONAL batch ratified) — `inert` takes focusability, hits and the tree; `disabled` narrowed to form controls; the click walk stopped by a disabled form control; toggle per role; all ratified at Epoch 2 close. Rows decision/inert-subtree-excluded, decision/disabled-control-stops-click-walk, decision/toggle-read-by-role stand on it; the merge keeps `inert` and the state reads named in the conflict rule.

@history_item n=25 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the double click read on the document's clock, a harness document using the virtual clock (decision/double-click-on-document-clock). The scope's conflict rule keeps "the clock invariant"; new drag events in the driver must not introduce a second clock read.

@history_item n=26 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — range input keys, click focusing with no value change, pointerdown setting value (contract/range-input/*). The "range and keyboard models" the scope's conflict rule keeps through the `events/driver.rs` hunks.

@history_item n=27 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — the builder writes `selected`, `toggled`, `invalid` from ARIA on every element, trimmed and lower-cased, and the snapshot takes them from the tree (decision/aria-state-lower-case-closed-values, decision/aria-states-on-every-element, decision/snapshot-reads-accessibility-tree). The "accessibility state reads" of the scope's conflict rule; with 2026-10-31-engine-debts' native toggle write they must survive the merge byte for byte.

@history_item n=28 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the engine's changed set made truthful and the tree refreshed on change, the diff a reader of the snapshot model; 2026-10-07-settle-detection added that a settle neither reads nor drains the set (decision/harness-settle-leaves-changed-set). The "changed set's contract" in the conflict rule; new drag events and `preventDefault` handling must mark it exactly as stated.

@history_item n=29 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — the previous sync of this series added rows for upstream behaviours (+ decision/empty-alt-image-decorative, + decision/hidden-checkable-label-toggle, both ratified as upstream's) and qualified decision/changed-set-marks. Precedent: an upstream behaviour crossing an a11y area that no row states gets a row at the wrap — the drop input reaching every windowed document may be one.

@history_item n=30 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** — the sync before last: ~ decision/a11y-tree-refresh-on-change and ~ decision/accessibility-tree-through-document-method (refresh moved from poll to redraw after resolve), ~ decision/hidden-content-excluded, ~ contract/accessibility-tree/hidden-and-inert-nodes, ~ decision/hidden-content-excluded-from-a11y-tree (`visibility: collapse` excluded, W9); landed PROVISIONAL on the overseer's stead and ratified by the founder in 2026-11-21-dependency-and-ci-debts. Precedent for an upstream take moving tree rows, and for the provisional-then-ratified path.

@history_item n=31 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — ~ decision/changed-set-marks (highlight moves mark), ~ decision/wcag-contrast-thresholds and the lint's highlight reader, ~ contract/snapshot/open-list-highlighted-row (active descendant, `highlighted`), ~ decision/select-list-rows-one-height. Part of the select model and its list the merge must keep; the stand contrast proofs must still pass.

@history_item n=32 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — ~ decision/changed-set-marks (post-layout focus loss marks), ~ decision/tab-and-shift-tab-move-focus (skip unshown nodes), − decision/focus-fixup-reads-inert-alone → + decision/focus-fixup-gives-up-unshown-focus, + decision/focus-within-hidden-holder-limit. The post-layout fixup reads whether a node shows, so `f559f3fe2`'s change to the `<br>`'s anonymous flex item in `construct.rs` and the Taffy 0.16 layout move can move it; 2026-11-18-a11y-re-read's ~ decision/removed-focused-node-drops-focus belongs with it.

@history_item n=33 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — the overlay list opens on a drop-down alone; a single list box reads ListBox, no expanded, its own keys and row click (~ decision/single-select-engine-overlay-list, ~ contract/aria-property-reads/aria-expanded, ~ decision/single-select-expanded-from-list, ~ contract/accessibility-tree/select-role, + contract/single-list-box/keys, + contract/single-list-box/row-click). The newest state of "the select model and its list" the conflict rule keeps.

@history_item n=34 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — + decision/circle-drawer-canvas-pixels-only (one named canvas node, role group, `Drawing area`; circles leave only as shot pixels), ratified at 2026-11-24T06-11-11-wrap. The "custom-widget paint" in the conflict rule and WATCH `cry-womrv2l4` (`stand_canvas_cross_os`) read this canvas; `f65baeca5`'s paint change and Taffy 0.16 must not move its shot or its tree node.

@history_item n=35 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — SC 1.4.3 met on the stand with the darker accent, `stand_contrast`'s tables, `stand_keyboard`/`stand_accessibility_ids` in the a11y leg; the adapter and its windowed AT witness re-homed to residual `res-llkgqygv`. "Every escher proof still passes": these stand checks run in both layout modes after the merge, and a moved reading is re-measured, never loosened.

@history_item n=36 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the driver's `screenshot` is one more reader of the model's bounds and changes nothing; a password paints as bullets in every renderer (decision/password-never-revealed, decision/screenshot-changes-nothing, contract/driver-screenshot/element-shot). The merge's paint change (`f65baeca5`) can move screenshot readings, which the scope says are re-measured and recorded, never loosened.

@history_item n=37 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (decision · contract · absence), each verified against the code. Every row in this chunk's picked set is in this form; an amendment from this merge names its row by handle (`+`/`~`/`−`).
