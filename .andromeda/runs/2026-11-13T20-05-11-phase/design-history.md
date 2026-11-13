@history_item n=1 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle answers every keeps-animating member alike and reads animation time only through the harness clock, never advancing it (decision/animation-driven-by-virtual-clock); `execute`'s per-verb settle step must keep that reading unchanged when its dispatch is split into helpers.

@history_item n=2 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's `advance` moves app time while `click`, `type` and `press` leave the animation clock alone; a click beside a running animation returns settled without waiting. The origin of contract/driver-verbs-and-virtual-clock/{click,press,type}: the per-verb arms of `execute` carry this clock discipline and a restructure must not move it.

@history_item n=3 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — into-view scroll writes every nested box at once, Smooth applying to the viewport alone; the driver's `scroll` is instant in every box, inside one settled step (decision/driver-scroll-instant, decision/into-view-scroll-instant-in-nested-boxes, founder-ratified 2026-10-07). `execute`'s scroll arm holds this; the refusal-detection chunk also set the refusal checks whose order this chunk must keep.

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — escher-session prints one uncoloured JSON line per command (answer, refusal or session error), exit class 0/1/2/3, no terminal test (decision/escher-session-agent-read-surface, decision/escher-session-no-colour, decision/driver-output-for-agents). Refusals produced by `execute` reach this surface; the boundary "no change to refusal order, outcomes" protects these lines.

@history_item n=5 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the hit walk stops at the padding box of a node clipping by overflow, root excepted (origin of decision/hit-testing-stops-at-paint-clip). Any hit read on `handle_click`'s path keeps this rule; the restructure must not reorder hit before/after default actions.

@history_item n=6 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's stdout: newline-delimited JSON-RPC lines only, no ESC byte, an `Err` ending exits 3 (contract/escher-session-mcp/stdout, contract/escher-session-exit-codes/3). Results routed from `execute` and `graph` keep this form.

@history_item n=7 marker=2026-10-12-self-description
**2026-10-12-self-description** — exit statuses read from the schema's `EXIT_STATUSES` table; a session error's object carries its fixed `remedy` (decision/exit-statuses-from-schema-table, contract/escher-session-session-error/remedy). No new cause, session error or exit may enter while `execute` is split.

@history_item n=8 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — a `screenshot` whose font context resolves no sans-serif is refused `fonts-unavailable` with nothing painted; the shot's `{"width","height","png"}` line and the MCP image block (decision/screenshot-never-skips-text, contract/driver-screenshot/fonts-unavailable, contract/escher-session-commands/screenshot, contract/escher-session-mcp/screenshot-result). This refusal sits in `execute`'s screenshot path and its place in the refusal order must hold.

@history_item n=9 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — the hit stop widened to a computed `contain` including `paint`, then the Epoch 2 close ratified it with the one-virtual-clock batch (decision/hit-testing-stops-at-paint-clip, decision/driver-advance-moves-one-clock). Ratified engine behaviour a behaviour-preserving restructure of `handle_click`/`execute` must leave intact.

@history_item n=10 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the documents' `VirtualClock`; `advance` moves it by exactly its ms and the app's time together, click/type/press leave it (contract/driver-verbs-and-virtual-clock/advance, decision/driver-advance-moves-one-clock; ratified 2026-10-18). `execute`'s advance arm is the carrier of this contract.

@history_item n=11 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `record-dir-not-private` and `recording-failed` join exit class 3 with no new exit (contract/escher-session-exit-status/3). Precedent that session errors stay inside the four classes; `execute`'s split adds none.

@history_item n=12 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — while an acting driver step waits on a load in flight, at most `LOAD_WAIT`, the `VirtualClock` stands still (decision/load-wait-moves-no-clock). The wait sits in `execute`'s acting-step path; extracting it must keep the clock untouched.

@history_item n=13 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the user-agent select box: a single-choice select lays out as a box holding its chosen option's label, engine defaults, no token (decision/single-select-box-shows-chosen-label, guarded by `select_interaction`). The select arms of `handle_click` change that chosen option; `select_interaction` is one of the checks holding the move.

@history_item n=14 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot [--part]` answers `{"text"}` or `{"text","part","parts"}`; over mcp a paged result is the JSON line (contract/escher-session-commands/snapshot, contract/escher-session-mcp/snapshot-result). `execute`'s snapshot arm and its argument refusals keep this.

@history_item n=15 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — right after a scroll the shown overlay scrollbar takes a main-button pointerdown in its band as a thumb drag and dispatches nothing, so a driver `click` on a Cells cell there needs an `advance` past the fade. A click-path trap on the pointer side of `handle_click`/`execute`: dispatch order around the scrollbar band must not move.

@history_item n=16 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — a `multiple` select's chosen row takes Chromium's inactive selected pair `#c8c8c8`/`#000000`, hardcoded (decision/select-chosen-row-chromium-pair, guard `select_interaction`); `multiple` rows and groups in the default sheet; the interim "no select popup" line later retired. The multiple-select toggle arm of `handle_click` sits under this.

@history_item n=17 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (open, choose, close) — a single select's list opens on a click, Enter or Space with its chosen option and no input/change; Enter, Space, Tab or a row click choose with one input then one change; a disabled row leaves the list open; Escape, a second click, a click outside, a wheel elsewhere and page scroll close it; a disabled/inert/hidden/empty or `multiple` select and a label's click open nothing (decision/single-select-open-gestures, decision/selects-that-open-no-list, contract/select-list-events/*). These are exactly `handle_click`'s select arms, held by `select_popup`; the event sequences may not change.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (hit, paint, motion, placement) — the open list answers the hit first in `hit_with_scrollbar`, so `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` read it and a button under it is refused `covered`; it paints last, whole past clips; it has no motion and needs no `advance`; it is placed inside the viewport, every edge finite; into view on a row writes the list's own scroll (decision/open-select-list-hit-first, decision/select-list-painted-after-root, decision/select-list-opens-without-motion, decision/select-list-placed-in-viewport, decision/into-view-scroll-instant-in-nested-boxes). The `covered` refusal lives in `execute`'s `check_target`; the click path reads the list before the page.

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (colours, overlay, tree) — the list's colours are `SelectListView` constants, not tokens; on desktop-native the list is an engine overlay via `paint_scene` alone; the tree reads an open select ComboBox/expanded, the highlighted row in no snapshot, a row's own bounds in the snapshot and a shot of a row leaving the list open (decision/select-list-colours-engine-constants, decision/engine-colours-hardcoded, decision/select-list-engine-overlay, contract/select-accessibility/*, contract/snapshot/select-list-row-bounds, contract/screenshot/select-list-row). Snapshot and screenshot dispatch in `execute` must keep these readings.

@history_item n=20 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — a focused select whose container lost its box keeps focus and takes a key with no panic (decision/boxless-focused-select-keeps-focus, pinned by `key_press_after_container_lost_its_box` and the select-made-inert test); the open list's paint-last row extended past the blend layers (decision/select-list-painted-after-root). Select-arm edge cases a restructure of `handle_click` must keep panic-free.

@history_item n=21 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — on every Dioxus document a render is recorded into a mutation buffer unborrowed, then replayed under one borrow; render-time `MountedData` calls see the previous render (decision/buffered-render-replay, contract/mounteddata-during-render/*). `DioxusDocument::graph` lives in the same crate; extracting its helpers must not take a borrow that crosses this replay.

@history_item n=22 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — `graph` takes no argument (refused `malformed`), answers `{"nodes","states"}` as one uncoloured line on socket, cli, mcp and kept check, carrying ids, owner ids, component, state and handler event names, the flags `undeclared`/`unconfirmed` and booleans only; the answer is unpaged (Cells 293,869 bytes, residual `res-7cc5bi4k`), PROVISIONAL delegate overseer (contract/escher-session-commands/graph-answer, contract/escher-session-commands/graph-arguments, contract/ui-graph-line/content). The function this chunk restructures produces this line; its content and order are bound unchanged, held by `stand_graph`.

@history_item n=23 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — `actions` decides each node's verbs by the same `validate` and per-verb `check_target` every id-taking verb meets before it acts, so a listed cause equals the call's refusal; stale nodes read `stale`; unpaged limit (decision/actions-share-the-verbs-checks, contract/actions-line/*, contract/escher-session-commands/actions-answer, contract/escher-session-commands/actions-arguments). `execute`'s `check_target` and refusal order are shared with `actions`: any reorder or split that changes which cause wins breaks this equality.

@history_item n=24 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — `coverage` replays kept checks and answers the UI graph's handlers no check exercised, read at boot and after each step; limits on canvas handlers and bubbling (contract/escher-session-commands/coverage-answer, contract/coverage-line/content). It reads `graph`'s handler list, so `stand_handler_coverage` holds the graph restructure.

@history_item n=25 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters became rows verified against the code at the Epoch 3 tree (fc2d17afd, the same sha as metric record `met-cnhk7bbe`), guards re-pointed; every row this chunk binds by was cut then, so a row's cited test (`select_interaction`, `select_popup`, `stand_act_popup`) is its guard and must stay green through the move.
