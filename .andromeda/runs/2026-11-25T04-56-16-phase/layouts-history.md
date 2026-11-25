@history_item n=1 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the lean tasks' controls got author `id`s as their stable element ids, attributes only, with no markup moved. This is the stable-id exit that a virtual node's `{owner}///{key}` id now rides (F3 A). The canvas owner's id is the prefix of the virtual id.

@history_item n=2 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — stand inputs are named by attributes alone, so every input carries a non-empty name in the tree. This set the accessible-name exit that a virtual node's author-written name now rides. It is the first canvas-side text on that carrier.

@history_item n=3 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the scrolling-boxes note: `visible_region` reads what an element can be seen through, and a row outside it is refused `off-screen`. This is the root of `decision/visible-region-through-clipping-boxes`. A virtual node's `off-screen` refusal is its region's centre against the canvas's `visible_region`.

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session` became the driver's command line, with its four endings (accepted 0 · refused 1 · usage 2 · session error 3). The virtual node's `click` and its refusals (`stale`, `not-found`, `disabled`, `off-screen`, `covered`) cross these endings unchanged, with no new ending (F3 A: no new cause or session error).

@history_item n=5 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — after the bounds reader and hit fix, a driver `click` lands at the named element's centre, and a standing check went red once the hit was fixed. A click on a virtual node is likewise a pointer at its region's centre that stops at the canvas element. The trap: what the centre hit reads, and when, follows the fixed hit path.

@history_item n=6 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — the stand states the states a snapshot reads by attributes alone (`aria-pressed`, `aria-invalid`, `aria-selected`). This is the precedent for the snapshot's state field. Under F8 A a virtual node's declared `disabled` is written on its tree node and read as the snapshot's `enabled`.

@history_item n=7 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role and the scoping of «no ending writes back anything the caller typed» to the commands. This is the origin of `decision/no-caller-input-echoed`. A virtual id the caller names in `click` is the author's key and not caller text written back, and the MCP carrier is one of F3 A's exits.

@history_item n=8 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot --id` shoots the element's visible part, and an element shot of no pixel is refused `off-screen`. The MCP image block precedes a text block. Under F10 B, `screenshot --id` on a virtual id reads `not-found`, a stated limit, which leaves the `contract/escher-session-screenshot/answer` and `contract/escher-session-mcp/screenshot-result` rows untouched.

@history_item n=9 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — Circle Drawer's `circle-canvas` and the other non-lean task ids landed, with `unkeyed_actionable()` reading 0. `circle-canvas` is the canvas whose virtual ids would be `circle-canvas///{key}` («Circle Drawer's circles as virtual nodes», out of this entry). The fixture canvas a test declares is this entry's proof surface instead.

@history_item n=10 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe runs the driver's own MCP server, and the agent's check stays in `target/cold-agent/check.json`, never printed. A cold-agent live run's model provider is one of F3 A's exits for a virtual node's id and name. `contract/cold-agent-cli/stdout` (JSON lines only) must hold.

@history_item n=11 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` keeps a session as one owner-only replayable check (`check-{n}.json` 0600 in a 0700 dir). This is the origin of `decision/session-recording-replayable-check` and `contract/escher-session-record/check-file`. A kept check at rest now holds a virtual id and name (F3 A).

@history_item n=12 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — a kept check holds, per call, the verb, arguments, acceptance, digest and masked answer, read back and replayed by `Check::replay`. A `click` on a virtual id, and its diff naming virtual nodes, are recorded and must replay, so the digest of a snapshot or diff answer now changes with virtual nodes present.

@history_item n=13 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads clean to the in-process visual lint at boot, and the lint's `clipped` agrees with the driver's `off-screen`. This is the origin of `decision/stand-lints-clean-at-boot`. The lint reads `DioxusDocument::snapshot`, so a virtual node put in the snapshot reaches it unless excluded (F9 A, `lint.rs`).

@history_item n=14 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part, amd-oeiyf7qg) — the `lint` verb answers `{"findings":[…]}` on every carrier. This is the origin of `contract/escher-session-lint/answer`. Virtual nodes are held out of the findings under F9 A, so the answer row stands unchanged.

@history_item n=15 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` answers `{"text"}` or `{"text","part","parts"}`, and a part past the last gives empty text. This is the origin of `contract/escher-session-snapshot/answer`. Virtual nodes now enter the snapshot text and its paging, inside the same answer form.

@history_item n=16 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the cold-agent live server runs `mcp --record` into `target/cold-agent/record`, owner-only and removed by `cleanup`, and the lint at boot was re-pinned as measured. This is the origin of `contract/cold-agent-cli/record-dir` and the boot-lint list. A virtual id reaches kept checks there, and a snapshot change that shifts the boot readings must be re-measured.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (amd-dde6irde) — the lint's `overlaps` skips a closed select's options, which the snapshot gives the select's client rect. It reads the element and never the role, so CRUD's `role="option"` rows are not taken as a select's. This is the precedent for a lint excluding snapshot entries that are not real layout boxes. F9 A's virtual-node exclusion in `lint.rs` follows the same pattern and must not catch DOM nodes by role.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-qpacbn5d) — an open select list's rows are painted (not layout boxes) yet each reads its own `bounds` in the snapshot and diff. A row's own `bounds` was ruled a closed set inside the ratified crossing by the 2026-10-10 field rule. This is the nearest precedent for non-DOM painted regions entering the snapshot and diff: virtual node bounds and role are likewise closed values inside ratified carriers (F3 A).

@history_item n=19 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-yt52bd3c) — added the exception to `decision/visible-region-through-clipping-boxes`: an open list's row is seen through the list's scrollport alone. Virtual nodes use the canvas's `visible_region` and add no new exception. If the plan changed `visible_region` for a virtual node, it would amend this row.

@history_item n=20 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (amd-2cfi74yl) — `Entry.in_closed_select` was retired for `Entry.in_select_box`, a lint flag on snapshot entries that are not ordinary elements. This is the closest code precedent for F9 A's exclusion of virtual nodes in `lint.rs`, and `at_boot` `[0, 0, 0, 0]` must stand.

@history_item n=21 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (amd-fvbepe6z) — the `graph` verb answers `{"nodes","states"}` of ids, component names and closed words only. This is the origin of `contract/escher-session-graph/answer`. `graph()` reads the snapshot (`graph.rs:139`), so F9 A's exclusion is this entry's code. The carried mutation control keeps the graph half of «a declaring canvas puts nothing on the snapshot or graph» standing.

@history_item n=22 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (amd-sn2yc5zx) — the `actions` verb answers `{"nodes"}` of id, owner, keyed and verbs, never a `NodeId`. This is the origin of `contract/escher-session-actions/answer`. Virtual nodes stay out of actions until «Virtual nodes in every read», and a virtual node's `clickable` reads false (F12 A). The answer row is unchanged.

@history_item n=23 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-c7r24qdb) — the `coverage` role replays kept checks on fresh boots, each read bounded at 1,048,576 + 1 bytes and refused `malformed` without echoing the path. This is the origin of `contract/escher-session-coverage/answer` and `contract/escher-session-coverage/check-bound`. A kept `click` on a virtual id must replay there, and coverage is held out of this entry (graph-based).

@history_item n=24 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (amd-uzo37rio) — the coverage role is a further consumer of the TaskShell mount, adding no markup. A replay of a virtual `click` boots through the same mount.

@history_item n=25 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (121), each verified against the code. Every picked row's current text dates from here unless a later entry changed it. A new or changed row for this chunk lands as `+`/`~` by handle.

@history_item n=26 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the graph verb and the coverage role (both were PROVISIONAL on a delegate). This is a precedent for delegate-word rows awaiting the founder's batch, as F7 A, F9 A, F10 B and F12 A of this entry land PROVISIONAL.

@history_item n=27 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — added `contract/escher-session-layout/answer` and the `decision/no-caller-input-echoed` exception (the layout answer's id field names the asked id). Under F10 B, `layout --id` on a virtual id reads `not-found`, so neither row moves. Any echo of a virtual id in an answer must fit this exception or be no echo at all.

@history_item n=28 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — ratified, by the founder's own word, the delegate confirm that landed the layout answer and the no-caller-input-echoed exception. Those rows stand as standing, not provisional.

@history_item n=29 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — changed `decision/stand-lints-clean-at-boot` to name the order page and moved `contract/cold-agent-cli/usage` to seven apps. The boot-lint list must still read zero with virtual nodes held out of lint (F9 A).

@history_item n=30 marker=2026-11-20-driver-debts
**2026-11-20-driver-debts** — `contract/escher-session-graph/answer` gained `--part` paging, with part 1 alone carrying states. Its current text is the picked row, and it stays unchanged under F9 A's graph exclusion.

@history_item n=31 marker=2026-11-23-circle-drawer-on-a-canvas
**2026-11-23-circle-drawer-on-a-canvas** — added `decision/circle-drawer-circles-painted-on-canvas` (circles painted from one `data-circles` attribute, never DOM nodes, reaching no snapshot, tree or graph) and Circle Drawer to the boot-lint list. This entry puts declared virtual nodes in the tree and snapshot, but Circle Drawer's circles stay out of it («Circle Drawer's circles as virtual nodes»). The row's «reach no snapshot, tree or graph» therefore stays true only while Circle Drawer's canvas declares no virtual nodes. Watch for a row change if the fixture is Circle Drawer.

@history_item n=32 marker=2026-11-24T06-11-11-wrap
**2026-11-24T06-11-11-wrap** — ratified `decision/circle-drawer-circles-painted-on-canvas` (the founder's word, state standing), so a change to its «reach no snapshot, tree» clause would amend a founder-ratified row.

@history_item n=33 marker=2026-11-24-canvas-draw-commands
**2026-11-24-canvas-draw-commands** — added the `canvas` verb on every carrier (sixteen verbs), `contract/escher-session-mcp/canvas-tool` (PROVISIONAL, delegate) and `fonts-unavailable` for canvas. The circle row was re-stated to cross as canvas draw commands. Under F10 B, `canvas --id` on a virtual id reads `not-found`, so the canvas-tool row stands unchanged.

@history_item n=34 marker=2026-11-24-canvas-lint
**2026-11-24-canvas-lint** — re-stated the circle row: lint reads canvas label ink boxes (`truncated`). It also extended `decision/stand-lints-clean-at-boot` to no canvas finding for Circle Drawer after a click and at 12 and 100 circles (PROVISIONAL, delegate). This is the latest change to both picked desktop-native rows. A virtual node held out of lint (F9 A) must leave both readings intact.
