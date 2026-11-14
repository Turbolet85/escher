@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the Typography Loading bullet: the seven_guis headless stand registers the bundled DejaVu Sans for every generic with system fonts off on native, `build_single_font_ctx(DEJAVU_SANS)` through `HarnessOptions.font_ctx`, the woff2 decoded by seven_guis' native `woff` feature; the WASM bullet stood. Why it matters: the precedent this chunk widens from the stand to every run path (served tasks, fixtures, harness-built documents naming no font context), and its stated purpose (the same layout on dev host and CI runner) is this chunk's cross-OS claim.

@history_item n=2 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added the Loading rule that a driver `screenshot` of an app whose font context resolves no `sans-serif` family is refused `fonts-unavailable` with nothing painted (text never painted skipped; the stand's bundled font passes), plus the shot's answer line and MCP image block on the cli surface. Why it matters: the origin of decision/screenshot-never-skips-text and both fonts-unavailable contracts the scope keeps unchanged; with system fonts off, the bundled set must still resolve the `sans-serif` generic or every shot is refused.

@history_item n=3 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — (Typography item) the bundled DejaVu Sans for every stand boot, `boot_cells` and `boot_load` among them; also set the six served tasks on `escher-session`. Why it matters: shows the bundled font was already extended boot by boot on the stand; the driver session's served tasks are among the paths this chunk must cover.

@history_item n=4 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — (the stand's font registration leaves for architecture's Headless stand, PROVISIONAL) the stand's `build_single_font_ctx(DEJAVU_SANS)` registration was removed from typography.md's values and is now held in architecture §Standard Contracts → Headless stand beside `font_ctx()`; the WASM bundled-font item and the custom `FontContext` item stayed. Why it matters: the run's font registration is an architecture contract, not a design value, so this chunk's bundling of every run path lands in architecture's Headless stand; design rows here are only decision/wasm-bundled-font-context, contract/native-config/font-context and the fonts-unavailable rows.

@history_item n=5 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — (F2's stand-row move ratified; the stand's colours, type and select padding leave design-system) ratified by the founder the move of the stand's font registration to architecture; removed the seven_guis typography row (sans-serif, base 14px, home title) and narrowed the web-spa Platform line to todomvc filling the window with a canvas. Why it matters: confirms stand typography is not a design-system value, and the web-spa overview (overview/surface-web-spa) names todomvc alone, the remote/browser pages the scope's boundary keeps off bundled fonts.

@history_item n=6 marker=2026-11-13-upstream-sync-determinism-and-exploration
**2026-11-13-upstream-sync-determinism-and-exploration** — (Typography: 2 rows) `+ decision/unavailable-families-use-default-font`: when none of an element's listed families is available text lays out in the engine's default font, font-relative units measuring against it, ratified as upstream's (guard `font_family_fallback`). Why it matters: the rule that decides what `serif`, `monospace` or an unknown family resolve to once only DejaVu Sans is registered with system fonts off; this chunk's generic-family claim rests on it and must not contradict it.

@history_item n=7 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows in every area (364 records → 137 rows), each verified against the code, with divergences recorded as the code shows. Why it matters: the picked rows (decision/test-engine-with-system-fonts, decision/tests-built-with-system-fonts, decision/no-generic-family-lays-out-sans, the native-config font-context contracts) took their current wording here; the two system-fonts test rows bind blitz-tests to system fonts, which this chunk's fonts-off run paths must not break.

@history_item n=8 marker=2026-10-13-upstream-sync-polish-and-ship
**2026-10-13-upstream-sync-polish-and-ship** — the user-agent-sheet line: every document gets the default sheet once, through `DocumentConfig::default()`, upstream #1189's path. Why it matters: decision/default-ua-sheet-via-default-config is picked; a harness-built document given a bundled font context must still take its config's defaults, so the font context is set without replacing the default config path.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the `escher-session` output form: one JSON line per command (answer, refusal or session error), uncoloured, no ESC byte, no terminal test, exit status 0 accepted · 1 refused · 2 usage · 3 session error. Why it matters: the origin of the picked cli rows (stdout command line output, exit 1 refused, no colour, agent read surface); `fonts-unavailable` stays a status-1 refusal on that line, and the scope adds no field or cause to it.

@history_item n=10 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role's stdout: JSON-RPC lines only, uncoloured, no ESC byte in any reply, no terminal test. Why it matters: decision/driver-output-for-agents covers the mcp role too; readings compared across OSes travel through these same uncoloured lines.

@history_item n=11 marker=2026-10-12-self-description
**2026-10-12-self-description** — the exit status's four classes read from the schema's `EXIT_STATUSES` table that `help` serves. Why it matters: contract/escher-session-exit-status/1 and exit-codes/1 are read from that table; the scope's no-new-cause boundary keeps it unmoved.

@history_item n=12 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `record-dir-not-private` and `recording-failed` to the exit-3 session-error class with no new exit. Why it matters: precedent that new failure text stays inside the four statuses; a font-loading failure on a run path must map to the existing classes, not a new one.

@history_item n=13 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `escher-session snapshot [--part <part>]` answers `{"text":…}` or `{"text":…,"part":n,"parts":m}`, uncoloured, status 0; the mcp result is that JSON line. Why it matters: contract/escher-session-commands/snapshot is picked; the snapshot text and its paging (which depends on text extents) are among the readings every CI OS must read the same way.

@history_item n=14 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (what the tree, the snapshot and a shot read of an open list) a row of an open list carries its own `bounds` in the snapshot and a `screenshot` of a row crops to it; (a group header heavier than a row) a group label paints with `synthesize_bold` through `stroke_text_as`, hinting tied to `FONT_EMBOLDEN_ENABLED`, a first attempt red on macOS alone; (row cap, placement and row height) a list row's height is the select's font on one line via `select_row_layout`. Why it matters: contract/snapshot/select-list-row-bounds is picked and its bounds depend on font metrics; the macOS-only failure is a precedent of per-OS text rendering differences, so cross-OS comparison must use readings, not shot bytes.
