@history_item n=1 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the driver command line's output form) — set the `escher-session` form: one line of JSON on stdout per command keyed by the schema's own words, no colour, no ESC byte, no terminal test, exit status 0 accepted · 1 refused · 2 usage · 3 session error; the new read's answer line inherits this form unchanged and adds no exit status.
@history_item n=2 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the `mcp` role's stdout) — the `mcp` role's stdout holds only newline-delimited JSON-RPC 2.0 lines, a tool result's text the JSON line (screen text for `snapshot`), no ESC byte asserted on every reply by the shared `Speaker`; the new read's MCP tool result must take this form and pass through `Speaker`.
@history_item n=3 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help` and the exit-status table) — exit classes read from the schema's `EXIT_STATUSES` table, `help` served from the schema's tables; the precedent for reading verb meaning (and the per-node verbs) from the schema's `VERBS` table rather than a list in code, and `help` must describe the new command.
@history_item n=4 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the two recording session errors in the exit-3 class) — new session errors went into the exit-3 class with no new status, as fixed text inside a ratified carrier; the precedent the new read follows if it adds any error text, and it touches the kept-check recording/replay path the read must also run through.
@history_item n=5 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** (the paged `snapshot` answer line) — `snapshot [--part]` answers `{"text":…}` or `{"text":…,"part":n,"parts":m}`, a founder-ratified fork (F1); the existing paging pattern if the new read is paged, and any paging argument is a carrier change the founder decides.
@history_item n=6 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line) — added `escher-session graph`: no argument (an argument refused `malformed`, nothing run), one JSON line `{"nodes":[…],"states":[…]}`, same answer on `call`, CLI, MCP and a kept check, carrying ids, owner ids, names, closed flag words and booleans only; a boundary widening ratified by the founder's P4 fork 1, its in-scope confirm PROVISIONAL (delegate the overseer) until the founder's epoch-boundary batch. The new read is graph's sibling on the same node model: its carriers, argument rule and closed-words-only content are set against this, and a new verb on every carrier is again the founder's fork.
@history_item n=7 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the unpaged graph, a pending limit) — the `graph` answer is unpaged, Cells at boot 293,869 bytes, stated limit «An unpaged graph» in docs/capabilities.md, residual `res-7cc5bi4k` (target next), subject `cli-unpaged-graph`; the new read over the same nodes must state whether it is paged or carries the same stated limit, measured on Cells at boot beside graph's, and a new subject would extend the subjects list after `cli-unpaged-graph`.
@history_item n=8 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (citation ranges re-read after the graph chunk's code) — re-pointed the `escher-session` item's citations (`EXIT_STATUSES` schema.rs:557-595, written JSON json.rs:400-462) and snapshot.rs's row bounds (163-166); this chunk edits schema.rs, json.rs and possibly snapshot.rs, so these citations will move again and need re-pointing at wrap.
@history_item n=9 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** (the hit walk's edge rule) — the hit walk stops at the padding box of a node clipping by `overflow`, root excepted, the node itself still answered in its border box; the base rule a `covered` cause must agree with on the clipping-ancestor fixture.
@history_item n=10 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** (the hit at an element's own box measured) — measured that an `img` answers itself, a block inside an `iframe`'s srcdoc answers the `iframe`, a filled `input` answers the `input`, and a transformed scrolled box hits where its rows show; these are what the shared covered path returns, so the read must not report such targets covered differently.
@history_item n=11 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the hit stops at a `contain: paint` box) — widened the stop to a computed `contain` including `paint`, PROVISIONAL then ratified by the founder (2026-10-18, by the Epoch 2 close entry of the same marker); a `covered` cause must agree with the stop at a `contain: paint` box as well as `overflow`.
@history_item n=12 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** (the hit walk skips a dropped node) — the hit walk skips a node dropped since the last resolve, as paint's walk does; a dropped node is never the cause a target reads covered.
@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (a non-invertible transform takes no hit) — a transform with determinant 0 or non-finite answers no hit for its whole subtree; the read must report for such a target the refusal the shared `action_point` path returns, not an accepted verb.
@history_item n=14 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through a clipping box's transforms) — `visible_region` exact under translate/scale, bounding box under rotation (holding corners not shown), `None` on a non-finite edge; the off-screen cause comes from this reader through `action_point`, rotation's extra corners included, never the read's own geometry.
@history_item n=15 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list answers the hit before the page's walk) — a row of an open list is answered by `select_list_hit` in `hit_with_scrollbar` before the page's walk, so a box clipping the select clips no row, and a button under the list is refused `covered` by the driver; the read's covered cause for rows and for targets under an open list must come from this path.
@history_item n=16 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`visible_region` on a row of an open list) — for a row or group of an open list `visible_region` answers the list's scrollport; an owner-served row's off-screen cause is judged against that scrollport.
@history_item n=17 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** (an into-view scroll never animates a nested box) — `scroll_into_view` writes every holding scrolling box at once, innermost first, behaviour applying to the viewport alone, and the driver's `scroll` is instant everywhere, founder-ratified (2026-10-07); the chunk that built refusal detection, whose shared `action_point` order (disabled, off-screen, covered) the read reuses, and the off-screen cause must agree with this into-view behaviour.
@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (into view on a row of an open list) — exception to the into-view rule: on a row of an open list `scroll_into_view` writes that list's `scroll_y` at once, moving no chain and no viewport; an owner-served row's off-screen answer must follow it, and the read itself must scroll nothing.
@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid's into-view move carries no animation) — the stand's Cells grid brings its current cell into view with `scroll_to(Instant)` synchronously; relevant to proving off-screen on Cells, the stand's scrolling grid, and the general into-view rule stands beside it.
@history_item n=20 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the flight booker's select, the served load page and the scrollbar's band) — right after a scroll the shown overlay scrollbar takes a pointerdown in its band and dispatches nothing until an `advance` passes its fade, so a Cells click there lands on the scrollbar; the read's accepted/refused answer must agree with this, a stand proof scrolling first must advance past the fade; also the stand serves six tasks (Cells, flight booker's select among them) the proof runs over.
@history_item n=21 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (the harness drives animation from the documents' clock) — the harness's `VirtualClock` moves only by `tick`, `advance_time` and `advance`, while `click`, `type` and `press` leave it where it was, ratified by the founder (2026-10-18); the read dispatches nothing and must leave the clock unmoved and wait on no settle.
@history_item n=22 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** (the driver's `advance` moves app time and leaves the animation clock alone) — the precedent separating the app's time from the animation clock for driver verbs; a read like `snapshot`/`graph` moves neither.
@history_item n=23 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's wheel and key, and no motion) — the select list has no motion and needs no `advance`: a read after a step reads the new state; the read lists an open list's rows right after the opening step.
@history_item n=24 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list's row cap, placement and row height) — an open list shows at most `MAX_ROWS` = 20 rows and scrolls past them; a row past the cap lies outside the scrollport and must read off-screen where a call would be refused so.
@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a single select's list is painted, an engine overlay) — a click, Enter or a Space opens a single select's list, an engine overlay via `paint_scene`, not rows as boxes; the owner-served rows have no layout box, so their causes come from the list's own answer, and the stand proof opens the list by `click` first.
@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (no select popup is painted, and who owns it) — the earlier state, superseded by the select-popup entry above; a trap: any reading that a single select opens nothing on Enter/Space or that a group's label is unpainted no longer holds.
@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (how a single select's list opens, moves, chooses and closes) — a disabled row changes nothing and leaves the list open; a disabled, inert or hidden select, or one with no row, opens nothing; a `multiple` select opens no list; Escape, a click outside, a wheel elsewhere and the page scrolling close the list; the disabled cause and listed verbs for selects and rows must agree, and the read must scroll and dispatch nothing so the list stays open.
@history_item n=28 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (a group in the open list, a header heavier than a row) — a group reads as a header that chooses nothing, a group with no label reads the whole list's box; the verbs listed on an owner-served group must agree.
@history_item n=29 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the tree, the snapshot and a shot read of an open list) — the highlighted row is in no snapshot and no tree; a row of an open list carries its own `bounds` in the snapshot (read as inside the ratified crossing); owner-served sub-targets are the snapshot's option/group nodes, not the painted highlight.
@history_item n=30 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open list's thumb) — a scrolled list's thumb takes no hit and is not draggable; the thumb is never a row's covered cause, and a row scrolled out of the list reads off-screen by the list's scroll.
@history_item n=31 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** (the user-agent select box) — a single select shows its chosen option alone (`option { display: none; }` beside the chosen one `display: inline`); its other options have no box yet are owner-served nodes the read must list with the cause a call would return.
@history_item n=32 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the default sheet's `multiple` rows and a select's groups) — a `multiple` select's options and groups lay out as block rows, a single select's group is `display: contents`; the boxes the owner-served nodes of each select kind have, which the off-screen and covered causes read.
@history_item n=33 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the shot's answer line, the MCP image block, no font refused) — a `screenshot` with no `sans-serif` family is refused `fonts-unavailable`, the stand's bundled font passing; the stand and fixture proofs of covered/off-screen must run where text measures non-zero, else they pass vacuously; also a precedent for a new answer line on cli and MCP ratified as a founder fork.
@history_item n=34 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — the stand's bundled DejaVu Sans registration (originally 2026-10-06-headless-stand) is now held in architecture §Standard Contracts → Headless stand, ratified 2026-10-27; a fixture added for a covered, off-screen or owner-served case takes its font from there, not from design-system typography.
