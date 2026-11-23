@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — first measured `disabled="false"` matching `:disabled` on enabled stand buttons and fixed the Dioxus falsy `disabled` write. This matters because the resize dialog's undo/redo and slider rely on the disabled reading staying one way, and the trap still holds: the focus path never had the defect, only state, styling and click targeting did.

@history_item n=2 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — named controls through `aria-label` and `<label>`, gave every Dioxus element its stable id as `author_id`, and wired the Dioxus `accessibility` feature so that the seven_guis stand binary builds no platform adapter (rows `contract/accessibility-tree/author-id`, `decision/dioxus-accessibility-feature-by-name`, `decision/stand-builds-no-platform-adapter`). This is the precedent for naming the canvas «Drawing area» and for keying the `<object>`.

@history_item n=3 marker=2026-10-06-snapshot-model
**2026-10-06-snapshot-model** — the snapshot takes role, name and focus from the tree and never from the element, with no second mapping (`decision/snapshot-reads-accessibility-tree`, `decision/snapshot-reads-the-tree`). The canvas's `role="group"` and its name reach the snapshot only through the tree builder.

@history_item n=4 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added the actionable-key rule and `unkeyed_actionable` (`decision/actionable-key-rule`). Circle Drawer read 3 unkeyed then. Trap for this chunk: never satisfy the check by adding a `tabindex` or a role, because that moves the pinned Tab order. Key the canvas element instead.

@history_item n=5 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — the falsy clear of the bridge's boolean attributes, so `hidden: false` stays in the tree (`decision/falsy-boolean-attribute-removed`, `decision/falsy-boolean-attributes-removed`). It also measured the snapshot's `enabled`, `checked` and `value` from the element (`contract/snapshot/enabled`, `contract/snapshot/value`). The trap it named, that a click on a plain button clears focus, was later reversed.

@history_item n=6 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** — `Snapshot::to_text` reads only the snapshot model (`decision/snapshot-model-single-source`, `decision/readers-of-one-snapshot-model`). Whatever the canvas states reaches an agent's text only through this one model.

@history_item n=7 marker=2026-10-07-change-tracking-and-diff
**2026-10-07-change-tracking-and-diff** — the changed set's contract and the tree refresh on change. The diff reads the snapshot model, and a text change is reported on the element whose name it changes (`contract/snapshot-diff/entries`). The canvas's `data-circles` attribute write is a mutation that marks the set.

@history_item n=8 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the stand's expected roles and names are stated once, in `tests/blitz-tests/tests/common/mod.rs` (`decision/role-name-tables-stated-once`, `decision/stand-expectation-tables`). This is where Circle Drawer's moved rows belong, never in a second mapping.

@history_item n=9 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the founder ratified the refresh on change, the changed set and the falsy clear. It also recorded that seven_guis' manifest names no accessibility feature of its own.

@history_item n=10 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — a harness settle neither reads nor drains the changed set, and a freshly booted document holds a non-empty set (`decision/harness-settle-leaves-changed-set`). A check that reads a canvas attribute write as "this step changed something" must drain first.

@history_item n=11 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `escher-driver` enables `accessibility` unconditionally, so both seven_guis binaries compile the tree override and neither builds an adapter (`decision/driver-requires-accessibility`, `decision/stand-builds-no-platform-adapter`). The widget crates this chunk adds to `examples/seven_guis/Cargo.toml` must keep `accesskit_xplat` and `accesskit_winit` out of the graph.

@history_item n=12 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the driver's `disabled` refusal is read from the snapshot's `enabled` and never from the attribute's value (`decision/enabled-by-disabled-presence`, `contract/actions-answer/causes`). This governs driver clicks on the dialog's controls.

@history_item n=13 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — an MCP tool result is one more reader of the snapshot model (`decision/snapshot-model-single-source`). The canvas adds no reader of its own.

@history_item n=14 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `DiffNode::differs`, which names the readings that differ in node-key order (`contract/snapshot-diff/differs`). A driver click on the canvas returns a diff read this way.

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the driver's `screenshot` reads the model's bounds: an element shot is the bounds rounded outward and cropped to what can be seen (`contract/driver-screenshot/element-shot`). A shot changes nothing (`decision/screenshot-changes-nothing`). The canvas element shot that joins the cross-OS digest table rests on this.

@history_item n=16 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the falsy predicate became upstream's 28 names, and a falsy `inert` is cleared (`decision/falsy-boolean-attributes-removed`).

@history_item n=17 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — built `Harness::text_contrast` and `stand_contrast` (`decision/stand-contrast-measured`, `decision/text-contrast-required-ratio`, `decision/wcag-contrast-thresholds`). The reader reads computed styles, not painted canvas pixels, so canvas text stays outside it.

@history_item n=18 marker=2026-10-13-range-input-interaction
**2026-10-13-range-input-interaction** — range input keys, a click that focuses without changing the value, a pointerdown that sets the value, and the snapshot's range `value` (the `contract/range-input/*` rows, `decision/range-input-keys`, `decision/range-click-focuses-not-value`, `decision/range-pointerdown-sets-value`). The resize dialog's diameter slider is a range input bound by these.

@history_item n=19 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (Enter/Space and the button click) — a pointer click focuses a button, and any other click still clears focus (`decision/click-focuses-buttons`). The canvas `<object>` instead takes focus on a click (`events/mod.rs:200-202`). The click synthesis this chunk adds to `handle_pointerup` must keep these focus rules.

@history_item n=20 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** (`disabled` by presence) — disabled-ness is read one way, by the attribute's presence (`decision/disabled-by-attribute-presence`, `decision/disabled-by-presence`). It also added `stand_keyboard` and the `common::tab_order` sequences (`decision/stand-shared-tab-order-table`).

@history_item n=21 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — the stand accent `#3a5ce5` / `#2a4cd3` / `#1f3fc1`, SC 1.4.3 asserted on the stand, and the a11y leg widened (`decision/stand-accent-colour`, `decision/stand-contrast-asserted`). Any colour this chunk adds to Circle Drawer, whether a stand CSS row or canvas text, follows the accent's held values.

@history_item n=22 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — `unkeyed_actionable` reads 0 on Circle Drawer with its dialog open, and Circle Drawer counted 1 element that reacts to a click but states no role, which was the `div` canvas. This chunk's `role="group"` and click path move that reading.

@history_item n=23 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — added upstream's six builder reads. `aria-labelledby` outranks `aria-label`, and only an `aria-disabled` of `true` sets the node disabled, which the snapshot's `enabled` never reads (`contract/aria-property-reads/aria-disabled`, `decision/one-read-per-aria-attribute`). This is how the canvas's group name is read.

@history_item n=24 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (§5, §6) — `inert` and `disabled` narrowed to form controls in focus order, in the click walk and in the hit (`decision/focusable-elements`, `decision/disabled-by-presence`). The click synthesis on a custom-widget target goes through the same click walk and its disabled gate.

@history_item n=25 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — two clicks on one point within 500 ms and 2 px are a double click on the document's clock, so two driver clicks with no `advance` are a double click (`decision/double-click-threshold`, `decision/double-click-on-document-clock`). F4's centre-only placement and the second centre click opening the resize dialog rest on this.

@history_item n=26 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the founder's ratification at the Epoch 2 close) — ratified the 13 PROVISIONAL marks, including the double click on the document's clock and the click walk's disabled gate.

@history_item n=27 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — `Session::lint`'s `contrast` reading goes through `text_contrast` with the stand's exempt rule and no second formula, and the stand reads no finding at boot (`decision/lint-contrast-single-formula`, `decision/stand-lint-clean-at-boot`, `contract/visual-lint-finding/content`). Circle Drawer's boot must read 0 findings in both layout modes.

@history_item n=28 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — the `a11y` leg runs fourteen files. It decides whether the new canvas checks join the leg, and the capability statement's pin reads the leg's file count.

@history_item n=29 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` publishes `## Accessibility reach`, edited by hand (`decision/capability-statement-reach-by-hand`). The adapter's AT witness is owned by the re-carried residual. This chunk edits `docs/capabilities.md` for the canvas sample.

@history_item n=30 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the served pages) — Cells and the load page are served by `escher-session` but are not a `LeanTask`, and the stand reads no lint finding at boot. This is the precedent for Circle Drawer's in-process stand boot, which stays unserved under F1 A.

@history_item n=31 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's and Cells' rows) — the first non-lean pages to get Tab-order, keyboard and contrast rows (`cells_meets_its_ratios`, `the_load_page_meets_its_ratios`, `decision/stand-contrast-measured`). This is the precedent for giving Circle Drawer its own rows in the shared module.

@history_item n=32 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells by keyboard) — the count of elements that react to input became 677 when `cells-grid`'s `onkeydown`, with no role, joined the 676 data cells. Precedent: a listener on a role-less container moves the count, which the canvas's listeners and its `role="group"` will move too.

@history_item n=33 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the tree's focus falls back to the window) — tree focus names the focused node only where it was built, otherwise `u64::MAX` (`contract/accessibility-tree/focus`). This governs the canvas `<object>` taking focus on a click.

@history_item n=34 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the lint's closed-select overlap ended) — the stand at boot reads no lint finding of any kind (`decision/stand-lint-clean-at-boot`).

@history_item n=35 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (how an open list's labels leave) — an engine-painted overlay's labels leave only as painted pixels in a `screenshot`, with no log site. This is the precedent for canvas text and circles crossing only as screenshot pixels (`decision/canvas-crosses-as-pixels`).

@history_item n=36 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no stand binary builds a platform adapter, on this take) — the stand graph is re-read with no `accesskit_xplat` or `accesskit_winit` at each manifest or lock move (`decision/stand-builds-no-platform-adapter`). The widget crates added to seven_guis call for the same re-read.

@history_item n=37 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (text_contrast reads None past a blend) — the contrast reader and the lint read nothing past opacity, a filter, a background image or a blend (`decision/contrast-unread-past-non-colour-paint`).

@history_item n=38 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the «reacts to a click» reading) — link 2 measured the role-less click readers.

@history_item n=39 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the listener mark's over-report) — current listeners are read from the vdom walk and never from the listener mark (`decision/listeners-from-vdom-walk`). This applies to the canvas's `onclick` and `ondoubleclick`.

@history_item n=40 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the stand's graph re-read on pulldown-cmark 0.13.6) — the latest adapter-absence re-read. The adapter and its AT witness are owned by residual `res-llkgqygv` (`absence/assistive-technology-reach`).

@history_item n=41 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (the residual owner clauses) — the adapter owner `res-llkgqygv` is re-carried at 0.3.0, target next (`decision/stand-builds-no-platform-adapter`).

@history_item n=42 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (NodeState.clickable) — built the `clickable` field from the vdom walk's current click listeners (`contract/snapshot/clickable`, `contract/snapshot-node/clickable`, `contract/snapshot-text/clickable`). `stand_non_lean_tasks` asserts the `CLICKABLE` table with Circle Drawer at 1, and the canvas `<object>` reads `clickable`. That table row is re-pinned in this chunk.

@history_item n=43 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph reader) — `DioxusDocument::graph` holds one node per snapshot node and carries no values (`decision/ui-graph-one-node-per-snapshot-node`, `decision/ui-graph-carries-no-values`, `decision/agent-readers-behind-accessibility`). The circle state must not leak into it as values.

@history_item n=44 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (graph state registration) — `use_graph_state` / `use_graph_signal` with `DeclaredWrite`, and a page with no registered state reaches no registry (`contract/ui-graph-state/no-registered-state`). Circle Drawer's circle state and its handler declarations follow this.

@history_item n=45 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the one actionable rule) — `unkeyed_actionable` and `actionable_nodes` share one rule: focusable, an interactive role, or current listeners other than `mounted` (`decision/one-actionable-rule`). The canvas is actionable through its listeners and must be keyed.

@history_item n=46 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable-node reader) — `actionable_nodes` lists nodes in snapshot pre-order with `id`, `owner` and `keyed` (`contract/actionable-nodes/listing`).

@history_item n=47 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb as a read-only reader) — answers each id-taking verb with its refusal cause (`contract/actions-answer/causes`). A driver click on the canvas is answered through `check_target`.

@history_item n=48 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the keyed reading, provisional on the carriers) — `keyed` is true where the id holds no `/` (`decision/keyed-wire-form`).

@history_item n=49 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actionable counts at boot) — `ACTIONABLE_AT_BOOT` in the shared module pins the actionable count per page. Circle Drawer's row, if added, follows this table.

@history_item n=50 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the ungated reached record) — `record_reached` / `reached_handlers` on `handle_event`'s path (`contract/dioxus-document/reached-handlers`, `contract/dioxus-document/record-reached`). The click and double click synthesized after the widget's event must reach the canvas's handlers through this path.

@history_item n=51 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (stand_handler_coverage's keyboard-activation check) — `stand_handler_coverage` opens Circle Drawer from Home by its card (`stand_handler_coverage.rs:58-65`). The canvas's handlers enter that check's coverage.

@history_item n=52 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (handler_coverage) — lists the graph's handlers that no check reached, as ids and event names only (`contract/handler-coverage/answer`).

@history_item n=53 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** — ratified `decision/keyed-wire-form`, so it now stands.

@history_item n=54 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (2. A11y Strategy) — `~ decision/changed-set-marks` adds a scroll that closes a list. This is part of the history of the changed-set row the canvas mutation relies on.

@history_item n=55 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — `~ decision/capability-statement-reach-by-hand`: the statement's pin reads only the a11y leg's file count, and no check reads the rest of the section. The canvas reach text added here is checked by no test.

@history_item n=56 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `~ decision/stand-shared-tab-order-table`: a Tab order reached only after a state change is a local table of the keyboard check. This decides where Circle Drawer's dialog-open order would go.

@history_item n=57 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (2. A11y Strategy, refresh) — `~ decision/a11y-tree-refresh-on-change`: the tree is pushed again in a redraw after style resolve when the changed set is non-empty, and a poll pushes no tree.

@history_item n=58 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (2. A11y Strategy, document method) — `~ decision/accessibility-tree-through-document-method`: the rebuild goes through the document trait's method, so DioxusDocument's stable ids reach the tree.

@history_item n=59 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (4. ARIA Patterns & Roles; 6. Visual Design Verification ×2) — `~ decision/hidden-content-excluded`, `~ contract/accessibility-tree/hidden-and-inert-nodes` and `~ decision/hidden-content-excluded-from-a11y-tree` add `visibility: collapse` to the exclusions. These bind how a hidden resize dialog leaves the tree.

@history_item n=60 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (2, 3, 6) — `~ decision/changed-set-marks` adds the highlight's move. `~ decision/wcag-contrast-thresholds`, `~ decision/lint-contrast-single-formula` and `~ decision/lint-contrast-shares-one-reader` add the highlight reader to the one formula. These are the contrast rows Circle Drawer's boot lint reads through.

@history_item n=61 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** (7. Screen Reader Support) — `~ decision/snapshot-reads-accessibility-tree` and `~ decision/snapshot-reads-the-tree` add `highlighted` to the readings taken from the tree.

@history_item n=62 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `~ contract/dioxus-document/reached-handlers`: a stopped or unwound dispatch counts the innermost listener alone, and a non-bubbling event its target alone. The synthesized click and double click on the canvas are read under this promise.

@history_item n=63 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (2. A11y Strategy) — `~ decision/changed-set-marks`: a focus loss after layout also marks the set. This is the row's current wording.

@history_item n=64 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (5. Keyboard Navigation) — `~ decision/tab-and-shift-tab-move-focus`: the Tab walk skips a node that does not show. The post-layout focus fixup for unshown focus was added. These bind how the canvas `<object>` and the resize dialog's controls sit in the Tab walk.

@history_item n=65 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** (4. ARIA Patterns & Roles) — `~ decision/hidden-content-excluded`'s why narrowed to a drop-down's options.

@history_item n=66 marker=2026-11-21-dependency-and-ci-debts
**2026-11-21-dependency-and-ci-debts** — ratified the hidden-content rows (`decision/hidden-content-excluded`, `contract/accessibility-tree/hidden-and-inert-nodes`, `decision/hidden-content-excluded-from-a11y-tree`). They now stand.
