@history_item n=1 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** — added the cold-agent run pipe and its contract to §3 (exit grammar 0 · 1 · 2 · 3, per-run `mktemp -d` session, isolation flags, `timeout 600`, stdin `/dev/null`, no propagated 124, verdict fields, positive-evidence `passed`, state under `target/cold-agent/` only) and `test_cold_agent.py` to §4; proof was the contract tests plus one live run, never in CI. The origin of decision/cold-agent-live-run-off-ci, decision/cold-agent-run-bounded and the cold-agent exit-code and state contracts this chunk's one firing runs under.

@history_item n=2 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — measured the agent-run 5-command contract (exit grammar, `run stand` with no stand file an empty run exiting 1, events never carrying captured test output). Source of decision/empty-run-is-never-a-pass and the event discipline the cold-agent pipe copied; the readiness checks this chunk runs before the firing lean on the same grammar.

@history_item n=3 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (the pipe on the escher server, §3) — one task `stand`, `run counter` a usage error, `RUST_LOG=info` on the server child only, exit 3 for an absent or non-executable `target/debug/escher-session` (the pipe builds nothing); `--model claude-opus-5-5`, `--max-budget-usd 3`; the reply's last fenced json block (≤ 65,536 B) becomes `check.json`; `judge_check` replays in full then ablated on two fresh children, 30 s per reply; a malformed check never replayed. Its proof was a live run fired once on the founder's «Один запуск, до $3». Precedent for this chunk's single firing, the build-before-run precondition and the check-judging rows (contract/cold-agent-check/*, decision/cold-agent-replayed-check, decision/cold-agent-replay-bounded).

@history_item n=4 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (the contract tests, §1 §4) — `ColdAgentPipeTest` under a `claude` shim and a fake `target/debug/escher-session`, no live model, credential or built binary; four planted markers for content-free events. Origin of decision/cold-agent-tests-offline and the scrub guards (decision/cold-agent-output-scrubbed); the readiness check runs `.github/scripts/test_cold_agent.py` without reaching the provider.

@history_item n=5 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the pipe over six apps by the 5 % bar and the second live run) — `passed` requires every app met, each counted capability used, `recording_kept` and `wrong_calls × 100 ≤ 5 × tool_calls`; added the `capability-unused` · `recording-not-kept` · `wrong-calls-over-bar` reasons; `record/` owner-only, never printed, copied or committed; `server.log` only counted. The live run fired once: `passed`, 66 tool calls, 0 wrong, 0.8244 USD, `claude` 2.1.308. This is the 0.2.0 baseline (six apps, 66 calls, 0 wrong) v030-15's bar is set from, and the form the docs' measured-run statement follows.

@history_item n=6 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the counts and the pipe's contract tests) — `test_cold_agent.py` went to 37 tests (the fake answering Cells as a paged line and the load page, keeping one check per accepted start under `--record`), `ci-scripts` 108; the `mcp` role's second real client named as the live run. The contract-test inventory the readiness check re-runs.

@history_item n=7 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the stand re-pinned to the flight booker's select) — `flight-type` became a select; its two options read `covered` at the select's client rect in `stand_act_ids`. A live agent clicking an option of the closed select is refused `covered`, a counted wrong call: the trap behind the scope's CARRY that `covered` can move `wrong_calls`.

@history_item n=8 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (six served slugs, the recording bound) — a full recorded Cells session keeps 312,431 bytes against the 1,048,576-byte recording bound (probe, no standing check). The figure that bears on `recording_kept` for a long live Cells session.

@history_item n=9 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`covered`'s closed-option clause) — `covered`'s fixed meaning also says an option of a closed select lies under it, to click the select first, then the option (contract/refusal-cause/covered); name, remedy and order unmoved. The clause CARRY cry-32fid2ry says may move `wrong_calls`, read by this run's `refused_by_cause`.

@history_item n=10 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — the session recording: directory `0700` or `RecordDirNotPrivate`, `check-{n}.json` `0600`, past 400 records or 1,048,576 bytes the recording ends and reads failed once (contract/session-recording/bounds, directory, check-file). The bounds a live `mcp --record` run's kept checks must stay inside for `recording_kept`.

@history_item n=11 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — MCP checks and the `tool_call` log line: the `mcp` role writes one `tool_call` line per `tools/call`, `tool` and `ending` only, the parent of `command`; first live trial figures recorded. The span lines the pipe's `counts_agree` counts from `server.log` (contract/stand-log/tool-call-span, contract/tool-call-span/*).

@history_item n=12 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — the closed-span line class and "Engine features by runner": `cargo test --workspace` compiles blitz-dom's tracing in, a per-package `-p seven_guis` build does not; a check counts sink lines by target, never in total. Trap for this chunk: the live server is built with `cargo build --locked -p seven_guis --bin escher-session`, a per-package build, so engine-sourced log items differ from a workspace build (decision/sink-capture-counted-by-target).

@history_item n=13 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the stand log's second record class (closed span lines) ratified by the founder. decision/closed-spans-logged-as-lines, the class `server.log` is counted by.

@history_item n=14 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — `host_log` covers a sink-installing host at `trace`; the three-outcome target allowlist; traps: a piped host at `trace` fills an undrained pipe, a needle list read from the stand outgrows a hand-kept one. Origin of decision/telemetry-target-allowlist and decision/sink-drops-third-party-targets, which keep the live run's `server.log` free of content.

@history_item n=15 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** (the recording clients' stderr) — `host_log` reads 0 needles over a recording session's clients at `trace`. decision/host-stderr-no-screen-content, the guard for what a recording live server may write.

@history_item n=16 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `test_capability_statement.py` (`CapabilityStatementTest`) pins `docs/capabilities.md` and README: every verb and cause a code span, counts in words from the schema's tables, README's «the {word} driver commands as tools»; claims mapped to checks, a renamed check changing the section by hand. The pin this chunk's README and capability-page rewrite must keep green (decision/capability-statement-pinned, decision/capabilities-doc-pinned-to-schema, decision/capability-claims-name-their-checks).

@history_item n=17 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (1. Test Scope Summary, two rows) — ~ decision/capability-statement-pinned and ~ decision/capabilities-doc-pinned-to-schema: the pin also holds the a11y leg's file count and stand-repeat's named-target count against the CI leg script. A rewrite of the page's measured-run section must keep those counts as pinned.

@history_item n=18 marker=2026-11-18-capability-statement
**2026-11-18-capability-statement** (2. Test Strategy and 4. Unit Test Strategy) — ~ decision/capability-check-names-held-to-schema and ~ decision/capability-statement-from-schema: only the claim section's names and counts are pinned; leg counts read from the leg script and requirement-witness counts stated by hand. Tells P3 which lines of the page a run-result rewrite moves unpinned.

@history_item n=19 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the cold-agent pipe speaks thirteen verbs) — `graph` added to the pipe's `VERBS`, `isolated` on thirteen tools, the fake answering an empty graph; no verdict then required `graph`. The start of the verb-count chain the `isolated` contract follows.

@history_item n=20 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the pipe's tools at fourteen) — `actions` joins after `graph`; `valid_step` and `isolated` on fourteen. Continues the chain behind contract/cold-agent-verdict/isolated and contract/cold-agent-check/step.

@history_item n=21 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (3. Test Harness Contract) — + decision/cold-agent-verbs-without-layout: the driver at fifteen verbs while the pipe stayed at fourteen, provisionally. Explains the later removal and why the pipe's verb list is held separately.

@history_item n=22 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (3. Test Harness Contract, 1 of 2) — ~ contract/cold-agent-check/step and ~ contract/cold-agent-verdict/isolated to fifteen verbs, `layout` after `actions`, on the founder's «Добавить сейчас». The current tool set the live run's `isolated` must read.

@history_item n=23 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (3. Test Harness Contract, removal) — − decision/cold-agent-verbs-without-layout: `layout` joins the cold-agent pipe by the founder's word; the fifteen-verb list stands in the two cold-agent contracts.

@history_item n=24 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (2. Test Strategy) — + decision/count-and-rename-sweeps: a missed numeric pin and a missed renamed witness set the convention for count changes. Applies when README and the capability page restate counts from the verdict.

@history_item n=25 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (1. Test Scope Summary and 6. E2E Test Strategy) — ~ contract/mcp-server/tools-list and ~ contract/mcp-tools-list/order: fifteen verbs, `layout` after `actions`, `help` last. The tools the live agent's init must list.

@history_item n=26 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (4. Unit Test Strategy) — + decision/prompt-goal-control-phrase-once: a control on the prompt test held only on phrases standing once. Binds any pipe change this chunk lands before the firing (for example, the timeout) with its contract test.

@history_item n=27 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (3. Test Harness Contract) — ~ decision/cold-agent-server-log-counted-only: the server log is read only to count its span lines, now including the command lines' causes for `refused_by_cause`; never printed or copied. Bounds what this chunk may show of the run's evidence.

@history_item n=28 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (3. Test Harness Contract, 4 rows) — ~ decision/cold-agent-passes-on-positive-evidence, ~ decision/cold-agent-prompt-names-no-ids, ~ contract/cold-agent-tasks/stand to seven apps (the order page added), + contract/stand-boot/boot-order. The immediate predecessor: the live run must meet all seven apps.

@history_item n=29 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (4. Unit Test Strategy) — ~ contract/cold-agent-verdict/capability-unused: also fails when `caused_by_kind` reads 0 for any of panic, error and log. The CARRY (cry-vrneu5x7) this chunk folds into v030-15's acceptance.

@history_item n=30 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (2. Test Strategy) — ~ decision/count-and-rename-sweeps gains the exception that a count-worded test name pinned as a witness string is kept, the name searched before any rename; ~ decision/cross-os-readings-by-answer-digest six → seven served tasks. Applies to the six → seven restatements in README and the capability page.

@history_item n=31 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (5. Integration Test Strategy) — + decision/caused-owner-pinned-as-graph-reads: a caused item carries the owner the UI graph reads (none on the order page). Bears on what the run's `caused` answers carry to the provider (F5).

@history_item n=32 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** — ~ contract/driver-session/acting-verbs gains `caused`; + decision/root-boundary-err-on-own-session. The `caused` field the live run's `caused_by_kind` counts rests on.

@history_item n=33 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (3. Test Harness Contract) — ~ decision/socket-carries-commands adds a step's caused with its error text; + contract/driver-session/payload-unread (a caught panic the hook missed still answers a `panic` item, so the panic count never loses one). Bears on the panic kind of `caused_by_kind` and on what crosses to the provider.

@history_item n=34 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (5. Integration Test Strategy) — ~ decision/root-boundary-err-on-own-session: caused carries the root Err's text and chain on a session of its own; + decision/mcp-result-decoded-whole.

@history_item n=35 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (2. Test Strategy) — + decision/engine-items-asserted-on-tracing-build: engine-sourced caused items are asserted only on a tracing build. Trap: the live server is a per-package `-p seven_guis` build, so engine log items may not appear; `caused_by_kind`'s log count must come from the order page's own log.

@history_item n=36 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (3. Test Harness Contract, 2 of 2) — ~ decision/socket-carries-commands adds layout answers out, on the founder's «Глагол `layout --id`». Part of what the run carries to the model provider.

@history_item n=37 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (the delegate confirms ratified) — the founder ratified the delegate confirms of the layout-carrier and cold-agent verb cards («Подтвердить обе»). The rows the run's carriage rests on stand ratified.

@history_item n=38 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (a graph answer crosses the session socket) — a boundary widening: ids, owner ids, component, state and handler names, declared writes, flag words, never a value; its confirm was provisional, delegated. Background to F4 ratifying `graph` answers to the model provider on the live run.

@history_item n=39 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the Cells screen and its graph answer, measured) — `graph` on a fresh Cells answers 293,869 bytes in one unpaged answer (residual `res-7cc5bi4k`); Cells' snapshot is 53,859 bytes in 6 parts. A live agent's `graph` on Cells is a large answer, which bears on the 600 s timeout and the $3 budget.

@history_item n=40 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (stand_actions) — the actions answer holds no value (decision/actions-answer-holds-no-value); flight-booker options `covered` until the list opens. The F4-ratified `actions` carriage and another `covered` source.

@history_item n=41 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the measured actionable counts) — the Cells `actions` answer is 106,088 bytes for 678 nodes. A second large answer for the run's time and budget estimate.

@history_item n=42 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — `advance` accepted on every session, moving the clock by exactly `ms`, `advanced_ms` exactly `ms`; eight causes, `fonts-unavailable` the eighth; ratified at 2026-10-18-engine-debts' Epoch 2 close. Rows decision/advance-*, decision/eight-refusal-causes-in-order: the counted `advance` capability and the cause set `refused_by_cause` is keyed by.

@history_item n=43 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (harness and session contracts gain the shot) — a shot's PNG crosses in band as padded base64; a no-font shot is refused `fonts-unavailable`, never skipped; an MCP screenshot is an image block then a text block; ratified by the founder. decision/screenshot-in-band and contract/mcp-server/screenshot; the run's counted screenshot crosses this way.

@history_item n=44 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (handler coverage's stated limits) — three limits: a canvas handler not counted, a handler on no replayed screen not listed, a bubbling dispatch counting every ancestor listener; mapped in the capability statement's claim map, pinned by `stand_handler_coverage` and `cli_commands`. The "where each is proven" reach statement this chunk writes names handler coverage among them (contract/handler-coverage-limits/*).

@history_item n=45 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (cli_commands and the coverage role) — `coverage <app> <check>...` replays kept checks and prints the coverage line; ratified at 2026-11-15T05-21-42-wrap. contract/escher-session-commands/coverage, part of the reach statement.

@history_item n=46 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** (2. Test Strategy) — + decision/requirement-witness-inside-stand-repeat: every release requirement names its witness inside stand-repeat, except a requirement with no screen. v030-15 is `dynamic-external`, so its claim at P5 reads against this rule and its exception.

@history_item n=47 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** (5. Integration Test Strategy) — ~ decision/typed-password-never-returned extended to graph, actions and layout answers and kept or explored checks, guarded by `stand_masks_on_new_answers`. Bounds what the live run's kept checks and answers carry.

@history_item n=48 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — + decision/start-in-state-reads-as-by-hand-path. One of the "fixture state" proofs the capability page's reach statement points to for what the run did not reach.

@history_item n=49 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — + contract/session-recording/exploration-checks, + decision/exhaustion-asserted-on-finite-page. The "exploration" proof the reach statement names.

@history_item n=50 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (3. Test Harness Contract, rows) — + contract/harness-construction/from-component-alone, + contract/harness-construction/vdom-alone, + contract/stand-boot/boot-alone. The "component alone" proof the reach statement names.

@history_item n=51 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — ~ contract/harness-construction/from-vdom: `SeededRandom::from_seed(DEFAULT_SEED)` provided as a root context. The "seed" proof the reach statement names.

@history_item n=52 marker=2026-11-14-bundled-fonts
**2026-11-14-bundled-fonts** — ~ decision/stand-checks-bundled-font guarded by `stand_bundled_font_families`; + decision/cross-os-readings-by-answer-digest. The "bundled fonts" proof the reach statement names.
