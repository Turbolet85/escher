@history_item n=1 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — added the IA note behind `decision/scroll-into-view-moves-every-scroller` and `decision/visible-region-through-clipping-boxes`: an into-view scroll moves every holding scroller then the viewport, and `visible_region` reads through clipping boxes. The founder ratified this engine change for every document. It is the precedent for this chunk's boundary that a fix to items 1–3 is an engine change for every document and needs its own ratification. A focus fixup that runs after layout must keep these two rules and the CRUD-list readings as they are.

@history_item n=2 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's rows, the exception to the scrolling boxes rule) — added the `except:` clauses of `decision/scroll-into-view-moves-every-scroller` and `decision/visible-region-through-clipping-boxes`. No ruling names this exception; it records what the code does. This is the last engine change to these rows. Any focus or scroll change made by item 1 must keep both exceptions, or a new amendment must state the change.

@history_item n=3 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list, the engine's select list in every document) — the latest focus behaviour on the stand, in every document: focus stays on the select while its list is open, and the Tab sequence holds with the list open and after each close (founder forks F1–F4, F7). Item 1's focus fixup ("focus given up after the next layout") changes focus in every document. It must not break this, and it follows the same ratification path: fork by the founder, then a confirmed card.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid, a keyboard region and one Tab stop) — the prior engine-debts chunk. `MountedData::scroll_to` was made synchronous because the planned async form panicked with `RefCell already borrowed` and did not ship. This is a trap for item 2, a deferred or replayed act for a task polled during a render. The grid's focus return to the grid after the editor's Enter is also a focus path that item 1's fixup must keep.

@history_item n=5 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the citations the chunk's line moves re-pointed) — re-pointed 3 citations into `packages/dioxus-native-dom/src/dioxus_document.rs` (skeleton, head and scrolling-box notes) and 1 into `scrolling.rs` after an engine-debts chunk edited them. Items 2–4 work in `packages/dioxus-native-dom/src/`, so expect the same citation re-point for `decision/fixed-document-skeleton-main-mount` and the related notes.

@history_item n=6 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the bridge's skeleton and head notes re-cited) — one added line in `dioxus_document.rs` moved the skeleton, `index.html` and head notes' citations. This shows that any edit to the bridge's document file (the boundary registry, `SeededRandom`, the buffered render) owes a re-point of the rows behind `decision/fixed-document-skeleton-main-mount`.

@history_item n=7 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the screens' source citations after the graph registrations) — another case where additions to `dioxus_document.rs` moved the skeleton and head notes' citations while no claim changed. Same trap for this chunk's bridge edits.

@history_item n=8 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the source of `decision/headless-stand-pinned-viewport`: one task in TaskShell through `task_in_shell` at a pinned 800 × 600, scale 1.0, Light viewport with DejaVu Sans. This chunk proves each fix headless on the stand under this mount.

@history_item n=9 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — re-pointed the harness-defaults citations to the `Default` impl that holds the pinned 800x600, scale 1, light values. Item 3 touches the harness's `from_vdom` (the seeded source's set-up), so this citation may move again.

@history_item n=10 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the source of `decision/held-instance-reads-as-fresh-boot`: a held instance reads as a fresh boot, and the mount and pinned viewport hold after commands. A seeded `Math.random` (item 3) and a pruned boundary registry on remount (item 4) both bear on what "fresh boot" reads. The rule must hold in both layout modes.

@history_item n=11 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role a further consumer of the TaskShell mount) — the coverage role replays each kept check on a fresh boot of the same mount, and the held-instance rule must hold in both layout modes. If page-script randomness stays host-seeded (item 3 kept), a replay can diverge. This bears on the fork's cost.

@history_item n=12 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's coverage role) — the source of `contract/escher-session-coverage/answer`, `contract/escher-session-coverage/check-bound` and `contract/escher-session-coverage/usage`: each check is replayed on a fresh boot, and a failed boot is a session error (exit 3). Replay determinism rests on the seeded source, which item 3 extends to page scripts.

@history_item n=13 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the source of `decision/session-recording-replayable-check`, `contract/escher-session-record/check-file`, `contract/escher-session-record/record-dir-not-private` and `contract/escher-session-record/recording-failed`. This was a ratified boundary widening. Seeding page scripts' `Math.random` must add no new recorded field or argument without the founder's ruling (scope: no new carrier, verb, argument or field).

@history_item n=14 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — a check now holds each answer and its digest, and `Check::replay` reads them back. If page-script randomness stays unseeded, recorded answer digests can fail to replay. This is part of item 3's cost, with no record field added.

@history_item n=15 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the origin of `decision/stand-ids-as-attributes-alone`: author ids are added as attributes alone, with no class, style, wrapper or order, and no CSS selects by them. Any stand fixture this chunk adds (a hidden focused control, a remounting boundary) must keep it.

@history_item n=16 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — keyed the non-lean tasks, an open dialog included, so that `unkeyed_actionable()` reads 0 on all seven tasks. This is the source of `decision/every-stand-control-keyed`. A new stand control or fixture this chunk adds must carry an author id.

@history_item n=17 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the founder's rule that every element an agent can act on reads an author key, added as attributes only. This is the precedent under `decision/every-stand-control-keyed` for any element a fixture adds.

@history_item n=18 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` takes focus (`tabindex="0"`) and its keys, giving a fixed CRUD Tab sequence of six stops. Item 1's fix, where a hidden control gives up focus, must leave the stand's Tab sequences and the keyed focus regions as they are.

@history_item n=19 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the load page's `load-done` is `display: none` until its stylesheet lands. It is a stand element hidden by CSS alone, the case item 1 names (a control hidden by `display:none` keeps focus). This entry also gave the six served screens and the lint at boot that a fix must keep at 0 findings in both layout modes.

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading, a closed select's options skipped) — lint at boot reads `[0, 0, 0, 0]` in both layout modes. A fix to items 1–4 must not move these readings, and a fixture must hold in both layout modes, as the scope's proof requires.

@history_item n=21 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Surface: desktop-native: 8 row(s)) — added the order page, whose controls are built to panic, return an error and log (`decision/order-page-built-to-cause`), and put it under the lint at boot. This is the stand surface that mounts error boundaries. Item 4, the growing boundary registry, is measured against it, and its ids must stay keyed.

@history_item n=22 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations into the files the take moved) — blitz-vibey-script's runtime was split into `src/runtime/` modules and its element bindings into `src/dom/element/`, and the master cites `src/runtime/window.rs` and `src/dom/element/geometry.rs`. Item 3 edits the runtime set-up (Boa's `Math.random` hook), so those citations may move.

@history_item n=23 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the source of the `escher-session` command form and the exit-code rows (`contract/escher-session-exit-codes/0` through `/3`, `contract/escher-session-refusals/unknown-verb`). The scope adds no verb, argument, field or cause without the founder's ruling, so a fix must leave these endings as they are.

@history_item n=24 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the source of `decision/mcp-server-local-stdio`, `contract/escher-session-mcp/stdio`, `contract/escher-session-mcp/end` and `contract/escher-session-host/log-service-name`, a crossing the founder ratified. A fix must cross no new carrier or log field.

@history_item n=25 marker=2026-10-12-self-description
**2026-10-12-self-description** — the source of `contract/escher-session-help/answer` (seven fixed keys) and `contract/escher-session-mcp/protocol-revisions`. Help's fixed text must not move unless the founder rules a new cause or field.

@history_item n=26 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the source of `contract/escher-session-screenshot/answer`, `contract/escher-session-refusals/off-screen`, `contract/escher-session-refusals/fonts-unavailable`, `contract/escher-session-mcp/screenshot-result` and `contract/escher-session-mcp/result-bound`. The off-screen refusal reads the visible region, so a layout-time focus fixup must leave it as it is.

@history_item n=27 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the source of `contract/escher-session-snapshot/answer` and `contract/escher-session-snapshot/part`. Snapshot readings (focus included) must not change their form.

@history_item n=28 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `escher-session lint`) — the source of `contract/escher-session-lint/answer` and the lint-at-boot readings that hold in both layout modes. A fix must leave these readings as they are.

@history_item n=29 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — the source of `contract/escher-session-actions/answer`, which lists actionable nodes with their verbs and causes. A hidden control that keeps focus (item 1) bears on what `actions` reports. No new cause word may be added without the founder's ruling.

@history_item n=30 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (escher-session gains the graph verb) — the source of `contract/escher-session-graph/answer` (ratified 2026-11-15). The graph reads registered signals and handlers. A boundary-registry or render-buffer change (items 2 and 4) must leave its answer as it is.

@history_item n=31 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** (Surface: cli: 1 row(s)) — the previous chunk. It changed `contract/escher-session-graph/answer` to take `--part`. The scope puts the driver debts out of scope, so this row is a constraint to keep and not to extend.

@history_item n=32 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Surface: cli: 4 row(s)) — set `contract/escher-session-command/form` and `contract/escher-session-mcp/tools` to fifteen verbs, added `contract/escher-session-layout/answer`, and gave `decision/no-caller-input-echoed` its exception (operator Q1, later ratified). This is the latest verb count, which this chunk must not move.

@history_item n=33 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into the rows this chunk is bound by, each verified against the code. It explains the picked rows' handles and the "NOT YET MEASURED" and "NO RECORDED INTENT" absences.
