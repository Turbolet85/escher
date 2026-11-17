@history_item n=1 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — added the headless stand: it skips Home and mounts one task in TaskShell through `task_in_shell` at the pinned 800 × 600, scale 1.0, Light viewport with the bundled DejaVu Sans (row `decision/headless-stand-pinned-viewport`). The isolated mount must reuse these exact boot conditions, but mount one component instead of a task in TaskShell, so it either extends this decision or needs a stated sibling to it.

@history_item n=2 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** — re-pointed the harness-defaults citations to the `Default` impl that holds 800x600, scale 1, light, and to the viewport built from them. The isolated mount goes through `Harness::from_vdom` and must get its pinned viewport from these same defaults.

@history_item n=3 marker=2026-10-06-stable-element-ids
**2026-10-06-stable-element-ids** — gave the lean tasks' controls author `id`s as their stable element ids, adding no class, style, wrapper or order. The scope's equality check rests on these ids: an isolated id may differ from the in-app id only by a component-path prefix, so the in-app ids are the baseline.

@history_item n=4 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** — changed the CRUD rows' key from index `{i}` to `{person.id}`, so a row's id follows its person. If CRUD is the isolated component, its rows' ids must read the same way under the stated equality (row `contract/stand-element-ids/crud-person-id`).

@history_item n=5 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — gave CRUD rows the author id `crud-person-{person.id}` and Home's cards `task-card-{slug}`, as attributes only, on the founder's rule that every actionable element reads an author key. Any isolated mount must keep every actionable element keyed (row `decision/every-stand-control-keyed`), and the ids it compares across mounts are these author ids.

@history_item n=6 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — keyed the non-lean tasks (temp, circle drawer including its open dialog, Cells' 676 cells and `cell-input`) with ids added as `id:` lines alone; `unkeyed_actionable()` reads 0 on all seven tasks. This sets the bar of zero unkeyed elements that an isolated component's document must also meet, and it fixes the ids that rows `contract/stand-element-ids/*` name.

@history_item n=7 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** — named the stand inputs by attributes alone (`for`, `aria-label`), changing no element. The snapshot equality compares names, so the isolated mount must carry these same attribute-borne names. Matches row `decision/stand-a11y-by-attributes-alone`.

@history_item n=8 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — stated roles and states by attributes alone (`aria-invalid`, `role="listbox"` and `role="option"` with `aria-selected`), the styling classes staying. The isolated snapshot's roles and states must equal the in-app ones, and these are the attributes it reads.

@history_item n=9 marker=2026-10-14-stand-a11y-assertions
**2026-10-14-stand-a11y-assertions** — named `crud-list` "People" by `aria-label` alone. This is a name that the snapshot equality for CRUD reads (row `contract/stand-element-ids/crud-list`).

@history_item n=10 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — gave `crud-list` `tabindex="0"` and keys, a six-stop Tab sequence starting at `back-btn`. An isolated CRUD component has no TaskShell `back-btn`, so its Tab sequence would differ from the in-app one by construction. P3/P4 must state this in the equality.

@history_item n=11 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** — relevant sub-entries: `cells-grid` as one Tab stop and keyboard region named "Cells" with no role (row `contract/stand-element-ids/cells-grid`); the load page's `LOAD_TAB_ORDER` and `load-fetch` taking Enter and Space (row `contract/stand-element-ids/load-fetch`); the lint at boot reading `[0, 0, 0, 0]` in both layout modes (row `decision/stand-lints-clean-at-boot`); citations re-pointed into `dioxus_document.rs` for the document-skeleton note. These are the readings a Cells or load isolated mount is compared against, and the lint baseline that must hold.

@history_item n=12 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — two sub-entries: the stand reads 0 lint findings at boot in both layout modes; and the gated part, the `lint` verb added on every carrier. The first is the baseline of row `decision/stand-lints-clean-at-boot`. The second is a precedent for how a new verb lands on the command line, MCP and `--record`, if the founder's fork opens a carrier for the isolated mount.

@history_item n=13 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — kept the flight booker's boot snapshot as `ComboBox "Flight type"` with two ListBoxOptions at the select's client rect, and gave an open list's rows their own bounds. Row `decision/flight-type-select-boot-reading` pins this reading. If the flight booker is the isolated component, its options' bounds are select-rect bounds, and the equality has to account for that.

@history_item n=14 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — measured the nested scroller `crud-list` (24, 112.796875, 369 × 419.203125 at boot) under `task-body` (0, 46, 800 × 554). These bounds come from the TaskShell placement, so an isolated CRUD mount's bounds differ by construction. This is the scope's «bounds where the component's box is the same» caveat.

@history_item n=15 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — named the session host a third consumer of the `task_in_shell` mount, booting through `seven_guis::stand` only and adding no markup, id, class, style, wrapper or order. Row `decision/session-roles-share-stand-boot` descends from this. Any session role that boots an isolated component must go through the same stand boot.

@history_item n=16 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — made `escher-session <verb> [arguments] --session <dir>` the command form, with `start <task>` and the four endings (0/1/2/3). If the fork opens a carrier for the isolated mount (a `start` argument or a verb), it extends this form and row `contract/escher-session-command/form`.

@history_item n=17 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `mcp` role: local over stdio with no listener (row `decision/mcp-server-local-stdio`), booting a task on a `start` tool call through the same boot. It also scoped «no ending writes back caller input» to the commands. An isolated mount over MCP would extend the `start` tool and must keep the stdio-only boundary.

@history_item n=18 marker=2026-10-12-self-description
**2026-10-12-self-description** — added `help` (seven keys, fixed table text) and set `tools/list` descriptions to the verb's help plus its result fields. A new verb or argument for the isolated mount must update `help` and the tools listing (rows `contract/escher-session-help/answer` and `contract/escher-session-mcp/tools`).

@history_item n=19 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — refuses `screenshot` on an app with no font as `fonts-unavailable`, exit 1 (row `contract/escher-session-refusals/fonts-unavailable`). The isolated mount uses the stand's bundled font, so it must keep this refusal.

@history_item n=20 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added `--record <dir>` on `start`, the host and `mcp`, keeping one owner-only replayable check, as a founder-ratified widening. If the fork brings the isolated mount onto a kept check, it extends this, and row `decision/session-recording-replayable-check` binds that.

@history_item n=21 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — made the kept check hold, per call, its verb, arguments, acceptance, answer digest and masked answer, replayed by `Check::replay`. This is the replay contract that an isolated-mount step in a check, or a `layout` step written by an agent, must satisfy.

@history_item n=22 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — set the `snapshot` answer as `{"text"}` or `{"text","part","parts"}` (row `contract/escher-session-snapshot/answer`). The isolated reading compares snapshots, so this is the snapshot form it reads.

@history_item n=23 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — served six tasks (counter, flight-booker, timer, crud, cells, load) through `boot_cells` and `boot_load`, named by the `start` tool's `enum` and the usage line's `apps:`. It also changed the flight booker to a select, and the cold-agent pipe's six apps now run `mcp --record` into `target/cold-agent/record`. This is the closed task table that the fork's «closed table of named components on `start`» option would extend, and the precedent for adding a boot path (rows `contract/escher-session-served-tasks/*`, `contract/cold-agent-cli/record-dir`, `decision/load-page-only-network`).

@history_item n=24 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — added `scripts/cold-agent.sh`: usage on stderr exits 2 before preconditions, stdout JSON lines only. The chunk adds `layout` to this pipe's verb list.

@history_item n=25 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — moved the pipe onto the driver's own MCP server `escher`, with a fixed `STAND_PROMPT` naming no element id or app slug, and made it build nothing (exit 3 without the binary). These are rows `decision/cold-agent-uses-driver-mcp`, `decision/cold-agent-prompt-names-no-id` and `decision/cold-agent-pipe-builds-nothing`. Adding `layout` to the pipe must keep all three.

@history_item n=26 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — added the `graph` verb, which carries component names among ids and closed words. Its citations were re-pointed after `use_graph_signal` and `dioxus_document.rs` additions. The isolated mount names its component across this graph's boundary (`DioxusDocument::graph`, row `contract/escher-session-graph/answer`). `graph` is also one of the verbs that joined the cold-agent pipe before `layout`.

@history_item n=27 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — three entries move from PROVISIONAL to ratified on the founder's own word: the graph verb, the coverage role as a mount consumer, and the coverage role. This is the precedent for this chunk's wrap lifting the delegate marks on the 24 widening operations.

@history_item n=28 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — added the tenth instance verb `actions` (fourteen verbs, MCP tools in table order, `help` last) on a founder-ratified crossing. This is the precedent for a verb joining every carrier, and `actions` is one of the verbs whose joining the cold-agent pipe the scope cites (row `contract/escher-session-actions/answer`).

@history_item n=29 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — four sub-entries. The `coverage` role becomes a further consumer of the TaskShell mount, booting through the same boot and adding no markup. It replays each check on a fresh boot, with check files read bounded and refused `malformed` without echoing. The bridge's skeleton notes in `dioxus_document.rs` are re-cited. Rows touched: `decision/session-roles-share-stand-boot`, `contract/escher-session-coverage/*` and `decision/fixed-document-skeleton-main-mount`. An isolated document must keep the fixed html/head/body/main skeleton, or state why it does not.

@history_item n=30 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — converted the master's text into rows (121 rows), each verified against the code. Every picked row's handle dates from this conversion, so rows this chunk changes are amended as row deltas (`+`/`~`/`−`).

@history_item n=31 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — added `session` as a closed selection word to `agent-run.sh run`, running the escher-session binary's flows. Isolated-mount flows or the `layout` pipe tests may run under this selection.

@history_item n=32 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — the verb table grows to fifteen with `layout` (MCP tools have `layout` after `actions`). It adds `contract/escher-session-layout/answer` and gives `decision/no-caller-input-echoed` an exception: the layout answer echoes its id. This is the immediate predecessor. The cold-agent pipe still names fourteen verbs, so its isolation check against the fifteen tools reads red, and this chunk adds `layout` to the pipe. That widening's 15 delegate marks are lifted at this chunk's wrap.
