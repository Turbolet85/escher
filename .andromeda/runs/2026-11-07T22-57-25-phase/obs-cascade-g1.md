@carried n=1 label=obs-plan/telemetry-strategy fact=fct-weudw6wt
why: The new verb and its MCP tool must reach telemetry only through escher-driver's ungated command and tool_call spans, in escher-session's host and mcp roles, and by no path in the windowed seven_guis_native.
quote: since 2026-10-10-driver-cli it is reached in `escher-session` by every call its host role runs — one closed span per hosted call — and since 2026-10-10-mcp-surface by its `mcp` role, where each `tools/call` writes one closed `tool_call` span and every instance verb run through `Session::run` adds its `command` child — and still by no code path in the windowed `seven_guis_native`
@carried n=1 run=2 label=obs-plan fact=fct-fmm6lpcc
quote: escher-telemetry and escher-driver take `tracing` with no feature, so the startup and panic events and the command span are always compiled into the seven_guis package's two native binaries, `seven_guis_native` and `escher-session`, whose engine features stay off in a package-alone build.
@carried n=2 label=obs-plan/telemetry-strategy fact=fct-46rgllvo
why: host_log and mcp_log must re-read the new verb and tool under both builds, the workspace build turning blitz-dom's tracing on and the package-alone build leaving it off.
quote: a build that resolves the whole workspace (`cargo test --workspace`, `ci-leg.sh fast`, CI's test leg) turns `blitz-dom/tracing` on — blitz's default `tracing` reaches blitz-shell and blitz-html, and each of those forwards to blitz-dom — while `cargo test -p blitz-tests` and a `-p seven_guis` build leave it off; so under the workspace build a test child that installs escher's sink at `info` also receives engine events
@carried n=2 run=2 label=obs-plan fact=fct-upbrcclf
quote: so under the workspace build a test child that installs escher's sink at `info` also receives engine events (test-plan §9 → Engine features by runner).
@carried n=3 label=obs-plan/telemetry-strategy fact=fct-atc72w5q
why: Moving the stand's signals onto the registry must keep the stand's files free of any telemetry or tracing site, the silence census re-reading 0 over stand.rs and the stand tasks.
quote: observed absent — telemetry or tracing framework · searched: `tracing::|log::|env_logger` and `sentry|opentelemetry|metrics::` over the 32 slice files (examples slice)
@carried n=3 run=2 label=obs-plan/span-trace-coverage fact=fct-sicgzqzx
quote: observed absent — spans · searched: `span!|instrument` over the 32 slice files (examples slice)
@carried n=3 run=3 label=obs-plan fact=fct-qu4fsnfx
quote: hold no `blitz_net`, print, `tracing` call, env read, socket or listener, the silence census (`println!|eprintln!|print!|dbg!|tracing|escher_telemetry|std::env|env::var|UnixListener|TcpListener|blitz_net`) reading 0 over `stand.rs`, `flight_booker.rs`, `stand_cells_served`, `stand_load_page` and the shared modules `tests/common/mod.rs` and `tests/fixtures/mod.rs`; no driver schema, span field or log line moves
@carried n=4 label=obs-plan/span-trace-coverage fact=fct-zuu6oxcq
why: The new verb's call through Session::run must open the one command span, its verb field the verb table's closed word, with only closed words, bools and counts recorded.
quote: every call handed to `Session::run` opens one `tracing` span — target `escher_driver`, name `command`, level INFO, built by hand with `tracing::info_span!` (no `#[instrument]`), created and entered before `validate` and held until `run` returns, so a refusal `validate` returns is inside it — whose eight fields are each created unrecorded and recorded only where they apply: `verb`, the verb table's own word for a known verb, run or refused, never the caller's verb text and unrecorded for a verb the table lacks;
@carried n=4 run=2 label=obs-plan/span-trace-coverage fact=fct-zuu6oxcq
quote: an accepted `lint` and a part read (`Outcome::Part`) record `verb` alone too, and a refused call records `verb` (when known) and `cause` and none of the other six;
@carried n=4 run=3 label=obs-plan fact=fct-yqpbkmod
quote: its eight fields (`verb` · `settled` · `busy` · `passes` · `added` · `removed` · `changed` · `cause`) closed words, bools and counts recorded only where they apply; never fielded — the screen text, any argument, or any `Command`, `Call`, `ArgValue`, `Outcome`, `Refusal` or `Fault`; the driver emits no event and installs no subscriber.
@carried n=5 label=obs-plan/span-trace-coverage fact=fct-jyfscbc7
why: The new read verb answered by the host must yield only command span lines that host_log counts, reading 0 id and 0 name needles under both builds.
quote: seven_guis `host_log`'s fourth test counts the host's closed `command` span lines and reads 0 id and 0 name needles under both builds (§6 Log coverage, §8 PII); no span, field or log line added
@carried n=6 label=obs-plan/span-trace-coverage fact=fct-gf4hwplk
why: After the new verb, outcome and help text land, escher-driver must still hold its 2 info_span sites and command its eight fields, the moved help text fixed text reaching no field.
quote: escher-driver keeps its 2 `info_span!` sites and `command` its eight fields (`stand_act_spans`), the `click` · `type` · `press` help and `covered`'s meaning being fixed `'static` text that reaches no field;
@carried n=6 run=2 label=obs-plan/span-trace-coverage fact=fct-xswdmupu
quote: escher-driver keeps its 2 `info_span!` sites and `command` its eight fields (`stand_act_spans`);
@carried n=7 label=obs-plan/span-trace-coverage fact=fct-mp6exhtv
why: The new MCP tool must open tool_call with tool its verb-table word and ending alone, parenting the command child of its Session::run call, adding no field.
quote: two fields and no other, each a closed word: `tool`, the verb table's word — one of its twelve, `help` among them since 2026-10-12-self-description and `screenshot` since 2026-10-12-headless-screenshot, and `lint` — unrecorded for an unknown tool, and `ending`, `accepted` · `refused` · `error` (a session error or a protocol error); an instance verb run through `Session::run` makes its `command` span the child by nesting — the executor unchanged
@carried n=7 run=2 label=obs-plan/span-trace-coverage fact=fct-bcvmu5pc
quote: the span keeps its two fields, `tool` and `ending`, none added
@carried n=7 run=3 label=obs-plan fact=fct-wa4xq2qh
quote: two fields and no other, each a closed word — `tool`, the verb table's word, and `ending`, `accepted` · `refused` · `error`; an instance verb run through `Session::run` makes its `command` span the child, while the session verbs and `help`, answered by the server, leave a `tool_call` with no `command` child.
@carried n=8 label=obs-plan fact=fct-rvanvmh2
why: The graph reader, signal registry and snapshot field land in dioxus-native-dom, whose optional tracing feature must gain no new site from them.
quote: **dioxus-native, dioxus-native-dom** — optional `tracing` logging plus `log-times` features (log-phase-times, log-frame-times) (packages/dioxus-native/Cargo.toml:60-69; packages/dioxus-native-dom/Cargo.toml:19)
@carried n=9 label=obs-plan fact=fct-d7dwo6bi
why: The escher-session command line must write the graph read's answer as one line of JSON on stdout, never a log line, the sink writing to stderr only.
quote: `escher_driver::command_line`, which installs no subscriber, reads no environment variable and writes each command's answer, refusal or session-error object as one uncoloured line of JSON on stdout — an answer, never a log line: the sink still writes to stderr only, and the host role ends quiet;
@carried n=10 label=obs-plan fact=fct-icom2cq7
why: The new Outcome variant must be written in one place by Outcome::to_json, its Debug never logged, fielded or printed.
quote: `Command`, `Call` and `ArgValue` derive `Debug`, which prints the id and the text they hold, and so does `Outcome`, whose `Debug` prints the ids, accessible names and control values of its diff, a password's or a file input's value as the snapshot's mask, and since 2026-10-10-snapshot-states each node's three state readings, and nothing in the crate logs or fields any of the four or prints one with `Debug` — an `Outcome` is written in one place, by `Outcome::to_json` as a command's answer;
@carried n=11 label=obs-plan fact=fct-cg2a6gz6
why: Any closed span the new verb or tool opens is printed by escher's sink as one line on stderr, so its fields must stay closed words, bools and counts.
quote: one non-ANSI fmt layer — stated once, in the private `sink_layer`, with `with_span_events(FmtSpan::CLOSE)` and the crate-private field formatter `SpanFields` — that prints each event and, since 2026-10-07-driver-command-spans, each closed span through the escher formatter (§6 Log format), writing to stderr only
@binding n=1
**span-trace-coverage ↔ telemetry-strategy**: the new verb's command span and the new tool's tool_call span (obs-plan/span-trace-coverage fct-zuu6oxcq) (obs-plan/span-trace-coverage fct-mp6exhtv) are compiled ungated into both seven_guis binaries and reached by escher-session's host and mcp roles (obs-plan/telemetry-strategy fct-weudw6wt), so host_log and mcp_log must read the new verb and tool at trace under both builds, the workspace build adding engine events (obs-plan/telemetry-strategy fct-46rgllvo), with 0 needles and the span counts unchanged (obs-plan/span-trace-coverage fct-jyfscbc7).
@binding n=2
**span-trace-coverage ↔ the body (§3 Logging stack)**: the graph read's content — component names, handler event names, registered state names, nodes, values — must never become a field of command or tool_call (obs-plan fct-yqpbkmod), since every closed span is printed by the sink (obs-plan fct-cg2a6gz6); it leaves only as the new Outcome's to_json answer (obs-plan fct-icom2cq7), one JSON line on stdout (obs-plan fct-d7dwo6bi).
@binding n=3
**span-trace-coverage ↔ span-trace-coverage (the cold-agent pipe)**: the new tool's tool_call keeps tool and ending alone (obs-plan/span-trace-coverage fct-mp6exhtv), the pair the cold-agent pipe's span_counts reads (obs-plan/span-trace-coverage fct-bcvmu5pc), and escher-driver keeps its 2 info_span sites (obs-plan/span-trace-coverage fct-gf4hwplk).
@binding n=4
**telemetry-strategy ↔ the body (§1)**: the stand's signals moved onto the registry keep the stand's files free of tracing, span and print (obs-plan/telemetry-strategy fct-atc72w5q) (obs-plan fct-qu4fsnfx), and the registry and graph reader in dioxus-native-dom add no site under its optional tracing feature (obs-plan fct-rvanvmh2).
