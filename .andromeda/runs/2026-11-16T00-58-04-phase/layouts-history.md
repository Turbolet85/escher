@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand: skips Home, mounts one task in TaskShell (`main#main > #task-shell`, `#task-header` above `#task-body`) at the pinned 800 × 600, scale 1.0, Light viewport with bundled DejaVu Sans. This is the source of decision/headless-stand-pinned-viewport; every placement/size the layout-explained read reports on the stand is measured inside this pinned viewport and TaskShell chain, in both layout modes.

@history_item n=2 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — re-pointed the harness-defaults citations to the `Default` impl holding 800x600, scale 1, light and the viewport built from them. Where the pinned viewport values live in code, if the new read names the viewport as the root containing block.

@history_item n=3 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — gave the lean tasks' controls author ids (timer-progress among them), attributes only, no class/style/wrapper/order, no CSS selecting by them. Source of contract/stand-element-ids/timer-progress and decision/stand-ids-as-attributes-alone: the read addresses elements by these ids, and any fixture or stand change for it must keep the attribute-only rule.

@history_item n=4 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — added Home's seven `task-card-{slug}` ids and CRUD rows' `crud-person-{id}`, attributes only, no CSS selecting by them. Source of the task-card-* contract rows picked here; the founder's rule that every actable element reads an author key.

@history_item n=5 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — keyed the non-lean tasks: `circle-canvas`, `cell-{A..Z}{0..25}` on the 676 cells, `cell-input`, etc., as `id:` lines alone. Source of contract/stand-element-ids/circle-canvas and cell-column-row; `circle-canvas` is the owner element the answer's reserved «placed by the app inside canvas X» kind would name in 0.4.0.

@history_item n=6 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — the session host became a third consumer of the `task_in_shell` mount, adding no markup; a held instance reads as a fresh boot, mount and pinned viewport hold after commands. Source of decision/held-instance-reads-as-fresh-boot (guard stand_session_fresh) and decision/session-roles-share-stand-boot: a read verb must not perturb layout state or the boot.

@history_item n=7 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the scrolling boxes note: into-view scroll moves every holding scroller then the viewport; `visible_region` reads what an element is seen through; the stand's nested scroller is `crud-list` (24, 112.796875, 369 × 419.203125 at boot), `task-body` (0, 46, 800 × 554) not clipping; the engine change ratified by the founder (2026-10-07). Source of decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes; gives the stand's scrolled case and measured bounds the new read's scroll/containing-block explanation must agree with.

@history_item n=8 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — fixed bounds reader/hit: a scrolled `crud-list` reads the same four figures; Home's `#home` is a scrolling box 600 high, Cells card at 615–684 below it until scrolled. Trap: a scrolled element's placement differs from where layout put it — exactly the «scroll moves it» clause; the Home/Cells case is a ready stand fixture for a scrolled box. Also bears on the scope's note that scrolled-case checks are re-read at «Taffy past #1372».

@history_item n=9 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the flight booker's mode list) — while a select list is open each row reads its own `bounds` in snapshot and diff, after close back at the select's client rect; rows are painted, not layout boxes. Source of decision/open-list-rows-own-bounds; a row's own `bounds` was read as outside the widening, a closed set inside a ratified crossing by the 2026-10-10 field rule — precedent for which new answer fields are the founder's. Trap: an open list's rows have no Taffy layout to explain.

@history_item n=10 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (an open list's rows, the exception to the scrolling boxes rule) — a row/group of an open select list is seen through the list's scrollport alone, `scroll_into_view` writes the list's `scroll_y` at once. Source of the `except:` clauses on decision/scroll-into-view-moves-every-scroller and decision/visible-region-through-clipping-boxes; the new read must handle these non-layout-placed rows (a placement kind not from Taffy, like the reserved canvas case).

@history_item n=11 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the lint's `overlap` exclusion, `Entry.in_select_box`) — snapshot gives a closed select's option its select's client rect, an open row its own bounds; `clip_region` takes an open row's region from `visible_region` alone. Bears on the scope's lint link: the lint's per-element readings the explanation would tell the cause of.

@history_item n=12 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the flight booker's boot lint reading, a closed select's options skipped) — `stand_lint` `at_boot` `[0, 0, 0, 0]` in both modes; the former select-option overlap retired (founder's F9). Closed select options placed at the select's client rect are a placement case the read must explain without inventing layout.

@history_item n=13 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the Cells grid, a keyboard region and one Tab stop) — `cells-grid` takes id, `tabindex="0"`, `aria-label` "Cells", no role; arrows bring a cell into view by synchronous `MountedData::scroll_to`; async form panicked (`RefCell already borrowed`). Source of contract/stand-element-ids/cells-grid; the grid is a scrolled stand case for the read, and the RefCell trap applies to any read reaching into the document during a handler.

@history_item n=14 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — the stand reads clean to the visual lint; after twelve Creates `crud-person-14` reads `clipped`, visible 367 × 11.203125 inside `crud-list`, agreeing with the driver's `off-screen` refusal. The observed gap (lint names a finding, not its cause) starts here; this clipped row is a ready stand case for the read to explain.

@history_item n=15 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the lint's PROVISIONAL mark dropped on the founder's «Новая команда lint» and F2 «Без поля (Рекомендую)» (no field). Precedent for P4's choice whether the lint's answer points to the new read: the founder previously chose no extra lint field.

@history_item n=16 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: `escher-session lint`) — `lint` became the eighth instance verb on every carrier, kept under `--record`, MCP listing after `screenshot`, ratified by the founder (F1). Precedent the scope names: a read-only verb added on every ratified carrier as a boundary widening.

@history_item n=17 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (escher-session gains the graph verb) — thirteen verbs, `graph` the ninth instance verb after `lint`, answer carrying ids, closed words and booleans only, never a value; usage line and MCP listing updated. Precedent for the new read's verb shape and for the field rule (closed words and numbers, no authored text).

@history_item n=18 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** (ratified — escher-session gains the graph verb; ratified — the coverage role a further consumer of the TaskShell mount; ratified — escher-session's coverage role) — the graph verb and the coverage role moved from PROVISIONAL (delegate) to ratified by the founder's own word. Shows how a delegate confirm made while the founder is away stays PROVISIONAL until his word — the situation this chunk is in (forks to `founder-forks.md`).

@history_item n=19 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (escher-session's tenth instance verb actions) — fourteen verbs, `actions` the tenth instance verb after `graph`, answer unpaged, closed words only; MCP lists fourteen tools, `help` last. Current state of contract/escher-session-command/form and contract/escher-session-mcp/tools: a new verb moves both to fifteen and must keep `help` last; the latest precedent of a read verb widening.

@history_item n=20 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — every command is `escher-session <verb> [arguments] --session <dir>` with four endings (0 accepted one uncoloured JSON line, 1 refused, 2 usage fixed stderr line, 3 session error); former shapes refused `unknown-verb`. Source of contract/escher-session-command/form, the exit-code rows and refusals/unknown-verb that a new verb extends.

@history_item n=21 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role, a further consumer of the TaskShell mount booting through the same boot; «no ending writes back anything the caller typed» scoped to the commands. Source of decision/no-caller-input-echoed and the MCP tools listing a new verb joins; an `--id` the caller typed must not be echoed in an ending.

@history_item n=22 marker=2026-10-12-self-description
**2026-10-12-self-description** — `escher-session help` answers `describe()`'s seven-key line of fixed table text; MCP tool descriptions are the verb's help plus result fields. Source of contract/escher-session-help/answer and the tools/list description form; a new verb needs fixed help text and result fields in `describe.rs`.

@history_item n=23 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` with optional `--id` (the element's visible part), refusals `fonts-unavailable`/`off-screen`; MCP result holding an image bounded at 1,048,576 bytes by its whole written form. Source of contract/escher-session-mcp/result-bound; precedent for an `--id`-taking read verb and its refusals.

@history_item n=24 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part` paging (1–1000), out-of-bound refused `malformed`, founder's F1/F2. Precedent if the explanation answer can grow large (a deep layout chain); `actions` chose unpaged instead.

@history_item n=25 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `--record <dir>` on `start`, host and `mcp` roles: one owner-only replayable check, `check-{n}.json` 0600 in a 0700 dir, session errors `record-dir-not-private` and `recording-failed` (exit 3), `help`/`tools/list` the same with or without it; founder-ratified widening. Source of the recording contract rows; a new verb's step must be kept and replayed.

@history_item n=26 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the kept check holds per call verb, arguments, acceptance, answer digest and the masked answer, replayed by `Check::replay`. Source of decision/session-recording-replayable-check: the new read's answer is recorded and digest-replayed, so it must be deterministic across fresh boots.

@history_item n=27 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role a further consumer of the TaskShell mount; escher-session's coverage role) — `coverage <app> <check>...` replays kept checks on fresh boots through the same boot, adding no markup; held instance still a fresh boot in both modes. Part of decision/session-roles-share-stand-boot; kept checks containing the new verb are replayed by this role too.

@history_item n=28 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the host and `mcp` roles boot six served tasks (`counter` · `flight-booker` · `timer` · `crud` · `cells` · `load`), `cells` by `boot_cells`; the snapshot gives each select option its select's client rect. Source of contract/escher-session-served-tasks/cells and timer; the stand cases the read is proven on.

@history_item n=29 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the master's text became rows (121 rows: decision 21 · contract 99 · absence 1), each verified against the code. Every picked row's handle dates from here; this chunk's amendments name rows by handle (`+`/`~`/`−`).

@history_item n=30 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — `~ contract/agent-run-cli/usage`: `agent-run.sh run session` runs the escher-session binary's flows in seven_guis. Latest row-form amendment and the route by which a new verb's binary-level proof would run.
