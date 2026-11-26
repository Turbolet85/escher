@history_item n=1 marker=amd-lzvb3u3d
**amd-lzvb3u3d** — 2026-10-13-stand-keyboard-harness: created `stand_keyboard` (Tab walk with wrap, three focus readings agreeing, Enter/Space as click), moved `tab_order` and `focused` into `tests/blitz-tests/tests/common/mod.rs`, restated `stand_accessibility_ids`' Tab order and `stand_act_keys`' press-as-click. The register this chunk extends with Circle Drawer's Tab stop and the agreeing-focus reading; the precedent for restating focus pins when a new stop enters.

@history_item n=2 marker=amd-54i6rjfu
**amd-54i6rjfu** — 2026-10-14-stand-a11y-assertions: `stand_keyboard` widened (every control reached by Tab), the shared module gained `ENABLED_TAB_ORDER`, `stand_accessibility_ids` 8, the a11y leg went to six files. Circle Drawer's Tab order joins this shared-table family; a test added to `stand_keyboard` moves the a11y leg count.

@history_item n=3 marker=amd-f6jmv7bb
**amd-f6jmv7bb** — 2026-10-06-accessibility-tree-identity: created `stand_accessibility_ids` with the Tab order unchanged check (`tab_order_is_unchanged`). The scope names it among checks to re-read once the canvas becomes a Tab stop.

@history_item n=4 marker=amd-6ku7k52f
**amd-6ku7k52f** — 2026-10-31-engine-debts: Cells' grid made one Tab stop (`tabindex` 0, `aria_label`, an `onkeydown`, no role), `stand_keyboard` 15 → 21, `tests/common/mod.rs` gained `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`. The direct precedent for a keyboard-owned composite (canvas) taking one Tab stop with arrow keys moving a current item, and for its row in the shared tab tables.

@history_item n=5 marker=amd-dbogce46
**amd-dbogce46** — 2026-10-31-engine-debts: `cells-grid`'s `onkeydown` with no role joined the elements that react to input, moving `stand_non_lean_tasks`' pin 676 → 677. Trap: a key handler on `circle-canvas` with no role can move the same measured counts.

@history_item n=6 marker=amd-aj6rkfgc
**amd-aj6rkfgc** — 2026-11-07-ui-graph: `stand_non_lean_tasks`' `CLICKABLE` table pins Circle Drawer at 1 element reading `clickable` without a role. A literal pin the canvas's focusability / key handling / selection may move; part of the literal-pin sweep.

@history_item n=7 marker=amd-uh6brrox
**amd-uh6brrox** — 2026-10-31-engine-debts: Cells' keyboard walk through the driver (`press` walking and opening the editor, `type`, `press` committing), `stand_cells_served` 4 → 5, synchronous `MountedData` calls. Precedent for driving a keyboard path through the driver on a served screen.

@history_item n=8 marker=amd-wzxmumz2
**amd-wzxmumz2** — 2026-10-25-cold-agent-bar: re-pinned `ENABLED_TAB_ORDER`, `stand_keyboard` activation rows, `stand_contrast`, `stand_lint` and `stand_requirement_sweep` when the flight booker's control changed. Precedent for the cascade of shared-table and stand pins a changed focus stop forces.

@history_item n=9 marker=amd-efvgllgo
**amd-efvgllgo** — 2026-10-31-select-popup: `stand_keyboard` 21 → 23 with walks held while a list is open; key rows restated where Enter now opens. Precedent for keyboard rows whose meaning moves when a key gains a new action (Enter/Space on a canvas circle).

@history_item n=10 marker=amd-eb6jc7iy
**amd-eb6jc7iy** — 2026-10-31-select-popup: the a11y leg read 14 files and 114 tests, `stand_keyboard` 23, tier-0 witnesses 88 by arithmetic. `stand_keyboard` runs in the a11y leg, so tests this chunk adds there move the a11y tally (and the capability pin's counts).

@history_item n=11 marker=amd-hva4ican
**amd-hva4ican** — 2026-10-31-engine-debts: the a11y leg's 14 files read 85 → 112, `stand_keyboard` 15 → 21 among the per-target moves. Same register: a11y leg per-target readings move with `stand_keyboard` growth.

@history_item n=12 marker=amd-d4acxs6q
**amd-d4acxs6q** — 2026-10-24-a11y-re-read: the a11y leg named fourteen files, its membership the SC 2.1.1 / 2.4.3 witnesses, tree readings and recorded gaps (`accessibility_active_descendant_unread` among them); per-job tallies strip ANSI written as ESC and as literal `^[`. Rests on rows a11y-leg-membership and ci-log-ansi-strip; a new SC 2.1.1 canvas keyboard witness file would be a membership decision.

@history_item n=13 marker=amd-j45pb4zi
**amd-j45pb4zi** — 2026-11-18-a11y-re-read: decision/a11y-leg-membership's rule reduced to its criterion (files witnessing SC 2.1.1 or 2.4.3, a tree reading, or a recorded gap), the list left to the leg script. The rule a new keyboard-reach witness meets.

@history_item n=14 marker=amd-3kuwk3el
**amd-3kuwk3el** — 2026-10-18-engine-debts: added `accessibility_active_descendant_unread` 1 (the engine reads no `aria-activedescendant`, a stated limit) and `inert_subtree`. The scope's open P3/P4 question of naming a virtual node as focus rests on this pinned limit.

@history_item n=15 marker=amd-kkp6rzzv
**amd-kkp6rzzv** — 2026-10-31-engine-debts: `inert_subtree` 4 → 6; the tree writes `focus` only where the focused node was built, else the window. The constraint the scope cites (`accessibility.rs`): a virtual node as the tree's focus is an engine change for every document.

@history_item n=16 marker=amd-nx5bnxd7
**amd-nx5bnxd7** — 2026-11-13-cognitive-load-corrective: + contract/harness-inspect/focused — with nothing focused the harness's focused reading answers the root html element. A focus check that the canvas gives up focus asserts the root element, not none.

@history_item n=17 marker=amd-3nw7osql
**amd-3nw7osql** — 2026-11-21-engine-debts: + decision/focus-test-pins-where-focus-lands. Focus checks of the canvas/current circle pin the landing node, never only an absence.

@history_item n=18 marker=amd-ffczzr5t
**amd-ffczzr5t** — 2026-11-21-engine-debts: − decision/key-after-dropped-anonymous-block; the post-layout focus fixup gives focus up and a key acts only after a re-focus. Trap for key handling on a focused canvas whose box changes.

@history_item n=19 marker=amd-6ah7rxit
**amd-6ah7rxit** — 2026-10-17-upstream-sync-engine-debts: the harness re-exports keyboard-types 0.8's `Key`, `Modifiers` and `NamedKey` (row harness-reexports-key-types). Arrow-key checks on the canvas name keys through these.

@history_item n=20 marker=amd-myqerkgx
**amd-myqerkgx** — 2026-10-06-snapshot-state-fidelity: trap — a headless key press differs by platform (Backspace arm compiled out on macOS; green on Linux, red on macOS CI). Canvas key handling checks prove state with keys that act on every OS.

@history_item n=21 marker=amd-t5mve4e6
**amd-t5mve4e6** — 2026-10-07-act-by-id: `stand_act_keys` and the driver's `press`; rules set — a driver check's failure message names a layout mode, task and index, never an id or screen text; no deleting key outside `stand_act_keys`. Binds the new `press` on a focused canvas checks.

@history_item n=22 marker=amd-ebg5mftg
**amd-ebg5mftg** — 2026-10-07-refusal-detection: the refusal order `disabled` · `off-screen` · `covered` and `stand_act_disabled` / `stand_act_obstructed` / `stand_act_scroll`. The readings this chunk owes on Circle Drawer's circles rest on this order.

@history_item n=23 marker=amd-umiuichp
**amd-umiuichp** — 2026-11-25-virtual-nodes-as-elements: ~ contract/element-id/virtual-form (form served in tree, snapshot, diff); + decision/inert-guard-dropped (PROVISIONAL). Virtual node ids and the inert-guard rule this chunk's focus/selection checks extend.

@history_item n=24 marker=amd-gfskjnev
**amd-gfskjnev** — 2026-11-25-virtual-nodes-as-elements: ~ contract/driver-session/stale-or-not-found and ~ contract/driver-session/refusal-order — a `click` on a virtual node refused by `disabled` · `off-screen` · `covered`, other verbs not-found (delegate, PROVISIONAL). `press` on a focused canvas and circle refusals rest on these rows.

@history_item n=25 marker=amd-2bpwc464
**amd-2bpwc464** — 2026-11-25-virtual-nodes-as-elements: ~ contract/driver-refusal-causes/not-found — every id verb but `click` refuses a virtual id not-found (PROVISIONAL). A picked row; a `press` naming a circle stays not-found unless this chunk moves it.

@history_item n=26 marker=amd-uboady24
**amd-uboady24** — 2026-11-25-virtual-nodes-as-elements: ~ contract/refusal-cause/not-found and ~ contract/refusal-cause/covered (a virtual node with an element over its region's centre is `covered`; F12 A, PROVISIONAL). The `covered` reading owed on a circle; F4 B (sibling node covering) is the open fork beside it.

@history_item n=27 marker=amd-uotg6lvr
**amd-uotg6lvr** — 2026-11-25-virtual-nodes-in-every-read: ~ contract/refusal-cause/not-found — `click` and `layout` take a virtual id. Picked-row history.

@history_item n=28 marker=amd-wxa3wmls
**amd-wxa3wmls** — 2026-11-25-virtual-nodes-in-every-read: ~ contract/driver-refusal-causes/not-found — `click` and `layout` excepted. Picked-row history.

@history_item n=29 marker=amd-43k7nfyq
**amd-43k7nfyq** — 2026-11-25-virtual-nodes-in-every-read: ~ contract/driver-session/stale-or-not-found — `click` and `layout` share the resolver taking a virtual id. Picked-row history; a verb gaining virtual ids extends this.

@history_item n=30 marker=amd-chla37uv
**amd-chla37uv** — 2026-11-25-virtual-nodes-in-every-read: ~ contract/element-id/virtual-form — served in the UI graph, actions and layout answers too. Picked row this chunk's focus naming may extend.

@history_item n=31 marker=amd-km7kwf2r
**amd-km7kwf2r** — 2026-11-25-circle-drawer-virtual-nodes: ~ contract/element-id/virtual-form — the coverage answer joins the reads serving the form. Latest state of the picked virtual-form row.

@history_item n=32 marker=amd-qdxfnv67
**amd-qdxfnv67** — 2026-11-25-virtual-nodes-as-elements: ~ decision/reserved-graph-id-form narrowed to graph, actions and lint. Picked-row history.

@history_item n=33 marker=amd-vajazivu
**amd-vajazivu** — 2026-11-25-virtual-nodes-in-every-read: ~ decision/reserved-graph-id-form — lint answer only, plus the stand's graph/actions while no virtual node is declared. Picked-row history.

@history_item n=34 marker=amd-wvvgwdli
**amd-wvvgwdli** — 2026-11-25-virtual-nodes-in-every-read: ~ decision/reserved-graph-id-form guard adds `the_lint_holds_no_virtual_node`. The lint half guard this chunk must keep (lint reading virtual nodes is out of scope).

@history_item n=35 marker=amd-mr5bc6np
**amd-mr5bc6np** — 2026-11-25-circle-drawer-virtual-nodes: ~ decision/reserved-graph-id-form — the stand's graph/actions hold the form only for Circle Drawer's circles; − decision/handler-coverage-canvas-uncounted → contract/handler-coverage-limits/canvas-handler. The current state of two picked rows this chunk rests on.

@history_item n=36 marker=amd-mhbit5gf
**amd-mhbit5gf** — 2026-11-25-circle-drawer-virtual-nodes: ~ contract/handler-coverage-limits/canvas-handler — a virtual node's click listed under its id and exercised where a replayed click named it; a click inside a node counts the canvas listener alone. A canvas key handler and keyboard activation of a circle must be read against this row.

@history_item n=37 marker=amd-qlmbl4bk
**amd-qlmbl4bk** — 2026-11-25-circle-drawer-virtual-nodes: + decision/circles-apart-written-through-attribute — Circle Drawer checks needing circles apart write `data-circles` through the mutator, driver clicks landing at the canvas's centre. The fixture convention for selection, keyboard-move and covered/off-screen readings.

@history_item n=38 marker=amd-hf3zl3jw
**amd-hf3zl3jw** — 2026-11-08-handler-coverage: handler coverage's stated limits (canvas deferred to 0.4.0, unreached screens, bubbling) in the capability statement's claim map. Coverage claims the chunk's canvas handlers touch; capability-claims-name-their-checks.

@history_item n=39 marker=amd-i7b3rmrc
**amd-i7b3rmrc** — 2026-11-08-handler-coverage: `stand_handler_coverage` with `HANDLERS_AT_BOOT` / `UNEXERCISED_AFTER_DRIVES`, `GRAPH_DRIVES` read by both graph and coverage checks. A canvas key handler and B1's first-click selection move these stated tables (scope: GRAPH_DRIVES' double click re-read).

@history_item n=40 marker=amd-z23dex63
**amd-z23dex63** — 2026-11-07-ui-graph: `GRAPH_DRIVES` in `tests/blitz-tests/tests/common/mod.rs` drives every conditional write once so none reads `unconfirmed`. Row graph-drives-confirm-every-write; Circle Drawer's drives must still confirm every write under B1.

@history_item n=41 marker=amd-ckioxq2y
**amd-ckioxq2y** — 2026-11-07-ui-graph: the `unconfirmed` flag's stated limit (a conditional write a run skipped reads unconfirmed). Row unrun-declared-write-reads-unconfirmed; a selection write that a click inside a circle now makes instead of adding is such a conditional write.

@history_item n=42 marker=amd-jiyzcs5d
**amd-jiyzcs5d** — 2026-11-07-ui-graph: the stand registers task signals through `tasks::use_graph_signal` / the fork's `SignalRegistry`. Row graph-signal-registration; a new selection signal in Circle Drawer registers this way.

@history_item n=43 marker=amd-qq6cg6bh
**amd-qq6cg6bh** — 2026-11-13T07-43-19-wrap: ~ decision/handler-coverage-reads-every-replayed-screen ratified (standing). Picked row.

@history_item n=44 marker=amd-lfwo5vi7
**amd-lfwo5vi7** — 2026-11-20-driver-debts: ~ decision/handler-coverage-bubbling-counts-ancestors (stop read, panic unwinding counts as stopped); ~ graph argument `part`; ~ decision/graph-answer-holds-no-value. Picked rows a canvas key dispatch is counted under.

@history_item n=45 marker=amd-gpqjgetu
**amd-gpqjgetu** — 2026-11-20-driver-debts: ~ contract/handler-coverage-limits/bubbling-dispatch — stopped dispatch counts the innermost listener; a non-bubbling event its target alone. Picked row.

@history_item n=46 marker=amd-poiocitr
**amd-poiocitr** — 2026-11-07-ui-graph: `stand_graph` (6) — every stand task's declarations agree with its code once `GRAPH_DRIVES` ran; a graph answer holds no value. A scope-listed check this chunk re-reads.

@history_item n=47 marker=amd-a2uwkadt
**amd-a2uwkadt** — 2026-11-07-ui-graph: `graph::tests` and `an_element_with_an_app_click_listener_reads_clickable` pin `NodeState.clickable`; two `element_id` tests pin the reserved `{owner}///{key}` form. Contract snapshot-node/clickable and the virtual form's unit pins.

@history_item n=48 marker=amd-nasxjktn
**amd-nasxjktn** — 2026-11-23-circle-drawer-on-a-canvas: + contract/stand-boot/boot-circle-drawer — Circle Drawer booted in process alone, served by no escher-session role («A: не сейчас»). The scope's boundary; v040-11's binary reading waits on it.

@history_item n=49 marker=amd-uy7rvg43
**amd-uy7rvg43** — 2026-11-23-circle-drawer-on-a-canvas: ~ decision/canvas-shots-cross-os-by-digest — Circle Drawer's canvas element shot joined the cross-OS digest table. B1's filled repaint of a selected circle moves these committed digests.

@history_item n=50 marker=amd-6at54l43
**amd-6at54l43** — 2026-11-23-circle-drawer-on-a-canvas: + decision/canvas-text-on-whole-device-pixels — Circle Drawer's canvas text held byte-equal across hosts only by whole-pixel glyph origins. Any new canvas text (selection, focus) must keep whole-pixel origins.

@history_item n=51 marker=amd-zbmtsabl
**amd-zbmtsabl** — 2026-11-24T06-11-11-wrap: decision/canvas-text-on-whole-device-pixels ratified by the founder; engine remedy stays residual res-ieayznir. Current standing of the picked row.

@history_item n=52 marker=amd-5bp3s733
**amd-5bp3s733** — 2026-11-24-canvas-draw-commands: + decision/canvas-commands-table-across-hosts — Circle Drawer's draw commands held to one committed table across hosts. The selection repaint and any focus ring move these pins (scope: `stand_canvas_commands` swept).

@history_item n=53 marker=amd-5e244nch
**amd-5e244nch** — 2026-11-23-canvas-in-a-headless-shot: + decision/app-widget-shows-in-shots (`stand_canvas_shot`). Picked row.

@history_item n=54 marker=amd-ipy6bzhk
**amd-ipy6bzhk** — 2026-11-23-canvas-shot-across-hosts: ~ decision/no-shot-in-cross-os-byte-checks except (canvas alone by digest) and + decision/canvas-shots-cross-os-by-digest (byte equality ratified, F1 A). Picked rows constraining any cross-OS reading of the canvas.

@history_item n=55 marker=amd-5xumfbc5
**amd-5xumfbc5** — 2026-11-23-canvas-shot-across-hosts: ~ decision/cross-os-checks-hold-no-shot gains the canvas-alone exception. Picked row.

@history_item n=56 marker=amd-d26vi7ce
**amd-d26vi7ce** — 2026-11-23-canvas-shot-across-hosts: ~ decision/no-screenshot-in-cross-os-checks gains the canvas-alone exception. Picked row.

@history_item n=57 marker=amd-wsteqwoi
**amd-wsteqwoi** — 2026-11-23-canvas-shot-across-hosts: ~ decision/no-screenshot-step-in-committed-checks' why scoped to a shot holding document text. Picked row.

@history_item n=58 marker=amd-p6pzkktu
**amd-p6pzkktu** — 2026-11-25-virtual-nodes-declared: ~ decision/no-screenshot-step-in-committed-checks — a canvas-alone shot held equal across hosts by digest. Latest state of the picked row.

@history_item n=59 marker=amd-siqngma2
**amd-siqngma2** — 2026-10-19-recorded-check-replay: `stand_screenshot_repeat` byte-identity restated as one host only; a cross-OS byte-compared check holds no `screenshot` step. Origin of the no-shot rows.

@history_item n=60 marker=amd-ndft4ude
**amd-ndft4ude** — 2026-11-24-canvas-lint: ~ decision/driver-executor-integration-tested — the canvas label reader is unit-tested, the executor verbs integration-tested. Picked row: `press` on a focused canvas in `execute.rs` is covered by stand checks, not unit tests.

@history_item n=61 marker=amd-7xb4smfb
**amd-7xb4smfb** — 2026-11-24-canvas-draw-commands: fonts-unavailable covers `canvas`; MCP tools sixteen. The verb counts the capability statement pins.

@history_item n=62 marker=amd-iu5jthk4
**amd-iu5jthk4** — 2026-11-24-canvas-draw-commands: ~ contract/driver-session/reading-verbs adds `canvas`; socket carries canvas answers. Picked row.

@history_item n=63 marker=amd-u4oqpxyb
**amd-u4oqpxyb** — 2026-11-24-canvas-draw-commands: + decision/control-leaves-earlier-target-green — cargo stops at the first red target, so a control must leave earlier targets green. Picked row binding the chunk's controls.

@history_item n=64 marker=amd-23w5gd5l
**amd-23w5gd5l** — 2026-10-31-select-popup: `cargo` stops at the first failing target, so a red job's later targets go unread; macOS-only red under font-embolden. Trap for reading the windows warnings' witness and controls.

@history_item n=65 marker=amd-stldjcwi
**amd-stldjcwi** — 2026-11-21-single-select-sized-as-a-list-box: ~ decision/mutation-controlled-tests — a case left green is widened until the control reddens it. Picked row; the operator's «a control enters with its red reading».

@history_item n=66 marker=amd-z6xuttzw
**amd-z6xuttzw** — 2026-11-16-error-text-in-the-answer: + decision/expected-value-pinned-as-literal. Picked row behind «literal pins swept».

@history_item n=67 marker=amd-xnhp3j2e
**amd-xnhp3j2e** — 2026-11-20-select-list-and-selection-debts: decision/expected-value-pinned-as-literal gains the font-embolden band exception. Picked row; a painted-pixel pin of a filled circle meets it.

@history_item n=68 marker=amd-fcxxug5t
**amd-fcxxug5t** — 2026-11-16-layout-explained: + decision/no-change-read-control-needs-pending-work. Picked row for controls of reads that must change nothing (snapshot/tree focus readings).

@history_item n=69 marker=amd-5rdehljq
**amd-5rdehljq** — 2026-11-21-single-select-sized-as-a-list-box: + decision/box-presence-read-with-display. Picked row.

@history_item n=70 marker=amd-ke4vohyd
**amd-ke4vohyd** — 2026-11-20-stand-test-setup-shared: ~ decision/shared-stand-tables-in-one-module except. Picked row: Circle Drawer's Tab order goes in the shared module, a differing helper stays in its check.

@history_item n=71 marker=amd-wjjnfnof
**amd-wjjnfnof** — 2026-11-20-select-list-and-selection-debts: ~ contract/session-run/changed-node-states — an acting call returns `selected`, `invalid` or `highlighted` as after the step. Picked row the B1 click's `differs: [selected]` diff rests on.

@history_item n=72 marker=amd-2672diu7
**amd-2672diu7** — 2026-10-10-snapshot-states: the snapshot's `selected` state, `stand_act_states` returning the selected node and the one it leaves, every node's states equal to its accessibility node's. Precedent for the circle's `selected` read through the snapshot and tree.

@history_item n=73 marker=amd-uqva53q6
**amd-uqva53q6** — 2026-11-16-errors-with-the-action: ~ contract/driver-session/acting-verbs adds `caused`. Picked row; `press` on the canvas returns this shape.

@history_item n=74 marker=amd-jhx4wjqd
**amd-jhx4wjqd** — 2026-10-07-settle-detection: harness `settle` and `Session::act` contracts (no clock read, no time advanced, NotSettled classes). Picked settle rows behind every keyboard step's settled result.

@history_item n=75 marker=amd-ybvlv3ad
**amd-ybvlv3ad** — 2026-10-18-one-virtual-clock: `advance` on every session, the harness's virtual clock, `stand_virtual_clock`. Rows one-virtual-clock / harness-virtual-clock / two-clicks-without-advance-double-click; the diameter dialog's double click and its keyboard counterpart rest on this.

@history_item n=76 marker=amd-oodkokb2
**amd-oodkokb2** — 2026-10-18-engine-debts: the one-virtual-clock PROVISIONAL batch ratified, `stand_virtual_clock`'s double click among it. Standing of the two-clicks-no-advance rule.

@history_item n=77 marker=amd-6u4kftng
**amd-6u4kftng** — 2026-10-18-one-virtual-clock: `virtual_clock_engine`, a wrapped document keeping its own clock (row wrapped-document-keeps-own-clock). Picked row.

@history_item n=78 marker=amd-472linzq
**amd-472linzq** — 2026-10-25-capability-statement: `test_capability_statement.py` pins `docs/capabilities.md`'s names and counts; claims map to checks. The scope sweeps `docs/capabilities.md`' stated sizes and counts.

@history_item n=79 marker=amd-m7fw5fjq
**amd-m7fw5fjq** — 2026-11-18-capability-statement: ~ decision/capability-statement-pinned adds the a11y and stand-repeat leg counts. Picked row: a new a11y-leg file or stand-repeat word reddens ci-scripts until the page restates it.

@history_item n=80 marker=amd-zn44tekk
**amd-zn44tekk** — 2026-11-18-capability-statement: ~ decision/capabilities-doc-pinned-to-schema — leg counts held to the CI leg script. Picked row.

@history_item n=81 marker=amd-aiakktz6
**amd-aiakktz6** — 2026-11-18-capability-statement: ~ decision/capability-check-names-held-to-schema. Picked row.

@history_item n=82 marker=amd-755msr7u
**amd-755msr7u** — 2026-11-18-capability-statement: ~ decision/capability-statement-from-schema. Picked row.

@history_item n=83 marker=amd-n2omhba3
**amd-n2omhba3** — 2026-11-21-engine-debts: ~ decision/count-and-rename-sweeps reaches a CI leg's file count held as a literal. Rule for the literal-pin sweep if `stand_keyboard` or a new file moves a leg count.

@history_item n=84 marker=amd-mmdtifvl
**amd-mmdtifvl** — 2026-11-18-served-error-page: ~ decision/cross-os-readings-by-answer-digest (seven served tasks); count-and-rename-sweeps keeps a count-worded test name pinned as a witness. Picked row; Circle Drawer is not among the served tasks it reads.

@history_item n=85 marker=amd-h47hxvhq
**amd-h47hxvhq** — 2026-11-16-error-field-in-the-answer: + decision/doc-leg-gates-public-doc-edits. Picked row: editing `VirtualNodeDeclaration`'s public doc comment in `packages/blitz-dom/src/node/custom_widget.rs` lists the doc leg in the gate block.

@history_item n=86 marker=amd-n5xqwtgz
**amd-n5xqwtgz** — 2026-10-14-quality-gates: per-crate coverage floors (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) and the zero-failure stand-repeat budget. Picked row; new driver/dioxus-native-dom code (`press` on a canvas, the virtual node's tree focus) counts against them.

@history_item n=87 marker=amd-hayhbwsb
**amd-hayhbwsb** — 2026-10-24-upstream-sync-polish-and-ship: the `keyboard_compat.rs` exclusion stands until escher takes upstream's retirement. The except clause of the picked coverage-floor row.

@history_item n=88 marker=amd-jy34nxjf
**amd-jy34nxjf** — 2026-11-07-mounteddata-buffered-render: in a GitHub Actions job log stderr `Running` and stdout result lines do not interleave in order, so attribute by name. Applies to the CARRY's windows warnings witnessed by the CI windows job log.

@history_item n=89 marker=amd-eiwetouu
**amd-eiwetouu** — 2026-11-08-handler-coverage: the coverage line's form, `owner` left out when absent. Picked contract/coverage-line rows.

@history_item n=90 marker=amd-gh4iy36p
**amd-gh4iy36p** — 2026-11-08-handler-coverage: `stand_replay` replays kept `actions` steps; replay through the ordinary call path. Picked rows replay-through-the-call-path and check-replay/stop.

@history_item n=91 marker=amd-rrbdvino
**amd-rrbdvino** — 2026-10-12-headless-screenshot: harness `screenshot` / `screenshot_rect` contracts (own time, white composite, `EmptyRect`, no font refused). Picked harness-screenshot rows a focused/selected canvas shot reads through.

@history_item n=92 marker=amd-lfmouigx
**amd-lfmouigx** — 2026-11-14-bundled-fonts: + decision/cross-os-readings-by-answer-digest. Picked row origin.
