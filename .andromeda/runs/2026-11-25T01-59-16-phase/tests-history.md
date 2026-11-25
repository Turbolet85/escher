@history_item n=1 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's unit-test groups for the graph, amd-a2uwkadt) — added the two `element_id` tests that pin the reserved `{owner}///{key}` form over an adversarial page, with `virtual_node_id` returning `None` for an empty or `/`-bearing key and the walk calling it nowhere. This chunk is the first producer of that form, so these pins are the unit precedent it extends. The "walk calls it nowhere" clause changes once declared nodes take ids.

@history_item n=2 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (dioxus-native-dom's unit scope and its one test-only shim, amd-xep3xm3z) — the unit scope now names the stable element id and its reserved virtual form. The element-id tests went 13 → 15, the reserved form's two among them, and the crate went to 100 unit tests in eleven files. The one `cfg(test)` shim allowed is `DioxusDocument::dispatch_to_vdom`. This sets the count sites a new declaration module and its unit tests must move, and the bar for any further test-only shim.

@history_item n=3 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb held in session and over MCP in process, amd-poiocitr) — `stand_graph` asserts that no served id holds `{owner}///{key}` (the guard of `decision/reserved-graph-id-form`) and that a graph read changes nothing and holds no typed text. Virtual ids will be served ids of that form, so this guard and the row both bear on how declared nodes may surface. Per scope, crossing them is left to «Virtual nodes as elements».

@history_item n=4 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (a graph answer crosses the session socket, amd-5zqg4udh) — `contract/session-wire/graph-answer` landed: ids, owner ids, component names, state names and similar, never a value or typed text, no `NodeId`. It was recorded as a boundary widening on the founder's word, with a delegate confirm held PROVISIONAL. This is the closest precedent for the "names' crossing" fork: authored names on a ratified carrier need the founder's own word.

@history_item n=5 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (a graph call logs its verb alone and no graph needle, amd-tvsd3b6b) — `host_log` and `mcp_log` each drive the new verb at `trace` against needles read from the stand (`graph_needles()`), and the `command` span records `verb` alone. If this chunk crossed a declared name or key onto any carrier, this is the needle pattern the session-logs rows (`decision/session-logs-no-screen-content`, `contract/driver-log/per-call-line`) would demand.

@history_item n=6 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — first counted the `element_id` unit tests and how they build a `DioxusDocument` (a re-render through a `GlobalSignal` in `vdom.in_runtime`, staling a node by dropping it). These are the helpers that tests of an id derived from a canvas's stable id would reuse.

@history_item n=7 marker=2026-10-07-audit-corrections
**2026-10-07-audit-corrections** — the bridge tests moved to the child module `dioxus_document::bridge_tests` because `DioxusEventHandler`'s fields are private. This is the origin of the `except` on `decision/unit-tests-in-source-file` and of the shared-module rule. Trap recorded: a sed or grep address ending at a function name also selects every item whose name opens with it.

@history_item n=8 marker=2026-10-16-file-splits
**2026-10-16-file-splits** — out-of-line `#[path]` `cfg(test)` modules went from one to five, moved whole with test paths unchanged. This is the precedent behind the `_tests` sibling form that `decision/inline-unit-test-modules` and `decision/inline-cfg-test-modules` now allow.

@history_item n=9 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — a `*_tests.rs` `#[path]` file is no record in the per-crate coverage reading, so moving a test module out of line moves its crate's reading (escher-driver 98.33 → 98.07 %). A chunk adding tests out of line should forecast a coverage move, not none. Relevant to `decision/per-crate-coverage-floors` (dioxus-native-dom 83 %).

@history_item n=10 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — `decision/inline-cfg-test-modules`'s `except` dropped its stated count of out-of-line modules, so a new `#[path]` test module needs no count edit there.

@history_item n=11 marker=2026-11-20-codebase-health
**2026-11-20-codebase-health** (2. Test Strategy, two entries) — reworded `decision/inline-cfg-test-modules` and `decision/inline-unit-test-modules` to the `_tests` sibling form with a fixtures child module. These are the two picked rows a new module's tests must follow.

@history_item n=12 marker=2026-11-20-codebase-health
**2026-11-20-codebase-health** (10. Quality Gates, oversize targets and complexity) — added `decision/oversize-test-targets-kept` (escher's targets over 800 code lines kept whole, no split) and ratified `decision/complexity-ceiling-host-only`'s why. A new or grown check file and new functions are read against these rows.

@history_item n=13 marker=2026-10-26-codebase-health
**2026-10-26-codebase-health** (the dev-host complexity ceiling) — recorded the cognitive-complexity ceiling of 15: read on the dev host only, test functions counted, and a closure or nested `fn` counted into its parent, so only module-level sibling items lower a reading. This binds how a validation function for declarations would be factored if it nears the ceiling.

@history_item n=14 marker=2026-11-13-cognitive-load-corrective
**2026-11-13-cognitive-load-corrective** — added an `except` on `decision/complexity-ceiling-host-only`: the engine's click handler is held below the ceiling, with a hand re-apply when a sync edits its arms. The precedent for engine code this chunk touches (blitz-dom `custom_widget.rs`).

@history_item n=15 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — added `decision/no-change-read-control-needs-pending-work`: a control of a read that must change nothing needs pending work, because a pump on a settled screen is invisible. The in-process read of declared nodes is such a read, and its control must follow this rule.

@history_item n=16 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — widened `decision/mutation-controlled-tests`: a control that removes a bound loosens it to twice the budget. This is the rule each new check's red-before-green control enters the plan under (scope condition: a control enters with its red reading).

@history_item n=17 marker=2026-11-21-single-select-sized-as-a-list-box
**2026-11-21-single-select-sized-as-a-list-box** — `decision/mutation-controlled-tests` now reads "a case it leaves green widened until it does", after a control read DOES NOT HOLD because other arms routed around the guard. This applies directly to controls of the refusal rules (empty key, `/` in key, empty name, region outside, duplicate key).

@history_item n=18 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (4. Unit Test Strategy) — added `decision/control-leaves-earlier-target-green`: Cargo stops at the first red target, so a control whose case lies in the later of two targets needs an edit that leaves the earlier one green. This binds how this chunk's controls are staged in its gate entries. The same chunk widened `decision/typed-password-never-returned` to canvas answers.

@history_item n=19 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — added `decision/harness-graph-tests-enable-accessibility` (a per-package build compiles the UI graph out unless the dev-dependency turns `accessibility` on). It also added `contract/harness-construction/from-component-alone`, `vdom-alone` and the props-maker and root-context-before-build decisions. A canvas fixture built in isolation, or any graph or tree-gated test, rests on these.

@history_item n=20 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `contract/harness-construction/from-vdom` now provides `SeededRandom::from_seed(DEFAULT_SEED)` as a root context. This is the construction promise of the picked row that any fixture built through `from_vdom` inherits.

@history_item n=21 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (5. Integration Test Strategy) — `decision/layout-tests-in-both-layout-modes` gained the `except` for upstream targets taken in a sync. Escher's own new checks of region and frame still run `for incremental in [false, true]`.

@history_item n=22 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the flow-classification file an exception) and **2026-10-31-engine-debts** (`pseudo_element_flow_classification` runs both layout modes) — the history of the both-modes rule's exception, ended by editing the file to loop both modes. This confirms the rule binds every layout assertion this chunk adds.

@history_item n=23 marker=2026-11-23-canvas-in-a-headless-shot
**2026-11-23-canvas-in-a-headless-shot** — added `decision/app-widget-shows-in-shots`: a Rust-painted widget witnessed in harness and driver shots, per package and in the workspace build. This is the canvas-widget test surface the declared nodes sit inside.

@history_item n=24 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** (`except` entries: `decision/cross-os-checks-hold-no-shot`, `decision/no-shot-in-cross-os-byte-checks`, `decision/no-screenshot-in-cross-os-checks`; plus `+ decision/canvas-shots-cross-os-by-digest`) — gave each no-shot row the canvas-alone `except`, held equal on three hosts on the founder's F1 A. Picked rows this chunk must keep whole if any region or paint check is compared across hosts.

@history_item n=25 marker=2026-11-23-canvas-shot-across-hosts
**2026-11-23-canvas-shot-across-hosts** (`why` entries: `decision/no-screenshot-step-in-committed-checks`, `decision/cross-os-checks-hold-no-shot`, `decision/no-shot-in-cross-os-byte-checks`, `decision/no-screenshot-in-cross-os-checks`, `decision/checks-hold-no-screenshot-step`, `decision/no-screenshot-in-recorded-check-scripts`) — scoped each row's reason to "a shot holding document text". `decision/no-screenshot-step-in-committed-checks` kept its rule, and its "asserted on one host only" clause is exactly the scope's CARRY (cry-ax6q6t4w) this chunk's wrap must carry. That clause is not edited by hand.

@history_item n=26 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (test counts, the third flow script and the shot's same-host limit) — first measured a shot's PNG bytes differing between linux x86_64 and macOS aarch64 (CI#39225375577). That gave the no-screenshot-in-cross-OS rows and the one-host clause now carried. It explains the CARRY's origin.

@history_item n=27 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added `contract/stand-boot/boot-circle-drawer` and `decision/canvas-text-on-whole-device-pixels`, and widened `decision/canvas-shots-cross-os-by-digest` to Circle Drawer's canvas element shot. Circle Drawer's circles as virtual nodes belong to a later entry, but its boot and canvas tables are the fixtures nearest this chunk.

@history_item n=28 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** — ratified `decision/canvas-text-on-whole-device-pixels` (standing). The engine-side remedy stays residual `res-ieayznir`. Constrains any canvas text a declared-node fixture paints if compared across hosts.

@history_item n=29 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** (Test Harness Contract and Strategy) — `canvas` became a sixteenth verb and reading verb on every carrier. `decision/socket-carries-commands` gained "canvas answers", with the `escher-session` command-line `canvas` row PROVISIONAL. `decision/canvas-commands-table-across-hosts` was added. This is the carrier set the names' crossing fork would widen. The scope adds no verb, carrier or socket reading, so these rows must stay unmoved unless the founder rules otherwise.

@history_item n=30 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — `decision/driver-executor-integration-tested` now allows a reader of recorded paint commands with no held instance to be unit-tested. `contract/lint-findings/order` gained canvas-label `truncated` in paint order (PROVISIONAL, delegate). This is the precedent for how a canvas-side reader that holds no instance, like the declaration validator, may be tested at the unit tier.

@history_item n=31 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (handler coverage's stated limits) — stated "a handler inside a canvas is not counted, deferred to 0.4.0". That limit is owned by the v040-11 entries, which this chunk opens but does not discharge. Exploration and coverage belong to «Virtual nodes in every read».

@history_item n=32 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (test strategy and harness contract) — the session wire's crossing of a call's id and typed text in and its content out was ratified by the founder (2026-10-10). This is the date of the founder's rule the scope cites for the names' crossing fork: authored text on a ratified carrier is a widening.

@history_item n=33 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** (the session-lifecycle key file: the founder's word respelled) — the source of `decision/socket-carries-commands`: «Сокет несёт команды», with playbook rule A for copying the founder's words. Any change to what the socket carries cites it.

@history_item n=34 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — the visual lint's crossing onto every ratified carrier was reconciled across ten carried-set sites (the session-lifecycle socket list among them) on the founder's word. A widening of what crosses must sweep every carried-set site, not one row.

@history_item n=35 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** and **2026-11-17-layout-on-the-carriers** (`~ decision/socket-carries-commands`) — each new answer kind (caused with error text, then layout answers) was added to the socket row on an operator-relayed founder ruling. This is the pattern if virtual-node names or keys ever cross.

@history_item n=36 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `host_log` and `telemetry_drop` cover a sink-installing host at `trace` against needles read from the stand. This is the origin of `decision/telemetry-target-allowlist`, `decision/host-stderr-no-screen-content` and `decision/no-ids-or-names-in-stand-logs`. Trap: a needle list read from the stand can outgrow a hand-kept one.

@history_item n=37 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — recorded that `stand_session_quiet` proves only a host with no sink. That is the origin of `decision/log-silence-read-on-a-sink-host`, which keeps a zero read from a sinkless host from passing as a log-safety witness.

@history_item n=38 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the same check file read green per package and red in the workspace leg (blitz-dom's `tracing` on in the workspace only). This is the origin of `decision/per-package-green-not-workspace-green` and the closed span line of `contract/driver-command-span/one-line-per-call`. Rule: a check reading a sink's capture counts lines by target.

@history_item n=39 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the bounds reader's two bodies (writing-mode on in the workspace build only) each gained a check, with each mutation control red under its own build only. This is the origin of `decision/bounds-reader-bodies-each-checked`, and it binds any region or content-box frame check that reads client rects.

@history_item n=40 marker=2026-10-10-upstream-sync-agent-surfaces
**2026-10-10-upstream-sync-agent-surfaces** (`BlitzTestsTargetsTest`) — kept one target per file with upstream's `all` table at `test = false`. This is the origin of `decision/one-test-target-per-behaviour`, `decision/blitz-tests-one-target-per-file` and `decision/blitz-tests-single-test-target`. A new behaviour check is its own auto-discovered target.

@history_item n=41 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — set the per-crate floors (escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90) and the zero-failure flakiness budget. These are the floors of `decision/per-crate-coverage-floors` and `contract/coverage-floor-exit-codes/*` that new code in dioxus-native-dom or blitz-dom is read against.

@history_item n=42 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** (dioxus-native-dom's coverage reading and mutation_buffer.rs) — a new crate-private module with no unit test of its own was counted inside dioxus-native-dom's 83 % floor with no exclusion. A precedent for how a new declaration module's lines join the floor.

@history_item n=43 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the coverage leg excludes `keyboard_compat.rs`) — the named-filename exclusion behind `decision/keyboard-compat-excluded-from-coverage` and `decision/keyboard-compat-coverage-exclusion`. The only standing coverage exclusion. A new module gets none.

@history_item n=44 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (9. CI Integration) — added `decision/doc-leg-gates-public-doc-edits` after a public doc comment linked a crate-private item and went red only in CI's Documentation job. A public `Widget` or declaration API with doc comments must list the doc leg in the gate block.

@history_item n=45 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** — `decision/local-pre-push-gate` gained the target sweep after a green fast leg, and `decision/target-sweep-safe-around-builds` was added. A probe worktree outside `target/` that no step deletes (scope condition) must stay clear of the sweep.

@history_item n=46 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` step of `fast`) — `fast` ends with the `embolden` witness, pinned by `test_fast_leg_ends_on_the_font_embolden_witness`. It is part of the local pre-push gate order a gate block runs. The same chunk's failed-load entry records a trap: a take that extends a trait sweeps that trait's implementers. Extending the `Widget` trait sweeps its implementers.

@history_item n=47 marker=2026-10-24-a11y-re-read
**2026-10-24-a11y-re-read** — set the a11y leg's membership criterion (an SC 2.1.1/2.4.3 witness, a tree reading, or a recorded gap) and `decision/joining-file-keeps-its-legs`. The accessibility tree for virtual nodes belongs to a later entry, so a check this chunk adds joins no a11y leg unless it meets the criterion.

@history_item n=48 marker=2026-11-18-a11y-re-read
**2026-11-18-a11y-re-read** — reworded `decision/a11y-leg-membership` to the criterion alone, the list now being the leg script's inventory. That is the row as picked.

@history_item n=49 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (2. Test Strategy) — added `decision/count-and-rename-sweeps`, later widened by **2026-11-18-served-error-page** and **2026-11-21-engine-debts** to CI leg file counts and kept witness names. If this chunk moves any count (node keys, unit-test counts, leg files), the sweep rule binds it.

@history_item n=50 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — restated `overview/session-lifecycle`: the session is no verb of the agent-run contract, and its `run session` selection runs `cli_flow`. This is the overview the picked session-lifecycle key holds.

@history_item n=51 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** — set CI triggers on PRs and on pushes to main, v0.* and build/**, with the workspace tests as the fast `test` leg (`decision/ci-trigger-branches`, `decision/workspace-tests-in-fast-ci-leg`). The basis of the local baseline (`decision/local-baseline`) was re-measured under line-tables-only debuginfo in its second entry.
