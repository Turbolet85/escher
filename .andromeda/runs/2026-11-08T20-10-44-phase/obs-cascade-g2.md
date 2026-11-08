@carried n=1 label=obs-plan/log-coverage fact=fct-gwvelgmp
why: The coverage read and every replayed actions step run under the command span, which must keep its eight closed-word, bool and count fields, a fifteenth verb joining the verb table's words.
quote: one closed-span line per call to `Session::run`: INFO at target `escher_driver`, `span="command"`, with the
  recorded ones of `verb`, `cause`, `settled`, `busy`, `passes`, `added`, `removed`, `changed`, then `message="close"`,
  `time.busy`, `time.idle` — the verb table's fourteen words (ten instance-level; the three session-level ones and the
  tool-level `help` only beside the cause `unknown-verb`), eight cause names (ratified by the founder, 2026-10-18),
  three busy-class words, bools and counts;
@carried n=1 run=2 label=obs-plan fact=fct-pzih6tk6
quote: one closed-span line per call to `Session::run`, `span="command"` with its recorded fields, and one per MCP tool
  call, `span="tool_call"` with `tool` and `ending`, each INFO at target `escher_driver`, closed words, bools and counts
  only — no shot byte, lint finding, part number or text; no line at the default `warn`
@carried n=2 label=obs-plan/log-coverage fact=fct-gwvelgmp
why: A coverage verb's command line must, like graph's and actions', hold no node, id, owner or name of its answer, host_log and mcp_log each driving one call of it at trace with 0 id, name and answer needles under both builds.
quote: a graph's line is `span="command" verb="graph"` (with `cause` when refused, an argument refused `malformed`) and
  holds no node, state, id or name — `host_log` and `mcp_log` each drive one `graph` call at `trace` with 0 id, name
  and graph needles under both builds (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md); an actions
  line is `span="command" verb="actions"` (with `cause` when refused, an argument refused `malformed`) and holds no
  node, id, owner, verb word or cause word of its answer — `host_log` and `mcp_log` each drive one `actions` call at
  `trace` with 0 id, name and actions needles under both builds (as measured at
  escher-0.3.0/chunks/2026-11-08-what-can-i-do-here/report.md);
@carried n=3 label=obs-plan/log-coverage fact=fct-gwvelgmp
why: If the coverage read becomes an MCP tool, its word joins the tool_call span's closed tool words and its ending stays accepted, refused or error.
quote: since 2026-10-10-mcp-surface, one closed-span line per MCP tool call: INFO at target `escher_driver`,
  `span="tool_call"`, with the recorded ones of `tool` (the table's word, one of its fourteen, `help` among them since
  2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, `lint`, `graph` and `actions`) and
  `ending` (`accepted` · `refused` · `error`), then `message="close"`, `time.busy`, `time.idle`
@carried n=4 label=obs-plan/log-coverage fact=fct-5ydei676
why: The chunk edits graph.rs and signal_registry.rs, which must stay free of any print, log site, subscriber or env read, and the coverage answer, like the graph's, must be written by to_json alone and never logged or fielded.
quote: no site added: dioxus-native-dom's modules `graph` (gated `accessibility`) and `signal_registry` and the check
  `stand_graph` hold no print, log site, subscriber or env read, the silence census reading 0 over them;
  `Outcome::Graphed` is written by `to_json` alone as `{"nodes":[…],"states":[…]}`, never logged or fielded,
  escher-driver keeping its 2 `info_span!` sites (as measured at escher-0.3.0/chunks/2026-11-07-ui-graph/report.md)
@carried n=5 label=obs-plan/log-coverage fact=fct-v53ms3q6
why: The handler-reach record that must hold on every app, registry or not, moves the bridge dispatch path, which must still add no log site.
quote: no site added: `DioxusEventHandler::handle_event` sets the `SignalRegistry`'s dispatch around
  `runtime.handle_event` and clears it after only where a registry context stands at the root scope, adding no log
  site; a page with no registered state reaches no registry and dispatches as before (as measured at
  escher-0.3.0/chunks/2026-11-07-ui-graph/report.md)
@carried n=6 label=obs-plan/log-coverage fact=fct-x7v6upkc
why: dioxus_document.rs is reached whole, so its initial_build and poll lines that render through the mutation buffer must keep holding no log site, print, env read, listener or panic token.
quote: no site added: dioxus-native-dom's crate-private module `mutation_buffer` (`MutationBuffer`, recording
  dioxus-core 0.7.10's 15 `WriteMutations` calls and replaying them into `MutationWriter`) and the lines of
  `DioxusDocument::initial_build` and `Document::poll` that render through it hold no log site, print, env read,
  listener or panic token, both silence censuses reading 0; the replay re-fires `MutationWriter`'s own `trace!` calls,
  so the log stream is the one the direct write gives
@carried n=7 label=obs-plan/log-coverage fact=fct-7tp6qj2l
why: The owed replay test re-asserts a kept actions step's answer on a fresh session, which requires the actions read to act on nothing and change nothing in the document.
quote: no site added: `Session::actions` reads what each actionable node can take without acting — one snapshot and
  one `element_ids` map per call, each call it names validated, then put through the shared `check_target` — adding no
  `pump`, `tick`, `dispatch`, `scroll_into_view`, `close_select_list` or `focus_select` call; the read changes nothing
  in the document
@carried n=7 run=2 label=obs-plan fact=fct-abwv7xhd
quote: the fifteenth escher-driver module, `recording`, holds no tracing site, env read or print, its unit tests
  reading the compile-time `CARGO_MANIFEST_DIR` alone, and writes each answered call's arguments, `accepted`, its
  answer's FNV-1a 64 digest and the answer itself as `answer` (a shot's without `png`) into the kept check through the
  crate's `json` writer — a file at rest, never a log line, field or print; the sixteenth, `replay`, holds no tracing
  site, env read or print and calls `serde_json` only to read
@carried n=8 label=obs-plan/log-coverage fact=fct-zygqqe4l
why: The scope leaves a fifteenth verb open and names the pipe's verb list as the Cold-agent bar entry's, so a coverage verb must be no counted verb and add no verdict field or capability-unused reason here.
quote: The pipe's `VERBS` (`scripts/cold-agent.sh`, `.github/scripts/test_cold_agent.py`) hold fourteen words, `graph`
  and `actions` among them, and the contract tests' fake server answers `graph` with `{"nodes":[],"states":[]}` and
  answers `actions` too; neither is a counted verb or required of a cold agent, so neither adds a verdict field or a
  `capability-unused` reason
@carried n=9 label=obs-plan/log-coverage fact=fct-nrv3hivl
why: A coverage verb's tool_call lines land in server.log, whose counted verbs stay lint, screenshot and advance, read by span name only and never printed or copied.
quote: `span_counts` also returns the accepted `tool_call` span lines of each counted verb (`lint`, `screenshot` and
  `advance`) from `target/cold-agent/server.log`, which the pipe reads by span name only and never prints or copies
@carried n=9 run=2 label=obs-plan fact=fct-h5qg7fkm
quote: which the pipe reads only to count ` span="tool_call" ` lines, their ` ending="refused" ` / ` ending="error" `
  and the accepted lines of each counted verb, `lint` · `screenshot` · `advance` (`span_counts`), never printing or
  copying it
@carried n=10 label=obs-plan/log-coverage fact=fct-3ogdy56e
why: The scope names the CLI's verb set as a surface a new verb moves: its usage line is built from VERBS and every ending of a coverage verb is one line of JSON on stdout or a fixed string, holding nothing the caller typed.
quote: usage — one fixed line on stderr built from `VERBS` and the six task names (`counter` · `flight-booker` ·
  `timer` · `crud` · `cells` · `load`), stdout empty, status 2; session error — the error object on stdout, carrying
  its fixed `remedy` beside `kind` and `message` since 2026-10-12-self-description, and its fixed message on stderr,
  status 3 (the host's was 1); accepted (status 0) and refused (status 1) — one line of JSON on stdout, stderr empty
  with `RUST_LOG` unset
@carried n=11 label=obs-plan/log-coverage fact=fct-sich66ga
why: docs/capabilities.md is a verb-count pin the chunk reaches, and its restated command and tool_call span set must stay true by hand since its pin reads none of those claims.
quote: The published statement `docs/capabilities.md` restates this emitter's reach: the `command` and `tool_call`
  spans with closed-word, bool and count fields, stderr only, no driver line at the default `warn`, and the scrub's
  limits (the raw panic message, `log.file` on a bridged `js_console` record, the dropped third-party WARN or ERROR,
  the unscrubbed upstream apps and WPT runner); its pin `.github/scripts/test_capability_statement.py` reads none of
  these claims, so a change to the span set or to the scrub's limits is made in that document by hand
@carried n=12 label=obs-plan/log-coverage fact=fct-4hkolobh
why: If a coverage verb lands, the needles host_log and mcp_log read it with (its listed node ids and handler event names) belong in the seven_guis shared module beside actions_needles.
quote: The seven_guis shared module holds `option_needles`, a select's option labels as needles for the popup log
  tests, and `actions_needles`, the CRUD screen's listed author-key ids holding a `-`, for `host_log`'s and
  `mcp_log`'s `actions` calls
@binding n=1
**log-coverage (command and tool_call span lines) ↔ span-trace-coverage (the body)**: a coverage read or a replayed actions step runs under the one `command` span with its eight fields, and a coverage tool's word joins the closed `tool` words with `ending` in accepted, refused or error (obs-plan/log-coverage fct-gwvelgmp); both spans gain no field and the driver keeps its 2 `info_span!` sites and no event (obs-plan fct-yqpbkmod) (obs-plan fct-wa4xq2qh).
@binding n=2
**log-coverage (graph, signal_registry and bridge dispatch silence) ↔ pii-scrubbing-compliance (the body)**: the handler-reach record and the coverage answer carry graph node ids, owners and handler event names, which no log site, span field or Debug print may carry (obs-plan/log-coverage fct-5ydei676) (obs-plan/log-coverage fct-v53ms3q6); any span field or tool word added is judged pair by pair by the sink's allowlist scrub, a content-named field printed only as `[redacted]` (obs-plan fct-cbcilbst).
@binding n=3
**log-coverage (CLI usage, capabilities statement, needles) ↔ the cold-agent pipe (log-coverage)**: a fifteenth verb moves the driver's VERBS, the CLI usage line and docs/capabilities.md by hand (obs-plan/log-coverage fct-3ogdy56e) (obs-plan/log-coverage fct-sich66ga) and gains host_log and mcp_log needles (obs-plan/log-coverage fct-4hkolobh), while the pipe's VERBS and counted verbs stay the Cold-agent bar entry's (obs-plan/log-coverage fct-zygqqe4l) (obs-plan/log-coverage fct-nrv3hivl).
@binding n=4
**log-coverage (recording and replay, actions read) ↔ ci-integration (the body)**: the owed replay test keeps an actions step through `recording`'s json writer and replays it through `replay`, neither gaining a tracing site, env read or print (obs-plan fct-abwv7xhd) (obs-plan/log-coverage fct-7tp6qj2l); any check it keeps lives in a 0700 recording directory uploaded by no CI leg, and any it commits holds no typed text, host path or secret (obs-plan fct-4rkn3jky).
