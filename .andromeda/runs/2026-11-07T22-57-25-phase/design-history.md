@history_item n=1 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (the driver command line's output form) set the `escher-session` stdout form: one line of JSON per command keyed by the driver schema's own words and nothing else on stdout, usage errors one fixed stderr line, no colour/colour dependency, no ESC byte, no terminal test, exit classes 0 accepted · 1 refused · 2 usage · 3 session error; the founder's answers of 2026-10-10. This is the form the new graph verb's CLI answer must take (the extract's first three constraints and its acceptance line).

@history_item n=2 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — (the `mcp` role's stdout) added the `mcp` role: newline-delimited JSON-RPC 2.0 lines only, a reply per non-notification request, a tool result's text the JSON line (screen text for `snapshot`), no ESC byte asserted on every reply by the shared `Speaker`, no-terminal-test search counting 0. The graph verb's MCP tool rests on this rule and passes through that same `Speaker` assertion.

@history_item n=3 marker=2026-10-12-self-description
**2026-10-12-self-description** — (`escher-session help` and the exit-status table) made the four exit classes read from the schema's `EXIT_STATUSES` table that `help` serves, and gave a `help` tool result's text as the self-description's line. The new verb must add no exit and stay in that table, and its addition to `VERBS` shows in `help`'s self-description.

@history_item n=4 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (the shot's answer line, the MCP image block) set `screenshot`'s answer as one uncoloured `{"width","height","png"}` line and its MCP reply as an `image` block then a text block, every reply still one line with no ESC byte; the founder's fork 1 (2026-10-12). This is the template the extract names for a new verb's written form and MCP tool, and the precedent that a new content shape on the carrier went to the founder.

@history_item n=5 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — (the paged `snapshot` answer line) set `snapshot [--part]`'s line `{"text":…}` / `{"text":…,"part":n,"parts":m}` and changed its MCP result from screen text to the JSON line, ratified by the founder (F1). The other template for the graph verb's answer, and the precedent that a change to an answer shape on the cli carrier is the founder's fork; a large graph answer may meet the same paging question.

@history_item n=6 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — (the two recording session errors in the exit-3 class) put `record-dir-not-private` and `recording-failed` in the exit-3 class with no new exit, as fixed text inside a ratified carrier. Precedent for the graph verb's own refusals or session errors fitting the existing classes, and for its recorded step (scope's «its recorded step»).

@history_item n=7 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (Cli item) `escher-session` serves six tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), the usage line and the MCP `start` tool's `enum` listing them; it also added the overlay scrollbar's band behaviour for a Cells click. The graph read runs over these served stand tasks (the scope's `stand_non_lean_tasks` pins and its v030-05 acceptance), and the Cells click caveat stays when the stand's signals move onto the registry.

@history_item n=8 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (what the tree, the snapshot and a shot read of an open list) the highlighted row is in no snapshot and no tree; a row of an open list carries its own `bounds` in the snapshot and a `screenshot` of a row crops to it, leaving the list open; the snapshot's builder reads the row's list bounds before its client rect; the answer card read the row's `bounds` as outside the boundary widening, a closed set inside the ratified crossing. This is fct-grd6lupt, the precedent for an agent-visible node with an optional owner the graph must key on, and the precedent for the «reacts to a click» boolean being no widening.

@history_item n=9 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open select list answers the hit before the page's walk) a row of an open select list stands outside the page's hit walk, answered first by `select_list_hit` in `hit_with_scrollbar`, so a box clipping the select clips no row, and `hit`, `set_hover_to`, `handle_pointerdown` and the driver's `covered` all read it. The extract's constraint that a sub-target served by its owner must not change hit-testing rests on this entry.

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (a single select's list is painted, an engine overlay) on desktop-native the list is an engine overlay reached through `paint_scene` alone, not a second OS window and not rows as boxes (`:open` plus rows as boxes stand rejected). The rows have no DOM element of their own, which is why the graph must key on the accesskit node with an owner element rather than on `Target.node`.

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (how a single select's list opens, moves, chooses and closes) the list's open/choose/close behaviour, fired events (`input` then `change`) and the openers (a click, Enter, Space) are all handled by the select's own handler. This is the owner-served sub-target the scope's «a handler sits on an owner element and serves sub-targets» direction generalises; the graph's handlers for a row are the select's.

@history_item n=12 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — (the hit walk's edge rule) the page's hit walk stops at the padding box of a node that clips by `overflow`. This is the clip rule the select list's rows stand outside of (n=9); keying the graph on owner-keyed nodes must leave it as it is.

@history_item n=13 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the hit stops at a `contain: paint` box) widened the stop to a box paint clips by `overflow` or computed `contain: paint`, later ratified by the founder. Part of the same hit rule the owner-served sub-targets must not change.

@history_item n=14 marker=2026-10-18-upstream-sync-determinism-and-traces
**2026-10-18-upstream-sync-determinism-and-traces** — (§Iconography: the cursor over no box reads Default) «Default otherwise» now covers a pointer over no box; with the 2026-10-18 ratification this is the cursor reading of fct-guxvotwt. The cascade binds that the «reacts to a click» field is read from handlers, never from this cursor reading.

@history_item n=15 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — (the driver's `advance` moves app time) `advance` moves an app's own time through the step a session's caller hands it — on the stand whole Timer ticks — while `click`, `type` and `press` leave the clock. The Timer state written through that step is what the registry's link 3 records once the stand's signals move onto it; the graph read must, like `click`, move no clock.

@history_item n=16 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (the harness drives animation from the documents' clock) `advance` moves the `VirtualClock` by exactly its `ms` and the app's own time together; `click`, `type` and `press` leave it where it was (fct-e2e2e6vg / fct-za7xpowz). Recording the Timer's writes in the registry must not change by how much or when that state moves, and the new verb advances no clock.

@history_item n=17 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the PROVISIONAL batch ratified by the founder, Epoch 2 close) ratified the `contain: paint` stop, the one-virtual-clock `advance` reading and the cursor fix. The clock and cursor facts the cascade binds this chunk to are ratified, not provisional.

@history_item n=18 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — (the load wait moves no clock) while an acting step waits on a load in flight for at most `LOAD_WAIT`, the `VirtualClock` stands still. Another «moves no clock» rule beside `advance`; the graph read on the `load` task must keep it.

@history_item n=19 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (the Cells grid's into-view move carries no animation) the Cells arrow keys' handler calls `MountedData::scroll_to(ScrollBehavior::Instant)` synchronously; the async form (a task spawned from a key handler) panicked `RefCell already borrowed` and did not ship. Moving the Cells signals onto the registry must keep this handler's synchronous instant call, and it is a key handler, so alone it does not make a node read «reacts to a click».

@history_item n=20 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — (render-time MountedData calls on every Dioxus document) a render is recorded into a mutation buffer with the document unborrowed, then replayed under one borrow, so render-time `MountedData` calls no longer panic; such a call acts on the document as the previous render left it. The graph reader walks the vdom (`element_id.rs`) and reads subscribers inside `VirtualDom::in_runtime`; it must hold to this borrow order and read the screen after the replay, not mid-render.
