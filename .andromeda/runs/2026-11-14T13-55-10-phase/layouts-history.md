@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand's TaskShell mount through `task_in_shell` (skips Home, pinned 800 × 600, scale 1.0, Light, bundled DejaVu Sans). This is the origin of decision/headless-stand-pinned-viewport. Exploration's fresh session per path boots this same mount, so its paths start from this pinned viewport.

@history_item n=2 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the lean tasks' controls got author ids as attributes alone: no class, style, wrapper or order, and no CSS selecting by them. This is the origin of decision/stand-ids-as-attributes-alone and the contract/stand-element-ids rows. The ids that the actions reader offers, and that exploration's recorded steps name, rest on this.

@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — CRUD rows were keyed by `{person.id}` (fixture 0–2, Create from 3) instead of by index, so an id follows its person after another person's Delete. Exploration replays a path on a fresh session, so a path's row ids must name the same person when the path is re-reached and replayed.

@history_item n=4 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the six stand inputs were named by attributes alone (`for` on the labels, `aria-label` on the flight date inputs). This explains the "named by its label's for" and "named by aria-label" clauses of the timer-duration, crud-filter/name/surname and flight-start/return-date id rows. No stand markup may move for this chunk.

@history_item n=5 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — CRUD rows carry `crud-person-{person.id}` beside the Dioxus key, and Home's cards carry `task-card-{slug}`. Rule: every element an agent can act on reads an author key (the founder). This is the precedent for decision/every-stand-control-keyed and the contract/stand-element-ids/crud-person-id row. Exploration's frontier comes from keyed actionable nodes.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host became a third consumer of the TaskShell mount, booting through `seven_guis::stand` and adding no markup. A held instance reads as a fresh boot, and the mount and viewport hold after commands. This is the origin of decision/held-instance-reads-as-fresh-boot and decision/session-roles-share-stand-boot. Any carrier exploration reaches must boot through the same stand boot.

@history_item n=7 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — an into-view scroll moves every scrolling box that holds its target, then the viewport, and `visible_region` reads what an element can be seen through. After twelve Creates the last CRUD row is refused `off-screen` until a driver `scroll` brings it in. The founder ratified this for every document. These are the origins of decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes. This is a refusal that exploration will meet and must catch by its closed cause, without stopping.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session <verb> [arguments] --session <dir>` became the driver's command line, with four endings: accepted exit 0, refused 1, usage 2, session error 3. The former form reads `unknown-verb`. This is the origin of contract/escher-session-command/form, contract/escher-session-exit-codes/3 and contract/escher-session-refusals/unknown-verb. A new exploration verb or role on this carrier is a widening (the scope's boundary fork).

@history_item n=9 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — `crud-list`'s scrolled readings were fixed, and a click naming the list lands on the row at its centre. Home's column scrolls, and the Cells card is hit only once scrolled into view. It was found when a standing check clicked without scrolling and went red. This is a trap for exploration: an action can be offered but sit off-screen and be refused or miss until it is scrolled.

@history_item n=10 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — states set by attributes alone: `crud-list` takes `role="listbox"`, rows `role="option"` with `aria-selected`, and date inputs `aria-invalid`. The flight mode toggle buttons were later replaced by a select (2026-10-25-cold-agent-bar). This explains the crud-list row's listbox role, which exploration's actions reading meets.

@history_item n=11 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added `escher-session mcp` as a further consumer of the TaskShell mount through the same boot. The "no ending writes back anything the caller typed" rule was scoped to the commands. Behind decision/no-caller-input-echoed and decision/session-roles-share-stand-boot: exploration's report must echo nothing the caller typed (the budget, the directory).

@history_item n=12 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `escher-session help` and the verb table in levels, `--session` exactly once on every command but `help`. The session error carries `remedy`, and statuses are read from `EXIT_STATUSES`. MCP tools are each the verb's help plus its result fields. This is the precedent for contract/escher-session-mcp/tools. A new verb would extend `VERBS`, help and `tools/list` together.

@history_item n=13 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added the `screenshot` verb, with refusals `fonts-unavailable` and `off-screen` (exit 1) and an MCP image block. This is the origin of contract/escher-session-screenshot/answer and contract/escher-session-refusals/fonts-unavailable and off-screen. The scope bars `screenshot` from explored paths, so exploration must skip this verb from the table.

@history_item n=14 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` takes `tabindex="0"` and `onkeydown`. Its Arrow, Home and End keys move the selection and fill the name fields. Focus stays on the list and no `aria-activedescendant` is read. This is the origin of decision/crud-list-keyboard-region. Key actions on the list are among those exploration drives.

@history_item n=15 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` is named `People` by `aria-label` alone. This is the source of the contract/stand-element-ids/crud-list row's name.

@history_item n=16 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — added author ids on the non-lean tasks (Cells `cell-{A..Z}{0..25}`, `cell-input`, Circle Drawer, Temperature Converter, the circle dialog). `unkeyed_actionable()` reads 0 on all seven tasks. This is the origin of the cell id rows and decision/every-stand-control-keyed (an open dialog included). Exploration's action set over Cells is 676+ cells.

@history_item n=17 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe uses the driver's own MCP server. The agent's check stays in `target/cold-agent/check.json` and is never printed. This is a precedent that kept checks are never echoed, so exploration's checks are not printed either.

@history_item n=18 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `--record <dir>`, which keeps a session as one owner-only replayable check: `check-{n}.json`, `0600`, in a `0700` directory. The endings `record-dir-not-private` and `recording-failed` are exit 3. The founder ratified it as a boundary widening. This is the origin of decision/session-recording-replayable-check and the contract/escher-session-record rows. Exploration must leave each path in this same format, with the same refusal of a group- or other-open directory.

@history_item n=19 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorded check holds, per call, the verb, arguments, acceptance, answer digest and masked answer. A shot is kept without its `png`. Checks are replayed by `Check::replay`. Explored paths must replay green through this same `Check::replay`. The versioned-format and two-target direction in the scope extends this format.

@history_item n=20 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads clean to the in-process lint at boot in both layout modes. After twelve Creates, `crud-person-14` reads `clipped`, agreeing with the driver's `off-screen` refusal. A later entry under this marker made `lint` the eighth instance verb on every carrier, kept under `--record`. This is the origin of decision/stand-lints-clean-at-boot and contract/escher-session-lint/answer. `lint` is a verb in the table that exploration may drive.

@history_item n=21 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the `settled` field of the acting tools states that a load in flight is waited for up to 5 s. This bears on exploration's steps on the load page (`load-fetch`), whose click waits out the network stand-in.

@history_item n=22 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — dropped the visual lint's PROVISIONAL "in process only" mark on the founder's word. This is a precedent for an in-process-only first build later reached through a verb, which matches this chunk's "which carriers" boundary fork.

@history_item n=23 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — set the six served tasks (`counter`, `flight-booker`, `timer`, `crud`, `cells`, `load`), booted by the lean boot, `boot_cells` and `boot_load`. The flight mode became a select (`flight-type`, `flight-type-one-way`, `flight-type-return`). The load page got `load-fetch`/`load-done` with `LoadNet` after 200 ms. The cold-agent pipe runs `mcp --record`. This is the origin of the contract/escher-session-served-tasks rows, the flight-type rows and decision/load-page-only-network. Exploration is proven over these six tasks.

@history_item n=24 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** — the ten further stand lines entry. `LoadNet` answers from a thread of its own, so a driver `click` on `load-fetch` waits it out and returns with the message shown. The flight select's options show the chosen one alone. This bears on decision/load-page-only-network and on the step timing on the load page.

@history_item n=25 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — this marker has three relevant entries:
- The load page's Tab sequence follows `LOAD_TAB_ORDER`, and `load-fetch` takes Enter and Space.
- `cells-grid` became one Tab stop and keyboard region with arrows and Enter. Its handlers are synchronous because the async form panicked (`RefCell already borrowed`). This is a panic trap in a handler, of the kind exploration must catch.
- The flight booker's boot lint now reads 0, because `overlaps` skips a closed select's options.

These are the origins of the cells-grid and load-fetch rows and of the `[0,0,0,0]` lint at boot.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — this marker has three relevant entries:
- A click, Enter or Space opens `select#flight-type`'s engine-painted list. Escape, a second click, a click outside or a scroll closes it. While it is open each row reads its own bounds, and after a close the rows read back at the select's rect.
- An open list's rows are the exception to the visible-region and scroll-into-view rules.
- The lint's `Entry.in_select_box` replaced `Entry.in_closed_select`.

These are the origins of decision/open-list-rows-own-bounds and the `except:` clauses of the scroll and visible-region rows. Exploration driving the select opens a popup state, and a path's re-reach and replay must reproduce it.

@history_item n=27 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — the `graph` verb was added as the ninth instance verb: `VERBS` went from 12 to 13, it appears in the usage line after `lint` and `tools/list` grew to 13. PROVISIONAL (delegate overseer). This is a precedent for extending the verb table, and `graph` is a read verb exploration meets in `VERBS`.

@history_item n=28 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the `actions` verb was added as the tenth instance verb (`VERBS` 14). It answers `{"nodes"}` with id, owner, keyed and verbs with verb and cause, never a value, typed text or `NodeId`, and is unpaged. The founder ratified it on every carrier. This is the origin of contract/escher-session-actions/answer and of the fourteen-verb command form and `tools/list`. Exploration's available actions come from this reader.

@history_item n=29 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — this marker has two relevant entries:
- The `coverage` role was added as a further consumer of the TaskShell mount, booting once per check through the same boot.
- `escher-session coverage <app> <check>...` reads checks bounded at `MAX_CHECK_BYTES + 1` and parses them with `Check::from_bytes`. It refuses `malformed` holding nothing of the path, replays each check on a fresh boot, and answers ids and counts only. PROVISIONAL (delegate overseer).

This is the closest precedent for a binary role that replays kept checks on fresh boots. It is the "binary role like `coverage`" option in the scope's carriers fork, and the reason `coverage` appears in decision/session-roles-share-stand-boot.

@history_item n=30 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows: 121 rows, verified against the code. This is why the picked rows exist in their current handle form. Any change this chunk makes is recorded against these handles.
