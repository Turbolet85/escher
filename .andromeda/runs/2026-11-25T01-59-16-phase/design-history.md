@history_item n=1 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — render-time MountedData calls on every Dioxus document — added the platform note behind `decision/buffered-render-replay`: on the windowed launch and the headless stand alike a render is recorded into a mutation buffer with the document unborrowed, then replayed under one borrow, so code run during a render can call `MountedData` with no `RefCell already borrowed` panic; its limit by construction: such a call sees the document as the previous render left it. Matters here because a virtual-node declaration, if it lives on the Dioxus renderer side or is read during a render or a polled task (the «where the declaration lives» fork), runs inside this buffered path and sees the previous render's document — a list read at paint vs at render differs by that lag.

@history_item n=2 marker=2026-11-21-engine-debts
**2026-11-21-engine-debts** — `~ contract/mounteddata-during-render/document-state` — the during-render remnant re-stated: a call acts on the document as the previous render left it; a task awaiting a node's handle focuses it once mounted; a call in the render that removes its target leaves focus off it. Matters as the current wording of the buffered-render limit beside the picked row: a declaration whose canvas is created or removed in the same render must not assume the canvas element (its stable owner id) is already in, or still in, the document.

@history_item n=3 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — the Cells grid's into-view move carries no animation — the stand handler calls `MountedData::scroll_to` synchronously because the async form (a task spawned from a key handler) panicked with `RefCell already borrowed`; the trap the buffered render (n=1) later removed. Precedent for this chunk's no-panic rule: any declaration path that borrows the document from inside a handler or task is the known panic site to check.

@history_item n=4 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the cli surface's graph answer line (PROVISIONAL then) and the unpaged graph limit — created what is now `contract/ui-graph-line/content`: the graph line carries ids, owner ids, component names, registered state names, handler event names, the closed flags `undeclared` · `unconfirmed` and booleans only, never a value or typed text; its crossing a boundary widening ratified by the founder's P4 fork 1; the unpaged-graph limit owned by `res-7cc5bi4k`. Matters for the names' crossing fork: a virtual node's author-written name or key reaching the graph would widen this closed list (a new crossing needing the founder), and «Virtual nodes in every read» owns the graph — this chunk must leave the line's content untouched.

@history_item n=5 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified — the cli surface's graph answer line and the coverage role's answer line — the 2026-11-07-ui-graph and 2026-11-08-handler-coverage PROVISIONAL records ratified by the founder's own word, text unchanged. Matters as the standing that makes any change to the graph line's closed content (n=4) a change to a founder-ratified crossing, not a delegate's.

@history_item n=6 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `~ contract/ui-graph-line/content` (with `~ …/graph-answer`, `~ …/graph-arguments`) — the graph line's closed content gained the part and parts numbers of a graph read in parts; ratified via the operator relaying «Подтвердить все 41». The latest change to a picked row: shows how the closed list is widened (an explicit row amendment naming exactly what joins it) — the pattern a later name/key crossing would follow; this chunk adds nothing to it.

@history_item n=7 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the `actions` answer line, and its unpaged limit — created what is now `contract/actions-line/content`: ids, owner ids, the verb table's words, closed cause words and booleans only, never a value, typed text or `NodeId`; a node's verbs decided by `validate` then `check_target`, an unresolved node reading `stale`; its crossing ratified as a boundary widening on pattern `rul-izni57qv` by the founder's word; the unpaged answer (Cells 106,088 bytes) owned by `res-7cc5bi4k`. Matters because the reserved id `{owner}///{key}` would appear in this line once virtual nodes are actionable («Virtual nodes as elements» / «in every read»): a key is author text, so whether an id carrying it is still «an id» under this closed list is part of the names' crossing fork.

@history_item n=8 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's stdout on the cli surface — a tool result's text is the screen text for `snapshot`, the JSON line otherwise; replies uncoloured, no ESC byte. Origin of `contract/escher-session-mcp/snapshot-result`; matters as the carrier a virtual node's name would reach through the snapshot (owned by «Virtual nodes as elements»), i.e. one of the ratified carriers the founder's 2026-10-10 widening rule guards.

@history_item n=9 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — the paged `snapshot` answer line — `{"text":…}` whole, `{"text":…,"part":n,"parts":m}` in parts; over `mcp` a paged result is that JSON line (was the screen text alone); ratified by the founder's F1. The change that gave `contract/escher-session-mcp/snapshot-result` its present wording; virtual nodes added to the snapshot later will count toward the parts split.

@history_item n=10 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — the driver command line's output form — one uncoloured JSON line per command keyed by the driver schema's own words, four exit statuses; the founder's answers of 2026-10-10. Matters as the cli carrier's fixed form: the scope claims no new verb, error, exit status or carrier, so this chunk must leave the form and the four statuses unmoved.

@history_item n=11 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — the coverage answer's three stated limits — handlers inside a canvas are not counted, owner 0.4.0 (the incubator's canvas plans); the coverage line carries ids, owner ids, event names and two counts only. Matters as a canvas-interior gap this epoch owns: handler coverage of virtual nodes belongs to «Virtual nodes in every read», not this chunk, but the declaration's form (list vs callbacks) decides what that later count can read.

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — what the tree, the snapshot and a shot read of an open list; the list as an engine overlay — rows of an open select list are not boxes, yet a row carries its own `bounds` in the snapshot and a shot crops to it; the answer card read a row's `bounds` and `expanded` as inside the ratified crossing (a closed set, a closed bool of engine state). Nearest precedent for nodes with no DOM element behind them standing in the tree and snapshot with engine-held bounds — the shape virtual nodes in a canvas region take, and the precedent for which additions count as no widening.

@history_item n=13 marker=2026-11-20-select-list-and-selection-debts
**2026-11-20-select-list-and-selection-debts** — `~ contract/select-accessibility/highlighted-row`, `~ …/open-select` — the highlighted row of an open list now carries the snapshot's closed bool `highlighted`, and the tree names it as the select's active descendant. Precedent for a non-DOM sub-node entering the snapshot and tree with only closed words, relevant to how the later entries will surface virtual nodes declared here.

@history_item n=14 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — `+ decision/circle-drawer-canvas-colours-in-code` — the circle colours moved out of CSS into the canvas widget as hardcoded constants. The canvas stand whose circles «Circle Drawer's circles as virtual nodes» will declare; shows the widget (not CSS) owns what it paints inside the canvas.

@history_item n=15 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — `+ decision/canvas-line-form-conventions` and `+ contract/driver-verbs-and-virtual-clock/canvas` — settled the canvas line's open form choices (a convention later readers and writers of the line must keep) and promised the `canvas` verb's relation to the virtual clock beside advance, click, press and type. Matters as the canvas carrier closest to this chunk: draw commands grouped by node is a later entry's, and it must follow these conventions; this chunk adds no verb and must not move the canvas line.

@history_item n=16 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `+ contract/escher-session-commands/layout-answer`, `+ …/layout-arguments` — the layout command on every carrier and its argument contract (a missing id refused `malformed`). Matters as the boundary: layout explained (`Placer::App { canvas }`) for virtual nodes is «Virtual nodes in every read»'s, so this chunk leaves the layout answer as it is.

@history_item n=17 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the masters as rows — the design master became 137 rows (decision · contract · absence · overview), each verified against the code. Explains the form of the picked rows (`decision/…`, `contract/…/…`) and that any change this chunk makes to the plan goes in as `+ / ~ / −` on a row handle.
