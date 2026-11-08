@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand's TaskShell mount: added `task_in_shell`, which mounts a task as `main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body` at the pinned 800 × 600, scale 1.0, Light viewport. This is the mount the extract requires to stay unchanged after the stand's signals move onto the registry.

@history_item n=2 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the lean tasks' controls carry author ids that are their stable element ids, with no class, style, wrapper or order added and no CSS selecting by them. This is the origin of the attribute-only discipline the signal-registry migration must follow, and of the ids that link 1 (node to component) reads.

@history_item n=3 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — CRUD rows were keyed by `{person.id}` and no longer by their index, so a row's id follows its person. Moving CRUD's signals onto the registry must keep these keys and this row identity, and any reserved virtual-node id form must not collide with the id forms they yield.

@history_item n=4 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — the stand's inputs were named by attributes alone (`for`, `aria-label`), and a markup probe confirmed the files differ only by those attributes. This is the precedent for proving that a stand migration leaves the markup unchanged.

@history_item n=5 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — CRUD rows read the author id `crud-person-{id}`, and Home's cards read `task-card-{slug}`, as attributes only. This rests on the founder's rule that every element an agent can act on reads an author key. The graph's node ids and the reserved virtual-node form sit beside these id forms (author key, anchored path, component path, document path).

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host became a consumer of the TaskShell mount. It adds no markup, id, class, style, wrapper or order, a held instance reads as a fresh boot, and the mount and viewport hold after commands. This is the baseline for checking that a graph read changes nothing.

@history_item n=7 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session <verb> … --session <dir>` over the driver's verb table, with the four endings: accepted (0), refused (1), usage (2) and session error (3). Any new graph verb must keep these four endings.

@history_item n=8 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — stand tasks state their role and state by attributes alone (`role="listbox"`/`option`, `aria-selected`, `aria-invalid`), adding no element, class, style, key, id or order. This is a precedent for a new snapshot reading that touches the stand by attribute only. The new «reacts to a click» field lands beside these role readings.

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role exposes the verbs as tools and is a further consumer of the TaskShell mount that adds no markup. A new graph verb appears in its tool listing.

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `escher-session help` and `describe()` built from fixed table text, the usage line listing the verbs, and `mcp` tool descriptions given as the verb's help plus its result fields. A new verb or snapshot field moves `describe()`, the help texts and the tool descriptions.

@history_item n=11 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` was added as the eleventh verb on both surfaces, and its in-band image was a founder-ratified fork. This is the precedent for how a new verb is added (its position, answer line, refusals and MCP block) and for counts that every pin must follow.

@history_item n=12 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `escher-session lint`) — `lint` was added as the twelfth verb (eight instance verbs), listed after `screenshot` in the `mcp` tools, with its step kept under `--record`. This established the twelve-verb table the extract binds the new read to. The same marker's earlier entry set the in-process clean lint reading of the stand at boot, the layout regression pin.

@history_item n=13 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — dropped the visual lint's PROVISIONAL mark: the lint is in process only, as built. This is part of the lint baseline the extract uses as the regression pin.

@history_item n=14 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record` keeps every call as a replayable check, and `help` and `tools/list` read the same with or without it. A new graph verb needs a recorded step, and its recorded answer bears on «values masked, never logged».

@history_item n=15 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorded check keeps each answer, masked as on every carrier. This was a founder-ratified boundary widening. A graph answer kept under `--record` must stay masked (`MASKED_VALUE`).

@history_item n=16 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the help texts of result fields are the program's fixed table text inside a ratified carrier. This is the precedent for the new «reacts to a click» field's help text in the schema, `describe()` and the tool descriptions.

@history_item n=17 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` added a flag and paged answers, and the usage line opens `snapshot [--part <part>]`. The new snapshot field must also hold for paged snapshots, and a graph read of a large screen (Cells, 676 cells) may meet the same bounds.

@history_item n=18 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the six served screens (lean four, Cells by `boot_cells`, the load page by `boot_load`) and the `start` enum. Some ids were re-pinned, and the flight booker became a select. This defines which served screens the migration of the stand's signals must leave unchanged, and which screens the graph read serves.

@history_item n=19 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — `crud-list` takes `tabindex="0"` and an `onkeydown`. The CRUD Tab sequence has six stops, and focus is on the list, never a row. The extract pins this sequence, and the list's handlers are among those the graph reads (link 2).

@history_item n=20 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` was named `People` by attribute alone, with the Tab sequence unchanged. This is part of the a11y binding and the attribute-only discipline.

@history_item n=21 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the non-lean tasks carry author ids (`temp-*`, `circle-*`, `cell-{A..Z}{0..25}`, `cell-input`) as `id:` lines alone, and `unkeyed_actionable()` reads 0 on all seven tasks. The extract requires that 0 to hold after the migration.

@history_item n=22 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid, a keyboard region and one Tab stop) — `cells-grid` (tabindex, `aria-label` "Cells", `onkeydown`, no role) is the 677th reacting element `stand_non_lean_tasks` reads, beside the 676 data cells. 677 was measured after the forecast of 676 was falsified. It counts elements with the listener mark that read `GenericContainer`. `CELLS_TAB_ORDER` applies. The «reacts to a click» field must agree with this count, which is a known trap.

@history_item n=23 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the load page's Tab sequence and its button's keys) — `LOAD_TAB_ORDER`, walked in both layout modes, and `load-fetch` taking Enter and Space. This is one of the Tab sequences the extract requires to read as recorded.

@history_item n=24 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading) — `stand_lint` `at_boot` moved to `[0, 0, 0, 0]` in both layout modes, and the load page reads 0. This is the exact regression pin the extract's acceptance cites.

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list) — the open list's rows read their own `bounds` in the snapshot and the diff, and the Tab sequence holds with the list open. A row's `bounds` was read as a closed set inside the ratified crossing by the field rule of 2026-10-10. This is the precedent for a closed snapshot field not being a widening, which is how the scope reads the «reacts to a click» boolean.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the lint's `overlap` exclusion, `Entry.in_select_box`) — the boot lint reading `[0, 0, 0, 0]`, the load page's 0, and the statement that no stand markup moved all stand as measured at this chunk. This is the latest state of the lint baseline the new chunk must keep.
