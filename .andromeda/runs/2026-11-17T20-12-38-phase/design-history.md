@history_item n=1 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — (the 0.2.0 sweep) re-pointed the list-bullet font's third citation after its citation sweep read the old range claim-false. This is the direct precedent for this chunk: a sweep that reads every cited design line finds stale citations and records them as the wrap's amendments, not as hand edits.

@history_item n=2 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — (the 0.1.0 sweep) measured the hit at an element's own box (img, iframe srcdoc, filled input, transformed and scrolled box) in both layout modes and both builds, widening `decision/hit-testing-stops-at-paint-clip`'s witnesses. It is the first sweep's precedent: measured readings go into the design rows, and the deferred `contain: paint` stop was left to a later owner.

@history_item n=3 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — added the hit walk's stop at the padding box of a node that clips by `overflow`, the root excepted. This is the origin of `decision/hit-testing-stops-at-paint-clip`, which the sweep's covered and hit witnesses rest on.

@history_item n=4 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the hit stops at a `contain: paint` box; ratified in the later 2026-10-18-engine-debts batch entry) widened the hit stop to a computed `contain` that includes `paint`. That gives `decision/hit-testing-stops-at-paint-clip` its present wording, and a sweep row citing that row must name this witness as well as the overflow one.

@history_item n=5 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open select list answers the hit before the page's walk) made `select_list_hit` answer first in `hit_with_scrollbar`, so `covered` refuses a button under the list and a box clipping the select clips no row. This is the source of `decision/open-select-list-hit-first` and of the except clause of `decision/hit-testing-stops-at-paint-clip`, both pinned in `stand_actions`.

@history_item n=6 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open list paints last, whole past clips and opacity) put the list's paint after the root's render in `paint_scene`, whole past an opacity ancestor and a clipping box. This is the source of `decision/select-list-painted-after-root`. The "stand layout held" check over the select list reads this paint order.

@history_item n=7 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** — (`mix-blend-mode` and `isolation` in the depth rules) made the list's paint-last row also say it paints past the blend layers. It also recorded that no test paints a blended element to a frame, and that gap was carried to a later owner. This is the current wording of `decision/select-list-painted-after-root`.

@history_item n=8 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (a single select's list is painted, an engine overlay) replaced the 2026-10-31-engine-debts line «no select popup is painted». Click, Enter or Space now opens the list, and on desktop-native it is an engine overlay through `paint_scene` alone, while the windowed list stays read and not run. This is the source of `decision/select-list-engine-overlay` and `decision/single-select-open-gestures`, and the sweep must state the windowed list as having no headless witness.

@history_item n=9 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (how a single select's list opens, moves, chooses and closes) added the open, highlight, choose, disabled-row, chosen-option and close events, and listed the selects that open nothing. These are the sources of every `contract/select-list-events/*` row and `decision/selects-that-open-no-list`, pinned by the `select_popup` checks, so each needs a named witness in the sweep record.

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (what the tree, the snapshot and a shot read of an open list) added these readings: the highlighted row is in no tree or snapshot, an open select reads ComboBox and expanded, its chosen option reads selected, and an author's `open` steers nothing. A row carries its own bounds, and a screenshot of a row crops to it with the list left open. These are the sources of `contract/select-accessibility/*`, `contract/snapshot/select-list-row-bounds` and `contract/screenshot/select-list-row`.

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open list's wheel and key, and no motion; the open select list's row cap, placement and row height) added wheel and key scrolling of the list and its having no motion, so no `advance` is needed. It also added the 20-row cap and placement inside the viewport with every edge finite. These are the sources of `decision/select-list-wheel-and-key-scroll`, `decision/select-list-opens-without-motion` and `decision/select-list-placed-in-viewport`.

@history_item n=12 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the open select list's colours are engine constants; the open list's thumb; into view on a row of an open list) added the list's colours as `SelectListView` constants with no token, a thumb that takes no hit, and an into-view on a row that moves only the list. These are neighbouring facts the "stand layout held" reading of the select list will meet.

@history_item n=13 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — added the user-agent select box: the border, padding and background of `input, textarea`, and a single select shows its chosen option alone. These are engine defaults with no design token, ratified as the founder's F2. This is the source of `decision/single-select-box-shows-chosen-label`, pinned by `select_interaction`.

@history_item n=14 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — made the headless stand register the bundled DejaVu Sans for every generic with system fonts off, so layout is the same on the dev host and the CI runner. The stand-layout and font readings of this chunk rest on this. The registration's statement later moved to architecture's Headless stand (see 2026-10-27-readme-and-records).

@history_item n=15 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — refused a `screenshot` `fonts-unavailable` with nothing painted when no `sans-serif` family resolves. It also set the screenshot answer line as `{width,height,png}` uncoloured, with the MCP image block. These are the sources of `decision/no-generic-family-lays-out-sans`'s why and of `contract/escher-session-commands/screenshot`.

@history_item n=16 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Typography: 2 rows) added `decision/unavailable-families-use-default-font`, upstream's default-font fallback ratified as upstream's behaviour and guarded by `font_family_fallback`. This picked typography row joins the stand's bundled-font reading.

@history_item n=17 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — made `escher-session` serve six tasks (counter, flight-booker, timer, crud, cells, load) listed in the usage line and the MCP `start` enum, and gave every boot the bundled font, `boot_cells` and `boot_load` among them. It also added the flight booker's select, the load page and the scrollbar band's thumb drag. This is the source of the six `contract/escher-session-tasks/*` rows, the stand tasks the sweep proves on.

@history_item n=18 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — (the stand's radii, card shadows and card padding; the stand's colours; the stand's font registration; ratified by 2026-10-27-upstream-sync-engine-debts) moved the stand's values out of design-system to layout-templates §Surface: desktop-native → Primary screens and architecture's Headless stand. As a result, "stand layout held" is checked against layout-templates, not against design-system values.

@history_item n=19 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (the stand's colours, type and select padding leave design-system; the flight-type select overrides and the load page's group leave design-system) moved the remaining stand-only lines, unchanged, to layout-templates' `values · desktop-native-screens`. It kept the six-task cli item and the load-wait items here. The stand layout this chunk re-reads lives in layout-templates, and design-system holds only pointer rows.

@history_item n=20 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the command line's output form: one JSON line on stdout per command, fixed lines on stderr, no colour, no ESC byte, no terminal test, and exit statuses 0, 1, 2 and 3. This is the source of `decision/escher-session-no-colour` and the exit-status rows, and the baseline for this chunk's "stderr content-free" reading.

@history_item n=21 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role's stdout: JSON-RPC lines only, no ESC byte, and a tool result's text equal to the JSON line. This is the source of `contract/escher-session-mcp/other-results`. The `mcp_log` stderr measurement this chunk extends drives this role.

@history_item n=22 marker=2026-10-12-self-description
**2026-10-12-self-description** — made the exit statuses read from the schema's `EXIT_STATUSES` table and added `help`. The `contract/escher-session-exit-status/0` and `contract/escher-session-exit-codes/0` rows come from that table.

@history_item n=23 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — added the paged `snapshot` answer line, `{text}` or `{text,part,parts}` uncoloured, which over MCP is the JSON line. This is the source of `contract/escher-session-commands/snapshot`.

@history_item n=24 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — (ratified in the 2026-10-18-engine-debts batch entry) made the harness drive animation from the documents' `VirtualClock`, moved by `tick`, `advance_time` and `advance`. This is the clock that `decision/load-wait-moves-no-clock` says stands still.

@history_item n=25 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — added the rule that the driver's load wait moves no clock, waiting at most `LOAD_WAIT` (5 s wall time) with the `VirtualClock` standing still. This is the source of `decision/load-wait-moves-no-clock`, which the load task's witnesses prove.

@history_item n=26 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the cli surface's graph answer line; the unpaged graph, a pending limit; ratified by 2026-11-15T05-21-42-wrap) added `graph` as one uncoloured JSON line on every carrier, carrying names, closed flags and booleans only, never a value or typed text, and recorded the unpaged limit owned by `res-7cc5bi4k`. These are the sources of `contract/escher-session-commands/graph-*` and `contract/ui-graph-line/content`, the no-value claim this chunk's masks and stderr readings re-prove.

@history_item n=27 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (citation ranges re-read after the graph chunk's code) re-pointed the `escher-session` item's citations to `schema.rs:557-595` and `json.rs:400-462`. `json.rs` has since been split into family children by 2026-11-17-answer-writer-by-family, so these citations are likely stale for this sweep's citation read.

@history_item n=28 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the `actions` answer line on the cli surface; the unpaged `actions` answer) added `actions`, decided by `validate` and `check_target`, carrying ids, verbs, cause words and booleans only, never a value, typed text or NodeId. It recorded the unpaged limit under `res-7cc5bi4k`, and matrix#v030-06 is its witness. These are the sources of `decision/actions-share-the-verbs-checks` and `contract/actions-line/*` and the actions-* rows.

@history_item n=29 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (the coverage role's answer line, endings and refusals; the three stated limits; ratified by 2026-11-15T05-21-42-wrap) added the coverage role: shape `coverage <app> <check>...`, an answer of ids, events and two counts only, the 1,048,576+1-byte read bound, and refusals that hold nothing of the path or the file. It also recorded three limits with their owners. These are the sources of `contract/escher-session-commands/coverage-*`, `contract/coverage-line/content` and `contract/coverage-check-files/*`, and of the stderr-content-free claim for the coverage role.

@history_item n=30 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — (Surface: desktop-native: 2 rows) added `contract/escher-session-commands/layout-answer` and `layout-arguments` for the `layout` verb on every carrier. This moved the premise of CARRY `cry-4cof7vvk`: v030-11 now reads verified, so its witness row names this chunk's checks.

@history_item n=31 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master into rows, each verified against the code. Every picked row's handle and guard dates from this conversion, and a row's `why: pinned by …` names the test the sweep must cite.
