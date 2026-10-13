# layout-templates — amendments

One entry per amendment to `layout-templates.md` (sidecar-contract.md §Entry form). Appended at wrap P2; rewritten only by the epoch-close consolidation.

## 2026-10-06-headless-stand — the headless stand's TaskShell mount; seven_guis citations re-pointed
**Section:** §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell)
**Change:** the seven_guis entry adds the headless stand: it skips Home and mounts one lean task (Counter, FlightBooker, Timer, Crud) in TaskShell through `task_in_shell` — `main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body` — at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans; its `app.rs` citations re-pointed to Home, TaskShell and the CSS constants after the chunk's line shift.
**Why:** the headless stand chunk added a second way into TaskShell; the windowed Home → TaskShell flow is unchanged.
**Ref:** .andromeda/runs/2026-10-06T02-44-36-wrap/

## 2026-10-06-stand-test-contract — agent-run.sh on the cli surface
**Section:** §Surface: cli → Primary screens
**Change:** adds `scripts/agent-run.sh` — usage `agent-run.sh <verb> [selection]` (verbs `boot · run · status · cleanup · logs`, selections `stand · all · <blitz-tests file name>`), usage on stderr with exit 2, stdout JSON lines only, raw cargo output kept in `target/agent-run/run.log`; `scripts/agent-run.ps1` its Windows pass-through; the exit grammar and event schema stay in test-plan §3.
**Why:** the stand test contract chunk added a repository CLI entry point beside `paint_bench` and `bump`.
**Kept:** the surface's NOT YET MEASURED marker (tooling context, expression level, signature placement) stands.
**Ref:** .andromeda/runs/2026-10-06T03-39-41-wrap/

## 2026-10-06-cold-agent-run-pipe — the cold-agent.sh CLI surface
**Section:** §Surface: cli → Primary screens
**Change:** adds `scripts/cold-agent.sh` beside agent-run.sh — usage `cold-agent.sh <verb> [task]`, verbs `run · status · cleanup · logs`, the one task `counter`; a usage error prints to stderr with empty stdout and exits 2 before any precondition; stdout is JSON lines only; the raw session transcript stays in `target/cold-agent/transcript.jsonl`; `scripts/cold-agent.ps1` its Windows pass-through; the exit grammar, schema and stub in test-plan §3.
**Why:** the cold-agent run pipe chunk added an agent-invocable CLI.
**Ref:** .andromeda/runs/2026-10-06T04-34-08-wrap/

## 2026-10-06-upstream-sync-element-identity — file:line citations re-pointed after the upstream merge
**Section:** every section citing a merged upstream file's lines
**Change:** 4 `file:line` citations into the merged upstream files re-pointed by the merge's measured line map; no claim text changed by the re-point.
**Why:** the chunk merged upstream `main` at `23354585`; the merged files' lines moved.
**Ref:** .andromeda/runs/2026-10-06T08-31-16-wrap/

## 2026-10-06-stable-element-ids — the lean tasks' controls carry author ids
**Section:** §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell)
**Change:** each lean task's controls and value displays carry author `id`s that are their stable element ids — counter-value · counter-increment; flight-one-way · flight-return · flight-start · flight-return-date · flight-book · flight-booked; timer-progress · timer-elapsed · timer-duration · timer-duration-value · timer-reset; crud-filter · crud-list · crud-name · crud-surname · crud-create · crud-update · crud-delete — and each CRUD row the Dioxus key `{i}`; no class, style, wrapper or order added, no task CSS selects by them.
**Why:** the chunk keyed the lean tasks' controls so agents aim at semantic ids (operator's P4 choice); the screens' structure is unchanged (markup-only gate).
**Ref:** .andromeda/runs/2026-10-06T09-35-44-wrap/

## 2026-10-06-id-persistence — CRUD rows keyed by person id
**Section:** §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell)
**Change:** each CRUD row's Dioxus key was `{i}`, its index in the people list (per 2026-10-06-stable-element-ids — the lean tasks' controls carry author ids). It is now `{person.id}`, the person's model-assigned id (fixture people 0–2, Create from 3), so a row's id `…/div[{person.id}]` follows its person under filter, Create and Delete. The `crud.rs` citation is re-pointed by the chunk's line map. No class, style, wrapper or order changed.
**Why:** an id an agent holds must name the same person after another person's Delete (operator's P4 choice of person keys over index keys).
**Ref:** .andromeda/runs/2026-10-06T10-55-07-wrap/

## 2026-10-06-accessibility-tree-identity — stand inputs named by attributes alone
**Section:** §Surface: desktop-native → Primary screens (seven_guis Home and TaskShell) · citations re-pointed
**Change:**
- The six task inputs are named by attributes alone: `for` on the existing labels of `timer-duration`, `crud-filter`, `crud-name` and `crud-surname`; `aria-label` "Departure date" / "Return date" on `flight-start` / `flight-return-date`. No element, class, style or order changed.
- 4 citations re-pointed by the measured line map: 1 into `flight_booker.rs` (+1 from old line 82, +2 from old 88) and 3 into dioxus-native-dom `dioxus_document.rs` (+2 from old line 18).
**Why:** the accessibility-tree chunk needed a non-empty name on every stand input without touching the stand's structure; its markup probe confirmed the files differ from the base only by those attributes.
**Ref:** .andromeda/runs/2026-10-06T12-55-34-wrap/

## 2026-10-06-id-stability-across-code-edits — CRUD rows and Home's cards read author keys
**Section:** §Surface: desktop-native → Primary screens (the seven_guis Home and TaskShell bullet)
**Change:**
- A CRUD row's id was `…/div[{person.id}]`, from its Dioxus key; now the row carries the author `id` `crud-person-{person.id}` beside that key and reads it (fixture `crud-person-0` · `crud-person-1` · `crud-person-2`, Create from `crud-person-3`).
- Adds Home's seven task-card ids `task-card-{slug}` — `counter`, `temp-converter`, `flight-booker`, `timer`, `crud`, `circle-drawer`, `cells`.
- Both are attributes only: no class, style, wrapper or order, and no CSS selects by them.
- Seven line citations re-pointed (`app.rs`, `crud.rs`); two added.
**Why:** every element an agent can act on reads an author key, the founder's rule (the founder, 2026-10-06); the stand's layout is unchanged.
**Ref:** .andromeda/runs/2026-10-06T20-55-58-wrap/

## 2026-10-07-driver-session — the `escher-session` host: a cli entry and a third consumer of the TaskShell mount
**Section:** §Surface: cli → Primary screens · §Surface: desktop-native → Primary screens
**Change:**
- cli: new entry `escher-session <counter|flight-booker|timer|crud> <state-dir>` — a closed two-argument argv; any other argv prints the usage line to stderr and exits 2 with nothing booted and no state created; exit 0 after its session is stopped, 1 with an error's fixed message; nothing on stdout; stderr lines carry `service.name=seven_guis`; unix only in 0.1.0.
- desktop-native: the seven_guis bullet names the session host as a third consumer of the `task_in_shell` mount, after the windowed app and the headless stand checks — it boots one lean task through `seven_guis::stand` only and adds no markup, id, class, style, wrapper or order; a held instance reads as a fresh boot and the mount and pinned viewport hold after commands.
**Why:** the chunk added the host binary; no screen, markup or viewport value changed.
**Ref:** .andromeda/runs/2026-10-07T06-51-52-wrap/

## 2026-10-07-settle-detection — the harness-defaults citations re-pointed
**Section:** §Surface: desktop-native → IA notes (the test-harness defaults bullet)
**Change:** the bullet's two citations into the harness source are re-pointed by measurement to the `Default` impl that holds the 800x600, scale 1, light values and to the viewport built from them. The claim itself is unchanged. Before the chunk the second citation already pointed at a closing brace.
**Why:** the chunk added lines above both ranges; a mechanical shift would have kept the second citation on a brace.
**Ref:** .andromeda/runs/2026-10-07T11-06-31-wrap/

## 2026-10-07-act-by-id — two citations into the session host re-pointed
**Section:** §Surface: desktop-native → Primary screens (the seven_guis bullet) · §Surface: cli → Primary screens (the `escher-session` bullet)
**Change:** two citations of `examples/seven_guis/src/session_host.rs` moved with the file's four added lines — the boot of a lean task, and the exit-code match. No claim changes: the host still takes exactly two arguments, prints nothing to stdout and keeps exit codes 0 · 1 · 2; the usage line already lists `timer`; no element, id, class, style, wrapper or order changed under the lean tasks or the shell.
**Why:** the host gained the lines that hand the Timer's session its time step, which shifted the cited ranges.
**Ref:** .andromeda/runs/2026-10-07T14-22-35-wrap/

## 2026-10-07-refusal-detection — the stand's nested scroller
**Section:** §Surface: desktop-native → IA notes
**Change:** a new bullet: an into-view scroll moves every scrolling box that holds its target, then the viewport, and `visible_region` reads what an element can be seen through; on the headless stand the nested scroller is the CRUD list — `crud-list` (24, 112.796875, 369 × 419.203125 at boot) clips its rows, `task-body` (0, 46, 800 × 554) does not clip that row; after twelve Creates the last row is refused `off-screen` and brought inside the list by a driver `scroll`; the pinned viewport is scrolled only on an in-file fixture. No stand markup moved.
**Why:** the stand's scrolling region became something an agent acts on. The engine change is ratified by the founder (2026-10-07), his own choice relayed verbatim by the overseer and confirmed by the operator at this wrap's escalation.
**Kept:** "a header over a scrolling body" for TaskShell stands as written: `task-body` was measured only as not the box that clips the CRUD row.
**Ref:** .andromeda/runs/2026-10-07T20-27-47-wrap/

## 2026-10-10-driver-cli — `escher-session` is the driver's command line
**Section:** §Surface: cli → Primary screens (the `escher-session` bullet) · §Surface: desktop-native → Primary screens (the seven_guis bullet)
**Change:**
- cli: was "the session host … usage `escher-session <counter|flight-booker|timer|crud> <state-dir>`, a closed argv of exactly two arguments", exits 0 · 1 · 2, nothing on stdout; now every command is `escher-session <verb> [arguments] --session <dir>` over the driver's nine verbs — the six instance-level ones with their arguments as flags (`--id`, `--text`, `--key`, `--ms`, `--shift`), then `start <task>`, `status`, `stop` — with four endings: accepted, one line of uncoloured JSON on stdout, exit 0; refused, exit 1; usage, one fixed line on stderr, exit 2; session error, exit 3. The host role is `serve <task> --session <dir>`, spawned by `start`. The former two-argument form reads refused, `unknown-verb`. The two citations of the old `main` are replaced by the reader's and the endings'.
- desktop-native: the third consumer of the stand's mount is named as the binary in its host role; a client command of the same binary boots nothing. The layout statement is unchanged: no markup, id, class, style, wrapper or order added, a held instance reads as a fresh boot.
**Why:** the chunk made the binary an agent's command line. The operator ruled at this wrap that the id `flow-names-no-element`, which the counter flow names and no screen reads, is the flow's own refusal step and no surface — so this plan lists no such id.
**Kept:** unix only in 0.1.0; the host's stderr log lines still carry `service.name=seven_guis`.
**Ref:** .andromeda/runs/2026-10-10T10-03-00-wrap/

## 2026-10-10-scrolling-box-bounds-and-hit — what a scrolled list reads, and Home's scrolling box
**Section:** §Surface: desktop-native → IA notes (the scrolling-boxes bullet) · §Surface: desktop-native → Primary screens (the seven_guis Home and TaskShell bullet)
**Change:**
- IA notes: gains what `crud-list` reads once its content is scrolled — the same four figures, unnamed in that scroll's diff, and a driver `click` naming the list lands on the row at its centre at 12 Creates and at 14. Nothing the bullet said is retired: the last row is still refused `off-screen` until a `scroll` brings it in.
- Primary screens: "Home is a centered 640px column of task cards" now says the column stands inside a box that scrolls — at the stand's 800 × 600 viewport, Home unscrolled, `#home` is 600 high and holds six of the seven cards; the seventh, Cells, lies at 615 to 684, below the box, and shows and is hit only once scrolled into view. The windowed stand is not measured.
**Why:** the bounds reader and the hit were fixed in the engine, so the list's own readings became true ones. The Home reading was found when a standing check that clicked the Cells card without scrolling went red on the fixed hit: the master had not said that the cards do not all show. No stand markup, id, class or style moved.
**Ref:** .andromeda/runs/2026-10-10T15-29-49-wrap/

## 2026-10-10-snapshot-states — two lean tasks state a role and a state by attributes alone
**Section:** §Surface: desktop-native → Primary screens (the seven_guis Home and TaskShell bullet)
**Change:** beside the six inputs named by attributes alone, the bullet now states that the flight booker's mode buttons `flight-one-way` and `flight-return` are toggle buttons stating `aria-pressed` `"true"` or `"false"`, its date inputs `flight-start` and `flight-return-date` state `aria-invalid`, `crud-list` takes `role="listbox"` and each CRUD row `role="option"` with `aria-selected` — adding no element, class, style, key, id or order; the three classes that style those states stay. The windowed stand was not booted.
**Why:** the stand's tasks state the three states a snapshot reads; the two task files differ from the chunk's base by added attribute lines only.
**Ref:** .andromeda/runs/2026-10-10T17-47-57-wrap/

## 2026-10-10-mcp-surface — `escher-session`'s third role, `mcp`
**Section:** §Surface: cli (the `escher-session` entry) · §Surface: desktop-native (seven_guis Home and TaskShell)
**Change:**
- cli: the binary's third role, `escher-session mcp` — `mcp` alone, any other shape after it usage, the usage line naming it — an MCP server over stdio, local to the invoking user, no listener or auth surface: JSON-RPC 2.0 requests on stdin, protocol lines only on stdout, the handshake revisions `2025-06-18` · `2025-03-26` · `2024-11-05` (`2025-06-18` to any other), the nine verbs as its tools, a tool call run on the one instance it holds in process; it echoes a request's admitted `id`; its end writes nothing on stdout, 0 on `Ok`, 3 with a fixed message on stderr on `Err`. Was "no ending writes back anything the caller typed", now scoped to the commands.
- desktop-native: the `mcp` role is a further consumer of the TaskShell mount, booting one lean task in process on a `start` tool call through the same boot, adding no markup.
**Why:** a new mode of an existing surface; the crossing it carries is the founder's ratified one (2026-10-10), recorded in the security plan.
**Ref:** .andromeda/runs/2026-10-11T08-35-17-wrap/

## 2026-10-12-self-description — `escher-session help`; the `mcp` role's ten tools and four revisions
**Section:** Surface: cli → Primary screens (the `escher-session` bullet)
**Change:** was "over the nine verbs of the driver's verb table … `--session <dir>` exactly once on every command"; now ten verbs in three levels — the six instance verbs, `start` · `status` · `stop`, then `help` — with `--session` exactly once on every command but `help`, which takes at most one and does not read it. `escher-session help` answers `describe()`'s line: seven keys, every text fixed table text; exit 0, no session reached. `help x` is `malformed` (1); `--session` with no directory, or twice, is usage (2). The session error's object carries `remedy`, the statuses are read from `EXIT_STATUSES`, and the usage line lists `help`. The `mcp` role: was three revisions, `2025-06-18` to any other; now four, `2025-11-25` to any other, `2026-07-28` included, with `server/discover` still `-32601`. It lists ten tools, each description the verb's help plus its result fields, and answers `start` · `status` · `stop` · `help` itself.
**Why:** the chunk added the `help` ending and tool to this surface, and the fourth revision on the operator's answer at the plan forks.
**Ref:** .andromeda/runs/2026-10-12T08-55-10-wrap/

## 2026-10-12-headless-screenshot — `escher-session`: eleven verbs, the shot's line and image block
**Section:** §Surface: cli → Primary screens (`escher-session`, its `mcp` role)
**Change:** was ten verbs, six asked of the instance; now eleven, seven — `screenshot` after `scroll`, an optional `--id` (the whole screen without it, with it the element's visible part), answering one uncoloured line `{"width","height","png"}` on stdout, exit 0; an app with no font it can paint text with refused `fonts-unavailable`, an element shot of no pixel `off-screen`, exit 1. The `mcp` role was ten tools, now eleven; a shot answers an `image` block then a text block, a refused one one text block with `isError`; a result holding an image is bounded at 1,048,576 bytes by its whole written form.
**Why:** the chunk's verb on both surfaces; the image in band is the founder's fork 1, ratified by his own word (2026-10-12, by question dialog in the overseer session, relayed verbatim by the overseer; the wrap's answer card confirmed the same way, 2026-10-13).
**Ref:** .andromeda/runs/2026-10-12T20-34-17-wrap/
