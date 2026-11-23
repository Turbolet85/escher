@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand: it skips Home and mounts one task in TaskShell through `task_in_shell` at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans. This is the origin of `decision/headless-stand-pinned-viewport`. A canvas shot measured across hosts rests on this fixed viewport and font, so any cross-host difference has to be read against these pinned values and not against a viewport that varies by host.

@history_item n=2 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — re-pointed the harness-defaults citations to the `Default` impl that holds 800x600, scale 1, light, and to the viewport built from them. The claim itself did not change. These are the values a stand canvas fixture boots with on every CI OS. If the chunk's check touches the harness, the citations may need re-pointing again.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — named the session host as the third consumer of the `task_in_shell` mount. It adds no markup, id, class, style, wrapper or order, and a held instance reads as a fresh boot, with the mount and pinned viewport holding after commands. This is the origin of `decision/held-instance-reads-as-fresh-boot` (guard `stand_session_fresh`) and of `decision/session-roles-share-stand-boot`. A canvas fixture booted alone has to keep these invariants.

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — made `escher-session` the driver's command line, with the host role `serve <task>` as the third consumer of the stand mount and a client command booting nothing. The layout statement stayed unchanged: no markup added, and a held instance reads as a fresh boot. This is the precedent for the scope's boundary that a shot's pixels cross only as they already do, with no new verb, argument, carrier, exit or refusal.

@history_item n=5 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role as a further consumer of the TaskShell mount, booting a task in process through the same boot. This is part of `decision/session-roles-share-stand-boot`. A shot taken over MCP on each CI host comes through that same boot, so a canvas shot's cross-host reading should not depend on the carrier.

@history_item n=6 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added the `screenshot` verb and its answer line `{"width","height","png"}`, either the whole screen or, with `--id`, an element's visible part. It added the `fonts-unavailable` refusal (exit 1) and the `mcp` image block followed by a text block, with a refusal answered by a single text block with `isError`. It bounded a result holding an image at 1,048,576 bytes by its whole written form. The founder ratified it (fork 1, 2026-10-12). This is the origin of four picked cli rows: `contract/escher-session-screenshot/answer`, `contract/escher-session-refusals/fonts-unavailable`, `contract/escher-session-mcp/screenshot-result` and `contract/escher-session-mcp/result-bound`. The chunk measures this shot's PNG bytes per host and must not widen any of them.

@history_item n=7 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `--record <dir>`, which keeps an owner-only replayable check holding the calls and no screen reading. The founder ratified it (F1 A, F2 B). This is a precedent for the scope's boundary that the kept check keeps no pixels.

@history_item n=8 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check now holds each answer, with a shot's answer kept as `{"width":…,"height":…}` without its `png`. The founder ratified it (F3 B). This sets the rule for a canvas-shot check: a committed table may hold digests or dimensions, but no PNG, consistent with the scope's "keeps no pixels".

@history_item n=9 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part, `escher-session lint`) — made `lint` refused `fonts-unavailable`, exit 1, on an app with no font, the same refusal `screenshot` gives. The refusal is shared across both verbs, so the bundled-font boot that the canvas text shot depends on must keep it unchanged.

@history_item n=10 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the headless stand gained further boots (`boot_cells`, `boot_load` in a TaskShell of its own), and the host and `mcp` roles boot each served task by its own boot. This is the precedent for stand fixtures booted alone beside the lean tasks, as the canvas fixture is (`stand_canvas_shot` and the like). It also put the `load-fetch` accent shades `#3a5ce5` / `#2a4cd3` / `#1f3fc1` on this surface.

@history_item n=11 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — landed the stand colour rows here (PROVISIONAL at the time): the accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) on `#f5f5f5` with text `#1a1a1a`. White text reads 5.47 against SC 1.4.3, while `#4a6cf7` reads 4.39, a fail, measured by `stand_contrast`. This is the origin of `decision/stand-accent-colour`. A canvas fixture that paints accent or text pixels should use these declared values, and the pixel readings it compares across hosts are those colours.

@history_item n=12 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (F2 ratified, and the ten further stand lines) — the founder ratified the stand colour, depth, spacing and radius rows and the PROVISIONAL gap marker was dropped. It added the stand's type line: sans-serif, the home title 700 at 36px over a 14px base. The accent row the chunk rests on stands ratified.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`stand_contrast` over the load page and Cells; the Cells grid outline) — `stand_contrast` now reads the load page and Cells too, in both layout modes, and the `cells-grid` focus outline uses the stand accent `#3a5ce5`. This is the precedent that the accent-colour guard extends to stand fixtures other than the lean tasks, looped over both layout modes. A canvas fixture declaring the accent falls under `decision/stand-accent-colour`'s "in every task CSS that declares it".

@history_item n=14 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role a further consumer of the TaskShell mount) — added the `coverage` role, which boots the kept checks' app once per check through the same boot and adds no markup, with a held instance still reading as a fresh boot and the mount and pinned viewport holding in both layout modes. This made `decision/session-roles-share-stand-boot` name host, mcp and coverage. A canvas fixture or check must not give any role a different boot.

@history_item n=15 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified: the coverage role a further consumer of the TaskShell mount, and `escher-session`'s coverage role) — the founder's own word replaced the delegate confirm, and the entries' text stands as written. The session-roles-share-stand-boot rule the chunk rests on is ratified, not PROVISIONAL.

@history_item n=16 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into rows (121 rows), with the founder's approval. This created the handles the picked rows carry, such as `decision/headless-stand-pinned-viewport` and `contract/escher-session-screenshot/answer`, from the prose the entries above amended. From then on, any change this chunk makes to them is recorded as `~ {handle}` row edits.
