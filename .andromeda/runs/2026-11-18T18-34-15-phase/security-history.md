@history_item n=1 marker=2026-10-05-fork-ci-reached
**2026-10-05-fork-ci-reached** (signing and PR-token jobs repository-guarded; CI legs locked) — `--locked` extended to every ci.yml cargo leg beside the flake and bundle; origin of decision/locked-builds, which binds the chunk's `cargo build --locked -p seven_guis --bin escher-session` before the one firing.

@history_item n=2 marker=2026-10-06-telemetry-bootstrap
**2026-10-06-telemetry-bootstrap** (escher's log sink scrubs; RUST_LOG read) — introduced escher's stderr-only scrubbing sink and the `RUST_LOG` read (contract/environment-variables/rust-log, decision/stderr-only-telemetry-sink); the pipe's `server.log` is that sink at `RUST_LOG=info`, counted by span lines for the verdict.

@history_item n=3 marker=2026-10-06-cold-agent-run-pipe
**2026-10-06-cold-agent-run-pipe** (the cold-agent login, input rows and log; PROVISIONAL) — created the `cold-agent.sh` argv row (unknown verb, wrong count or task outside the allowlist exits 2 before any precondition or `claude` call), the fixed prompt naming no element id, the operator's own Claude Code login as the only credential path (`apiKeySource` none, no env read, no CI secret), and the pipe's metadata-only log with a raw gitignored transcript and a host-path-masked committed copy; the root of decision/cold-agent-call-validation, decision/cold-agent-fixed-prompt, decision/live-runs-use-operator-login and decision/cold-agent-transcript-raw this live run fires under.

@history_item n=4 marker=2026-10-06-founder-rulings
**2026-10-06-founder-rulings** (cold-agent crossings ratified) — the cold-agent operator-login clause and stdio IPC surface moved from PROVISIONAL to ratified by the founder; the credential path the one firing uses is the ratified one, and any new secret source or credential flow would reopen it.

@history_item n=5 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** (a password's snapshot value is masked) — introduced `MASKED_VALUE` for a password input's snapshot value; the start of decision/masked-input-values and contract/snapshot-value/masked-value, the mask that must hold in everything the live run carries to the provider.

@history_item n=6 marker=2026-10-06-compact-snapshot-serialization
**2026-10-06-compact-snapshot-serialization** (one text form; a file input's value is masked) — the mask extended to a file input (host path never read), the snapshot text's quoted `id=`/name Debug form fixed (contract/snapshot-text/id-field, name-field); decision/password-file-value-masked's DOM exception stems here.

@history_item n=7 marker=2026-10-06-accessibility-tree-identity
**2026-10-06-accessibility-tree-identity** (the stable id reaches the platform accessibility API) — the first exit of the stable id and of accessible names, with the standing rule that any further consumer of the id re-opens the row; precedent for decision/stable-id-exits and decision/accessible-name-exits, whose carrier lists the live run's provider crossing rests on.

@history_item n=8 marker=2026-10-06-id-persistence
**2026-10-06-id-persistence** (the CRUD row's id key is a model-assigned u64) — a stable id is never a list index, pointer, hash, clock or process-local value (contract/stable-element-id/stability), what lets an agent-written check replay on fresh MCP children.

@history_item n=9 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** (four tiers; a third in-process reader) — scoped "no NodeId, ElementId, ScopeId or pointer" to the id and snapshot, with the trap that a wire form of the actionable check must leave `node` behind; precedent for decision/no-process-handle-crosses and the actions answer's no-NodeId rule.

@history_item n=10 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (the driver session socket: threat vector, access, input rows) — the session state directory 0700 / socket 0600 with no handshake, token or peer check (decision/driver-access-by-filesystem-permissions, decision/driver-session-no-peer-authentication), ratified by the founder as a boundary widening.

@history_item n=11 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** (outside-target records dropped) — the sink drops every record from a target outside its allowlist at every level, so a sink-installing host logs no id or name (decision/telemetry-sink-target-allowlist); the trap: a "nothing in the logs" check must run against a host that installs the sink, as the live server child does.

@history_item n=12 marker=2026-10-07-settle-detection
**2026-10-07-settle-detection** (a settle reports a load in flight and never waits on one) — decision/settle-never-waits-on-loads with its 64-pass bound; the load page among the run's apps relies on it.

@history_item n=13 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** (a driver call is validated before anything runs) — the closed verb table, one fixed failure order, no unwrap/expect/panic on call input, a refusal holding nothing of the call; the root of decision/closed-driver-verb-schema, decision/single-validation-gate, decision/no-panic-on-call-input and decision/refusal-holds-no-call-input, the validation every agent call and every replayed step meets.

@history_item n=14 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** (the command schema is executed in process; an id is taken in) — an id-taking command looks its id up anew on every call, never by index (decision/id-lookup-anew-each-call); an id that names nothing is refused with nothing run.

@history_item n=15 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** (six verbs; the detected causes; the session holds id text) — the stale / not-found split and the fixed disabled → off-screen → covered order (decision/target-refusal-order, decision/unknown-id-refused-stale-or-not-found, contract/driver-id-refusal/*) plus the bounded 4096-id session record (decision/session-id-record-bounded); these are the refusals the run's `refused_by_cause` counts.

@history_item n=16 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (one span, eight fixed-word or count fields) — the `command` span records the verb table's word and a refusal's cause name only (decision/refusal-log-cause-name-only, contract/driver-command-span/verb, cause); the pipe's `refused_by_cause` is read from exactly these span lines in `server.log`.

@history_item n=17 marker=2026-10-09-founder-rulings
**2026-10-09-founder-rulings** (the sink's second record class ratified) — one sink line per closed span of an admitted target, ratified (decision/closed-span-lines), with the standing rule that a count may reach a log but no node, id, name or value; the span-line counting the verdict reads stands on it.

@history_item n=18 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the session socket carries calls and answers; the command line is an external input surface) — the socket's bounds (16,384-byte request, 1,048,576-byte answer, 2 s / 30 s) (decision/session-socket-bounds), the no-echo-of-caller-input rule, and ids, names and values leaving in an answer (decision/socket-carries-screen-reading's base), all ratified by the founder as widenings.

@history_item n=19 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (three state readings inside the ratified carriers; the closed-field rule) — selected, pressed and invalid as closed words and booleans (decision/snapshot-state-readings, decision/control-states-closed-values-only) and the founder's standing rule that a closed-value field inside a ratified carrier is no widening while authored text, an open set, a new carrier or exit is (decision/boundary-widening-rule, decision/closed-value-field-no-widening) — the test for anything this chunk's run newly carries.

@history_item n=20 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (the MCP server's input and its crossing, ratified) — the stdio MCP server spawned by the client, no authentication, bounded request lines and results, results reaching the model provider through the client in the one live trial (decision/mcp-stdio-no-auth, decision/mcp-bounded-lines, decision/mcp-result-crossing); the second entry also made `serde_json` read-only, added the `tool_call` span and recorded the live trial's use of the operator's login — the carrier the cold-agent run's `claude` drives.

@history_item n=21 marker=2026-10-12-self-description
**2026-10-12-self-description** (help and fixed text through the validated surfaces; a session error's remedy and its table) — the program's own fixed table text crosses the ratified carriers (decision/session-errors-fixed-table), the rule under which the tools' descriptions the agent reads stand.

@history_item n=22 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (the screen's pixels in band; the engine's password mask) — the PNG crosses in band as an MCP image block, what a client does with it, a model provider included, being the client's (decision/screenshot-in-band, decision/screen-pixels-in-band, decision/mcp-results-client-responsibility); a password painted as bullets (decision/password-painted-masked) so the run's one screenshot leaks no typed password.

@history_item n=23 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** (the whole-workspace audit, the licence gate) — the locked whole-workspace audit leg; relevant to the readiness build only through decision/locked-builds.

@history_item n=24 marker=2026-10-15-cold-agent-test
**2026-10-15-cold-agent-test** (the cold-agent pipe on the escher server: the stand's screen to the provider, the agent's check as input) — task allowlist `stand`, the `escher-session` precondition (exit 3, the pipe builds nothing), the fixed `STAND_PROMPT`; the agent-written check (last fenced json block, ≤ 65,536 bytes, ≤ 400 steps, closed keys) never replayed when invalid, replayed on fresh MCP children with a 30 s bound, `check.json` never printed; the first carriage of the stand's screen to the model provider, ratified with the founder's «Один запуск, до $3» — the origin of decision/cold-agent-screen-to-provider, decision/cold-agent-check-validation, decision/model-reply-untrusted-input and decision/cold-agent-replay-isolation, and the precedent for one firing under $3.

@history_item n=25 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** (eight causes, advance on every session, the fade on the document's clock) — the cause set closed at eight, `advance` moving the virtual clock exactly, the scrollbar fade on the document clock (decision/closed-refusal-cause-set, decision/advance-moves-virtual-clock-exactly, decision/scrollbar-fade-on-document-clock); the eight words `refused_by_cause` counts against.

@history_item n=26 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (the kept check at rest, its directory, its reader and --record) — the 0700 recording directory, 0600 create-new checks, the 400-record / 1,048,576-byte bounds ending a recording with `recording-failed`, the strict `Check::from_bytes` reader, masked `type` kept as `MASKED_STAND_IN`, wire v3 with `ok v3 recording-failed`; the rules the live child's `--record target/cold-agent/record` runs under (decision/recording-dir-owner-only, decision/recorded-check-strict-reader, decision/recording-bounds-end-recording, decision/recorded-type-masked).

@history_item n=27 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** (each kept answer whole at rest, the committed checks, the replay) — a kept check holds each answer whole, masked, with a shot's width and height and no pixels; the only committed kept checks are the two stand-Counter ones (decision/committed-counter-checks, decision/kept-checks-owner-only) — so no kept check of the live run may be committed; replay runs each step through the session's validated run (decision/replay-through-session-run).

@history_item n=28 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the lint verb's findings on every carrier) — lint findings cross every ratified carrier as ids, closed defect words and numbers only and reach no log (decision/lint-findings-crossing, decision/lint-findings-closed-content); the cold-agent check's tool set grew with it; `lint` is one of the capabilities the verdict requires used.

@history_item n=29 marker=2026-10-20-load-wait
**2026-10-20-load-wait** (the acting step's bounded load wait) — an acting command waits on a load for at most `LOAD_WAIT` (5 s) (decision/load-wait-bound, decision/bounded-load-wait), under the 30 s answer read; the load page's click in the run waits it out.

@history_item n=30 marker=2026-10-20-large-screen-in-parts
**2026-10-20-large-screen-in-parts** (snapshot's part and the paged answer on every carrier) — the `part` argument (1–1000) and the paged answer `{text, part, parts}`; the part read the verdict requires and the paged decode the pipe does (decision/paged-answer-closed-decoding).

@history_item n=31 marker=2026-10-25-capability-statement
**2026-10-25-capability-statement** (the carried sets name a lint's findings and the diff) — `docs/capabilities.md` states the MCP crossing (spawning client the boundary, no authentication, the carried set, onward use the client's) adding no carrier (decision/capability-statement-mcp-crossing); a secret search (`/home/|api[_-]?key|authorization:|bearer ` and two e-mail domains) over the statement, 0 hits — the page this chunk rewrites from the verdict, whose carried set must stay what is ratified.

@history_item n=32 marker=2026-10-25-cold-agent-bar
**2026-10-25-cold-agent-bar** (the pipe's second live run, a third carriage; the live child's kept checks; six served slugs) — the precedent run: fired once through the operator's login, six apps, 66 calls, 0 wrong, its committed evidence (`live-check.json`, a host-path-masked `live-transcript.jsonl`) read clean by hygiene and no kept check committed; the live child records under `--record target/cold-agent/record` (`mkdir -m 700`, removed at cleanup, replays without `--record`), read by the pipe for bools and counts only; the paged decode; the verdict's fields and reasons (`capability-unused`, `recording-not-kept`, `wrong-calls-over-bar`) as closed words; the closed six-slug `TASKS` table and the load page's in-process `LoadNet` — the baseline and shape this chunk's seven-app run repeats (decision/cold-agent-records-private, decision/cold-agent-check-reader-counts-only, decision/cold-agent-wrong-call-bar).

@history_item n=33 marker=2026-10-27-readme-and-records
**2026-10-27-readme-and-records** (the capability statement's secret search extended to README.md) — the four-term credential search reads over `README.md`, `docs/capabilities.md` and `.github/scripts/test_capability_statement.py`, 0 hits, the e-mail half over the statement and pin alone; this chunk rewrites the README's 0.3.0 heading and the capability page from the run, so the search (decision/live-evidence-holds-no-credentials) must still read 0 on the changed files.

@history_item n=34 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`covered`'s closed-option clause; a shot of an open list holds labels as pixels; what else of the list crosses; which select opens a list) — `covered` now also refuses an option of a closed select (the CARRY cry-32fid2ry this run reads through `refused_by_cause`); an open list's shot carries option and group labels as pixels to the client, a model provider included (decision/open-list-shot-pixels); the list crosses only as row bounds and pixels, no `expanded` key (decision/select-list-crossing); the open gate (decision/select-list-open-gate) — the list opening the verdict requires.

@history_item n=35 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the UI graph joins what the carriers take out; the thirteenth verb; a recorded graph answer at rest; the verb counts the checks read; the graph answer's size) — `graph` answers ids, owner ids, component, state and handler event names, closed flag words and booleans on every carrier and in kept checks, never a value (decision/graph-answer-crossing, decision/kept-check-graph-answer, contract/driver-verbs/graph), unpaged under the answer bound (decision/graph-answer-unpaged, Cells 293,869 bytes); the cold-agent pipe's `VERBS` gained `graph`; the snapshot's `clickable` closed bool (contract/diff-node/clickable).

@history_item n=36 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions answer in the carried sets, on the socket and in kept checks; the verb table at fourteen; the unpaged answer; the shared target check; the one actionable rule) — `actions` answers ids, owner ids, `keyed`, verb and closed cause words, never a value or NodeId (decision/actions-answer-crossing, decision/actions-keyed-field, contract/actions-answer/*), unpaged (Cells 106,088 bytes, decision/actions-answer-unpaged), its causes from the executor's own shared check (decision/actions-read-shares-target-check), the read changing nothing (decision/actions-read-changes-nothing), one actionable rule (decision/one-actionable-rule); the pipe's `VERBS` gained `actions`.

@history_item n=37 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the coverage role; the kept actions step's replay; the coverage line's silence in the logs) — the coverage role refuses an unusable check `malformed` holding nothing of path or file (decision/coverage-refuses-bad-checks-malformed); a kept `actions` step's replay now exercised; the coverage check among what reaches no log — named by the scope as the proof site for what the run does not reach.

@history_item n=38 marker=2026-11-13T07-43-19-wrap
**2026-11-13T07-43-19-wrap** (Input Validation and Data Protection) — `~ decision/actions-keyed-field`, `~ contract/actions-answer/keyed`, `~ decision/kept-check-actions-answer`: the `keyed` field's PROVISIONAL mark dropped on the founder's word, so the actions answer the run carries is wholly ratified.

@history_item n=39 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** (Input Validation; Error Handling) — `+ decision/fixture-name-closed-table`, `+ decision/reach-error-closed-set`: a fixture state name is checked against a closed table before boot and a reach error reads fixed text; named by the scope as where a fixture state is proven outside the run's reach.

@history_item n=40 marker=2026-11-14-auto-exploration
**2026-11-14-auto-exploration** (Input Validation; Data Protection) — `~ contract/recorded-check/start-record` (the `format` key, any other value refused `malformed`), `~ contract/kept-check-file/entry`, `~ contract/kept-check-file/medium`: the kept checks the live child writes open with the format record and sit in the same owner-only medium.

@history_item n=41 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** (Input Validation) — `~ decision/agent-run-selection-allowlist`, `~ contract/agent-run-cli/2`: `agent-run.sh` takes the closed selection `session`; a likely path for the readiness checks that drive `escher-session` locally before the firing, under the same exit-2 allowlist.

@history_item n=42 marker=2026-11-16-layout-explained
**2026-11-16-layout-explained** (Input Validation; Logging & Monitoring) — `+ decision/session-layout-read-in-process`, `~ decision/no-screen-content-in-logs`: the layout read answers ids, closed words and numbers only and a layout answer reaches no log.

@history_item n=43 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (API Security; Threat Model Summary; Input Validation) — `~ contract/driver-answer/settled` (the class-only promise scoped to the busy reading), `+ decision/caused-answer-crossing`, `+ decision/root-boundary-err-as-screen-content`, the scoped log capture exception: the first `caused` crossing on acting answers, since replaced.

@history_item n=44 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Threat Model Summary; Logging & Monitoring; Input Validation) — `− decision/caused-answer-crossing → decision/caused-text-crossing`, `− decision/root-boundary-err-as-screen-content → decision/root-boundary-err-crossing`, `~ decision/mask-holds-on-every-carrier` (a caused item's authored message excepted), `~ decision/socket-carries-screen-reading`, `~ decision/no-screen-content-in-logs` (a step's caused text reaches no log), `password` joining the scrub set: the caused text the order page's controls produce and this run carries is this crossing.

@history_item n=45 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** (Threat Model Summary; Input Validation; Logging & Monitoring) — `~ decision/caused-text-crossing` (error kept as written within the text bound), `~ decision/mask-holds-on-every-carrier`, `~ decision/socket-carries-screen-reading`, `~ decision/escher-telemetry-allowlist-sink`: a caused item's `error` field crosses as its author wrote it, every other content-named field redacted — the shape of the order page's error text reaching the provider.

@history_item n=46 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** (Input Validation; Threat Model Summary; Logging & Monitoring; Data Protection) — `+ contract/driver-verbs/layout`, `~ decision/session-layout-read-in-process`, `~ decision/socket-carries-screen-reading`, `~ contract/recorded-check/tool` (eleven instance verbs), `~ decision/select-list-crossing` (the select-list placing word), `~ contract/actions-answer/verbs` (layout listed with no cause), `~ contract/command-span/verb`, `+ decision/kept-check-layout-answer`: the `layout` verb the verdict requires used, on every carrier and in kept checks.

@history_item n=47 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** (Threat Model Summary; Input Validation; the delegate confirms ratified) — `~ decision/cold-agent-screen-to-provider` (layout answers join what the live run carries), `~ contract/driver-session-reply/no-attribute-text` (an author's record message or caused `error` excepted); the founder ratified the delegate confirms of the error-field and layout cards by his own word — what lets caused error text and layout answers reach the provider on this run.

@history_item n=48 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** (Input Validation) — `~ decision/mask-holds-on-every-carrier` guard gains `stand_masks_on_new_answers`, holding the mask on graph, actions and layout answers, kept checks with replay and a file input over MCP — the guard behind the claim that nothing the run carries unmasks a value.

@history_item n=49 marker=2026-11-18-cold-agent-bar
**2026-11-18-cold-agent-bar** (Logging & Monitoring; Threat Model Summary) — `~ decision/cold-agent-events-metadata-only` (refusals counted per closed cause word, an outside word not counted), `~ decision/cold-agent-screen-to-provider` (graph and actions answers join on the founder's F4 «Утвердить graph и actions»): the chunk that readied this run; its caused clause was left to the served error page landing first (F5).

@history_item n=50 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** (Threat Model Summary; Logging & Monitoring) — `~ decision/cold-agent-screen-to-provider` (acting answers' caused text, the order page's authored panic payload, error and log, join what the run carries, on the founder's F5 «В 0.3.0, до прогона»), `~ decision/cold-agent-events-metadata-only` (`caused_by_kind` counted per closed kind word, only the kind read): the last widening before the one firing, and the seventh app and `caused_by_kind` the CARRY cry-vrneu5x7 requires at the claim.
