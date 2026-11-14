@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand's TaskShell mount at the pinned 800 × 600, scale 1.0, Light viewport "with the bundled DejaVu Sans" (the origin of decision/headless-stand-pinned-viewport). This chunk widens that one bundled font from the stand to every run path, so it extends this row and must keep its viewport values unchanged.

@history_item n=2 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — re-pointed the IA note on the test-harness defaults (the `Default` impl holding 800x600, scale 1, light, and the viewport built from them). This chunk's likely edit point is `HarnessOptions::font_ctx` in that same `Default` (`packages/blitz-test-harness/src/harness.rs`), so those citations will move again and must be re-pointed by measurement, not by a mechanical shift.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — named the session host as a third consumer of the `task_in_shell` mount, booting through `seven_guis::stand` only and adding no markup, with a held instance reading as a fresh boot. This is the root of decision/session-roles-share-stand-boot: bundling fonts on the stand boot reaches every role only while they all share that boot.

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the endings that contract/escher-session-exit-codes/0 and /1 hold: accepted is one uncoloured JSON line with exit 0, refused is exit 1, usage 2, session error 3. A `fonts-unavailable` refusal must keep using the exit-1 ending, and the scope adds no new ending or cause.

@history_item n=5 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — made the `mcp` role a further consumer of the TaskShell mount, booting a task in process through the same boot. That boot is one of the run paths whose fonts this chunk must bundle (decision/session-roles-share-stand-boot).

@history_item n=6 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added `screenshot`, which refuses an app with no font it can paint text with as `fonts-unavailable` (exit 1), and the MCP shot result (image block then text block, `isError`). These are the origin of contract/escher-session-refusals/fonts-unavailable and contract/escher-session-mcp/screenshot-result. The scope keeps this refusal and its text unchanged, and the cross-OS proof never compares a shot's PNG bytes.

@history_item n=7 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `--record <dir>`, which keeps a session as an owner-only replayable check (`check-{n}.json`). One option for the cross-OS proof is a committed reading that each OS replays, and this register is a candidate carrier for it. The scope forbids any new carrier, field or argument, and the boundary widening was the founder's.

@history_item n=8 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — made a kept check hold each call's answer and its digest, with a shot kept as `{"width","height"}` without `png`, replayed by `Check::replay`. This is the existing mechanism that compares readings rather than image bytes, a precedent for how this chunk's per-OS comparison can be held (P3/P4) without a new field.

@history_item n=9 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — two sub-entries. (1) The stand reads clean to the in-process visual lint: 0 findings at boot, and `crud-person-14` reads `clipped` (visible 367 × 11.203125) after twelve Creates. (2) The gated part added the `lint` verb, which answers `{"findings":[…]}` and refuses an app with no font as `fonts-unavailable`, exit 1. These are the origins of decision/stand-lints-clean-at-boot, contract/escher-session-lint/answer and the lint side of the fonts-unavailable refusal. The lint findings are among the readings that every CI OS must read the same way.

@history_item n=10 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — dropped the PROVISIONAL "in process only" mark from the stand's visual lint note, on the founder's word. decision/stand-lints-clean-at-boot now stands ratified, and this chunk must keep it true under the bundled fonts.

@history_item n=11 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — added `snapshot --part`, which answers `{"text"}` or `{"text","part","parts"}` (contract/escher-session-snapshot/answer). The snapshot text is the first reading the cross-OS proof compares. How a screen splits into parts may depend on text measurement, so it must not drift between hosts.

@history_item n=12 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — recorded measured stand bounds with fractional figures that depend on the font: `crud-list` at (24, 112.796875, 369 × 419.203125), `task-body` (0, 46, 800 × 554), and the last row `off-screen` after twelve Creates. These are the kind of bounds and text-extent readings that must match across Linux, macOS and Windows once fonts are bundled. They are a trap if any host still shapes text with a system font.

@history_item n=13 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — measured Home at the stand's viewport: `#home` is 600 high, holds six of seven cards, and Cells lies at 615 to 684, shown only once scrolled. These readings depend on text metrics, and a standing check went red when the layout disagreed with the master. Bundled fonts must keep such measured figures, or the masters must restate them.

@history_item n=14 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — set the six served tasks (`counter`, `flight-booker`, `timer`, `crud`, `cells`, `load`) booted by the host and `mcp` roles. Cells boots by `boot_cells` and the load page by `boot_load`. The entry also made the flight booker's mode a select, `flight-type`, and pinned the lint readings at boot. These are the origins of every contract/escher-session-served-tasks/* row and decision/flight-type-select-boot-reading. Each boot path (lean, `boot_cells`, `boot_load`) is a run path that must get the bundled font context.

@history_item n=15 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — moved the stand's colour rows onto this surface, including white text on the four accent controls (14–16 px, weight 500), whose contrast `stand_contrast` reads as 5.47 / 6.83 / 8.27. These are glyph-dependent readings that rest on the stand's font, so bundling must leave them unchanged.

@history_item n=16 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — two sub-entries bear on this chunk. (1) The ten further stand lines added the stand's type line: sans-serif, the home title 700 at 36px over a 14px base, and the select at 15px, weight 400. The generic family `sans-serif` is exactly what the scope says must resolve to the bundled DejaVu Sans and not to the host font. (2) F2 was ratified, so the stand's value rows stand without the PROVISIONAL marker.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — two sub-entries. (1) The flight booker's boot lint now reads `[0, 0, 0, 0]` in both layout modes, because a closed select's options are skipped in `overlaps`, and the clause owned by a residual is retired. (2) The citations into `stand_lint.rs` and `stand_contrast.rs` were re-pointed. Together these are the current form of decision/stand-lints-clean-at-boot and its guard `stand_lint`, which this chunk's bundled fonts must keep at zero on every OS.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — four sub-entries. (1) The engine's open select list: each row reads its own `bounds` while open and reads back at the select's client rect after a close (the origin of decision/open-list-rows-own-bounds), in every document, remote pages included. (2) The lint's `overlap` exclusion moved to `Entry.in_select_box`. (3) An open list's rows are the exception to the scrolling boxes rule. (4) Citations were re-pointed. The open list's painted rows and their bounds are text-measured readings that must match across OSes. The list's reach into remote pages also touches the scope's boundary that a remote page's or the browser app's font must not change.

@history_item n=19 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — two sub-entries. (1) Added the `coverage` role as a further consumer of the TaskShell mount, booting each check's app through the same boot `main` hands to the command line, and with no markup added. (2) Its CLI shape: `coverage <app> <check>...`, which replays checks on fresh boots. decision/session-roles-share-stand-boot names this role (host, mcp, coverage), so the coverage boot must also get the bundled fonts. The role's PROVISIONAL status (delegate overseer) still awaits the founder's own word.

@history_item n=20 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into rows (121 rows), verified against the code. This created every row picked for this chunk and the surfaces' NOT YET MEASURED and decisions-log markers. Any amendment this chunk makes must change rows by handle (`+`/`~`/`−`).
