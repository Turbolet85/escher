@history_item n=1 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added `+ decision/circle-drawer-canvas-colours-in-code`: Circle Drawer's canvas colours (stand accent stroke, dark label, accent fill at alpha 38/255) moved out of CSS into widget constants. This is exactly the scene this chunk dumps: fill/stroke colours in the commands come from code, not the cascade, so a dump writes these constants; any change to them is a palette amendment.

@history_item n=2 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's colours leave the product surface; radii and shadows; font registration) — moved every seven_guis stand value (accent `#3a5ce5` and shades, card shadows, stand font registration) out of design-system into layout-templates `values · desktop-native-screens` and architecture §Headless stand. Precedent: stand values the canvas dump will print (accent stroke, fill) are not design-system values; a dump's colours or font facts belong to those homes, not new palette rows.

@history_item n=3 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (F2's stand-row move ratified; the stand's colours, type and select padding leave design-system) — ratified the stand-row move by the founder's own word and removed the remaining seven_guis text/type rows. Confirms the rule this chunk rests on: Circle Drawer stand values are layout-templates', and the colours-in-code row is the only palette exception.

@history_item n=4 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the stand accent's use in the Cells grid) — the `seven_guis stand` palette row lists the accent's uses without stating a value. Precedent for recording a new accent use (Circle Drawer's canvas stroke/fill) as a use, not a token.

@history_item n=5 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's colours are engine constants; the open list paints last in `paint_scene`) — precedent for colours hardcoded as constants in code with no token, and for an engine overlay painted inside `paint_scene` after the root's render. This chunk reads widget scenes out of `paint_scene` (`build_custom_widget_scenes`); the paint-order and overlay placement there must stand unchanged.

@history_item n=6 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`mix-blend-mode` and `isolation`; citations re-pointed) — `render_element` pushes a viewport-wide blend layer through blitz-paint's `blend` module; citations into `custom_widget.rs` and blitz-paint `lib.rs` re-pointed. Bears on the dump's push-layer/pop-layer commands and on the files this chunk edits (line citations will move again).

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the cli output form: one uncoloured JSON line per command keyed by the schema's words, no ESC byte, no terminal test, exit status 0/1/2/3 (rows `decision/escher-session-agent-read-surface`, `decision/escher-session-no-colour`, `decision/driver-output-for-agents`, the exit-status/stderr contracts). A new canvas-read verb must answer in this form on every carrier.

@history_item n=8 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role: newline-delimited JSON-RPC 2.0 lines only on stdout, every reply one uncoloured line (rows `contract/escher-session-mcp/stdout`, `other-results`). The new verb's MCP tool result must be its JSON line.

@history_item n=9 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` self-description and exit statuses read from schema `EXIT_STATUSES`; session error objects carry a fixed `remedy` (rows `decision/exit-statuses-from-schema-table`, `contract/escher-session-commands/help`, `contract/escher-session-mcp/help-result`, `contract/escher-session-session-error/remedy`). A sixteenth verb or new argument must appear in `help` (`describe.rs`).

@history_item n=10 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot [--id]` answer line `{"width","height","png"}`, MCP image block, and a shot refused `fonts-unavailable` when no sans-serif family resolves (row `decision/no-generic-family-lays-out-sans` explains `can_paint_text`). The scope places the canvas read beside `screenshot`: precedent for an id-taking read of painted content, its answer shape on the carriers, and whether glyph runs need the same font gate; ratified as the founder's fork on carrying an image in band.

@history_item n=11 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — paged `snapshot [--part]` answer `{"text",…,"part","parts"}`, ratified by the founder's fork. Precedent for an answer that would exceed the 10,000-byte snapshot budget the scope measures the dump against: paging versus a summary level.

@history_item n=12 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer line; the unpaged graph, a pending limit) — `graph` answers one JSON line carrying closed words, names and booleans, never a value or typed text; its unpaged 293,869-byte answer recorded as a stated limit owned by `res-7cc5bi4k`. Precedent for what a read may carry (the glyph-run authored-text fork) and for recording an oversized answer as a stated limit.

@history_item n=13 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph answer line; ratified — the coverage role) — the founder's own word replaced the overseer's PROVISIONAL delegate confirm on the graph and coverage cli items. Shows that a new crossing on the carriers is a boundary widening needing the founder's word, as this chunk's carrier fork is.

@history_item n=14 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `~ contract/escher-session-commands/graph-answer`, `~ graph-arguments` (optional `--part` 1..1000, other args refused malformed), `~ contract/ui-graph-line/content`. The current form for an argument added to an existing verb and for a paged answer; a level argument on the canvas read follows the same malformed-refusal pattern.

@history_item n=15 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the `actions` answer line; the unpaged `actions` answer) — origin of `decision/actions-share-the-verbs-checks`, `contract/actions-line/content`, `contract/actions-line/nodes`, `contract/escher-session-commands/actions-answer`: a new verb lists itself in `actions` per node with its target check, and the line carries closed words only. A canvas-read verb id-taking on a canvas element enters each canvas node's verb list; its crossing was a boundary widening on pattern `rul-izni57qv` ratified by the founder.

@history_item n=16 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (coverage role's answer line, endings and refusals; three stated limits) — origin of `contract/coverage-check-files/read-bound` and `refusal`; the coverage answer's limit «handlers inside a canvas are not counted» is owned by 0.4.0's canvas plans. A kept check carrying the new verb must replay under coverage; the canvas limit stays until virtual nodes, outside this chunk.

@history_item n=17 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `+ contract/escher-session-commands/layout-answer`, `+ layout-arguments` (one required `--id`, missing/over-bound/extra refused malformed, status 1). The nearest precedent for an id-taking read verb on every carrier; the canvas read's `--id` argument contract follows it.

@history_item n=18 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — `~ contract/escher-session-tasks/*` six → seven, `+ contract/escher-session-tasks/order`. The served-task set is a closed list in the rows; Circle Drawer is not served, so serving it (or a fixture) to reach a stand canvas on the carriers would amend these rows and is the founder's fork.

@history_item n=19 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the recording's `record-dir-not-private` and `recording-failed` session errors sit in exit class 3 with no new exit (row `contract/escher-session-exit-status/3`). The chunk touches `recording.rs`/`replay.rs`; a new verb in recordings adds no exit status.

@history_item n=20 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle answers the keeps-animating set alike, never waiting; a canvas reads animating for as long as it exists; settle reads animation time only through the harness clock (row `decision/animation-driven-by-virtual-clock`). A canvas read that runs no step must not wait on or advance the clock.

@history_item n=21 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the driver's `advance` moves app time; `click`, `type`, `press` leave the animation clock where it was. Origin of the verb-and-clock contracts the new read verb joins: a read leaves the `VirtualClock` where it was.

@history_item n=22 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the one `VirtualClock`; `advance` moves it by exactly its ms and app time together, `click`/`type`/`press` leave it (rows `contract/driver-verbs-and-virtual-clock/*`, `decision/driver-advance-moves-one-clock`). A new verb needs its own clock line in this family (it moves nothing).

@history_item n=23 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder, Epoch 2 close) — ratified the one-virtual-clock motion rule, the advance/click/type/press clock contracts this chunk's verb extends.

@history_item n=24 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the stand registers the bundled DejaVu Sans for every generic with system fonts off on native, so text measures the same on dev and CI. Bears on the dump's glyph runs (font data by reference, determinism across hosts); its registration now lives in architecture §Headless stand per 2026-10-27-readme-and-records.

@history_item n=25 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — `+ decision/unavailable-families-use-default-font` (with `font_element_face_and_size`). Which font a canvas label's glyph run carries when no listed family is available; the dump's font reference must follow that fallback.

@history_item n=26 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — origin of `decision/default-ua-sheet-via-default-config`: every document gets the default user-agent sheet once through `DocumentConfig::default()`. The test-harness read of a canvas boots documents by this path.

@history_item n=27 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master into rows (decision · contract · absence · overview); every amendment since names a row handle. This chunk's expected amendments (a new verb's answer, argument and clock contracts) land as `+`/`~` rows in that form.
