@history_item n=1 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — set the screenshot's design contract this chunk rests on: a driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted (text never painted skipped), and `escher-session screenshot [--id]` answers one uncoloured `{"width","height","png"}` line, the MCP role an image block then a text block without `png`. The image in band is the founder's fork 1; a canvas's painted pixels now leave through exactly this answer on every carrier, so a canvas shot must keep this shape and the fonts-unavailable refusal order (rows contract/driver-screenshot/fonts-unavailable, contract/escher-session-commands/screenshot, contract/escher-session-mcp/screenshot-result).

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the stand registers the bundled DejaVu Sans for every generic with system fonts off on native (`build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`), which is why stand shots pass the fonts-unavailable gate; a canvas page or task on the stand must boot with the same font context or its shot is refused.

@history_item n=3 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the stand's font registration leaves for architecture's Headless stand) — the stand font registration is no longer a design-system value but held in architecture §Standard Contracts → Headless stand beside `font_ctx()`; the custom `FontContext` item (row contract/native-config/font-context) stands here. A canvas boot's font is to be checked there, not in typography. Ratified by the founder in 2026-10-27-upstream-sync-engine-debts.

@history_item n=4 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the bundled DejaVu Sans applied to every boot (`boot_cells`, `boot_load` among them) and `escher-session` served tasks listed in the usage line's `apps:` and the MCP `start` tool's `enum`, an unknown task a usage ending; precedent for how a new stand page or task (a canvas page is a fork candidate) enters the served-task list and its boot font.

@history_item n=5 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — the served-task rows moved from six to seven (`~ contract/escher-session-tasks/cells` … `timer`, `+ contract/escher-session-tasks/order`), order added as a new item of the closed served-task list on the operator's ruling; if the stand gains a canvas task, every contract/escher-session-tasks/* row's count moves again and the addition is a boundary item needing a ruling.

@history_item n=6 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — settle answers the keeps-animating set without waiting on it, returning `Settled` with `animating` read off the flag, explicitly because "a canvas reads animating for as long as it exists"; a trap for this chunk: a shot of a canvas page must not wait on the animating flag, and a canvas left on the stand keeps `animating` true.

@history_item n=7 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the harness drives animation from the `VirtualClock` its documents read, settle's `animating` reading the document flag; ratified in 2026-10-18-engine-debts. Bears on a canvas's animating membership and on a shot running no step and moving no clock.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — fixed the cli output form: one line of JSON per command keyed by the schema's words, no colour, no ESC byte, no terminal test, exit statuses 0/1/2/3 (rows decision/escher-session-agent-read-surface, decision/escher-session-no-colour, decision/driver-output-for-agents, contract/escher-session-stdout/command-line-line, contract/escher-session-exit-*); a canvas shot's answer must keep this form.

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's stdout: newline-delimited JSON-RPC 2.0 lines only, a reply per request, no ESC byte asserted on every reply by the shared `Speaker` (rows contract/escher-session-mcp/stdout, contract/escher-session-stdout/mcp-line); the canvas shot's MCP reply rides this.

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** — exit statuses read from the schema's `EXIT_STATUSES` table that `help` serves (row decision/exit-statuses-from-schema-table); the scope forbids a new exit or cause unless a fork's ruling adds one, and any such addition lands in that table.

@history_item n=11 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — precedent for new session errors entering the exit-3 class as fixed text inside a ratified carrier with no new exit; the pattern a canvas fork would follow if it added a session error.

@history_item n=12 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the paged `snapshot` answer `{"text":…}` / `{"text":…,"part":n,"parts":m}` (row contract/escher-session-commands/snapshot), ratified by the founder's F1; a snapshot of a canvas page must keep this answer shape.

@history_item n=13 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — every document gets the default user-agent sheet once through `DocumentConfig::default()` (row decision/default-ua-sheet-via-default-config); a canvas (`<canvas>` or `<object>`) takes its default box styling from that sheet, the harness path included.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the open select list paints last; a single select's list is painted, an engine overlay; what the tree, the snapshot and a shot read of an open list) — the open list paints last in `paint_scene` after the root's render and before the devtools overlays, an engine overlay reaching the screen through `paint_scene` alone, and a `screenshot` of a row crops to it leaving the list open (rows decision/select-list-painted-after-root, contract/screenshot/select-list-row, contract/snapshot/select-list-row-bounds); the canvas widget's scene appended by `blitz-paint` must keep this paint order, and the shot of a widget crosses the same render path. Its citation re-point entry also moved `render.rs` lines.

@history_item n=15 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (`mix-blend-mode` and `isolation` in the depth rules; the citations re-pointed past the take) — the stacking-root list gained `mix-blend-mode`, `render_element` pushes a viewport-wide blend layer popped last, the select list paints past the blend layers, and no test paints a blended element to a frame; the citation entry re-pointed `custom_widget.rs` (+2) and `blitz-paint`'s `lib.rs` (+1, the `blend` module) — the very files that hold the `Widget` trait and `build_custom_widget_scenes` this chunk measures.

@history_item n=16 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the inline box's paint skip) — `render_element` skips the own shadow, background and border of an inline box holding only block children, with no frame test reading it; a trap in the same render path the custom widget's scene is drawn through.

@history_item n=17 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the hit walk stops at the padding box of a node clipping by `overflow`, as paint clips (area of row decision/stacking-context-groups-paint-and-hit); paint and hit agree on the same grouping, which a canvas element's paint must not break.

@history_item n=18 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — hit answers measured at an element's own box (an `img` with raster data answers the `img`), in both layout modes and both builds; the closest precedent for a replaced element painting its own content, and for measuring per build selection.

@history_item n=19 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — on every Dioxus document, windowed and headless alike, a render is recorded into a mutation buffer with the document unborrowed then replayed under one borrow (row decision/buffered-render-replay); a widget handed through `dioxus-native-dom`'s mutation writer (`<object data>` `WriteOnceAttr`) arrives through this replay, and a call during render sees the previous render's document.

@history_item n=20 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** (Surface: desktop-native: `~ contract/mounteddata-during-render/document-state`) — re-stated the during-render limit; a canvas widget set during a render reaches the document only on replay.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid's into-view move carries no animation) — the async form, a task spawned from a key handler, panicked with `RefCell already borrowed`; the trap the buffered render later removed, relevant if a canvas app paints from a task.

@history_item n=22 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage answer's three stated limits and their owners) — "handlers inside a canvas are not counted", owner 0.4.0, the incubator's canvas plans; this chunk opens the canvas epoch and must not claim that limit closed (canvas as data is out of scope, Epochs 4 and 5).

@history_item n=23 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the `actions` answer line and its decision by the verbs' own checks (rows decision/actions-share-the-verbs-checks, contract/actions-line/*, contract/escher-session-commands/actions-answer), a boundary widening ratified by the founder; a canvas node's listing in `actions` must stay inside that closed content.

@history_item n=24 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cli surface's graph answer line) — `graph` answer on every carrier carrying names, closed flag words and booleans only; ratified in 2026-11-15T05-21-42-wrap and reshaped for parts in 2026-11-20-driver-debts (row contract/escher-session-commands/graph-answer); a canvas page's graph keeps this content.

@history_item n=25 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `~ contract/escher-session-commands/graph-answer` to its whole-or-part form; the row this chunk picks as it now reads.

@history_item n=26 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `+ contract/escher-session-commands/layout-answer`: the layout answer on every carrier; a canvas element's bounds would be read through it (placement at the content box is the next chunk's).

@history_item n=27 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — `~ contract/select-accessibility/highlighted-row`, `~ contract/select-accessibility/open-select`: the picked select rows as they now read; the stand's readings that rest on them must keep their digests through this chunk.

@history_item n=28 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** (Typography: 2 row(s)) — `+ decision/unavailable-families-use-default-font`: text in an unavailable family falls back to the default font, beside the picked row decision/no-generic-family-lays-out-sans that the fonts-unavailable refusal reads; bears on which family a canvas's text or the page around it resolves before a shot.

@history_item n=29 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (137), each verified against the code; the picked rows (typography, depth-strategy, surface, surface-cli) take their current wording from this conversion, and any canvas row this chunk adds enters as a `+ {handle}` amendment in that form.
