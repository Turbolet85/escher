@task_prose part=session
The session added two stand checks and changed no production code, manifest, workflow, committed check or dependency.
One check reads the six served tasks (counter, flight-booker, timer, crud, cells, load) by the driver's answer digests: snapshot of every part of a paged screen (Cells' 6), lint and actions, both layout modes, at boot and after one acting step by click, type or advance, against one 48-row table; the table read the same on linux x86_64, windows x86_64 and macOS aarch64 in CI.
The other check reads the text extent of sans-serif, serif, monospace, system-ui, an unavailable named family, cursive and fantasy on the stand's bundled DejaVu Sans, both layout modes: every family reads the sans-serif extent.
Measured: the stand's font context registers its one font for four generics only (sans-serif, serif, monospace, system-ui); cursive and fantasy are registered for nothing yet lay out in DejaVu Sans because a one-font collection falls back to that font for any generic.
The fonts-unavailable refusal is kept unchanged; nothing was retired or deferred, and no screenshot is held by either check.
@task_file n=1 path=tests/blitz-tests/tests/stand_cross_os_readings.rs
what: the stand check that holds each served task as the session host holds it and compares its snapshot, lint and actions answer digests at boot and after one step, both layout modes, against one table read the same on every CI OS
@task_file n=2 path=tests/blitz-tests/tests/stand_bundled_font_families.rs
what: the stand check that boots a fixture page through the stand's options and reads each generic family's text extent on the bundled font against the sans-serif extent, both layout modes
@task_item n=1 part=touches
Headless stand pinned viewport: the stand boot's bundled typography with system fonts off now holds on every run path and is read the same on three OSes; a row naming the stand's one bundled font or its guard may take the new cross-OS readings check beside it.
@task_item n=2 part=touches
Session roles share the stand boot: the host, mcp and coverage roles' shared stand boot is now read identically across linux, windows and macOS for all six served tasks; a guard naming only the replayed Counter checks as the cross-OS reading is now narrower than the reach.
@task_item n=3 part=touches
Held instance reads as a fresh boot: the new check holds each served task as the session host holds it and reads it at boot and after one step in both layout modes, a further reader of the held-instance rule.
@task_item n=4 part=touches
Stand lints clean at boot: the lint answer of each served task at boot is now part of the cross-OS digest table.
@task_item n=5 part=touches
escher-session served tasks contracts (counter, flight-booker, timer, crud, cells, load): each served task is now read by answer digest the same on three OSes; their guard cells may take the new check.
@task_item n=6 part=touches
escher-session snapshot --part contract: every part of a paged screen (Cells' 6 parts) is read by digest in the cross-OS table.
@task_item n=7 part=touches
escher-session snapshot, lint and actions answer contracts: their answers are now read by digest and compared across OSes; no answer shape changed.
@task_item n=8 part=touches
escher-session refusals fonts-unavailable: kept unchanged, its checks green; a row stating it holds as before is unchanged, a row tying it to the bundled set stays true.
@task_item n=9 part=touches
Load page alone has a network provider: the load page is held through its own stand boot in the cross-OS check, a reader of that boot, no change to the rule.
@task_item n=10 part=touches
Flight type select reads its two options at boot and CRUD list as one keyboard region: their boot screens are now within the cross-OS digest table, an indirect reader, no change to the rule.
@task_item n=11 part=touches
Any row stating the stand's bundled font is registered for every generic family: false for cursive and fantasy, which lay out in DejaVu Sans by fallback to the collection's only font, not by registration.
@task_fact n=1 slug=surface-desktop-native
fact: The stand's six served tasks read the same on linux x86_64, windows x86_64 and macOS aarch64 — snapshot of every part, lint and actions, both layout modes, at boot and after one step — compared by the driver's answer digests in a stand check that holds no screenshot.
@task_fact n=2 slug=surface-desktop-native
fact: Every generic family, cursive and fantasy and an unavailable named family included, lays out in the stand's one bundled font DejaVu Sans by fallback to the only font loaded; only sans-serif, serif, monospace and system-ui are registered to it.
@task_item n=1 part=untouched
ShellProvider window chrome controls (drag, maximize, minimize, close, decorations): no window chrome reached.
@task_item n=2 part=untouched
Stand stable element ids: no id added, renamed or removed; the checks read ids, never change them.
@task_item n=3 part=untouched
Stand accent colour, ids and accessible names by attributes alone, every stand control keyed, the fixed document skeleton with its main mount: no stand markup or style changed.
@task_item n=4 part=untouched
agent-run and cold-agent command contracts and their decisions: no script, exit code, log or record dir changed.
@task_item n=5 part=untouched
multi_doc and paint_bench command lines: no argument, environment variable or output changed.
@task_item n=6 part=untouched
escher-session exit codes, help, graph, screenshot, recording, coverage role and host log contracts, and the MCP server's protocol revisions, tools list, result bound and stdio: no command, answer shape or bound changed.
@task_item n=7 part=untouched
Scroll into view, visible region through clipping boxes and open list rows' bounds: no geometry or scroll behaviour changed.
@task_item n=8 part=untouched
The mobile-native and web-spa surfaces, the decisions log and the arbitrary index.html templates absence: nothing reached them.
