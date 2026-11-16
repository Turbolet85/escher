@history_item n=1 marker=2026-10-07-driver-session
**2026-10-07-driver-session** — (a second sink installer) made `escher-session` the second sink-installing binary and recorded its stderr by level. Trap for this chunk: a zero read from a host that installs no sink says nothing about one that does, so the layout read's 0-needle proof belongs in `host_log` / `mcp_log`, not in process alone (decision/sink-host-read-at-trace, decision/scrub-sink-reach).

@history_item n=2 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — the sink drops every target outside `ENGINE_TARGET_PREFIXES` / `ESCHER_TARGET_PREFIXES` and redacts content-named fields. Rule for later chunks: a target is admitted only together with a scrub rule for it. If the layout read adds any engine-side (Taffy/Stylo) event or a new target, it falls under this rule.

@history_item n=3 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the cause names became the command span's `cause` domain. Standing rule: field a cause by `Cause::name`, never field a `Command`, `Call` or `ArgValue` with `Debug`, nor an argument by its schema name. A new layout verb's id argument must not be fielded (decision/command-span-no-call-content, contract/command-span/cause).

@history_item n=4 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — `Outcome` joined the `Debug`-deriving types that nothing prints, logs or fields. A new outcome variant carrying placement, sizes or style inputs falls under the same rule (decision/debug-types-never-logged).

@history_item n=5 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — all eight cause names are returned by code. The session's record of ids is bounded, has no reader outside the crate and no `Debug`, and is never logged (decision/session-id-record-unprinted). The layout read looks elements up by stable id through that record and must not print it.

@history_item n=6 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** — (the command span as built; the closed span's line) built the `command` span with eight fields, each recorded only where it applies, and a never-fielded list: screen text, arguments and any Command/Outcome. It also added the closed-span line as a second record class. These are the contract rows a new verb extends. A ninth field would be a new log field and needs the founder's word.

@history_item n=7 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** — the founder ratified the closed-span line (decision/closed-span-lines). He also ruled that a diff's three sizes may reach a log while its content never does. That is the precedent that sizes are a ruled exception: the layout read's bounds or sizes are not covered by it and must not be fielded.

@history_item n=8 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** — (the command line writes answers, not logs; typed text measured in a host's log) A command's output is its answer on stdout, never a log line (contract/escher-session-stdout/command-answer). Standing rule: the next surface adds no span or log field that holds what a call supplied or a screen reads. This entry also measured the host by level under both builds, the baseline the new verb's `host_log` reading extends.

@history_item n=9 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** — added the `tool_call` span (`tool`, `ending`), with the command span as its child, ratified as a boundary widening. A new MCP tool adds a word to `tool`'s closed set and no field (decision/tool-call-span-two-closed-fields, contract/tool-call-span/*).

@history_item n=10 marker=2026-10-12-self-description
**2026-10-12-self-description** — put `help` in both spans' closed sets and kept `describe()` silent. Its rule: fixed help texts are the program's own text and add no log site. That rule covers a new verb's help and meaning lines in `describe.rs` (contract/command-span/field-count).

@history_item n=11 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** — the precedent for a new instance verb across both spans: `verb` alone when accepted, `verb` + `cause` when refused. Its never-fielded list grew to the image, its byte size and its width/height. The same applies to a layout answer's sizes, and 0 needles must be read on host and mcp stderr at `trace` under both builds.

@history_item n=12 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the cause set was cut to eight (`time-unavailable` retired), later ratified (2026-10-18-engine-debts). The refusal-cause domain is closed. A new refusal class for the layout read (e.g. a no-box element) would extend contract/refusal-cause/* and change the span line's closed values.

@history_item n=13 marker=2026-10-19-session-recording
**2026-10-19-session-recording** — recording opens no span and logs nothing. The ratified Recording directory row holds call content at rest only (decision/recording-dir-private, decision/recording-bounds, contract/recording-file/*). A new verb's answer is kept there whole.

@history_item n=14 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the recorder keeps each answer whole as `answer`, and each replayed step writes its own `command` span (contract/kept-check/*). Committed checks are the repository's own Counter sessions only (decision/committed-checks-hold-no-private-data, decision/committed-kept-checks-clean). A layout answer joins the at-rest carried set.

@history_item n=15 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** — (silent and spanless; the gated part: the `lint` verb inside the command span, no field) The lint verb records `verb` alone and its findings are never fielded or counted (contract/command-span/lint, contract/command-answer/linted). The founder's «Без поля» is the no-new-field ruling this chunk's verb copies. It also bears on the scope's lint-link question.

@history_item n=16 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — dropped the lint's «PROVISIONAL: no new field» on the founder's word, which settles the precedent that a new read verb adds no span field.

@history_item n=17 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** — a part read records `verb` alone, and its number, count and size are never fielded. `stand_act_spans`' per-mode call count moved with it, a count the new verb also moves.

@history_item n=18 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — `stand_act_spans` calls per mode and `host_log`'s recording-session clients read at `trace` with 0 needles. These are the registers a new verb's call and needles extend.

@history_item n=19 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** — `docs/capabilities.md` restates the driver's spans and the scrub's limits, and its pin reads none of those claims. A change there is made by hand (decision/capability-statement-updated-by-hand). The scope names this statement and its pin as a touched surface.

@history_item n=20 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** — the cold-agent pipe reads the live child's `server.log` only to count `tool_call` span lines (decision/cold-agent-server-log-counted-only, contract/cold-agent-verdict/counts-agree). A new tool word reaches those counts.

@history_item n=21 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** — the counted verbs are `lint` · `screenshot` · `advance` (with `capability-unused`), and the live child records into the owner-only `record/` directory (decision/cold-agent-record-owner-only, decision/cold-agent-kept-checks-*). A new verb must be decided counted or uncounted.

@history_item n=22 marker=2026-10-10-scrolling-box-bounds-and-hit
**2026-10-10-scrolling-box-bounds-and-hit** — the scrolled-box engine checks joined the silence census. These are the existing scrolled-box fixtures the layout read's scrolled case can lean on, and a new minimal fixture beside them must be recorded silent the same way.

@history_item n=23 marker=2026-10-18-engine-debts
**2026-10-18-engine-debts** — (the silence census gains the engine-debts checks) `transformed_client_rect` and `stand_act_transformed` are recorded silent. These are the transformed-box precedents for the chunk's transformed case and its fixtures.

@history_item n=24 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — `Harness::text_contrast`, a test-harness reader that reads and changes nothing, was recorded silent and spanless. That is the precedent if the layout read gains a harness-side reader.

@history_item n=25 marker=2026-10-31-select-popup
**2026-10-31-select-popup** — (the select list adds no log site or span; an open row's bounds) It placed new help and meaning text outside the widening as the program's own fixed text, under the rule of 2026-10-12. It placed a row's `bounds` outside the widening as a closed set inside the ratified crossing, under the field rule of 2026-10-10. Both are precedents for what the layout answer's numeric fields and closed words need, and for the widening rule the scope cites.

@history_item n=26 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** — (the graph verb: harness contract, command/tool_call lines, the graph answer's carried set, cold-agent VERBS, graph modules silent) This is the closest precedent: a new read verb under one `command` span recording `verb` alone, with `Outcome::Graphed` written only by `to_json` and never logged (decision/graph-answer-never-logged). `host_log` / `mcp_log` got `graph_needles`, `stand_act_spans` gained one call per mode, and `session_common` gained a reader. The cold-agent `VERBS` gained `graph`, uncounted (decision/graph-and-actions-not-counted-verbs). The carried set went at rest in the Recording directory as a ratified boundary widening.

@history_item n=27 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** — (the actions verb: spans, carried set, read changes nothing, needles, VERBS fourteen, stand_actions silent) This precedent took `VERBS` to fourteen. An accepted read-only verb records `verb` alone, and its `tool_call` parents one `command` child (contract/escher-driver-tool-call-span/command-child). It set `actions_needles` in `host_log` / `mcp_log`, `stand_act_spans` at 38 calls, and a carried set ratified as a widening (contract/actions-answer/logging). The read changes nothing in the document (decision/actions-read-changes-nothing), the same claim the layout read must prove.

@history_item n=28 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** — (coverage role, command span reach, session_common readers 35, host_log ordinals) It added the coverage role (decision/coverage-line-never-logged, contract/escher-session-coverage/*). Each replayed step writes its own `command` line, so a recorded layout read replays under the span. `session_common` readers moved to 35 and `host_log`'s test ordinals shifted, registers this chunk moves again.

@history_item n=29 marker=2026-11-15T05-21-42-wrap
**2026-11-15T05-21-42-wrap** — (ratified: the graph answer's carried set; the command span's reach gains the coverage role) It moved the delegate-PROVISIONAL graph and coverage widenings to the founder's ratification. A new verb's carried set needs the founder's own word, not a delegate confirm, or else stays PROVISIONAL.

@history_item n=30 marker=2026-11-14-seeded-randomness
**2026-11-14-seeded-randomness** — widened decision/checks-silent's exception list (a re-run child prints ids or digests). New layout checks and fixtures must fit that rule or be named in it.

@history_item n=31 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** — added decision/exploration-adds-no-span: exploration is instrumented only through the command spans of the calls it drives. A new verb that exploration reaches inherits that. It also extended contract/escher-session-coverage/malformed with the format-1 rule.

@history_item n=32 marker=2026-11-12T03-36-23-rows
**2026-11-12T03-36-23-rows** — the obs master became rows, each verified against the code. This chunk's amendments name rows by handle (`+` / `~` / `−`), not section prose.

@history_item n=33 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** — the founder ruled no OTel export in escher (absence/telemetry-backend). The layout read must add no exporter or backend.
