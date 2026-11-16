@history_item n=1 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** — wired escher's own stderr sink: engine targets print seven safe fields, `url` · `href` · `src` · `html` · `text` · `value` · `attrs` · `path` · `request` · `error` · `panic.payload` redacted; its reach is that sink only, the std panic hook's raw message staying as-is. This is the scrub the chunk must hold unchanged, and the raw panic-hook message is the known leak beside it: a caught handler panic's payload taken into an answer is authored text on a carrier (the founder's fork), never a reason to loosen `panic.payload`'s redaction.

@history_item n=2 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (what a sink-installing host logs) — recorded that third-party records (Stylo, dioxus_core) printed ids and names at `debug`/`trace` on the session host, and set the trap: a "nothing in the logs" check over a process that installs no sink passes vacuously. Any proof that the new answer content reaches no log must run against a sink-installing host (`host_log`, `mcp_log`), not an in-process session alone.

@history_item n=3 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — made the sink three-outcome: engine targets safe fields only, `escher_` targets with content-named fields redacted, any other target dropped whole at every level, WARN and ERROR included, a bridged record judged by its target. The named cost is that a third-party WARN or ERROR no longer prints. That bears directly on capture: an app handler's own log record (an app target) and most error-level records are dropped by the sink today, so a per-step capture layer must sit beside the sink without changing what it prints (`decision/outside-target-dropped`, `decision/other-log-targets-dropped`, `decision/telemetry-sink-target-allowlist`).

@history_item n=4 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — built `validate` over a closed verb table and the `Refusal` as a closed set of causes with fixed texts, no `String` field and a `Fault` carrying only the schema's argument name, with no `unwrap`/`expect`/`panic!` on call input. Its rules for later chunks: keep every refusal text fixed and free of call content, and add no cause outside the set. A caught handler panic must not become a new free-text refusal (`decision/closed-refusal-cause-set`, `decision/refusal-holds-no-call-input`, `decision/no-panic-on-call-input`).

@history_item n=5 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Session::run` executes the schema in process, validating first, so nothing public runs a `Command` that did not come from `validate`; an id is looked up anew each call; no panic path on call-supplied input. The acting run path this chunk wraps for panic capture is this one (`decision/one-validation-path-for-every-carrier`, `decision/id-lookup-anew-each-call`), and a caught panic must leave that single path intact.

@history_item n=6 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — defined the one INFO `command` span per call under `escher_driver` with eight fixed-word or count fields, never an argument, outcome, refusal text or screen content. It also made the sink print closed spans as a second record class (later ratified). Scope's "scrub held": the `command` span gains no field for what an answer now carries (`decision/one-command-span-per-call`, `decision/command-span-content-free`, `contract/driver-command-span/*`).

@history_item n=7 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — ratified the sink's closed-span line and that a diff's three sizes may reach a log while no node, id, name or value of it may. The standing rule: a log field holding any content is a new crossing question. A count of captured errors on the span would itself be a new log field to weigh (`decision/diff-counts-only-in-logs`, `decision/closed-span-lines`).

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — widened the session socket to carry calls and answers (ids, names, masked values, snapshot text, diff; no `NodeId`). It also made the command line an external surface whose endings echo nothing typed, and gave refusals and session errors written forms from fixed tables. These are the ratified carriers and the no-echo rule the new answer field rides (`decision/session-socket-carries-calls`, `decision/cli-echoes-nothing-typed`, `decision/session-error-fixed-table`, `contract/cli-exit-codes/*`).

@history_item n=9 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** — the founder's rule: a new field of a closed value set inside an already ratified carrier is no widening, while a field holding authored text, an open value set, a new carrier or a new exit still is. This is the rule the scope invokes. Error kinds, node ids and handler event names fit inside it; a panic payload, a log message or a log field's value does not and goes to the founder (`decision/boundary-widening-rule`, `decision/closed-value-field-no-widening`).

@history_item n=10 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — ratified the MCP stdio crossing (bounded lines and results, fixed protocol errors with no data, refusals as `isError` results) and the `tool_call` span (`tool`, `ending` only) parenting `command`. The new field must appear in MCP tool results and, per the scope, leave the `tool_call` span unchanged (`decision/mcp-result-crossing`, `decision/mcp-bounded-lines`, `contract/mcp-tool-call-span/*`, `contract/mcp-tool-result/is-error`).

@history_item n=11 marker=2026-10-12-self-description
**2026-10-12-self-description** — `help` and the MCP listing became the program's own fixed `'static` table text crossing on ratified carriers, ratified with the playbook rule that text from a page, a call or a screen, a new carrier or exit still widens. Session errors became one fixed table with remedies. The new field's `help` and listing text must be fixed table text (`decision/mcp-listing-fixed-text`, `decision/session-errors-fixed-table`).

@history_item n=12 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the precedent for open content onto every carrier: pixels are no closed value set, so the in-band image was a boundary widening the founder ratified himself. It added a cause, `fonts-unavailable`, later retired. The rows say the image, its size and dimensions are never fielded. A panic or log text in an answer is the same class: open authored content needing the founder's own word (`decision/screenshot-in-band`).

@history_item n=13 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — retired `time-unavailable` and closed the cause set at eight, ratified. A caught panic must not reopen it with a new cause unless the plan names one, which the scope also requires (`decision/closed-refusal-cause-set`).

@history_item n=14 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — kept checks at rest: owner-only `0700`/`0600` medium, bounds of 400 records and 1,048,576 bytes ending the recording `recording-failed`, the strict `Check::from_bytes` reader, and masked `type` kept as `MASKED_STAND_IN`. The new answer field joins what a kept check holds at rest and its bound (`decision/recording-directory-owner-only`, `decision/recording-bounds-end-recording`, `decision/recorded-check-strict-reader`, `decision/recorded-type-masked`).

@history_item n=15 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — a kept check keeps each answer whole as the driver wrote it, with a digest. Replay runs each step through `Session::run` and stops on a differing answer. This is the design point the scope leaves to P4: whether the error field enters the digest. A non-deterministic payload in the digest would make replay stop (`decision/kept-check-answer-whole`, `decision/replay-through-session-run`, `contract/recorded-check/digest`, `contract/recorded-check/answer`).

@history_item n=16 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part) — the precedent for new answer content riding every carrier: `lint`'s findings (ids, closed defect words, numbers) crossed on the socket, command line, MCP and a kept check, ratified. The findings and their count are never fielded on the span, and `host_log`/`mcp_log` drive the verb with 0 needles. The same proof shape applies to the error field.

@history_item n=17 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — the acting step (`click` · `type` · `press` · `advance` · `scroll`, on every carrier) settles again within `LOAD_WAIT`, the session reading the wall clock for that wait alone, with no wait argument or result field. This is the settled step whose run the chunk captures errors over. Any capture window must not add a wait field or a new clock read beyond those registered (`decision/load-wait-bound`, `decision/no-wait-verb-fields`, `decision/driver-installs-no-subscriber`).

@history_item n=18 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — made `host_log` a standing check at `RUST_LOG=trace` over a recording host, with sentinel, id, name and recorded needles at 0 under both builds. This is the check the new answer content's log silence must extend.

@history_item n=19 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — reconciled the carried sets at every site (socket, MCP, command line, kept check at rest) and added the committed statement `docs/capabilities.md` of what an MCP client carries onward. A new answer field must be named in every carried-set site and in the statement, or the sets go stale (`decision/capability-statement-mcp-crossing`).

@history_item n=20 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the `MountedData` panic from a spawned task) — first recorded that an app's task can panic `RefCell already borrowed` inside the render. This is a measured handler/task panic class the chunk's catch on the run path meets.

@history_item n=21 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (where a failed load's error text stops) — a failed load's error text stops in `ResourceLoadResponse` and the `Resource load failed` line, where the sink prints the message `[redacted]`, `url` and `error` in its scrub set, and no carrier takes it. An error-level record's text entering an answer would be the first carrier for it: a widening for the founder (`contract/resource-load-failure-log/*`, `contract/network-tracing-log/error`).

@history_item n=22 marker=2026-11-04-dioxus-runtime-probe
**2026-11-04-dioxus-runtime-probe** (the MountedData hazard re-measured and pinned) — pinned the spawned-task panic with `should_panic` tests in both layout modes, its mechanism `poll` holding the document borrow across `render_immediate`. This is history of how an app panic on the run path was measured.

@history_item n=23 marker=2026-11-07-mounteddata-buffered-render
**2026-11-07-mounteddata-buffered-render** — removed that panic by rendering into a buffer before borrowing, with a known limit for a task polled during render. The recorder holds no log site and no panic path. Catching a handler panic around `Session::run` must not unwind with the document borrowed or regress this (`decision/dioxus-render-into-buffer`, `decision/try-doc-none-while-borrowed`).

@history_item n=24 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — ratified the graph answer crossing (ids, owner ids, component, state and handler event names, closed flag words, booleans; never a value, typed text or `NodeId`) on every carrier including kept checks. It also added the Dioxus bridge dispatch around `runtime.handle_event` with the `SignalRegistry`, inert without a registry, with no log site. The graph's handler identity (owner id, event name) is what the error field ties to, and the dispatch is where handler capture would hook (`decision/graph-answer-crossing`, `decision/dispatch-recorded-only-with-signal-registry`, `decision/signal-registry-holds-no-state-value`, `contract/graph-answer/nodes-states`).

@history_item n=25 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — the latest precedent for a ratified widening of answer content onto every carrier (`actions`). Its kept-step replay was left unmeasured and owed as a CARRY. The verb's log silence is proven by 0 needles. Treat the error field's kept-check replay as an owed proof, not an afterthought (`decision/kept-check-actions-answer`).

@history_item n=26 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — added the reached record, written in `handle_event` before dispatch with each reached handler's stable id and event, deduplicated, surviving element removal, with no log site and no `NodeId`. It also added the coverage role reading kept checks as untrusted. The reached record is one candidate attribution mechanism the scope names (`decision/reached-handler-record-without-log`, `decision/reached-record-deduplicated-survives-removal`, `decision/coverage-check-files-untrusted`).

@history_item n=27 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — ratified the coverage role and its line's content and log silence on the founder's own word, replacing the delegate's PROVISIONAL. This confirms that the ratification of a carrier's content is the founder's alone.

@history_item n=28 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — added `decision/reach-error-closed-set`: reach errors as a closed set with fixed texts holding nothing supplied. The scope says `Session::reach` keeps its contract, and this is the precedent for any error type the chunk adds.

@history_item n=29 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — added `decision/exploration-report-closed-content`: an exploration's report holds closed endings, never a panic payload, `NodeId`, screen text or digest, and is never logged. The scope forbids widening `explore`'s per-path panic ending to carry a payload. It also moved the kept check's opening record to `format: 1`: a changed answer shape in kept checks may bear on the format.

@history_item n=30 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** — added `decision/session-layout-read-in-process` and widened `decision/no-screen-content-in-logs` to "no driver call or in-process read", the layout answer included. The scope says the layout read keeps its contract. The new answer content must join that row's list of answers kept out of logs.
