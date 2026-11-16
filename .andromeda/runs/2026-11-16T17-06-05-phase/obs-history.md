@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** (escher-telemetry: stderr subscriber, line format, chaining panic hook, allowlist scrub) — built the sink this chunk's scrub reuses: the engine prefixes print only the seven safe fields, and `url`, `href`, `src`, `html`, `text`, `value`, `attrs`, `path`, `request`, `error` and `panic.payload` are redacted. It also built the chaining panic hook (`panic` ERROR at `escher_telemetry::panic`, the payload redacted, std still printing the raw message) and left as-is residue the raw panic message and the `log.file` host paths. Why it matters: the chunk's per-record scrub must reuse these sets (`packages/escher-telemetry/src/format.rs`), and it will carry `panic.payload` and `log.file` into the answer, the two values this entry left redacted or as-is on the log line.

@history_item n=2 marker=2026-10-06-headless-stand
**2026-10-06-headless-stand** — made the headless stand (in-process boot) install no subscriber, so a headless boot has no escher sink. This is the origin of `decision/headless-stand-no-sink`. Any text the chunk collects in process must come from the scoped capture, never from an installed sink.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (a second sink installer; the scrub's reach scoped to the measurement) — made `escher-session` the second sink installer and scoped the scrub's reach to what was measured. Standing trap: a zero read from a host that installs no sink says nothing about one that does. The chunk's no-text-on-the-log-line claim must be read on a sink-installing host (`host_log`, `mcp_log`), not inferred from in-process silence.

@history_item n=4 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — replaced the pass-through for other targets with three outcomes: an engine target prints only the safe fields; an `escher_` target (underscore included) prints everything but the eleven content-named fields; any other target is dropped whole at every level, whatever `RUST_LOG` names. The accepted cost is that a third-party WARN or ERROR never prints. Standing rule: a target is admitted only together with a scrub rule for it. The chunk makes the capture admit framework, engine and app targets into `caused`, and per this rule each target it admits needs a scrub rule. The dropped "every other target" class (app and third-party) has no scrub rule on the sink today.

@history_item n=5 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — set the rule never to field a `Command`, `Call` or `ArgValue` with `Debug`, nor an argument by its schema name. Of the argument names only `text` is in the scrub set. This is the root of `decision/debug-types-never-logged`. Record text the chunk builds must not be produced through those types' `Debug`.

@history_item n=6 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — added `Outcome` to the `Debug`-deriving types that nothing prints, logs or fields, because its `Debug` would print an id, a name or a control's value. Typed text was measured in process only. The chunk widens `Outcome`'s `caused`, so `Outcome` must stay written in one place (`to_json`), never `Debug`-logged.

@history_item n=7 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (the command span as built; the closed span's line, PROVISIONAL) — built the `command` span with eight fields and its never-fielded list (any argument, screen text, `Outcome`, `Refusal`, `Fault`). It turned `tracing` on ungated in escher-driver. A workspace build turns `blitz-dom/tracing` on, so a test child at `info` also gets engine events, the origin of `decision/engine-tracing-by-invocation`. It also built the closed span's line under the scrub, with content-named span values never stored. Why it matters: the chunk must add no span field (Boundaries), and under the workspace build the engine records it captures differ from those under a package-alone build.

@history_item n=8 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the founder ratified the closed span's line. The precedent is that a new record class or log field is a boundary widening that only the founder ratifies, and an operator's or delegate's answer stays PROVISIONAL. The chunk's own widening (error text on the carriers) is owed its amendments at the wrap on this footing.

@history_item n=9 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the command line writes answers, not logs; typed text measured in a host's log) — ruled that what a command prints is its answer under the ratified crossing, never a log field. Standing rule: the next surface adds no span or log field holding what a call supplied or a screen reads. The typed-sentinel reading at `trace` in the host's stderr was 0 under both builds. The chunk's text crosses only as the answer, and that 0-needle reading must still hold with error text in the answer.

@history_item n=10 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `tool_call` span (`tool`, `ending`, closed words, ratified). In the `mcp` role stdout carries protocol lines only and replies are never log lines. `mcp_log` reads the role at `trace`. The MCP tool result is one of the carriers `caused` text rides, while `tool_call` keeps exactly its two fields.

@history_item n=11 marker=2026-10-12-self-description
**2026-10-12-self-description** — made `help` and `describe()` an answer on stdout, never a log line, and `describe` installs no subscriber and prints nothing. `help` was added to both spans' closed sets with no field added. The chunk extends `describe()` with `caused`'s new fields and the meanings of the cut and the count, so it must stay fixed `'static` text.

@history_item n=12 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — re-read the workspace-built host: the `blitz_dom::document` `focussed node` INFO line printed twice, and why was recorded, not established. Engine records under the workspace build are real, and their counts can move. The chunk's measurement of engine record lengths and counts per acting step must name the build.

@history_item n=13 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe's live `mcp` child runs with `RUST_LOG=info`, and the replay children write stderr to `/dev/null`. This is the origin of `decision/replay-children-uninstrumented`. A replayed kept check whose answers now carry `caused` text is still replayed uninstrumented.

@history_item n=14 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — made `advance` accepted on every session, recording `verb` and the acting fields like every acting verb, with no `cause`. `advance` is one of the five acting verbs whose `caused` gains text.

@history_item n=15 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** (the PROVISIONAL batch ratified by the founder: one-virtual-clock) — ratified the `advance` rows. This is the ratification behind `decision/advance-accepted-every-session` in the picked rows.

@history_item n=16 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — added the recording directory (`0700`, one `0600` check per session, call content at rest) and the `RecordingFailed` ending. It is the origin of `decision/recording-dir-private` and the recording bounds. A kept check now holds `caused` text at rest, inside the 400-record and 1,048,576-byte bounds.

@history_item n=17 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — made the recorder keep each answer whole as `answer` (a shot's without `png`) beside its digest. It committed `tests/blitz-tests/checks/counter-{cli,mcp}.json`, written by `record_checks.sh` and read by `stand_replay` and seven_guis' freshness check. Why it matters: the chunk changes the answers, so both the kept answer and the FNV digest move. A committed check whose answers carry `caused` may need re-recording, and the freshness check will catch drift.

@history_item n=18 marker=2026-10-19-test-splits
**2026-10-19-test-splits** — moved the needle helpers (`Needles`, `stand_needles`, `found`, `occurrences`, `recorded_needles`) into seven_guis' shared `tests/common/mod.rs` for `host_log` and `mcp_log`. Their assertion messages name a kind and a count, never a needle. The chunk's proof that no record text reaches the sink's lines extends these helpers.

@history_item n=19 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` restates the driver's spans and the scrub's limits, and its pin reads none of these claims. A change to what crosses or to the scrub's limits is made there by hand, which this chunk's text in `caused` does.

@history_item n=20 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (the take read silent, and the sink re-check restated) — measured that for the uncaught-JS-error `tracing::error!` at the engine target `blitz_vibey_script`, the sink prints the safe fields and the message `[redacted]`. Trap: under the engine safe-field rule an engine record's message is itself redacted. The scope says "the rest of the message still crosses", so the chunk must settle how the engine safe-field rule applies to the message it carries into `caused`.

@history_item n=21 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`focussed node` re-read beside the focus fixup) — the engine's `focussed node` INFO line, emitted once per focus move under the workspace build, holds a node number and no id or name. It is a concrete engine record an acting step causes, and a candidate for the chunk's length and count measurement.

@history_item n=22 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (a failed load reaches its handler, and where its error text stops) — `NetHandler::error` now hands the error to the handler. A failed load's error text stops in `ResourceLoadResponse` and the `Resource load failed` line, where `url` and `error` are in the scrub set and the message prints `[redacted]`. "No other carrier takes the text": this is `contract/telemetry-scrub-set/error`, which a failed fetch's text crossing into `caused` would contradict. Standing trap: a trait default can hide a fix, so sweep the implementers of a trait the chunk extends.

@history_item n=23 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the read values stay in the test) — the probe's reading quoted no captured stderr or panic payload; the backtrace confirming the hazard's panic entered it by its coordinates alone. This is a precedent for panic text: so far a payload has crossed nowhere, and only its coordinates have.

@history_item n=24 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the graph answer's carried set) — precedent for widening an answer's carried set. The set (ids, names, closed words, booleans, never a value; a sentinel read 0 times) was recorded as a boundary widening in §8, with the recording directory row's at-rest list extended in §9. The confirm was PROVISIONAL until the founder's word. The chunk's error text on every carrier owes the same §8 and §9 rows.

@history_item n=25 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer's carried set) — the same precedent: a new answer's carried set was recorded as a ratified boundary widening, the recording directory's at-rest list was extended, and a typed sentinel and `MASKED_VALUE` occurred 0 times. The chunk must keep the mask (`decision/answers-echo-nothing-typed`), with a password's or file input's value never crossing in record text.

@history_item n=26 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — added `decision/exploration-catches-panics`: a caught panic in an explored call is reported as the closed ending `Panicked`, holding no payload. It also amended `contract/escher-session-coverage/malformed` (format other than 1). Conflict to resolve: the chunk carries a panic's payload in `caused`, while exploration's `Panicked` holds none. The plan must say whether an explored call's answer now carries the payload text or keeps the closed ending.

@history_item n=27 marker=2026-11-13-local-files-for-local-documents
**2026-11-13-local-files-for-local-documents** — made an engine `file:` sub-resource fetch from a non-`file:` document fail with fixed text naming no URL or path, adding no log site. This is an existing engine error text that, carried into `caused`, already holds no path.

@history_item n=28 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — amended `decision/headless-stand-no-sink`'s exception to name the layout stand check's own recording subscriber. That subscriber, the MCP tools recorder and `stand_act_spans`' re-run children are the named exceptions. Any new subscriber the chunk's checks install (for example, to read record text) must be named in that exception.

@history_item n=29 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (§3, §4, §6 rows) — the chunk this one extends. It reworded `decision/headless-stand-no-sink`, `decision/driver-library-installs-no-subscriber` and `decision/driver-emits-no-event` to name escher-telemetry's scoped capture around an acting step, which counts and prints nothing. It scoped `contract/telemetry-sink/rust-log` and `contract/escher-sink-env/rust-log` to the sink layer alone, with the capture layer unfiltered. It added `decision/capture-framework-targets-warn-up`, the operator's ruling that the capture counts framework targets only at warn and up and engine and `escher_` targets not at all. Why it matters: the chunk turns that counting capture into a text-carrying one that admits engine targets, so each of these rows ("counts records", "prints nothing", the capture's target set) needs a reworded amendment. The framework warn-up ruling must be revisited or kept on purpose.

@history_item n=30 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — turned the master into rows verified against the code, the fates approved by the founder. This is the basis of every picked row's handle; an amendment the chunk owes is written as a `~`, `+` or `−` on these handles.
