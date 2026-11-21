@history_item n=1 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — added `snapshot --part` (whole number 1 to 1000, part 1 without it, `{"text","part","parts"}`, a part past the last the empty text with the same `parts`, outside the bound refused `malformed` exit 1, usage line `snapshot [--part <part>]`, over MCP the JSON line as one text block), on the founder's F1/F2. This is the form `graph --part N` is ruled to copy ("paged as `snapshot` is"), and it sets the rows contract/escher-session-snapshot/part and /answer that the graph part will mirror. `MAX_PART` and the usage-line pattern are the precedent.

@history_item n=2 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (escher-session gains the graph verb) — added the ninth instance verb `graph`: no argument (one named is refused `malformed`), answering `{"nodes","states"}` with ids, names, flag words and booleans only, never a value; usage line lists `graph` after `lint`; MCP lists it after `lint`. This is the row contract/escher-session-graph/answer ("No argument") that `graph --part N` must amend: the no-argument clause, the refusal of an argument, the usage-line entry and the MCP tool schema all change. It was PROVISIONAL (delegate overseer) when it landed.

@history_item n=3 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified: the graph verb; the coverage role as a TaskShell consumer; escher-session's coverage role) — the founder's own word replaced the delegate confirms on the graph verb and both coverage entries, leaving their text unchanged. The graph and coverage rows this chunk would change stand ratified, so any change goes through a new founder fork. They are not open drafts.

@history_item n=4 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (escher-session's coverage role; the coverage role a further consumer of the TaskShell mount; the bridge's skeleton and head notes re-cited) — added `escher-session coverage <app> <check>...`, which replays kept checks on a fresh boot and answers `{"unexercised":[{id,owner,event}],"checks","held"}`, with a failed boot ending as the session error, exit 3 (row contract/escher-session-coverage/answer). It added the reached-record line to `packages/dioxus-native-dom/src/dioxus_document.rs` `handle_event`, which moved the IA notes' citations one line down. Debt 3 (a stopped bubbling dispatch read as exercised) is a stated limit of this role. A reached-record fix would change what `unexercised` reports and would move those `dioxus_document.rs` citations again. The `graph --part` form was also ruled at this wrap's relay.

@history_item n=5 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid, a keyboard region and one Tab stop) — in the Cells editor, Enter commits, **stops propagation** and returns focus to `cells-grid`. The handles were made synchronous because the async form panicked (`RefCell already borrowed`), and a click inside an open editor returns early. This is the stand's one handler that stops a bubbling dispatch, so it is the likely carrier for proving or stating debt 3. That chunk also kept `press` without Ctrl as the residual (the founder's «Ограничение и residual»), which is debt 1 here. Related entries in the same chunk: the flight booker's lint skipping a closed select's options, where a `multiple` select's rows are linted as usual, and the Tab sequences `LOAD_TAB_ORDER` and `CELLS_TAB_ORDER`.

@history_item n=6 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list; the lint's `overlap` exclusion `Entry.in_select_box`) — gave the closed select an engine-painted list opened by click, Enter or Space. Enter, Space, Tab or a click chooses, Escape closes, and focus stays on the select. A closed select's options read the select's client rect and an open list's rows their own bounds, and a `multiple` select's rows are linted as usual. These are the select key paths that a Ctrl/Meta/Alt `press` modifier would reach, and the non-adjacent `multiple` set is the case debt 1 names. It bears on rows decision/flight-type-select-boot-reading and the flight-type ids. The row bounds sit inside the crossing by the field rule of 2026-10-10.

@history_item n=7 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` takes `tabindex="0"` and an `onkeydown`. ArrowDown, ArrowUp, Home and End move the selected row and fill the name fields; focus stays on the list and no `aria-activedescendant` is read; CRUD has six Tab stops. This is the row decision/crud-list-keyboard-region. A modifier on `press` must leave these keys' readings unchanged, or the plan must state what Ctrl+Arrow does here (the list is single-select by role).

@history_item n=8 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` named `People` by `aria-label` alone, adding no element, class, id or order. This is the row contract/stand-element-ids/crud-list ("a listbox named People"). A keyboard or coverage check on the list reads this name.

@history_item n=9 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — `crud-list` `role="listbox"` and its rows `role="option"` with `aria-selected`, stated by attributes alone while the styling classes stay. This sets the row decision/stand-a11y-by-attributes-alone. Any stand markup a check needs here, for example a `multiple` select or a stopping handler, must stay within attributes-only.

@history_item n=10 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the stand's inputs (`crud-name` and `crud-surname` among them) are named by `for` or `aria-label` alone, with no element, class, style or order changed. This is the origin of decision/stand-a11y-by-attributes-alone and of the `crud-name` and `crud-surname` rows.

@history_item n=11 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the lean tasks' controls carry author ids (`crud-list`, `crud-name`, `crud-surname` and others) added as attributes only, with no CSS selecting by them. This is the origin of decision/stand-ids-as-attributes-alone. Any new id a check needs follows this rule.

@history_item n=12 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — the Dioxus key of each CRUD row moved from index `{i}` to `{person.id}`, so a row follows its person under filter, Create and Delete. This is the basis of contract/stand-element-ids/crud-person-id, which keyboard and coverage checks on the CRUD rows rely on.

@history_item n=13 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — each CRUD row carries the author id `crud-person-{person.id}` beside its key, and Home has `task-card-{slug}`, both added as attributes only on the founder's rule that every actionable element reads an author key. These are the row contract/stand-element-ids/crud-person-id and the source of decision/every-stand-control-keyed.

@history_item n=14 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the non-lean tasks are keyed (Cells `cell-{A..Z}{0..25}` and `cell-input`, Circle Drawer including its dialog), and `unkeyed_actionable()` reads 0 on all seven tasks. This is decision/every-stand-control-keyed. Any element added for a stopped-dispatch or multi-select check must be keyed, or that reading breaks.

@history_item n=15 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the flight booker's mode became a select, with ids `flight-type`, `flight-type-one-way` and `flight-type-return` (previously toggle buttons with `aria-pressed`). The served tasks became six, and the `start` usage, the `apps:` line and the MCP `start` enum name them. The cold-agent pipe runs `mcp --record`. These are the rows for the flight-type ids, decision/flight-type-select-boot-reading and the served-tasks rows (crud, flight-booker), which are the stand carriers for a select-key or graph check.

@history_item n=16 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — set the command form `escher-session <verb> [arguments] --session <dir>`, with instance arguments as flags (`--id`, `--text`, `--key`, `--ms`, `--shift`) and the four endings 0/1/2/3. `--shift` on `press` is the flag a Ctrl, Meta or Alt modifier would sit beside. This is the origin of contract/escher-session-command/form and the exit-code rows 0, 1 and 2. A new flag reaches the usage line and the usage-ending cases.

@history_item n=17 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `escher-session mcp` role, which serves over local stdio with no listener and lists the verbs as its tools. "No ending writes back anything the caller typed" was scoped to the commands. These are the rows decision/mcp-server-local-stdio and decision/no-caller-input-echoed. A new `press` modifier and a graph `part` are new MCP tool arguments on this ratified crossing.

@history_item n=18 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `escher-session help`, which answers `describe()` with seven keys of fixed table text, and made each MCP tool description the verb's help followed by its result fields. This is the row contract/escher-session-help/answer and the description clause of contract/escher-session-mcp/tools. A new argument and the `press` limit text (its `help` states the Ctrl limit) are fixed table text that would change.

@history_item n=19 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — set the MCP result bound: a result holding an image is bounded at 1,048,576 bytes by its whole written form, a text-only result by its text. This is the row contract/escher-session-mcp/result-bound, the "answer bound" that every graph part must fit under. The scope cites 293,869 bytes on Cells at boot.

@history_item n=20 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `--record <dir>`, which keeps a session as one owner-only replayable check holding per call the verb and its arguments. It was ratified as a boundary widening by the founder. This is the row decision/session-recording-replayable-check. A new `press` modifier or graph `part` argument must be kept and replayed in a recorded check's argument shape.

@history_item n=21 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorded check keeps, per call, its verb, arguments, acceptance, answer digest and masked answer, and `Check::replay` reads it back and replays it (founder's F3 B). The coverage role replays these checks, so new argument shapes must round-trip through `recording` and `replay`, and a paged graph answer's digest must be considered.

@history_item n=22 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `escher-session lint`) — precedent for widening the verb table on every carrier: the verb count, the usage line, MCP tool order, being kept under `--record`, and a founder fork per crossing. A new argument on `press` or `graph` follows the same path across the carriers.

@history_item n=23 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — added `actions`, with no argument and an explicitly **unpaged** answer, and set the MCP tool order to `graph`, `actions`, ..., `help` last. This is precedent for stating in a row whether an answer is paged, which bears on graph becoming paged while `actions` stays unpaged. It also re-pointed `tests/blitz-tests/tests/common/mod.rs` citations.

@history_item n=24 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — the verb table grew to fifteen with `layout` after `actions` (rows contract/escher-session-command/form and contract/escher-session-mcp/tools), and decision/no-caller-input-echoed gained the exception that the layout answer echoes the `id` it was called with. This sets the current wording of three picked rows. A graph `part` echoed in the answer, as snapshot's `part` is, should be checked against the no-echo rule and its single exception.

@history_item n=25 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — the founder ratified the delegate confirms behind the 2026-11-17-layout-on-the-carriers rows, so the fifteen-verb form, the tools listing and the no-echo exception now stand on his word.

@history_item n=26 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe gives the client the driver's own MCP server `escher` (previously a stub). This is the row decision/cold-agent-uses-driver-mcp. Changes to the MCP tool schemas (a `press` modifier, a graph `part`) reach the cold agent directly.

@history_item n=27 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Surface: cli: 2 rows) — added `order` as the seventh served task, in the usage line and the MCP `start` enum, with contract/escher-session-served-tasks/order. This is the latest change to the served-tasks rows (crud, flight-booker) and the usage line that a new flag would also change.

@history_item n=28 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the screens' source citations after the graph registrations) — citations into the task files, `dioxus_document.rs` and the driver's `schema.rs` were re-pointed after the graph registrations, with no claim moved. This is a trap: edits to `schema.rs` and `dioxus_document.rs` in this chunk would shift these cited lines again and owe a re-pointing entry.

@history_item n=29 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — converted the master to rows (66 records became 121 rows), each verified against the code. All the picked rows date from this conversion, so later changes to them are row-level `~`, `+` or `−` entries.
