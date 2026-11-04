@carried n=1 label=test-plan/test-scope-summary fact=fct-z2ynupl3
why: the probe runs on this stand in both layout modes and link (1)'s node-to-component reading must go through the stable element id the stand holds across a re-render, never a NodeId a remount refreshes.
quote: the seven_guis headless stand — each lean task mounted in TaskShell offline at the pinned viewport with the bundled font, fresh-boot and incremental `dom_string` identity, one driven interaction per task, timer ticks, one stable element id per element in both layout modes, the same id across a re-render, a remount (fresh `NodeId`s bar the four document-skeleton elements) and a fresh process
@carried n=1 run=2 label=test-plan fact=fct-65u33zgx
quote: The seven_guis headless stand boots each lean task (Counter, FlightBooker, Timer, Crud) through `seven_guis::stand::boot(LeanTask, HarnessOptions)` or `boot_timer(HarnessOptions) -> (Harness<DioxusDocument>, TimerTicks)`, each a fresh `VirtualDom` over `from_vdom` rendering the task in TaskShell, with `stand::options(incremental)` the pinned options
@carried n=2 label=test-plan/test-scope-summary fact=fct-z2ynupl3
why: the probe's lean-task run of links (3) and (4) must reproduce this standing reading, a handler on `counter-increment` writing state that `counter-value` reads, as the ground truth its handler-to-state and state-to-node verdicts are measured against.
quote: the diff of the snapshots taken before and after a step (`Snapshot::diff`) in both layout modes — a click on `counter-increment` naming `counter-value`, Create naming the new row added and Delete naming it removed
@carried n=3 label=test-plan/test-scope-summary fact=fct-z2ynupl3
why: link (1) is read through the bridge's `element_ids()`, and the probe must resolve a node by the id this check pins as the snapshot's and the accessibility node's, so its verdict names that API.
quote: `stand_act_ids` 2 (for each of the 77 nodes of the four lean tasks' snapshots, the 14 controls among them, the id the snapshot lists is the `author_id` of the one accessibility node carrying it, the element `element_ids()` resolves it to is that node's
@carried n=4 label=test-plan/test-scope-summary fact=fct-z2ynupl3
why: `DioxusDocument::unkeyed_actionable` is a bridge-held read over the elements that carry handlers, an exposed API link (2) must weigh and name when its verdict is exposed, measured in both layout modes.
quote: every actionable element of the four lean tasks and of Home reading an author key (`DioxusDocument::unkeyed_actionable` empty) in both layout modes and across state
@carried n=5 label=test-plan/test-scope-summary fact=fct-z2ynupl3
why: the "reacts to a click" reading the probe answers as link (2) for `click` is the deferred snapshot field this check measured and pinned, so the probe's answer must be read against these counts and must leave the field deferred, adding no snapshot field.
quote: the elements that react to input and state no role counted against a stated 7-row table (0 on the four lean tasks and Home, Circle Drawer 1, Cells 677, the 676 data cells and `cells-grid`, whose `onkeydown` and no role join them
  — measured and pinned, the snapshot field for it deferred, `NODE_FIELDS` still thirteen
@carried n=6 label=test-plan/test-scope-summary fact=fct-z2ynupl3
why: the MountedData hazard re-measured at HEAD through the harness's pump must be stated beside this standing reading, a `mounted` handler's write landing, so the reading separates the mounted handler that runs from a MountedData call in a key-spawned task that panics.
quote: on in-file fixtures a write made in a `mounted` handler is present after the step
@carried n=6 run=2 label=test-plan fact=fct-e36kow75
quote: pump polls the document with no waker context and resolves at the harness time; dispatch and dispatch_recorded do not pump
@carried n=7 label=test-plan/test-scope-summary fact=fct-z2ynupl3
why: a probe check the plan keeps in tests/blitz-tests or in dioxus-native-dom is a new instance this record must list, with the agent-run reading it moves re-counted.
quote: coverage spans accessibility, layout, text, paint, hit testing, input events, scrolling, style invalidation, DOM mutation leaks, network-driven stylesheet loading
@carried n=7 run=2 label=test-plan fact=fct-aitehs62
quote: coverage spans the engine's behaviours, the seven_guis headless stand, the driver session over the stand and process telemetry
@carried n=7 run=3 label=test-plan fact=fct-jydhmppz
quote: **dioxus-native-dom:** unit tests of the snapshot builder, its text form, its parts and its diff, the stable element id, the actionable-key check, the bridge, the mutation writer and the document's events
@carried n=7 run=4 label=test-plan fact=fct-glytb3w3
quote: `run stand`'s `run.end` reads passed 237 · failed 0 · ignored 7 over 51 files and `run all`'s reads passed 733 · failed 0 · ignored 12
@carried n=8 label=test-plan/test-scope-summary fact=fct-bzkoiqdf
why: the served task the probe runs beside the lean one is Cells or the load page, whose key-driven handlers writing state another node reads are what the served-task reading must reproduce in both layout modes, on the key path the hazard needs.
quote: **tests/blitz-tests — `stand_keyboard` on Cells and the load page:** each test in both layout modes: Cells' Tab sequence, its arrows clamped at the grid's ends and bringing a cell into view, Enter opening the cell's editor and committing by keyboard, Tab leaving an open editor committing it, a click on another cell committing an open editor to its own cell, and the load page's sequence with its button taking Enter and Space
@carried n=8 run=2 label=test-plan fact=fct-65u33zgx
quote: beside the lean tasks it boots Cells through `stand::boot_cells(HarnessOptions)` (`Task::Cells` in TaskShell, no `LeanTask`) and the load page through `stand::boot_load(HarnessOptions)` (TaskShell titled "Load", its options' `net_provider` replaced by the stand's `LoadNet`, every other field kept)
@binding n=1
**test-scope-summary (the stand's stable element id, `element_ids()`) ↔ the harness's stand boot (body §3)**: the probe boots a lean task through `stand::boot` with the pinned options and reads link (1) through the stable element id that `element_ids()` resolves, in both layout modes (test-plan/test-scope-summary fct-z2ynupl3) (test-plan fct-65u33zgx).
@binding n=2
**test-scope-summary (the elements that react to input, the deferred field) ↔ the scope's boundary**: link (2)'s `click` answer is read against the pinned react-to-input counts and leaves the snapshot field deferred, `NODE_FIELDS` unchanged, no snapshot field added (test-plan/test-scope-summary fct-z2ynupl3).
@binding n=3
**test-scope-summary (the mounted-handler write) ↔ the harness's pump semantics (body §3)**: the hazard's re-measurement goes through the harness's `pump`, the `poll(None)` path, and the reading states it beside the standing mounted-handler write (test-plan/test-scope-summary fct-z2ynupl3) (test-plan fct-e36kow75).
@binding n=4
**test-scope-summary (the coverage record) ↔ the agent-run proof (body §3) ↔ quality gates (body §10)**: a kept probe check is recorded in this summary's lines, re-counts the newest `run stand` and `run all` reading, and, if it is kept in dioxus-native-dom, is recorded in that crate's line and keeps the crate's per-crate coverage floor (test-plan/test-scope-summary fct-z2ynupl3) (test-plan fct-glytb3w3) (test-plan fct-jydhmppz) (test-plan fct-4t3fuckc).
@binding n=5
**test-scope-summary (Cells and the load page under keys) ↔ the harness's served boots (body §3)**: the served task beside the lean one boots through `boot_cells` or `boot_load` and is driven by keys in both layout modes (test-plan/test-scope-summary fct-bzkoiqdf) (test-plan fct-65u33zgx).
