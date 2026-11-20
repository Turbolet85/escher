@history_item n=1 marker=2026-10-06-stand-test-contract
**2026-10-06-stand-test-contract** — added the agent-run.sh argv row: a selection other than `stand`, `all` or an existing `^[a-z0-9_]+$` blitz-tests file stem exits 2 before any cargo call; harness logs metadata only. Matters because the selection keys on test-file stems, which is why this chunk renames, merges or splits no stand test file.

@history_item n=2 marker=2026-11-15-session-binary-smoke
**2026-11-15-session-binary-smoke** — `~ decision/agent-run-selection-allowlist`, `~ contract/agent-run-cli/2`: the selections are now `stand`, `all`, `session` or a file stem. The stand selection and the file-stem arm both depend on the test target set this chunk must leave as it is.

@history_item n=3 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (socket, access, input rows) — registered the session socket's owner-only state directory (0700, 0600 socket, refused if open), label 1–32 `a-z0-9-`, non-recursive cleanup and the session checks' two build-time `env!` values. These are the picked state-dir and label rows. Shared session helpers that boot hosts in a state directory must keep the directory modes, the label form and the test-only build-time env reads.

@history_item n=4 marker=2026-10-07-driver-session
**2026-10-07-driver-session** (what a sink-installing host logs) — trap: a "nothing in the logs" check over a process that installs no sink passes vacuously (`stand_session_quiet`'s host is one). A shared helper must not move a log-silence check onto a host that installs no sink, or the check passes vacuously and no longer proves the same thing.

@history_item n=5 marker=2026-10-07-sink-target-allowlist
**2026-10-07-sink-target-allowlist** — outside-target records are dropped whole. The session checks' build-time env citation names both host-spawning files and their shared module, and `host_log` is the check that must run against a sink-installing host. Moving setup into `common`/`session_common` moves these cited lines, and the log checks must still run against a host that installs the sink.

@history_item n=6 marker=2026-10-07-command-and-refusal-schema
**2026-10-07-command-and-refusal-schema** — the closed verb schema and a refusal that holds nothing of the call. Rule: keep every refusal text fixed and free of call content. This is the basis of the scope's boundary that no id, name, value or typed text newly enters a test name or assertion message.

@history_item n=7 marker=2026-10-07-act-by-id
**2026-10-07-act-by-id** — the executor runs in process, ids are looked up anew on each call, and a password typed through the driver occurs 0 times in what is returned. Five citations into `session.rs`, `session_host.rs` and `session_common/mod.rs` were re-pointed. Precedent: edits to `session_common` move security-plan citations, which must be re-read at wrap.

@history_item n=8 marker=2026-10-07-refusal-detection
**2026-10-07-refusal-detection** — stale vs not-found, then disabled, off-screen and covered, in that order with nothing run; the session's bounded id record; one `session_common` citation. These are the picked target-refusal-order and unknown-id rows. The refusal readings that the stand helpers would centralise must keep their causes and order exactly.

@history_item n=9 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (one span, eight fields) — one INFO `command` span per call with fixed-word or count fields only; only a refusal's cause name is logged. These are the picked one-command-span and refusal-log rows, which `stand_act_spans` asserts.

@history_item n=10 marker=2026-10-07-driver-command-spans
**2026-10-07-driver-command-spans** (sink's second record class; typed text read in process) — measured a sink capture of the six verbs and a refused call per cause holding 0 supplied texts and 0 id and name needles in both layout modes. That is the in-process needle reading the stand span checks hold; any shared capture or needle helper must keep the same needles and both modes.

@history_item n=11 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (socket carries calls and answers) — the socket went to wire v2 with call requests, the 16,384-byte request bound and the 1,048,576-byte answer bound; the trust boundary stayed with the owning user by file mode. Source of the picked socket-bounds, grammar and session-socket-carries-calls rows that the session-driving stand checks exercise.

@history_item n=12 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (the command line is an external input surface) — the closed argv reader. No ending writes back anything typed, and a call is validated before the session address is asked for. These are the picked cli-closed-argv-reader and no-echo rows.

@history_item n=13 marker=2026-10-10-driver-cli
**2026-10-10-driver-cli** (ids, names and values leave in an answer) — refusals and session errors got their written forms; typed text in a host's log was measured at 0 under both builds. The picked refusal-JSON contracts and the readings that stand checks parse come from here.

@history_item n=14 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (three state readings inside the ratified carriers) — selected, pressed and invalid as closed words and booleans, plus the founder's closed-field rule: a closed value inside a ratified carrier is no widening. This is the origin of the picked snapshot-state-readings and snapshot-text/diff-node state contracts that the stand snapshot checks read.

@history_item n=15 marker=2026-10-10-snapshot-states
**2026-10-10-snapshot-states** (a Markup attributes row for the three control states) — exact lower-case match against a closed set, with no attribute text written anywhere. Source of the picked aria-state-lowercase-exact-match and control-states-closed-values-only rows.

@history_item n=16 marker=2026-10-06-snapshot-state-fidelity
**2026-10-06-snapshot-state-fidelity** — a password's snapshot value masked as `MASKED_VALUE`. Also re-pointed the `dangerous_inner_html` citation (raw inner HTML via `set_inner_html`), the parse that the stand_keyboard and stand_snapshot_state fixtures rely on. Covers the picked masked-value and dangerous-inner-html rows; helpers that build those fixtures must keep the markup parsed as given.

@history_item n=17 marker=2026-10-12-self-description
**2026-10-12-self-description** (`help`) — `stand_mcp_masked` measured a password typed over MCP as masked (0 occurrences, both modes); `StateDirTooLong` states its bound with no path. `stand_mcp_masked` is a named guard of the picked mask rows, so its assertions must stay exactly as they are.

@history_item n=18 marker=2026-10-12-self-description
**2026-10-12-self-description** (session error remedy and table) — each session error is one row of a fixed table, holding no path. This is the picked session-errors-fixed-table row.

@history_item n=19 marker=2026-10-12-headless-screenshot
**2026-10-12-headless-screenshot** (password mask, causes, no image in a log) — `stand_act_spans` also reads a refused shot's `cause=fonts-unavailable`, and a shot's image reaches no log. The span-check readings a shared helper would carry must keep this cause reading.

@history_item n=20 marker=2026-10-14-stand-requirement-sweep
**2026-10-14-stand-requirement-sweep** — measured the off-Linux state-dir bound (90 holds, 91 ends `state-dir-too-long`, status 3, no path) and stated `covered`'s second limit. Covers the picked `contract/session-errors/state-dir-too-long` and covered-cause rows.

@history_item n=21 marker=2026-10-14-quality-gates
**2026-10-14-quality-gates** — the whole-workspace audit and licence gate. Lockfile verification lists the `stand-repeat` leg; `TMPDIR` is a test-only read. Covers the picked licence-gate, dependency-audit and locked-builds rows. The stand-repeat leg's targets are named, which is why this chunk leaves them unchanged.

@history_item n=22 marker=2026-10-18-one-virtual-clock
**2026-10-18-one-virtual-clock** — the closed cause set went to eight (`time-unavailable` retired), and `stand_act_spans` reads an accepted `advance` line holding schema fields only. These are the picked closed-refusal-cause-set and time-moves-only-on-advance rows; centralised refusal readings must keep the eight-cause set.

@history_item n=23 marker=2026-10-19-session-recording
**2026-10-19-session-recording** (wire v3, fifteen session errors, `CARGO_MANIFEST_DIR`) — wire v3 replies and requests (the picked `contract/session-socket-*` rows). `CARGO_MANIFEST_DIR` is also read at build time by tests. A test-only env read added by a helper must still match a registered read.

@history_item n=24 marker=2026-10-19-recorded-check-replay
**2026-10-19-recorded-check-replay** — the committed checks `tests/blitz-tests/checks/counter-*.json` are read by `stand_replay` and compared byte for byte; kept answers are masked as on every carrier. A refactor of stand setup must not change what a recorded session writes, or the byte comparison breaks.

@history_item n=25 marker=2026-10-19-visual-lint
**2026-10-19-visual-lint** (the gated part: the lint verb) — `stand_act_spans` reads a lint's line and a refused no-font lint's line; `host_log`/`mcp_log` drive a lint with 0 needles. These are more readings held by the span and log checks that must keep their subjects.

@history_item n=26 marker=2026-10-20-load-wait
**2026-10-20-load-wait** — an acting step waits on a load for at most `LOAD_WAIT` (5 s wall). This is the picked bounded-load-wait and load-wait-bound rows; helpers that settle or act must not add a wait of their own.

@history_item n=27 marker=2026-10-24-stand-requirement-sweep
**2026-10-24-stand-requirement-sweep** — the paged part read is measured in process by `stand_act_spans` (part 2 of `many_items`, 36 calls per mode, later 38), plus the `host_log` standing check. Call counts the span check asserts are a trap: a helper that adds or drops a call moves `run.calls`.

@history_item n=28 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (`visible_region` through transforms, `off-screen`'s meaning) — `off-screen`'s meaning under a rotation, pinned by `stand_act_transformed` and others. Covers the picked `contract/refusal-causes/off-screen` row; the stand refusal readings must stay the same.

@history_item n=29 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the free-disk action at its two CI sites) — the `Stand checks repeated` job runs a SHA-pinned free-disk step. It ties the stand-repeat leg's named job to its CI shape, which this chunk leaves unchanged.

@history_item n=30 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (the engine's `change` on a checkbox and a radio) — the source of the picked engine-change-event-checkable row, guard `checkable_change_event`.

@history_item n=31 marker=2026-10-31-engine-debts
**2026-10-31-engine-debts** (Cells' `cells-grid` keys) — the source of the picked cells-grid-keyboard row.

@history_item n=32 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (`covered`'s closed-option clause) — `covered`'s meaning gains the closed-option text, measured by `stand_act_popup`. Covers the picked `contract/refusal-causes/covered` row.

@history_item n=33 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (citations moved with their lines) — re-pointed citations into `stand_act_select.rs`, `stand_snapshot_state.rs`, `host_log.rs` and `mcp_log.rs`. Precedent: test-file edits move security-plan citations, which must follow at wrap.

@history_item n=34 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (what else of the list crosses the carriers) — the select list crosses only as a row's bounds and pixels, with no `expanded` key. This is the picked select-list-crossing row.

@history_item n=35 marker=2026-10-31-select-popup
**2026-10-31-select-popup** (the driver's `type` on any select) — the source of the picked driver-type-on-select row.

@history_item n=36 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (no served port, two test-only TCP binds) — test-only binds are registered as test-only in the plan, under the 2026-10-15 session rule. A shared test helper that binds anything would owe such a registration.

@history_item n=37 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `matrix_test` free-disk step retired) — `test_only_stand_repeat_frees_disk` pins the free-disk action to `Stand checks repeated`. This is a CI pin on the stand-repeat leg that the chunk must leave unchanged.

@history_item n=38 marker=2026-11-01-upstream-sync-ui-graph
**2026-11-01-upstream-sync-ui-graph** (the `embolden` leg, locked) — a locked `--test select_popup_painted` leg, last in `FAST_LEGS`. This is a named blitz-tests target that `ci-leg.sh` pins, so it cannot be renamed or merged.

@history_item n=39 marker=2026-10-27-upstream-sync-engine-debts
**2026-10-27-upstream-sync-engine-debts** (`TMPDIR` read at a second test-only site) — every env read in test code is listed among the environment values; a helper that reads a temp dir adds a site to register.

@history_item n=40 marker=2026-10-13-stand-contrast-harness
**2026-10-13-stand-contrast-harness** — blitz-test-harness takes `color`; the harness takes no `tracing` and no `style`, and blitz-tests gains no dependency. Covers the picked test-harness-no-tracing-no-style row; matches the scope's rule of no dependency and `Cargo.toml` unchanged.

@history_item n=41 marker=2026-10-10-mcp-surface
**2026-10-10-mcp-surface** (`serde_json` reviewed) — `serde_json` is read-only and reads untrusted MCP stdin. Covers the picked serde-json rows; a test helper's JSON reading must not make serde_json a writer in product code.

@history_item n=42 marker=2026-10-06-id-stability-across-code-edits
**2026-10-06-id-stability-across-code-edits** — the anchored stable-id tier, ratified. Covers the picked anchored-id-tier row whose ids the stand checks assert.

@history_item n=43 marker=2026-10-13-stand-keyboard-harness
**2026-10-13-stand-keyboard-harness** — keyboard activation (Space keyup, Enter) and a button click's focus, ratified. Covers the picked space-keyup-activates and button-click-focuses rows behind the stand keyboard fixtures.

@history_item n=44 marker=2026-10-17-upstream-sync-engine-debts
**2026-10-17-upstream-sync-engine-debts** (the merged ARIA reads) — `aria-labelledby` outranks label, descriptions reach the adapter only, one trimmed `aria-label`. Covers the picked aria-labelledby-outranks-label, aria-label-trimmed-once and descriptions-adapter-only rows.

@history_item n=45 marker=2026-10-20-select-by-id
**2026-10-20-select-by-id** — the select row and the labelable label; disabled by presence feeds the activation gates. Covers the picked disabled-read-by-presence row family.

@history_item n=46 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the snapshot's clickable reading) — `NodeState.clickable` as a closed bool, `NODE_FIELDS` fourteen. Covers the picked `contract/snapshot-text/clickable` and `contract/diff-node/clickable` contracts.

@history_item n=47 marker=2026-11-07-ui-graph
**2026-11-07-ui-graph** (the thirteenth verb, graph) — a `graph` call with an argument is refused `malformed`, and the cause set stays eight. Covers the picked `contract/refusal-causes/malformed` row.

@history_item n=48 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the actions verb's validation) — `actions` with an argument is refused `malformed` in process, on the CLI and over MCP. Also part of the picked malformed contract.

@history_item n=49 marker=2026-11-08-what-can-i-do-here
**2026-11-08-what-can-i-do-here** (the line locators the chunk's edits moved) — locators in `session_common` and `stand_act_spans` re-pointed; `stand_act_spans`' driven calls went to 38. This is the latest precedent for test-file edits moving citations and asserted call counts.

@history_item n=50 marker=2026-11-08-handler-coverage
**2026-11-08-handler-coverage** (the kept actions step's replay now exercised) — `stand_replay` holds the kept `actions` step's replay in both modes. A stand check whose assertions must stay as they are.

@history_item n=51 marker=2026-11-14-start-in-a-state
**2026-11-14-start-in-a-state** — `+ decision/fixture-name-closed-table`: a fixture state name is checked against a closed table before any boot. The stand fixture boot that the helpers would share goes through this table.

@history_item n=52 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Input Validation) — `~ decision/driver-installs-no-subscriber`: an acting step runs under escher-telemetry's scoped log capture, which prints nothing. Covers the picked row; it bears on what a shared act-and-read helper can observe in logs.

@history_item n=53 marker=2026-11-16-errors-with-the-action
**2026-11-16-errors-with-the-action** (Threat Model Summary) — added the caused-answer and root-boundary-Err rows (later replaced). This is the origin of the `caused` field that `stand_act_error_text`/`stand_act_errors`, this chunk's first candidate family, read.

@history_item n=54 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Threat Model Summary) — `− decision/caused-answer-crossing → decision/caused-text-crossing`, `− decision/root-boundary-err-as-screen-content → decision/root-boundary-err-crossing`. These are the picked caused-text and root-boundary-Err rows. `stand_act_error_text` proves them, so a shared caused-reading helper must keep those assertions word for word.

@history_item n=55 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Input Validation) — `~ decision/mask-holds-on-every-carrier` (caused message excepted), `~ decision/socket-carries-screen-reading` (caused text). These are picked rows; the error-text checks assert these exceptions.

@history_item n=56 marker=2026-11-16-error-text-in-the-answer
**2026-11-16-error-text-in-the-answer** (Logging & Monitoring) — `~ decision/no-screen-content-in-logs`: a step's caused text reaches no log, guarded by `stand_act_spans`, `host_log` and `mcp_log`. A picked row whose guards are stand checks.

@history_item n=57 marker=2026-11-16-error-field-in-the-answer
**2026-11-16-error-field-in-the-answer** — `~ decision/caused-text-crossing` (error kept within the text bound), `~ decision/mask-holds-on-every-carrier`, `~ decision/socket-carries-screen-reading`. These give the current text of the picked rows that the first candidate family's assertions hold.

@history_item n=58 marker=2026-11-17-layout-on-the-carriers
**2026-11-17-layout-on-the-carriers** — `~ decision/socket-carries-screen-reading` (layout readings), `~ decision/select-list-crossing`, `~ contract/command-span/verb`. These are the current text of picked rows read by stand span and layout checks.

@history_item n=59 marker=2026-11-17-a-component-in-isolation
**2026-11-17-a-component-in-isolation** — `~ contract/driver-session-reply/no-attribute-text`: the author-text exception for a record's message and a caused item's `error`. This is the picked contract. It also ratified the delegate confirms of the error-field and layout cards.

@history_item n=60 marker=2026-11-17-stand-requirement-sweep
**2026-11-17-stand-requirement-sweep** — `~ decision/mask-holds-on-every-carrier` guard gains `stand_masks_on_new_answers`. The picked row's three guards (`stand_act_filled`, `stand_mcp_masked`, `stand_masks_on_new_answers`) are stand checks. Folding their setup must keep each guard's file, name and assertions, or the row's guard citation goes stale.

@history_item n=61 marker=2026-11-18-served-error-page
**2026-11-18-served-error-page** — the order page's authored caused texts (panic payload, quantity rule, log message) now cross. `stand_order_served`, a clone in the first candidate family, reads that served page; its readings belong to this widening and must keep their subjects.

@history_item n=62 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (API Security) — `~ decision/session-socket-sole-listener` (one listener of every default build) and `+ decision/cdp-server-opt-in-loopback`. A picked row; the stand session helpers use that one listener.

@history_item n=63 marker=2026-11-19-upstream-sync-ahead-of-the-debts
**2026-11-19-upstream-sync-ahead-of-the-debts** (Input Validation) — `+ decision/cancelled-pointerup-still-clicks` (guard `pointerup_cancelled`) among the merge's rows. This is a picked row and the most recent source of the merge-era rows.

@history_item n=64 marker=2026-11-19-build-directory-held
**2026-11-19-build-directory-held** — `~ decision/locked-builds`: every build, CI cargo leg and sweep run passes `--locked`. A picked row; the chunk must keep `Cargo.lock` unchanged and every leg locked.

@history_item n=65 marker=2026-10-05-ci-gate-legs
**2026-10-05-ci-gate-legs** — introduced the cargo-deny audit job (slow tier, uncached) and SHA-pinned ci.yml actions. This is the origin of the picked dependency-audit-ci-gate row.
