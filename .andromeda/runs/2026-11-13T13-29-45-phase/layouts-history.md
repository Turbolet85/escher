@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand mounting one task in TaskShell at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans. This is the origin of decision/headless-stand-pinned-viewport. A take that moves Taffy, Stylo or Parley can move the numbers measured at that viewport, and they are re-measured, never loosened.

@history_item n=2 marker=2026-10-06-upstream-sync-element-identity
**2026-10-06-upstream-sync-element-identity** — an earlier upstream merge re-pointed 4 `file:line` citations into merged upstream files using the merge's measured line map, with no claim text changed. This is the precedent for how this take handles the layouts doc: re-point citations, keep claims.

@history_item n=3 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — gave the lean tasks' controls author ids as attributes alone, with no class, style, wrapper or order, and no CSS selecting by them. This is the origin of the contract/stand-element-ids rows and decision/stand-ids-as-attributes-alone. "The stable ids and the snapshot" is one of the intents every conflict resolution must keep.

@history_item n=4 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — CRUD rows are now keyed by `{person.id}` instead of their index, so a row's id follows its person under filter, Create and Delete. This stands behind contract/stand-element-ids/crud-person-id. It is a trap for any take that touches Dioxus keying or mutation order in dioxus-native-dom.

@history_item n=5 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — named the six stand inputs by attributes alone (`for`, and `aria-label` "Departure date" / "Return date"). This backs decision/stand-a11y-by-attributes-alone and the crud-filter, crud-name, crud-surname, timer-duration, flight-start and flight-return-date rows. The take's AccessKit refresh (`be60597f6`) must keep these names.

@history_item n=6 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — gave CRUD rows the author id `crud-person-{person.id}` and Home's cards the `task-card-{slug}` ids, as attributes only. This is the origin of the task-card-* rows and crud-person-id, on the founder's rule that every actionable element reads an author key.

@history_item n=7 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — made the session host a third consumer of the `task_in_shell` mount. It adds no markup, a held instance reads as a fresh boot, and the mount and pinned viewport hold after commands. This is the origin of decision/held-instance-reads-as-fresh-boot and decision/session-roles-share-stand-boot (guard stand_session_fresh).

@history_item n=8 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — an into-view scroll now moves every holding scroller, then the viewport, and `visible_region` reads through clipping boxes. The founder ratified both for every document, 2026-10-07, with the CRUD list as the stand's nested scroller. This is the origin of decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes. It is an engine surface escher changed, so a take reaching scrolling must keep it.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — made `escher-session` the driver's command line: `<verb> … --session <dir>`, four endings, exit codes 0/1/2/3. The host role is now `serve`, and the third consumer of the mount is still layout-neutral. This backs the cli rows and decision/no-caller-input-echoed ("no ending writes back anything the caller typed").

@history_item n=10 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — after the engine fixes to scrolled-box bounds and hit testing, the scrolled `crud-list` reads the same four figures. Home is a scrolling box with Cells below the fold at 800 × 600. "The hit stops and the scrolled-box bounds" is a named merge intent, and a take touching hit testing (`0980cce5f`'s `hit_inner` conflict) must keep these readings.

@history_item n=11 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — the stand now states roles and states by attributes alone: `aria-invalid` on the date inputs, `role="listbox"` on crud-list, and `role="option"`/`aria-selected` on its rows, with the styling classes kept. This backs decision/stand-a11y-by-attributes-alone and the accessibility state reads the take must preserve.

@history_item n=12 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role: an MCP server over stdio, local, with no listener or auth surface, echoing a request's admitted id. It also consumes the TaskShell mount and adds no markup. This is the origin of decision/mcp-server-local-stdio and contract/escher-session-mcp/stdio. It matters because the take's DevTools/CDP may open a socket, a boundary widening to be listed at P4.

@history_item n=13 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added `screenshot`, which answers `{"width","height","png"}`. Over MCP it gives an image block then a text block, bounded at 1,048,576 bytes. This is the origin of contract/escher-session-screenshot/answer and contract/escher-session-mcp/result-bound and screenshot-result. A vello/AnyRender move can change the pixels, which are re-measured, not loosened.

@history_item n=14 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` takes `tabindex="0"`, and ArrowDown, ArrowUp, Home and End select and fill the name fields. Focus stays on the list, no `aria-activedescendant` is read, and the Tab sequence has six stops. This is the origin of decision/crud-list-keyboard-region. The keyboard model is a merge intent the take must keep.

@history_item n=15 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — named `crud-list` "People" by `aria-label` alone. This backs contract/stand-element-ids/crud-list. The AccessKit refresh in the take must keep the name in the tree and the snapshot.

@history_item n=16 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — gave the non-lean tasks author ids (temp-*, circle-*, cell-*, cell-input), with `unkeyed_actionable()` at 0 on all seven tasks. This is the origin of decision/every-stand-control-keyed and the circle/temp/cell id rows.

@history_item n=17 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** — an upstream take (`23b9b26a9`, "Delete the bump crate") removed the `bump` line from §Surface: cli. This is a precedent: when a take removes or adds a CLI surface, the cli area is amended, and the paint_bench rows sit in that area.

@history_item n=18 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `--record <dir>`: owner-only `check-{n}.json` files written 0600 in a 0700 dir, with session errors `record-dir-not-private` and `recording-failed` (exit 3). The founder ratified it as a boundary widening. This is the origin of decision/session-recording-replayable-check and the contract/escher-session-record rows.

@history_item n=19 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorded check now keeps each call's verb, arguments, acceptance, answer digest and masked answer, replayed by `Check::replay`. This backs decision/session-recording-replayable-check. Answer digests are a trap: snapshot or lint numbers moved by the take change the kept answers.

@history_item n=20 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (both entries: the in-process note, then the gated `lint` verb) — the stand reads 0 lint findings at boot in both layout modes, `crud-person-14` reads `clipped` after twelve Creates, and `lint` answers `{"findings"}`. This is the origin of decision/stand-lints-clean-at-boot and contract/escher-session-lint/answer. Layout moved by a Taffy/Stylo/Parley move can surface findings.

@history_item n=21 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — each acting tool's `settled` field states that a load in flight is waited for up to 5 s. This bears on decision/load-page-only-network, where a driver click on load-fetch waits out the 200 ms answer, and on any lossy page decoding or load change in the take.

@history_item n=22 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` answers `{"text"}`, or `{"text","part","parts"}` for a screen read in parts. This is the origin of contract/escher-session-snapshot/answer. A take moving layout or text can change the snapshot's size and part count, which are re-measured and recorded.

@history_item n=23 marker=2026-10-24-upstream-sync-polish-and-ship
**2026-10-24-upstream-sync-polish-and-ship** — the ten-commit cherry-pick sync ("Десять коммитов (Рекомендую)") listed upstream's `multi_doc` example among the CLI's primary screens. This is a precedent for this take: new upstream command-line surfaces (here `parley_code_editor`, devtools, the WPT runner) are added to §Surface: cli when taken.

@history_item n=24 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the stand now serves six screens, including Cells and the load page through `boot_load`, with `LoadNet` answering after 200 ms. The flight booker's mode became a select with ids `flight-type`, `flight-type-one-way` and `flight-type-return` (the toggle buttons with `aria-pressed` were retired). This is the origin of the flight-type rows, the load-* rows and decision/load-page-only-network.

@history_item n=25 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (citations into the files the take moved, and the ten further stand lines) — the previous upstream take re-pointed 4 citations into `apps/browser` and blitz-shell, and rewrote 2 paths after blitz-vibey-script was split into modules. The same wrap stated that `LoadNet` answers from a thread of its own, so a driver click on load-fetch waits it out. This is the direct precedent for re-pointing citations and rewriting paths when upstream moves code between files, and it backs decision/load-page-only-network.

@history_item n=26 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the citations re-pointed; load-fetch's keys; the Cells grid; the boot lint reading) — re-pointed citations into `scrolling.rs`, `dioxus_document.rs` and `stand_lint.rs`, and made `load-fetch` take Enter and Space. `cells-grid` became one Tab stop and a keyboard region named "Cells" (founder's F13). Lint `at_boot` moved to `[0, 0, 0, 0]`, with a closed select's options skipped (founder's F9). These back contract/stand-element-ids/load-fetch and cells-grid, and decision/stand-lints-clean-at-boot.

@history_item n=27 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the mode list; citations re-pointed; the scrolling boxes exception; `in_select_box`) — the engine paints an open select list in every document. While it is open, rows read their own bounds, and after a close they read back at the select's client rect. A row is seen through the list's scrollport alone, and its scroll-into-view writes the list's `scroll_y`. The lint skips `Entry.in_select_box`. This is the origin of decision/open-list-rows-own-bounds and of the exceptions on decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes, while decision/flight-type-select-boot-reading was kept. "The select model and its list" is a named merge intent.

@history_item n=28 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the screens' citations; the graph verb) — re-pointed citations into `dioxus_document.rs` and `schema.rs` after the graph registrations, and added the `graph` verb answering `{"nodes","states"}` (PROVISIONAL, delegate overseer). This is the origin of contract/escher-session-graph/answer. dioxus-native-dom is escher's most-modified crate and a merge surface for the take.

@history_item n=29 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the tab-order citations; the actions verb) — added `actions` as the tenth instance verb, an unpaged `{"nodes"}` that never carries a value, typed text or NodeId, and re-pointed the `common/mod.rs` citations. This is the origin of contract/escher-session-actions/answer. Upstream's new blitz-tests files run under the fork's one-file, one-target rule beside this shared module.

@history_item n=30 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage consumer of the mount; the coverage role) — the `coverage` role replays each kept check on a fresh boot through the same boot, adding no markup (PROVISIONAL, delegate overseer). It answers `{"unexercised","checks","held"}`, and a failed boot ends with exit 3. This backs decision/session-roles-share-stand-boot and contract/escher-session-coverage/answer.

@history_item n=31 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master's text into the 121 rows this chunk is bound by, each verified against the code at edd0cfaea7f0 and re-read at f84ccd9f58bc, with the fates approved by the founder. The picked rows rest on this conversion. A take that moves code these rows describe must be read against the row text, not the older prose.
