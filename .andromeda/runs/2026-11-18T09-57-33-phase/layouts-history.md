@history_item n=1 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — six served screens: the host and `mcp` roles boot one of six served tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), Cells by `boot_cells`, the load page in a TaskShell of its own by `boot_load`; `start <task>` takes one of the six, the usage line's `apps:` and the MCP `start` tool's `enum` the same six; the cold-agent pipe's `stand` task names six apps, its live server child running `mcp --record` into `target/cold-agent/record`; the load page's screen added as a new served-screens group with its colours and lint at boot. The direct precedent for this chunk's seventh served app: the served-task rows (`contract/escher-session-served-tasks/*`), `contract/cold-agent-cli/usage`, `contract/cold-agent-cli/record-dir` and `decision/stand-lints-clean-at-boot` all grow from this entry's shape, and a new served screen lands as a served-screens bullet beside the load page's.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — the headless stand skips Home and mounts one task in TaskShell through `task_in_shell` (`main#main > #task-shell`, `#task-header > #back-btn + #task-title` above `#task-body`) at the pinned 800 × 600, scale 1.0, Light viewport with DejaVu Sans. The origin of `decision/headless-stand-pinned-viewport` and the shell the new app's boot through `seven_guis::stand` must mount into, under `decision/fixed-document-skeleton-main-mount`.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host becomes a consumer of the `task_in_shell` mount, booting through `seven_guis::stand` only and adding no markup, id, class, style, wrapper or order; a held instance reads as a fresh boot. Origin of `decision/session-roles-share-stand-boot`, `decision/held-instance-reads-as-fresh-boot` and `contract/escher-session-host/log-service-name`: the new app's host boot must keep them.

@history_item n=4 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — `escher-session <verb> [arguments] --session <dir>`, the host role `serve <task> --session <dir>` spawned by `start`, the four endings (exit 0 · 1 · 2 · 3); the third consumer of the mount named as the binary's host role with no markup added. Origin of `contract/escher-session-command/form` and the exit-code rows; the scope adds no verb, ending or exit, so these stand while the served-task table grows.

@history_item n=5 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role over stdio, local to the invoking user, no listener or auth surface, a further consumer of the TaskShell mount booting a task on a `start` tool call through the same boot; "no ending writes back anything the caller typed" scoped to the commands. Origin of `decision/mcp-server-local-stdio` and `decision/no-caller-input-echoed`; the new app must be reached through this role's `start` enum as the six are.

@history_item n=6 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — adds `scripts/cold-agent.sh` (`<verb> [task]`), usage errors on stderr with empty stdout and exit 2 before any precondition, stdout JSON lines only, the transcript kept in `target/cold-agent/transcript.jsonl`, `cold-agent.ps1` its pass-through. Origin of `contract/cold-agent-cli/exit-2`, `contract/cold-agent-cli/stdout` and `contract/cold-agent-cli/windows-pass-through`: the verdict count read "from the transcript" must stay in files, never printed.

@history_item n=7 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the pipe's one task `stand` under a fixed `STAND_PROMPT` naming no element id or app slug; `run` needs `claude`, `python3` and a built `target/debug/escher-session` (exit 3, the pipe builds nothing); the client gets the driver's own MCP server; the agent's check stays in `target/cold-agent/check.json`, never printed. Origin of `decision/cold-agent-prompt-names-no-id`, `decision/cold-agent-pipe-builds-nothing`, `decision/cold-agent-uses-driver-mcp` and `contract/cold-agent-cli/exit-3`: the chunk's new prompt goal must name no element id or app slug.

@history_item n=8 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` on `start`, the host role and the `mcp` role: one owner-only replayable check (`check-{n}.json` `0600` in a `0700` directory), endings `record-dir-not-private` and `recording-failed`, a founder-ratified boundary widening. Origin of `decision/session-recording-replayable-check` and `contract/escher-session-record/check-file`; the verdict's `caused` count is to be read from these kept checks.

@history_item n=9 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check holds per call its verb, arguments, acceptance, the answer's digest and the masked answer, replayed by `Check::replay` (founder's F3 B). What the verdict's `caused` count reads from the live server child's checks, as bools and counts only, never the text; also where the new app's authored error text lands, masked as on every carrier.

@history_item n=10 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — the lean tasks' controls carry author `id`s as stable element ids, attributes only, no class, style, wrapper or order added and no CSS selecting by them (operator's P4, markup-only gate). Origin of the `contract/stand-element-ids/*` register and `decision/stand-ids-as-attributes-alone`, which the new app's control ids extend.

@history_item n=11 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — every element an agent can act on reads an author key (the founder's rule); CRUD rows `crud-person-{person.id}`; Home's task-card ids `task-card-{slug}`. The founder's rule behind `decision/every-stand-control-keyed`; and if the new app joins the windowed stand's Home (`src/app.rs`), a `task-card-{slug}` id follows this register.

@history_item n=12 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — the three non-lean tasks keyed (`temp-*`, `circle-*`, `cell-*`, `cell-input`) as `id:` lines alone; `unkeyed_actionable()` reads 0 on all seven tasks. The guard the new app's controls must also meet under `decision/every-stand-control-keyed`.

@history_item n=13 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — stand inputs named by attributes alone (`for`, `aria-label`), no element, class, style or order changed. Origin of `decision/stand-a11y-by-attributes-alone`: the new app's controls take names the same way.

@history_item n=14 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — roles and states stated by attributes alone (`aria-invalid`, `role="listbox"`/`option`, `aria-selected`), the styling classes staying. Extends `decision/stand-a11y-by-attributes-alone`, which the new app's screen falls under.

@history_item n=15 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — `crud-list` named `People` by `aria-label` alone, naming the stand's one unnamed interactive-role element. Precedent that every interactive element of a stand screen carries an accessible name, which the new app's controls must carry.

@history_item n=16 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads clean to the visual lint, at boot each lean task 0 findings in both layout modes; the gated part (`amd-oeiyf7qg`) adds the `lint` verb, the stand read clean through it on every carrier. Origin of `decision/stand-lints-clean-at-boot` and `contract/escher-session-lint/answer`; the scope's "its boot reads 0 lint findings" extends it to the seventh app.

@history_item n=17 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading; the load page's Tab sequence; `stand_contrast` over the load page and Cells; the citations re-pointed) — `stand_lint`'s `at_boot` reads `[0, 0, 0, 0]` with the load page 0; the load page's Tab sequence named by `LOAD_TAB_ORDER` and walked in both layout modes, `load-fetch` taking Enter and Space; `stand_contrast` extended to non-lean screens in both layout modes; thirteen citations into changed stand files re-pointed. Precedents for the new served screen's lint, Tab order constant, contrast row and the citation re-points its added lines in `common/mod.rs`, `stand_lint.rs` and `stand_contrast.rs` will force.

@history_item n=18 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** — the stand's colour, depth, spacing and radius rows land on the desktop-native screens: accent `#3a5ce5` (hover `#2a4cd3`, active `#1f3fc1`) on `#f5f5f5` with text `#1a1a1a`, white on accent reading 5.47 against SC 1.4.3, `#4a6cf7` a fail; ratified as F2 by `amd-d2v52ayt`. Origin of `decision/stand-accent-colour` (guard `stand_contrast`): the new app's CSS must declare these values if it declares an accent.

@history_item n=19 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the ten further stand lines; F2 ratified) — the served-screens bullet reads the load page's two keyed elements and `LoadNet` answering from a thread of its own; the stand's invalid state `#e53e3e` / `#fff5f5` / `#c53030` and type (sans-serif, 14px base) added; the stand rows' PROVISIONAL marker dropped. The served-screens bullet a seventh screen joins, and the stand's error-state colours a page of errors would reuse.

@history_item n=20 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the acting tools' `settled` field states that a load in flight is waited for up to 5 s, fixed table text. Bears on `decision/load-page-only-network`: only the load page's boot has a network provider; the new app's boot keeps none.

@history_item n=21 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role a consumer of the TaskShell mount; escher-session's coverage role) — `escher-session coverage <app> <check>...` replays kept checks on a fresh boot through the same boot, adding no markup, answering `{"unexercised","checks","held"}`; ratified by `2026-11-15T05-21-42-wrap`. Origin of `contract/escher-session-coverage/answer` and part of `decision/session-roles-share-stand-boot`: the seventh app's `<app>` word must boot through this role too.

@history_item n=22 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the tab-order citations; the tenth instance verb `actions`) — `actions` answers `{"nodes"}` of id, owner, keyed and verbs with verb and cause, never a value or typed text; `common/mod.rs` gained `ACTIONABLE_AT_BOOT`, shifting cited lines. Origin of `contract/escher-session-actions/answer`; the new app's actionable controls join that table, and its added lines trap the load and Cells tab-order citations again.

@history_item n=23 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph verb; the screens' citations after the graph registrations) — `graph` answers `{"nodes","states"}`, ids, component names, handler event names and state names only, never a value; every task's signals registered through `use_graph_signal`, shifting task-file citations; ratified by `2026-11-15T05-21-42-wrap`. Origin of `contract/escher-session-graph/answer`: the new task registers its signals the same way, and its handlers' event names surface here.

@history_item n=24 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `~ contract/escher-session-command/form` and `~ contract/escher-session-mcp/tools` to fifteen verbs, `layout` after `actions`; `~ decision/no-caller-input-echoed` gains its one named exception; `+ contract/escher-session-layout/answer`; ratified by `2026-11-17-a-component-in-isolation`. The current shape of these picked rows; the scope adds no verb, so the fifteen-verb counts stand, and the new app's authored error text is no caller input.

@history_item n=25 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the `mcp` role's result holding an image bounded at 1,048,576 bytes by its whole written form. Origin of `contract/escher-session-mcp/result-bound`, which an acting call's answer carrying `caused` on the new app stays inside.

@history_item n=26 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` answers `{"text"}` or `{"text","part","parts"}`. Origin of `contract/escher-session-snapshot/answer`; the scope keeps the new app's snapshot under `SNAPSHOT_TEXT_BUDGET`, one whole screen.

@history_item n=27 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — `~ contract/agent-run-cli/usage`: `agent-run.sh run session` runs the escher-session binary's flows in seven_guis. Where the binary's served-task checks in `examples/seven_guis/tests/` are run, which the scope moves for the seventh app.

@history_item n=28 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master became rows (121 rows), each verified against the code. Every picked row's handle and current text date from this conversion; edits land as `+`/`~`/`−` row entries, as later amendments do.
