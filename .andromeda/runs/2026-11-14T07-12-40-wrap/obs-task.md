@task_prose part=session
The session added a seeded randomness source to dioxus-native-dom: SeededRandom (a hand-written splitmix64 generator whose clones share one stream), the hook use_seeded_random, and DEFAULT_SEED = 0. Outside a run, the source falls back to a host-seeded generator.
The new seed module is ungated and sits beside reached and signal_registry with no accessibility feature, so the wasm build compiles it.
Harness::from_vdom now provides SeededRandom::from_seed(DEFAULT_SEED) as a root context when the vdom's root holds none, the same way it supplies the one VirtualClock. Its signature, Harness::wrap and HarnessOptions are unchanged. Every driver session boots through it.
A new stand check, stand_seeded_randomness, shows that one seed gives the same state at every step in both layout modes, in a second process (a test-binary re-exec), and in a kept-check replay. It is the 36th reader of the shared session_common module.
The production silence census measured 0 new lines. No log site, span, span field, print, env read, verb, carrier field, kept-check key, MCP tool field, exit or dependency was added. The seed module and the harness take no tracing.
F1 was answered A, PROVISIONAL until the founder confirms or replaces it: no seed crosses the driver's carriers. Page-script Math.random stays unseeded, recorded as a residual with target next.
@task_file n=1 path=packages/dioxus-native-dom/src/seed.rs
what: the seeded randomness source of the Dioxus document crate — SeededRandom, use_seeded_random and DEFAULT_SEED; ungated, logs nothing, and its Debug writes the type name alone
@task_file n=2 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus-native-dom crate root and its module list — the ungated modules (reached, signal_registry, now seed) and the accessibility-gated ones, with its public exports
@task_file n=3 path=packages/blitz-test-harness/src/harness.rs
what: the test harness — Harness construction (from_vdom, from_component, wrap), the one VirtualClock and now the default run seed; every headless stand and driver session boots through it
@task_file n=4 path=tests/blitz-tests/tests/stand_seeded_randomness.rs
what: a stand integration check over both layout modes — one seed gives one state, two seeds differ, no seed reads the default, a second process (test-binary re-exec) and a kept-check replay
@task_item n=1 part=touches
decision/driver-library-installs-no-subscriber and the silence census it stands for: the seed module and the harness's default-seed provide join the census and add no log site, span, field, print or env read (census last line 0).
@task_item n=2 part=touches
decision/checks-silent (Silent checks and shared modules): a new stand check joins, and it reads the shared session_common module (readers 35 to 36). Its ignored child test prints seeded-digest lines on stdout under nocapture, and the parent captures them and reads them only into its assertion. A picker should judge whether that child's output is an exception to the silent-check rule.
@task_item n=3 part=touches
decision/headless-stand-no-sink and decision/harness-log-not-telemetry-sink: the harness boot path changed (from_vdom now provides a root context), but the harness still takes no tracing and installs no sink.
@task_item n=4 part=touches
decision/engine-tracing-feature-gated and decision/escher-crates-tracing-ungated (and their twins decision/engine-tracing-by-invocation and decision/escher-crates-always-trace): dioxus-native-dom gains a new ungated module, seed, which takes no tracing at all. The feature and gating split over that crate stays as written.
@task_item n=5 part=touches
decision/debug-types-never-logged and decision/state-registry-holds-no-values: SeededRandom is a new type with a hand-written Debug that writes the type name alone and no state. It holds a number and nothing else, so no value of it can reach a log.
@task_item n=6 part=touches
decision/reached-record-ids-and-events-only and decision/no-state-page-skips-registry: these sit over the ungated sibling modules of dioxus-native-dom (reached, signal_registry), which seed now joins. Those modules themselves did not change.
@task_item n=7 part=touches
decision/replay-children-uninstrumented and decision/empty-stderr-check-unsets-rust-log: the new check re-executes its own test binary as a child with a fixed argv. It sets or removes no env var (RUST_LOG included), and Command::output captures the child's output, so no output is shown.
@task_item n=8 part=touches
decision/session-state-dir-private: the new check reads session_common's state_dir under CARGO_TARGET_TMPDIR, which gives that directory one more reader. The directory's rule is unchanged.
@task_item n=9 part=touches
contract/kept-check/arguments, contract/kept-check/answer and contract/kept-check/answer-digest: a kept check now replays a seeded run, and the replay reads the default seed. No kept-check key was added, and under F1's provisional A no seed is written into a kept check.
@task_item n=10 part=touches
contract/host-wire/v3, contract/escher-session-commands/help and contract/escher-session-commands/mcp: under F1's provisional A, no seed crosses a carrier. No verb, argument, wire field or MCP tool field was added, and these stand as written.
@task_item n=11 part=touches
contract/command-span/span, contract/command-span/field-count and contract/tool-call-span/span (and their escher-driver twins): every driver session now boots with the default run seed, but no span or span field carries a seed. The field count is unchanged.
@task_item n=12 part=touches
decision/session-logs-carry-no-page-content and decision/hosted-call-through-session-run: the driver's session boot (start, host, mcp, coverage, replay) now goes through a from_vdom that provides the run seed. The answers and the logs are unchanged.
@task_fact n=1 slug=3-observability-harness-contract
fact: The seeded randomness source (SeededRandom, use_seeded_random, DEFAULT_SEED) and the harness's default run seed add no log site, span, span field, print or env read. They join the production silence census beside the driver library that installs no subscriber.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: The run seed's generator holds one number and nothing else. Its Debug writes only the type name and never the state or the seed, so no draw or seed value reaches a log.
@task_fact n=3 slug=3-observability-harness-contract
fact: PROVISIONAL (F1 answered A, for the founder to confirm or replace): no carrier carries a run's seed. That rules out a start argument, a kept-check key, an MCP tool field, and a command or tool_call span field. A session always reads the default seed, and a seed given in-process stands.
@task_item n=1 part=untouched
The telemetry-backend, metrics-export, metric-emission and error-reporting-service absences are untouched: no exporter, backend, metric or dependency was added.
@task_item n=2 part=untouched
escher's sink log line, sink configuration, scrub set and resource keys are untouched (text format, escaping, span lines, RUST_LOG, service.name, service.version, sink target allowlist): the telemetry sink and escher-telemetry did not change.
@task_item n=3 part=untouched
The agent-run harness events and the cold-agent pipe (events, verdict fields, counted verbs, transcripts, server log, cleanup) are untouched: none of those scripts changed.
@task_item n=4 part=untouched
Error capture and reporting is untouched: script error source labels, the window error event, window.onerror, the WPT report, the panic hook and the stylesheet and file-load rules.
@task_item n=5 part=untouched
CI integration is untouched: no workflow, leg, artifact upload, recording bound or recording-file contract changed. The new stand target joins the stand-repeat leg through its existing glob.
@task_item n=6 part=untouched
The answer and content rules for snapshot, graph, actions, the coverage line, select and optgroup, and page script events are untouched: no answer shape or content changed.
@task_item n=7 part=untouched
The refusal causes, the tool_call endings, the escher-session exit codes and the stderr diagnostics are untouched: no refusal, exit or diagnostic was added.
