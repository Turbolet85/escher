@history_item n=1 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — named the six stand inputs by attributes alone (`for`, `aria-label`), no element, class, style or order changed; the origin of decision/stand-a11y-by-attributes-alone, which §Accessibility reach describes when it restates the a11y leg and its named limits.

@history_item n=2 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — added roles and states by attributes alone (`aria-invalid`, `role="listbox"`/`option`, `aria-selected`, then `aria-pressed`), the classes styling those states kept; widened decision/stand-a11y-by-attributes-alone. Its `aria-pressed` clause was later retired (see 2026-10-25-cold-agent-bar), a trap for any a11y claim the page states.

@history_item n=3 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — made `crud-list` one Tab stop taking ArrowDown/ArrowUp/Home/End, tree focus on the list and never the row, no `aria-activedescendant`; the source of decision/crud-list-keyboard-region and of the stated tree-focus gap, a limit the §Stated limits / §Accessibility reach sections must state as it stands beside the new hidden-focus SC 2.4.3 gap.

@history_item n=4 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — named `crud-list` `People` by `aria-label` alone; the current-option gap stays stated (no `aria-activedescendant`). Bears on how the page phrases the CRUD region's a11y limit.

@history_item n=5 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid, a keyboard region and one Tab stop; the load page's Tab sequence; `stand_contrast` over the load page and Cells; the flight booker's boot lint reading) — Cells grid one Tab stop with its editor's focus return, `LOAD_TAB_ORDER`/`CELLS_TAB_ORDER`, `stand_contrast` widened to the load page and Cells, and the lint's select-option overlap retired with the `.andromeda/residuals.md` owner clause; grounds for the a11y reach (the hidden-focus limit "the stand's Cells editor never reaches"), the contrast witness of decision/stand-accent-colour, and an earlier precedent of a `residuals.md` owner line retired.

@history_item n=6 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — moved the stand's colour rows here (PROVISIONAL, F2): accent `#3a5ce5`, white text reading 5.47 vs SC 1.4.3's 4.5, `#4a6cf7` 4.39 a fail, guard `stand_contrast`; the source of decision/stand-accent-colour, whose numbers any contrast claim on the page must match.

@history_item n=7 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (F2 ratified: the stand rows' gap marker drops) — the founder ratified F2, dropping the `OPEN — PROVISIONAL` line; decision/stand-accent-colour stands ratified, so the page may state it without a provisional mark.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — made `escher-session <verb> [arguments] --session <dir>` the command form with four endings (0 accepted, 1 refused, 2 usage, 3 session error) and the `unknown-verb` refusal; the origin of contract/escher-session-command/form, exit-codes/1 and /3 and refusals/unknown-verb the page restates.

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role, an MCP server over stdio local to the invoking user, no listener or auth surface, the verbs as tools; the origin of decision/mcp-server-local-stdio and the README MCP sentence the pin holds.

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `help` (seven keys, no session reached), `--session` exactly once but on `help`, and `help` last among MCP tools; the origin of contract/escher-session-help/answer and the `help`-last ordering in contract/escher-session-mcp/tools.

@history_item n=11 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — added `screenshot` (`{"width","height","png"}`, `--id` crop) with refusals `fonts-unavailable` and `off-screen`; the origin of contract/escher-session-screenshot/answer and refusals/fonts-unavailable and /off-screen.

@history_item n=12 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `--record <dir>` on `start`, the host and `mcp` roles, an owner-only replayable check, and the session errors `record-dir-not-private` and `recording-failed` (exit 3); the origin of decision/session-recording-replayable-check and contract/escher-session-record/* rows.

@history_item n=13 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorded check keeps per call its verb, arguments, acceptance, answer digest and masked answer, replayed by `Check::replay`; the current wording of decision/session-recording-replayable-check the page states.

@history_item n=14 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the stand reads clean; the gated part: `escher-session lint`) — added the in-process lint then the `lint` verb answering `{"findings"}`, refused `fonts-unavailable` without a font; the origin of contract/escher-session-lint/answer and refusals/fonts-unavailable on lint.

@history_item n=15 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — dropped the lint's PROVISIONAL «in process only» mark on the founder's word; a precedent that a stated limit the page carries is retired only by a recorded ruling.

@history_item n=16 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — added `snapshot --part` (1 to 1000, `{"text","part","parts"}`, empty text past the last, out of bound `malformed`); the origin of contract/escher-session-snapshot/answer and /part.

@history_item n=17 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the host and `mcp` roles serve six tasks (counter, flight-booker, timer, crud, cells, load), the `start` tool's enum naming them; the flight booker's mode became a select (no `aria-pressed`); the lint's select overlap owned by a `.andromeda/residuals.md` line. The origin of contract/escher-session-served-tasks/* rows; a precedent for the retired `residuals.md` owner the chunk re-points; and the earlier cold-agent bar the boundary keeps out of this chunk.

@history_item n=18 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list; the lint's `overlap` exclusion) — the engine paints an open select list in every document, focus staying on the select, rows read in the snapshot; `Entry.in_select_box` replaces `Entry.in_closed_select`. A capability with its own stated reads (the windowed list read and not run) that §What it can do / §Stated limits may need to name.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (escher-session gains the graph verb) — thirteen verbs, `graph` after `lint` answering `{"nodes","states"}`, never a value; the origin of contract/escher-session-graph/answer and of the UI graph capability the page describes.

@history_item n=20 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — the graph verb; the coverage role as a TaskShell consumer; escher-session's coverage role) — the three PROVISIONAL delegate confirms ratified by the founder's word, text unchanged; the page may state graph and coverage with no provisional mark.

@history_item n=21 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — fourteen verbs, `actions` after `graph`, answer `{"nodes"}` with `owner`/`cause` left out when absent, never a value; the origin of contract/escher-session-actions/answer and the "what can be done here" capability.

@history_item n=22 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role a TaskShell consumer; escher-session's coverage role) — added `escher-session coverage <app> <check>...`, checks bounded at 1,048,576 + 1 bytes and refused `malformed`, answer `{"unexercised","checks","held"}`, a failed boot exit 3; the origin of contract/escher-session-coverage/usage, /check-bound and /answer and the handler-coverage capability.

@history_item n=23 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `~ contract/escher-session-command/form` and `~ contract/escher-session-mcp/tools` fourteen → fifteen verbs with `layout` after `actions`, `+ contract/escher-session-layout/answer`, and the `decision/no-caller-input-echoed` exception for the layout id; the current verb count (fifteen) the page and README must state, which `test_capability_statement.py` pins in the README's MCP sentence.

@history_item n=24 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — ratified by the founder's word the delegate confirm behind 2026-11-17-layout-on-the-carriers; the fifteen-verb rows and the layout answer stand ratified for the page to state.

@history_item n=25 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — `~ contract/agent-run-cli/usage` gained the `session` selection running the escher-session binary's flows; bears on §Where each claim is proven, which names the witnesses and leg counts (stand-repeat targets) the chunk corrects.

@history_item n=26 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master text became 121 rows (decision 21 · contract 99 · absence 1), the form every picked row now has; any master line the rewrite contradicts surfaces at the wrap as a row change, not a phase amendment.
