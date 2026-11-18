@history_item n=1 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — measured the agent-run 5-command contract (exit grammar 0 · 1 · 2 usage before precondition · 3; an empty run never a pass; JSON-line events printed and appended to `events.jsonl`). The cold-agent pipe copies this exit grammar and event form (rows contract/cold-agent-exit-codes/*, decision/empty-run-is-never-a-pass); any new verdict field or reason must keep usage checked before any precondition.

@history_item n=2 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — first cold-agent pipe block in §3 and `test_cold_agent.py` in §4: `.ps1` a pass-through, per-run `mktemp -d` session, isolation flags, verdict with positive-evidence `passed`, `wrong_calls` recorded but never deciding, proof = contract tests + one live run never in CI. Origin of rows decision/cold-agent-live-run-off-ci, decision/cold-agent-tests-offline, contract/cold-agent-invocation/windows-entry; this chunk's harness work must stay offline under the shim.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — `stand_session_quiet` proves only that a host with no sink writes nothing; a sink-installing host at debug/trace is a separate claim (row decision/log-silence-read-on-a-sink-host). Trap: a sink host at `trace` overruns an undrained pipe — relevant if the widened pipe's fake or live server log is read at higher levels.

@history_item n=4 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `host_log` covers the sink-installing host at `RUST_LOG=trace` (needles read from the stand, both streams drained, failure printing counts only). Basis of decision/host-stderr-no-screen-content and decision/session-logs-no-screen-content: any `cause` counting this chunk reads from `server.log` must stay a count, never content.

@history_item n=5 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the closed cause set and a refusal's printed form pinned in `refusal.rs`; every table stated with its row count asserted. Precedent for the closed-word cause counts this chunk adds to the verdict (rows contract/refusal-causes/cause-set, decision/eight-refusal-causes-in-order).

@history_item n=6 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — the lookup answers `stale` or `not-found`, refusal order `disabled` · `off-screen` · `covered`; nine `stand_act_*` files. Defines the causes a by-cause wrong-call count would read (rows contract/driver-session/refusal-order, contract/driver-session/stale-or-not-found, contract/refusal-cause/*).

@history_item n=7 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the closed span line (event prefix, `span={name}`, fields, `message="close"`, `time.busy`/`time.idle`) enters the stand log format; one command span per call. Trap/rule: a check reading a sink's capture counts lines by target, never in total, and a green per-package run is not the workspace leg's green (rows contract/stand-log/span-line, decision/sink-capture-counted-by-target). The pipe's span-line counting of `server.log` rests on this line form.

@history_item n=8 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the stand log's second record class (closed span lines) ratified by the founder, no longer PROVISIONAL (row decision/closed-spans-logged-as-lines). The pipe may rely on span lines as a stable read; reading the `command` span's `cause` field adds no log field (scope boundary).

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (test strategy and harness contract) — a command's stdout is its answer and nothing else; the session host writes one command-span line per hosted call; session-wire's crossing ratified. Rows contract/cli-command/stdout, contract/session-host-stderr/command-span, decision/socket-carries-commands rest here.

@history_item n=10 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — the `mcp` role writes one `tool_call` line per `tools/call`, `tool` and `ending` only, parent of `command` (founder-ratified span); first live MCP trial. Basis of rows contract/stand-log/tool-call-span, contract/tool-call-span/ending, decision/mcp-capture-counts-by-span. Trap: `tracing` callsite interest is process-global — a `with_default` check needs its own recorder per test.

@history_item n=11 marker=2026-10-12-self-description
**2026-10-12-self-description** — a `help` call writes one `tool_call` line, `tool="help"`, with no `command` beneath (row contract/tool-call-span/help). A per-verb count from command lines must not expect a `command` child under `help`; counts by `tool_call` vs `command` differ by design.

@history_item n=12 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — `screenshot` joins `command.verb` and `tool_call.tool` with no field added; a shot's line holds no image byte; `fonts-unavailable` refuses a no-font shot (rows contract/command-span/screenshot, contract/driver-refusals/fonts-unavailable). `screenshot` is one of the pipe's existing `COUNTED` verbs.

@history_item n=13 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (§3 pipe on the escher server) — one task `stand`, `run counter` a usage error; `RUST_LOG=info` on the server child only; `STAND_PROMPT` names no id or slug; the check judged by full and ablated replays on fresh children; `counts_agree` = span lines vs tool uses AND refused+error vs `is_error`; `wrong_calls` = refused + error + foreign + denials; no check text reaches an event (fourth planted marker). Founder-chosen design for rows decision/cold-agent-replayed-check, contract/cold-agent-verdict/counts-agree, contract/cold-agent-tasks/stand, contract/cold-agent-invocation/environment — a widened prompt or a second task must keep these.

@history_item n=14 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (§1, §4 contract tests) — `ColdAgentPipeTest` under a `claude` shim and a fake `target/debug/escher-session` that writes `tool_call` span lines and records its spawns; no live model, credential or built binary; the cases read red against the parent pipe. The harness this chunk extends; red-before-green against the base pipe is the standing control.

@history_item n=15 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — causes nine → eight, `fonts-unavailable` the eighth (`the_eight_causes_are_named_in_order`), `time-unavailable` retired; `advance` accepted on every session. Fixes the closed cause vocabulary a by-cause count may use (rows decision/eight-refusal-causes-in-order, decision/advance-accepted-on-every-session); later ratified by the founder (amd-oodkokb2, 2026-10-18-engine-debts).

@history_item n=16 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — `mcp --record <dir>`, the recording's caps (400 records, 1 MiB), `recording_failed()`; wire `v3`. Basis of the pipe's `recorded_sessions` / `recording_kept` and reason `recording-not-kept`.

@history_item n=17 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — `Check::replay` runs each step through `Session::run`, signature and span unchanged, result types holding no reading or JSON type (row decision/replay-through-the-call-path). Any replay counting this chunk does must not touch the call path.

@history_item n=18 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part) — `lint` becomes the twelfth verb; the cold-agent pipe's server, `valid_step`, `isolated` and `FAKE` take twelve tools. Precedent for widening the pipe's verb list in lockstep with the fake; `lint` is one of the existing `COUNTED` verbs.

@history_item n=19 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — `snapshot --part`, `Outcome::Part`; the paged screen read the pipe's `tasks_met` joins and its `part_reads` counts (row contract/cold-agent-verdict/tasks-met).

@history_item n=20 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `CapabilityStatementTest` pins `docs/capabilities.md` and README's driver-command count; claims mapped to checks, only names and counts pinned, a renamed check changing it by hand (row decision/capability-claims-name-their-checks). F2 (what the run may claim on README/capabilities) edits text this pin reads.

@history_item n=21 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six apps by the 5 % bar, the second live run) — `stand` names six apps; `server.log` counted for ` span="tool_call" ` lines, refused/error endings and accepted lines of each counted verb (`lint` · `screenshot` · `advance`); verdict gains `wrong_call_bar_pct`, `{verb}_calls`, `part_reads`, `recorded_sessions`, `recording_kept`; reasons add `capability-unused` · `recording-not-kept` · `wrong-calls-over-bar`; `wrong_calls × 100 ≤ 5 × tool_calls` decides. Live run: passed, 66 calls, 0 wrong, 0.8244 USD, twelve tools. The 0.2.0 baseline (v030-15) and the exact counting pattern this chunk widens (rows decision/cold-agent-wrong-call-bar, decision/cold-agent-passes-on-positive-evidence, decision/cold-agent-server-log-counted-only, contract/cold-agent-verdict/reasons).

@history_item n=22 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (counts and contract tests) — `test_cold_agent.py` 33 → 37: the fake answers Cells as a paged two-part line and keeps one check per accepted start under `--record`; cases: six-app pass, 5 of 100 wrong passing and 6 failing (the bar's edge), each unused capability (four sub-cases), a start with no kept check; 34 of 37 red against the base pipe. Template for this chunk's new count, cause and `capability-unused` cases and for pinning the bar at its edge whatever its value.

@history_item n=23 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the flight booker's select) — `flight-type` became a select; its two options refused `covered` at the select's client rect in `stand_act_ids`; the agent's flight task chooses the return booking on the flight type control. Explains why `covered` refusals on the select can move `wrong_calls` in a live run — the cause this chunk reads by cause.

@history_item n=24 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (served pages and slugs) — Cells (`stand::boot_cells`, 5+ parts) and the load page (`stand::boot_load`, `LoadNet`) served; `cli_commands` starts all six slugs; `host_log` serves `cells` for a paged part read. The six apps the prompt names and the paged answers the fake must mimic.

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`covered`'s closed-option clause) — `covered`'s fixed meaning adds that an option of a closed select lies under it — click the select first, then the option; name, remedy and order unmoved (row contract/refusal-cause/covered). The scope's CARRY: this clause and the select list can move `wrong_calls`, unread at that chunk; this chunk reads them.

@history_item n=26 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what the list leaves unmeasured) — the select list paints through `paint_scene` alone, driven headlessly by harness, session and binary; windowed path read, not run (row decision/select-list-painted-by-paint-scene). The cold run reaches the list only over the headless MCP carrier.

@history_item n=27 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (cold-agent pipe at thirteen verbs) — `graph` joins the pipe's and the test's `VERBS`, fake answers `{"nodes":[],"states":[]}`, `isolated` on thirteen tools, `capability-unused` unchanged because `graph` is required of no cold agent. The CARRY this chunk decides: whether the 0.3.0 bar counts `graph`.

@history_item n=28 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (spans, answer, socket) — `graph` a word of `tool_call.tool` and `command.verb` closed sets, no span field; its command line records `verb` alone (rows contract/command-span/graph, contract/command-span/verb); a graph answer holds ids, names, flag words and booleans, never a value (row decision/graph-answer-holds-no-value); the graph answer crossing the socket was a provisional boundary widening. Bears on F3: a live run asking for `graph` carries these names to the model provider.

@history_item n=29 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (registry and reserved form) — the stand registers task signals through `use_graph_signal` with no dioxus-core change (row decision/graph-signal-registration); no served id holds `{owner}///{key}` (row decision/reserved-graph-id-form).

@history_item n=30 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — `actions` joins the pipe's `VERBS` (fourteen), uncounted, test count staying 37; `stand_actions` and the answer holding no value (row decision/actions-answer-holds-no-value); `cli_commands`/`mcp_server`/`stand_mcp_tools` refuse an argument (`isError` over MCP; rows contract/mcp-server/actions-argument, contract/mcp-actions/argument); `host_log`/`mcp_log` and `stand_act_spans` read `actions`' command line recording `verb` alone (row contract/command-span/actions); one actionable rule from current listeners (row decision/one-actionable-rule). The CARRY on whether the 0.3.0 bar counts `actions`.

@history_item n=31 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — handler coverage is the `coverage` command-line role (founder's carrier choice, later ratified at 2026-11-15T05-21-42-wrap), no MCP tool; `stand_handler_coverage` replays kept checks on fresh sessions; three stated limits (canvas, unreached screen, bubbling dispatch) mapped in the capability statement; screens read ratified at 2026-11-13T07-43-19-wrap. Supports the scope's claim that handler coverage is out of a cold agent's reach, its headless witness standing (rows decision/handler-coverage-by-replay, contract/escher-session-commands/coverage, contract/handler-coverage-limits/*).

@history_item n=32 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — decision/stand-checks-bundled-font gains guard `stand_bundled_font_families`; bundled fonts implicit in every answer — the scope names them as no cold-agent-reachable capability.

@history_item n=33 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — stand checks assert unconditionally under bundled DejaVu Sans with system fonts off, the pre-existing font-dependent tests alone skipping (rows decision/stand-checks-bundled-font, decision/stand-checks-bundled-font-no-skip).

@history_item n=34 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — `Harness::from_vdom` provides `SeededRandom::from_seed(DEFAULT_SEED)` where the root holds none (row contract/harness-construction/from-vdom); the seed is in-process only, out of the cold run's reach.

@history_item n=35 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — decision/mutation-controlled-tests: a control removing a bound loosens it to twice the budget instead. Applies to controlling the bar-edge and count cases this chunk adds.

@history_item n=36 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — overview/session-lifecycle restated: the session is no verb of the agent-run contract; `run session` runs `cli_flow` flows. Context for keeping the cold-agent pipe's own selection set (`stand` only) separate.

@history_item n=37 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — `contract/session-layout/changes-nothing` added (a layout read changes nothing, both modes); `layout` is one of the 0.3.0 reads the widened prompt may ask for.

@history_item n=38 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — acting verbs return `caused` where the step caused something (row contract/driver-session/acting-verbs); root-boundary Err checked on its own session (row decision/root-boundary-err-on-own-session). The scope's open question: whether any served stand task causes a `caused` item a cold agent could read.

@history_item n=39 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** — the socket carries a step's `caused` with its error text; `<payload unread>` fixed-text panic item; root Err text crosses in `caused`; MCP tool result decoded whole (Windows red); engine caused items asserted only on a tracing build (rows decision/socket-carries-commands, contract/driver-session/payload-unread, decision/engine-items-asserted-on-tracing-build). Any `caused`-reading in the pipe or fake must respect the build-conditional rule.

@history_item n=40 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (4. Unit Test Strategy) — decision/expected-value-pinned-as-literal: a check pins its measured value as a literal, never derived from the constant its control mutates. Directly binds the bar-edge tests: pin the edge cases as literals, not computed from `WRONG_CALL_BAR_PCT`, or a mutated bar reads green.

@history_item n=41 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `layout` the fifteenth verb on every carrier (tools/list, command form, usage line, reading verbs, verb table, socket carrying layout answers, `contract/session-run/layout-answer`); the cold-agent pipe held at fourteen by a provisional row (decision/cold-agent-verbs-without-layout); decision/count-and-rename-sweeps added after a missed numeric pin and a missed renamed witness. Trap for this chunk: sweep every count site (`fifteen`, verb lists) when widening counted sets.

@history_item n=42 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — founder's word «Добавить сейчас»: `layout` joins the cold-agent pipe, contracts cold-agent-check/step and cold-agent-verdict/isolated move to fifteen; decision/cold-agent-verbs-without-layout removed with no heir (its uncounted read the obs-plan's); delegate confirms ratified. Also adds component-alone rows (from-component-alone, vdom-alone, boot-alone, props as maker, root context before build, pinned viewport from document) — in-process only, out of the cold run's reach. The CARRY on whether the bar counts `layout`.

@history_item n=43 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — decision/requirement-witness-inside-stand-repeat added (every 0.3.0 requirement names its witness inside stand-repeat, pinned by function name; a requirement with no screen names none); decision/typed-password-never-returned widened to graph, actions and layout answers and kept/explored checks, guard `stand_masks_on_new_answers`. v030-15's own witness and what a live run's answers may carry rest here.

@history_item n=44 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** — the capability-statement pin also holds the a11y leg's file count and stand-repeat's named-target count against `ci-leg.sh`; the claim section's names and counts pinned, a renamed check changed by hand. Any F2 edit to `README.md` or `docs/capabilities.md` from this run's result reddens `ci-scripts` unless it keeps pinned names and counts (row decision/capability-claims-name-their-checks).
