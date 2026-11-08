@carried n=1 label=test-plan/integration-test-strategy fact=fct-t7rg232i
why: the read names, per verb, the one refusal cause a call would return, so it must name only causes of this closed set, agreeing with the refusal a real call returns, and run nothing, leaving the instance unchanged.
quote: a refused call — any of the eight causes (ratified by the founder, 2026-10-18, `advance` accepted on every session): `unknown-verb`, `malformed`, `not-found`, `stale`, `disabled`, `off-screen`, `covered` and, since 2026-10-12-headless-screenshot, `fonts-unavailable` — leaves the instance unchanged and reads as a text holding none of the bytes the call supplied, and for each of the five the executor detects on the screen a refused `click` and a refused `type` leave the snapshot and both focus readings equal;
@carried n=1 run=2 label=test-plan fact=fct-gq2zcc44
quote: a refused call leaves the instance unchanged and reads as a text holding none of the bytes the call supplied
@carried n=2 label=test-plan/integration-test-strategy fact=fct-t7rg232i
why: the read's `covered` reason must agree with these measured readings, including a select's option under its closed select (an owner-served node), in both layout modes.
quote: two readings stay as measured, each stated in `covered`'s fixed meaning, which also says an option of a closed select lies under it and to click the select first, then the option: a box with no width or no height is hit at its centre on the element that holds it, so `covered` can read where nothing covers it (`stand_act_ids`), and a box clipped on one axis stops a hit at its whole padding box, so a target lying outside it along the axis it does not clip is refused `covered` though it shows, the instance unchanged, in both layout modes
@carried n=2 run=2 label=test-plan/integration-test-strategy fact=fct-6jloyoi7
quote: an option under a closed select's group refused `covered`
@carried n=2 run=3 label=test-plan/integration-test-strategy fact=fct-gkuwtzih
quote: a closed option and a button under the list refused `covered`, a row off screen until `scroll`
@carried n=3 label=test-plan/integration-test-strategy fact=fct-t7rg232i
why: the read's `off-screen` reason must agree with the stand's measured off-screen row, and the read must change no scroll, so a row it lists as off-screen stays so until a `scroll`.
quote: on the stand a CRUD row past its list's box is refused `off-screen`, brought into view by `scroll` and then clicked, the row's centre read against the list's `bounds` taken once, before the scroll
@carried n=4 label=test-plan/integration-test-strategy fact=fct-t7rg232i
why: the scope names the read's `command` span recording `verb` and, refused, `cause` alone, so a call of the read must leave exactly this one fixed-words line, read by `stand_act_spans`.
quote: leaves exactly one `escher_driver` line holding fixed words and counts only (its verb, a refused call's cause, an acting call's settle reading and the three lengths of the returned diff) and none of the ids, names, values or typed text it handled, and at the default level no driver line — `stand_act_spans`
@carried n=4 run=2 label=test-plan fact=fct-lm4npbmj
quote: with escher's sink installed at `info`, each call run through `Session::run` leaves exactly one `escher_driver` line holding fixed words and counts only, none of the ids, names, values or typed text it handled, and at the default level no driver line
@carried n=5 label=test-plan/integration-test-strategy fact=fct-csattoyz
why: the read's node model is the UI graph's, so its listed nodes, handlers and owners must be the graph's, each a snapshot node, and like a graph read it must change nothing and hold no typed text.
quote: the served screens' handlers, owners and clicks; every graph node a snapshot node, no served id holding the reserved `{owner}///{key}` form; a graph read changing nothing and holding no typed text
@carried n=5 run=2 label=test-plan fact=fct-gq2zcc44
quote: the `graph` verb reads as `DioxusDocument::graph` on every served screen in both layout modes and changes nothing (`stand_graph`)
@carried n=6 label=test-plan/integration-test-strategy fact=fct-eja3jr5q
why: the read's answer carries ids, verb names and cause words, so it must hold no value, a typed sentinel occurring 0 times in it, and an argument it does not take must be refused `malformed` with nothing run, on the same four checks.
quote: a `graph` answer carries ids, component names, handler event names, state names, closed flag words and booleans only, never a value — a sentinel typed into `crud-name` occurs 0 times in it — and an argument to `graph` is refused `malformed` with nothing run (`stand_graph`, `cli_commands`, `mcp_server`, `stand_mcp_tools`)
@carried n=7 label=test-plan/integration-test-strategy fact=fct-5t64ong4
why: the read on the MCP carrier must be held in process in both layout modes, its `tool_call` span parenting one `command` span recording its verb alone.
quote: holds a `graph` tool call in process in both layout modes, answering the reader's graph, its `tool_call` span parenting one `command` span that records `verb` alone
@carried n=8 label=test-plan/integration-test-strategy fact=fct-adkwyvky
why: a new verb on the host and the `mcp` role is driven by `host_log` and `mcp_log` at `trace`, so their call and command-span counts move and their 0-needle readings must hold over the read's ids and verb names.
quote: each also drives one `lint`, one `snapshot` part read and one `graph` call — `host_log` eight calls and 8 command-span lines, `mcp_log` eleven tool calls and 9 `command` lines — still reading 0 needles on stderr at `trace`
@carried n=8 run=2 label=test-plan fact=fct-kudmubnq
quote: a host that installs escher's sink and the `mcp` role write no id, name or typed text on stderr at `trace`
@carried n=9 label=test-plan/integration-test-strategy fact=fct-xs6og5lj
why: the actionable set counts an element with a listener, so the read must take its handlers from a reading that reports current listeners, never a listener attribute left standing after a re-render — the fork P3 measures.
quote: so a listener removed on a re-render leaves its attribute and the mark standing while a click no longer acts, and a `mounted` listener writes none; the vdom walk's dynamic attributes, `VNode.dynamic_attrs` with `VNode::mounted_dynamic_attribute`, read each element's current listeners exactly, `mounted` included
@carried n=10 label=test-plan/e2e-test-strategy fact=fct-csryo2zo
why: the read on the command line must be run as a process of the binary and answer one line, as `graph` does in `cli_commands`.
quote: `graph_answers_one_line_of_the_ui_graph` runs `graph` as a process and reads one line of the UI graph (examples/seven_guis/tests/cli_commands.rs:467-492)
@carried n=10 run=2 label=test-plan fact=fct-3c5ihsc6
quote: each driver command is run as a process of the package's own binary against a host its `start` command spawned — `cli_commands`, the two shell flows of `cli_flow` and `host_timer`
@carried n=11 label=test-plan/e2e-test-strategy fact=fct-csryo2zo
why: the self-description is edited, so `help` must stay equal to `describe()` with the read in it, and every cause the read names must keep its table remedy.
quote: since 2026-10-12-self-description `help` answers status `0` and one line equal to `describe()`, reaching no session, every cause the binary provokes (`unknown-verb`, `malformed`, `disabled`, `not-found`, `off-screen`, `stale`) reads `remedy` equal to its cause's
@carried n=12 label=test-plan/e2e-test-strategy fact=fct-csryo2zo
why: recording and replay are edited for the read's step, so the committed kept checks must stay byte-equal to what the binary records.
quote: reads both kept checks byte-equal to `tests/blitz-tests/checks/counter-{cli,mcp}.json`, the scripts holding no `screenshot` step since a shot's PNG bytes differ by OS (CI#39225375577)
@carried n=12 run=2 label=test-plan fact=fct-3c5ihsc6
quote: a recorded session run as a process keeps its check, and the committed checks are read byte-equal to what the binary records
@carried n=13 label=test-plan/e2e-test-strategy fact=fct-gtzxj2kr
why: a new verb on the MCP carrier moves the ordered thirteen-verb listing `mcp_server` reads, and joins it as a tool call answering its line.
quote: `tools/list` offering the verbs in order — thirteen, `graph` after `lint` — `start`, `snapshot`, `click`)
@carried n=13 run=2 label=test-plan fact=fct-pnkaq5az
quote: the binary's `mcp` role is spoken to as a child over piped stdin through the shared `Speaker` (`mcp_server`, `mcp_log`) and in process in both layout modes (`stand_mcp_tools`, `stand_mcp_masked`)
@carried n=14 label=test-plan/e2e-test-strategy fact=fct-gtzxj2kr
why: the read joins the self-description, so `help` must stay the same bytes through MCP and the command line.
quote: since that chunk the newest revision `2025-11-25` spoken and `help` answered before any `start`, and, unix only, `help` the same bytes through MCP and the command line
@binding n=1
**integration-test-strategy ↔ e2e-test-strategy**: the read's per-verb refusal causes must be the ones an actual call returns on the stand in process (test-plan/integration-test-strategy fct-t7rg232i) and on the command line, where a refused call ends with its cause at status `1` and keeps its remedy (test-plan/e2e-test-strategy fct-csryo2zo) — one shared refusal path, read on both carriers.
@binding n=2
**integration-test-strategy ↔ e2e-test-strategy**: a new verb for the read is pinned on every carrier the graph verb is — in process by `stand_graph` and `stand_mcp_tools` (test-plan/integration-test-strategy fct-eja3jr5q), by process on the command line and over MCP (test-plan/e2e-test-strategy fct-csryo2zo) (test-plan/e2e-test-strategy fct-gtzxj2kr), and in the host's and `mcp` role's logs at `trace` with their counts moved (test-plan/integration-test-strategy fct-adkwyvky).
@binding n=3
**integration-test-strategy ↔ the UI graph node model**: the actionable set the read lists draws on the graph's handlers and owners (test-plan/integration-test-strategy fct-csattoyz) and on listener elements, whose bridge attributes over-report (test-plan/integration-test-strategy fct-xs6og5lj); the read's set must be one a click acts on.
@binding n=4
**e2e-test-strategy ↔ the body's cold-agent pipe**: the MCP listing's thirteen verbs (test-plan/e2e-test-strategy fct-gtzxj2kr) are the same thirteen the cold-agent pipe's isolated tool set and `valid_step` admit (test-plan fct-63656bwh); a new verb on the MCP carrier moves both together.
