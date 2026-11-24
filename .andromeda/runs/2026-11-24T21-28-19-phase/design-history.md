@history_item n=1 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — (Surface: desktop-native) + decision/canvas-line-form-conventions: settled the canvas line's form (rgba hex colours, Custom paint as #00000000, blend= mix,compose, cap=, dash= lengths@offset, a glyph run with no glyph as at=none). A canvas finding or any lint reading the recorded commands must keep these conventions; the glyph-run form holds origins only, which is why a label's overflow needs a measured extent.

@history_item n=2 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — (Motion) + contract/driver-verbs-and-virtual-clock/canvas: the driver's canvas read changes nothing (snapshot, focus, diff, VirtualClock) and applies no pending Timer tick. A canvas lint built on `Session::canvas` or inside `Session::lint` must keep that read side-effect free.

@history_item n=3 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — (Color Palette) + decision/circle-drawer-canvas-colours-in-code: Circle Drawer's canvas colours (accent stroke, dark label, accent fill at alpha 38/255) moved from CSS into widget constants. These are the stand colours the canvas lint reads clean, and any contrast check would judge them, the fill being translucent over the canvas background.

@history_item n=4 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — (the stand's colours leave the product surface; the stand's font registration leaves for architecture's Headless stand) the stand accent and its uses are held in layout-templates `values · desktop-native-screens`, and the bundled DejaVu Sans registration in architecture §Standard Contracts → Headless stand, not in design-system. The stand's accent stroke and the label's font that a glyph-extent measure would use are read from those homes, not from palette or typography rows.

@history_item n=5 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (F2's stand-row move ratified; the stand's colours, type and select padding leave design-system) ratified the move of stand-only values to layout-templates by the founder's word. Precedent: a stand fixture's own values (the planted canvas fixture's colours or sizes) belong in layout-templates, not as design-system rows.

@history_item n=6 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — (Typography → Loading) the headless stand registers bundled DejaVu Sans for every generic, system fonts off, through `HarnessOptions.font_ctx`, so text measures the same on host and CI. A glyph-advance read through the shared Parley font context depends on this registration, in both layout modes and across OSes (the cross-OS shot carry).

@history_item n=7 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Typography) + decision/unavailable-families-use-default-font: when no listed family is available, text lays out in the engine's default font and font-relative units measure against it. A canvas label's measured extent depends on which face resolves; a fixture whose font is missing measures in the default font, not zero.

@history_item n=8 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — (Typography → Loading; Surface: cli) a screenshot of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable`, text never painted skipped; underlies decision/no-generic-family-lays-out-sans and `can_paint_text`. A canvas-lint glyph measure should not report an overflow (or clean) against text the engine cannot measure.

@history_item n=9 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (`visible_region` on a row of an open list) `BaseDocument::visible_region` answers an open list's scrollport for its rows, and the driver's lint takes an open row's region from it alone. Precedent for how the DOM lint derives a region for content not laid out as boxes — the closest prior to settling a canvas label's region (content box plus pushed clips).

@history_item n=10 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — (`visible_region` through a clipping box's transforms) the visible region reads each clipping box through its transforms: exact under translate/scale, a rotation's bounding box, `None` for a non-finite edge. A canvas region intersected with the canvas element's own visible region inherits these readings; transformed clips pushed by a widget raise the same bounding-box question.

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open list's plain pair is measured, its value unstated) the open list's plain pair is measured by a check while a highlighted row's contrast is unread by the lint. Precedent for a lint contrast reading over engine-painted content outside the DOM boxes — a candidate canvas text-contrast check against the fill painted under it.

@history_item n=12 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the coverage answer's three stated limits and their owners) records that handlers inside a canvas are not counted, owner 0.4.0's canvas plans. A sibling canvas-blindness limit of the same family as the DOM lint seeing a canvas as one element; the capability statement's limit wording is the precedent for stating what the canvas lint cannot see (virtual nodes pooled).

@history_item n=13 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the `actions` answer line) a new verb's crossing onto every ratified carrier was a boundary widening ratified by the founder (pattern `rul-izni57qv`), its line carrying closed cause words only. Precedent for the crossing question this chunk faces: new closed defect words inside `lint`, a field of `canvas`, or no carrier.

@history_item n=14 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the cli surface's graph answer line) a new answer line carrying ids, names, closed flag words and booleans only, never a value or typed text, recorded PROVISIONAL then ratified (2026-11-15T05-21-42-wrap). Precedent for a canvas finding naming the canvas by its stable id with closed words and no glyph, text or font content crossing.

@history_item n=15 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (Surface: cli → Component Patterns) `escher-session` prints one JSON line per command keyed by the schema's words, no colour, no ESC byte, no terminal test; exit 0 accepted · 1 refused · 2 usage · 3 session error. Origin of decision/escher-session-agent-read-surface, decision/escher-session-no-colour, decision/driver-output-for-agents and the stdout/exit rows; any lint or canvas answer change keeps this form.

@history_item n=16 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — (the `mcp` role's stdout) the mcp role writes JSON-RPC lines only, a tool result's text the JSON line, no ESC byte, asserted by the shared `Speaker`. A changed `lint` or `canvas` answer must reach the mcp role uncoloured, the guard of decision/escher-session-no-colour.

@history_item n=17 marker=2026-10-12-self-description
**2026-10-12-self-description** — (`escher-session help` and the exit-status table) exit statuses read from the schema's `EXIT_STATUSES` table whose meanings `help` serves; `help` answers one uncoloured JSON line, status 0, reaching no session. The scope names `describe.rs`'s self-description as changing if a carrier changes; rows contract/escher-session-commands/help, decision/exit-statuses-from-schema-table and contract/escher-session-mcp/help-result come from here.

@history_item n=18 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — (Surface: cli → Component Patterns) two new session errors joined exit class 3 as fixed text inside a ratified carrier, no new exit. Precedent for the playbook rule this chunk's fork test reads: fixed closed text inside a ratified carrier is no widening; the scope forbids new exits, causes or session errors unless a fork adds them.

@history_item n=19 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — (§Motion → Animation runtime) a canvas reads animating for as long as it exists; settle returns with `animating` read from the document's flag and never advances the clock. Background for a canvas lint run on a settled screen: it must not wait on or move the clock (with the canvas row's no-tick promise).

@history_item n=20 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — (every area) the master's text became rows, each verified against the code; the origin of the picked rows with no later amendment (decision/test-engine-with-system-fonts, decision/tests-built-with-system-fonts, decision/logical-viewport-from-scale, decision/no-generic-family-lays-out-sans, the escher-session exit and stdout contracts). The system-fonts rows matter for a glyph-extent check: without system fonts text measures 0x0 and font assertions pass vacuously.
