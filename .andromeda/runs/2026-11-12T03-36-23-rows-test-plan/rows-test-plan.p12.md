@row_add n=1 row=decision/mutation-controlled-tests area=unit-test-strategy rec=1,9,10,11
name: Tests controlled red-before-green
rule: A new check is controlled: a named mutation of what it guards turns it red, and the restored tree reads green again.
@row_add n=2 row=decision/assertions-print-no-argument-value area=unit-test-strategy rec=1,3
name: Test failures print no argument value
rule: An assertion over a driver call names only its row and index and never an argument value.
why: A failing test log then prints no argument value.
@row_add n=3 row=decision/table-tests-count-their-rows area=unit-test-strategy rec=1
name: Table-driven tests assert their row count
rule: Each table a driver test reads is stated in the test itself and its row count asserted.
@row_add n=4 row=decision/session-error-messages-fixed area=unit-test-strategy rec=1
name: Session error messages are fixed text
rule: Every session error message is a non-empty fixed string holding no `/` and no exit code; the input-or-output arm adds only the kind's name and is spelled «input or output», since «I/O» holds a `/`.
@row_add n=5 row=contract/refusal-causes/cause-set area=unit-test-strategy rec=1
boundary: Refusal causes
item: cause set
promise: A closed set of eight causes in a stated order, `fonts-unavailable` the eighth; each name, meaning and remedy is a fixed text, the remedies pairwise distinct and no `/` in any.
@row_add n=6 row=contract/refusal-causes/printed-form area=unit-test-strategy rec=1
boundary: Refusal causes
item: printed form
promise: A refusal's printed form opens with its cause's name and ends with its remedy.
@row_add n=7 row=contract/lint-defects/overlap area=unit-test-strategy rec=1
boundary: Lint defects
item: overlap
promise: `overlap` is the first of the six lint defects, in their stated order.
@row_add n=8 row=contract/lint-defects/clipped area=unit-test-strategy rec=1
boundary: Lint defects
item: clipped
promise: `clipped` is the second of the six lint defects, in their stated order.
@row_add n=9 row=contract/lint-defects/truncated area=unit-test-strategy rec=1
boundary: Lint defects
item: truncated
promise: `truncated` is the third of the six lint defects, in their stated order.
@row_add n=10 row=contract/lint-defects/off-screen area=unit-test-strategy rec=1
boundary: Lint defects
item: off-screen
promise: `off-screen` is the fourth of the six lint defects, in their stated order.
@row_add n=11 row=contract/lint-defects/zero-size area=unit-test-strategy rec=1
boundary: Lint defects
item: zero-size
promise: `zero-size` is the fifth of the six lint defects, in their stated order.
@row_add n=12 row=contract/lint-defects/contrast area=unit-test-strategy rec=1
boundary: Lint defects
item: contrast
promise: `contrast` is the sixth of the six lint defects, in their stated order.
@row_add n=13 row=contract/lint-findings/order area=unit-test-strategy rec=1
boundary: Lint findings
item: order
promise: Lint findings come in document order, then in the stated order of the defects.
@row_add n=14 row=contract/driver-verbs/verb-table area=unit-test-strategy rec=1
boundary: Driver verbs
item: verb table
promise: Fourteen verbs in a stated order at three levels — `instance`, `session`, `tool` — each with help, arguments and at least one result field.
@row_add n=15 row=contract/driver-verbs/lookup area=unit-test-strategy rec=1
boundary: Driver verbs
item: lookup
promise: A verb is found by its exact name only; a name differing in case or padding finds nothing.
@row_add n=16 row=contract/driver-verbs/level-check area=unit-test-strategy rec=1
boundary: Driver verbs
item: level check
promise: A verb is checked at its own level only: a session-level verb given as an instance call is refused `unknown-verb` before its arguments are read; the session commands and `help` are admitted at session level, an argument to `help` refused `malformed`.
@row_add n=17 row=contract/driver-argument-kinds/name area=unit-test-strategy rec=1
boundary: Driver argument kinds
item: ArgKind::Name
promise: A name argument is 1 to 32 bytes of `a-z`, `0-9` and `-`.
@row_add n=18 row=contract/driver-argument-kinds/part area=unit-test-strategy rec=1
boundary: Driver argument kinds
item: ArgKind::Part
promise: A part argument is a whole number from 1 to `MAX_PART`, 1000.
@row_add n=19 row=contract/driver-json/number area=unit-test-strategy rec=1
boundary: Driver JSON answers
item: number
promise: A number is written as its shortest decimal, and as `null` when not finite.
@row_add n=20 row=contract/driver-json/masked-value area=unit-test-strategy rec=1
boundary: Driver JSON answers
item: masked value
promise: A masked node value is written as `MASKED_VALUE`.
@row_add n=21 row=contract/driver-cli/refused-line area=unit-test-strategy rec=1
boundary: Driver command line
item: refused line
promise: A command line that is no command, or that the schema refuses, runs nothing.
@row_none n=1 rec=2 type=5
@row_none n=2 rec=4 type=5
@row_none n=3 rec=5 type=5
@row_none n=4 rec=6 type=5
@row_add n=22 row=contract/coverage-line/shape area=unit-test-strategy rec=7
boundary: The coverage line
item: shape
promise: One JSON object `{"unexercised":[{"id":…,"owner":…,"event":…}],"checks":n,"held":m}`, keying each unexercised handler.
@row_add n=23 row=contract/coverage-line/owner area=unit-test-strategy rec=7
boundary: The coverage line
item: owner
promise: A handler's `owner` is left out when it has none.
@row_add n=24 row=contract/coverage-role/unreadable-check area=unit-test-strategy rec=7
boundary: The coverage role
item: unreadable check
promise: A missing file, a non-check, a check past its bound and another app's check are each refused `malformed`.
@row_add n=25 row=decision/actions-pinned-to-commit-sha area=unit-test-strategy rec=8
name: CI actions pinned to commit SHAs
rule: Every `uses:` in the CI workflows is pinned to a 40-hex commit SHA, and every Rust toolchain step names its toolchain.
guard: CiWorkflowTest
@row_add n=26 row=decision/ci-read-only-permissions area=unit-test-strategy rec=8
name: CI runs with read-only permissions
rule: The workflow grants `contents: read` at workflow level and no job adds a grant.
guard: CiWorkflowTest
@row_add n=27 row=decision/ci-uses-no-secrets area=unit-test-strategy rec=8
name: CI uses no secrets
rule: No CI workflow step reads a `secrets.` value.
guard: CiWorkflowTest
@row_add n=28 row=decision/bounded-package-installs area=unit-test-strategy rec=8
name: Bounded package installs in CI
rule: No CI step runs `apt-get` directly: packages are installed through the install script with package names only, and the one action step that installs packages carries a `timeout-minutes` above 0.
guard: test_o_package_installs_are_bounded
@row_add n=29 row=decision/upstream-repository-guard area=unit-test-strategy rec=8
name: Upstream-only publishing jobs
rule: Every job of publish-browser, wpt and wpt-post-results runs only in the `DioxusLabs/blitz` repository.
guard: UpstreamGuardTest
@row_add n=30 row=decision/fast-jobs-gate-the-rest area=unit-test-strategy rec=8
name: Fast jobs gate the rest
rule: The four fast CI jobs need nothing and every other job needs all four.
guard: CiWorkflowTest
@row_add n=31 row=decision/ci-legs-through-leg-script area=unit-test-strategy rec=8
name: CI legs run through the leg script
rule: Each Linux CI job runs its leg through `ci-leg.sh`, and the leg is in the script's list.
guard: CiWorkflowTest
@row_add n=32 row=decision/failure-logs-uploaded area=unit-test-strategy rec=8
name: Leg logs uploaded on failure
rule: Each leg job uploads its log on failure, kept 7 days.
guard: CiWorkflowTest
@row_add n=33 row=decision/coverage-floor-per-crate area=unit-test-strategy rec=8
name: Coverage floor per crate
rule: The coverage leg exits 0 when every crate is at its floor and exits 1 when one is under it.
guard: LegScriptTest
@row_add n=34 row=contract/ci-leg-exit-codes/2 area=unit-test-strategy rec=8
boundary: CI leg script exit codes
item: 2
promise: An unknown leg exits 2.
@row_add n=35 row=decision/apt-install-retries area=unit-test-strategy rec=8
name: CI package install retried
rule: A failed install attempt is tried again from `update`, a stalled call ends at its bound and is tried again, and an install failed by its bound is repaired before the next attempt.
guard: InstallScriptTest
@row_add n=36 row=contract/apt-install-exit-codes/1 area=unit-test-strategy rec=8
boundary: CI install script exit codes
item: 1
promise: Every attempt failing exits 1 after three attempts, never 124 or 137.
@row_add n=37 row=contract/apt-install-exit-codes/2 area=unit-test-strategy rec=8
boundary: CI install script exit codes
item: 2
promise: A bad call — no package, a name outside the closed form, an unknown or malformed option, zero attempts or a zero bound — exits 2 before anything runs.
@row_add n=38 row=decision/blitz-tests-manifest-difference area=unit-test-strategy rec=9
name: blitz-tests manifest's difference from upstream
rule: The blitz-tests manifest leaves `autotests` on and holds exactly one `[[test]]` table, `all`, with `test = false`.
why: The fork's standing two-line difference from upstream's manifest.
guard: BlitzTestsTargetsTest
@row_add n=39 row=decision/cold-agent-tests-offline area=unit-test-strategy rec=10
name: Cold-agent pipe tested offline
rule: The cold-agent pipe's tests use no live model, credential or built binary, running under a client shim and a fake session binary.
guard: ColdAgentPipeTest
@row_add n=40 row=decision/cold-agent-wrong-call-bar area=unit-test-strategy rec=10
name: Cold-agent wrong-call bar
rule: A cold-agent run fails `wrong-calls-over-bar` when its wrong calls exceed 5 % of its calls.
guard: ColdAgentPipeTest
@row_add n=41 row=contract/cold-agent-verdict/capability-unused area=unit-test-strategy rec=10
boundary: Cold-agent verdict
item: capability-unused
promise: A run that leaves a capability unused fails `capability-unused`.
@row_add n=42 row=contract/cold-agent-verdict/recording-not-kept area=unit-test-strategy rec=10
boundary: Cold-agent verdict
item: recording-not-kept
promise: A `start` that left no kept check fails `recording-not-kept`.
@row_add n=43 row=contract/cold-agent-exit-codes/2 area=unit-test-strategy rec=10
boundary: Cold-agent pipe exit codes
item: 2
promise: A usage error exits 2.
@row_add n=44 row=contract/cold-agent-exit-codes/3 area=unit-test-strategy rec=10
boundary: Cold-agent pipe exit codes
item: 3
promise: A missing client or a missing binary exits 3 before any state is written.
@row_add n=45 row=decision/cold-agent-isolation area=unit-test-strategy rec=10
name: Cold-agent run isolated
rule: A cold-agent run fails on an isolation breach: an extra tool, a second server, a working directory naming the root, or an API key source.
guard: ColdAgentPipeTest
@row_add n=46 row=decision/cold-agent-output-scrubbed area=unit-test-strategy rec=10
name: Cold-agent output scrubbed
rule: A cold-agent run's events and verdict hold no scrub-set key and no tool argument, tool answer, reply or check text.
guard: ColdAgentPipeTest
@row_add n=47 row=decision/cold-agent-session-outside-root area=unit-test-strategy rec=10
name: Cold-agent session outside the root
rule: A cold-agent session runs outside the repository root and is removed after, and its prompt names no stand id.
guard: ColdAgentPipeTest
@row_add n=48 row=decision/capability-statement-from-schema area=unit-test-strategy rec=11
name: Capability statement read from the schema
rule: The capability statement names every verb and every refusal cause and states its counts in words read from the schema's tables, none hand-copied.
guard: CapabilityStatementTest
@row_none n=5 rec=12 type=5
@row_none n=6 rec=13 type=5
@row_none n=7 rec=14 type=5
@row_none n=8 rec=15 type=5
@row_none n=9 rec=16 type=5
@row_none n=10 rec=17 type=5
@row_none n=11 rec=18 type=5
@row_none n=12 rec=19 type=5
@row_none n=13 rec=20 type=5
@row_none n=14 rec=21 type=5
@row_none n=15 rec=22 type=5
@row_add n=49 row=contract/snapshot-node/clickable area=unit-test-strategy rec=23
boundary: Snapshot node
item: clickable
promise: `clickable` is true where the element's current listeners hold `click` and it is no disabled form control.
@row_add n=50 row=contract/element-id/virtual-form area=unit-test-strategy rec=24
boundary: Element ids
item: virtual form
promise: The form `{owner}///{key}` is reserved: no id the walk writes holds the virtual separator, and a key that is empty or holds `/` makes no virtual id.
@row_add n=51 row=decision/one-actionable-rule area=unit-test-strategy rec=25
name: One actionable rule for both readers
rule: Both actionable readers read one rule from an element's current listeners, never from the `data-dioxus-id` mark.
why: The mark outlives a listener removed on a re-render, so a reading of it would still count that listener.
guard: a_listener_removed_on_a_re_render_is_read_by_neither_reader
@row_none n=16 rec=26 type=5
@row_none n=17 rec=27 type=7
@row_none n=18 rec=28 type=7
@row_none n=19 rec=29 type=7
@row_none n=20 rec=30 type=7
@row_none n=21 rec=31 type=9
@row_none n=22 rec=32 type=S
@row_add n=52 row=overview/session-lifecycle area=session-lifecycle rec=33
text: The session checks drive one headless app instance held by one process, and a lifecycle another process drives, with the calls it sends, over a Unix-domain socket. The session is not part of the five-command agent-run contract, which starts and binds nothing.
@row_add n=53 row=contract/driver-session/start-label area=session-lifecycle rec=34
boundary: Driver session
item: Session::start
promise: Takes a label of 1 to 32 bytes of `a-z`, `0-9` and `-`, checked before the caller's boot runs once on the calling thread; an invalid label is `InvalidLabel` with nothing booted.
@row_add n=54 row=contract/driver-session/act area=session-lifecycle rec=34
boundary: Driver session
item: act
promise: Runs one step on the held harness and waits for quiet; an instance not quiet is `NotSettled(Busy)`, one still loading at the bound `NotSettled(Loads)`; the step is not rolled back, and nothing it reads or returns crosses the socket.
@row_add n=55 row=contract/driver-session/load-wait area=session-lifecycle rec=34
boundary: Driver session
item: LOAD_WAIT
promise: A load in flight is waited for up to 5 s of wall time, settled again every 10 ms, no time moving on the held instance; a failed load ends the wait.
@row_add n=56 row=contract/driver-session/run-validation area=session-lifecycle rec=34
boundary: Driver session
item: run
promise: A call is validated first, and a refused call has run nothing.
@row_add n=57 row=contract/driver-session/stale-or-not-found area=session-lifecycle rec=34
boundary: Driver session
item: stale · not-found
promise: An id that names no element on the screen as it reads now is refused with nothing run: `stale` when the session's record of read ids holds it, `not-found` when not.
@row_add n=58 row=contract/driver-session/id-record area=session-lifecycle rec=34
boundary: Driver session
item: record of ids
promise: Holds id text only, at most 4096 ids, for the session's life, no id longer than `MAX_ID_BYTES`, read by nothing outside the crate.
@row_add n=59 row=contract/driver-session/refusal-order area=session-lifecycle rec=34
boundary: Driver session
item: refusal order
promise: A `click` or `type` on an element that cannot take it is refused with the first that holds of `disabled` · `off-screen` · `covered`, leaving the snapshot and both focus readings as they were.
@row_add n=60 row=contract/driver-session/acting-verbs area=session-lifecycle rec=34
boundary: Driver session
item: acting verbs
promise: `click`, `type`, `press`, `advance` and `scroll` are each one settled step that waits on a load in flight and returns the diff of the snapshots before and after; `scroll` adds `in_view`.
@row_add n=61 row=contract/driver-session/reading-verbs area=session-lifecycle rec=34
boundary: Driver session
item: reading verbs
promise: `snapshot`, `screenshot`, `lint` and `graph` run no step and wait on nothing.
@row_add n=62 row=contract/driver-session/snapshot area=session-lifecycle rec=34
boundary: Driver session
item: snapshot
promise: Returns the screen's text, or, for a screen over the snapshot budget or a `part` asked, one part's text with the counts `part` and `parts`.
@row_add n=63 row=contract/driver-session/graph area=session-lifecycle rec=34
boundary: Driver session
item: graph
promise: Returns `Outcome::Graphed` holding the screen's `UiGraph`.
@row_add n=64 row=contract/driver-session/not-settled-result area=session-lifecycle rec=34
boundary: Driver session
item: settled
promise: A verb on an instance that did not go quiet is a result, `settled` false with its `busy` class, and is not rolled back.
@row_add n=65 row=contract/driver-session/screenshot area=session-lifecycle rec=34
boundary: Driver session
item: screenshot
promise: Returns a PNG of the whole screen, or of an element's bounds cropped to the visible part and rounded outward, changing nothing; a crop of no pixel is `off-screen`, an app with no font to paint text `fonts-unavailable`.
@row_add n=66 row=decision/advance-moves-clock-exactly area=session-lifecycle rec=34
name: advance moves the clock exactly
rule: Every session accepts `advance`, which moves the session's clock by exactly the asked milliseconds and then runs the caller's time step where one was handed.
why: Ratified.
@row_add n=67 row=contract/driver-session/session-verb-in-run area=session-lifecycle rec=34
boundary: Driver session
item: session-level verb
promise: `start`, `status` or `stop` handed to the instance call path is refused `unknown-verb`.
@row_add n=68 row=contract/driver-session/type-replaces area=session-lifecycle rec=34
boundary: Driver session
item: type
promise: After the focusing click, where the snapshot reads a value for the target, the driver selects all and types, so an empty text clears the control; a select is focused with no click and no DOM focus event.
@row_add n=69 row=contract/driver-session/run-carriers area=session-lifecycle rec=34
boundary: Driver session
item: run carriers
promise: The instance call path is the one thing a host does with a `call` request and the MCP server with an instance verb's tool call.
@row_add n=70 row=contract/session-recording/directory area=session-lifecycle rec=34
boundary: Session recording
item: directory
promise: Unix only, `Unsupported` elsewhere; the directory is created `0700` or refused `RecordDirNotPrivate`.
@row_add n=71 row=contract/session-recording/check-file area=session-lifecycle rec=34
boundary: Session recording
item: check file
promise: `check-{n}.json` is created `0600`, never replacing a file, and rewritten whole through a `0600` temporary and a rename.
@row_add n=72 row=contract/session-recording/record-fields area=session-lifecycle rec=34
boundary: Session recording
item: record fields
promise: Each answered instance call is recorded as `tool`, `arguments` in schema order, `masked`, `accepted`, `digest` (FNV-1a 64 over the whole written line) and `answer` (the kept line as a JSON object, a shot's without `png`).
@row_add n=73 row=contract/session-recording/bounds area=session-lifecycle rec=34
boundary: Session recording
item: bounds
promise: Past 400 records or 1,048,576 bytes, or on a failed write, the recording ends, its check removed, and the failure reads true once; the call is not rolled back.
@row_add n=74 row=contract/session-recording/check-read area=session-lifecycle rec=34
boundary: Session recording
item: Check::from_bytes
promise: A kept check is read strictly, `answer` required as a JSON object; anything else is refused `malformed`.
@row_add n=75 row=contract/check-replay/another-app area=session-lifecycle rec=34
boundary: Check replay
item: another app
promise: A check replayed on a session whose label is not the check's app runs nothing.
@row_add n=76 row=contract/check-replay/stop area=session-lifecycle rec=34
boundary: Check replay
item: stop
promise: Each step's call runs validated again, and the replay stops at the first step whose acceptance differs or whose live kept line is not equal to the kept `answer`, naming the result fields that differ.
@row_add n=77 row=decision/replay-through-the-call-path area=session-lifecycle rec=34
name: Replay through the ordinary call path
rule: A kept check replays through the session's ordinary call path, whose signature and span stay unchanged, and its result types hold no reading and no JSON-library type.
why: Ratified.
@row_add n=78 row=contract/session-host/idle-expiry area=session-lifecycle rec=35
boundary: Session host
item: IDLE_EXPIRY
promise: A host holds one session until a `stop` request or until no request has been answered for 1,800 s, ending then exactly as on `stop`.
@row_add n=79 row=contract/session-host/served area=session-lifecycle rec=35
boundary: Session host
item: served
promise: Every answered request, a refused or malformed one included, counts toward `served` and starts the expiry count again.
@row_add n=80 row=contract/session-host/start area=session-lifecycle rec=35
boundary: Session host
item: start
promise: Spawns the host command and returns its first answer.
@row_add n=81 row=contract/session-host/attach area=session-lifecycle rec=35
boundary: Session host
item: attach
promise: Returns `Hello { pid, label, served, idle_expiry_s }`.
@row_add n=82 row=contract/session-host/stop area=session-lifecycle rec=35
boundary: Session host
item: stop
promise: Returns when the state directory is gone, bounded at 5 s.
@row_add n=83 row=contract/session-host/call area=session-lifecycle rec=35
boundary: Session host
item: call
promise: Sends one call and returns `Answer { accepted, json }`; the call is validated first and a schema refusal is answered without reaching a session.
@row_add n=84 row=contract/session-errors/error-set area=session-lifecycle rec=35,1
boundary: Session errors
item: error set
promise: Fifteen session errors, each stating a kind, a fixed meaning and a remedy; the state directory's message states its bound.
@row_add n=85 row=contract/session-errors/answer-too-large area=session-lifecycle rec=35
boundary: Session errors
item: AnswerTooLarge
promise: A call's answer over the socket's bound, read from an `oversize` reply, the call having run.
@row_add n=86 row=contract/session-errors/no-session area=session-lifecycle rec=35
boundary: Session errors
item: NoSession
promise: No socket file in the state directory.
@row_add n=87 row=contract/session-errors/dead area=session-lifecycle rec=35
boundary: Session errors
item: Dead
promise: A socket file that nothing answers on.
@row_add n=88 row=contract/session-errors/already-running area=session-lifecycle rec=35
boundary: Session errors
item: AlreadyRunning
promise: A start where a session already answers.
@row_add n=89 row=contract/session-errors/timeout area=session-lifecycle rec=35
boundary: Session errors
item: Timeout
promise: An answer read past its 30 s per-read bound.
@row_add n=90 row=contract/session-errors/unsupported area=session-lifecycle rec=35
boundary: Session errors
item: Unsupported
promise: A session asked of on a non-unix target.
@row_add n=91 row=contract/session-errors/record-dir-not-private area=session-lifecycle rec=35
boundary: Session errors
item: RecordDirNotPrivate
promise: A recording directory open to its group or others.
@row_add n=92 row=contract/session-errors/recording-failed area=session-lifecycle rec=35
boundary: Session errors
item: RecordingFailed
promise: A recording that ended, read from an `ok v3 recording-failed` reply.
@row_add n=93 row=contract/session-wire/connection area=session-lifecycle rec=36
boundary: Session wire
item: connection
promise: One request line and one reply line per connection, every request carrying version `v3`.
@row_add n=94 row=contract/session-wire/hello area=session-lifecycle rec=36
boundary: Session wire
item: hello v3
promise: Answered `ok v3 pid={pid} label={label} served={n} idle_expiry_s={s}`.
@row_add n=95 row=contract/session-wire/stop area=session-lifecycle rec=36
boundary: Session wire
item: stop v3
promise: Answered `ok v3 stopping`.
@row_add n=96 row=contract/session-wire/call area=session-lifecycle rec=36,1
boundary: Session wire
item: call v3
promise: `call v3 <verb> [<name>=<kind>:<value>]…`, kinds `t` text, `n` whole number, `f` flag; the verb, the names and a text value percent-escaped byte for byte, the decoder strict.
@row_add n=97 row=contract/session-wire/accepted area=session-lifecycle rec=36
boundary: Session wire
item: ok v3 accepted
promise: `ok v3 accepted <json>` carries an accepted call's answer.
@row_add n=98 row=contract/session-wire/refused area=session-lifecycle rec=36
boundary: Session wire
item: ok v3 refused
promise: `ok v3 refused <json>` carries a refused call's refusal.
@row_add n=99 row=contract/session-wire/oversize area=session-lifecycle rec=36
boundary: Session wire
item: ok v3 oversize
promise: Sent when a call's answer is over the answer bound.
@row_add n=100 row=contract/session-wire/recording-failed area=session-lifecycle rec=36
boundary: Session wire
item: ok v3 recording-failed
promise: Sent in place of the answer when the host's recording ended after the call, read as `SessionError::RecordingFailed`.
@row_add n=101 row=contract/session-wire/refused-version area=session-lifecycle rec=36
boundary: Session wire
item: refused version
promise: A known word with another version is answered `refused version`.
@row_add n=102 row=contract/session-wire/refused-malformed area=session-lifecycle rec=36
boundary: Session wire
item: refused malformed
promise: Any other line is answered `refused malformed`.
@row_add n=103 row=contract/session-wire/request-bound area=session-lifecycle rec=36
boundary: Session wire
item: request bound
promise: A request line is at most 16,384 bytes.
@row_add n=104 row=contract/session-wire/lifecycle-reply-bound area=session-lifecycle rec=36
boundary: Session wire
item: lifecycle reply bound
promise: A lifecycle reply is at most 128 bytes.
@row_add n=105 row=contract/session-wire/answer-bound area=session-lifecycle rec=36
boundary: Session wire
item: answer bound
promise: An answer is at most 1,048,576 bytes on its JSON line.
@row_add n=106 row=contract/session-wire/answer-read-bound area=session-lifecycle rec=36
boundary: Session wire
item: answer read bound
promise: A call's answer is read under 30 s per read; a listener that never answers ends the read as `SessionError::Timeout`.
@row_add n=107 row=contract/session-wire/other-io-bound area=session-lifecycle rec=36
boundary: Session wire
item: other read and write bound
promise: Every read and write other than a call's answer is bounded at 2 s.
@row_add n=108 row=decision/socket-carries-commands area=session-lifecycle rec=36
name: The socket carries commands
rule: The session socket carries a call's element id and typed text in, and ids, accessible names, control values (a password's or file input's as the fixed mask), the snapshot's text, the diff and lint findings out.
why: Ratified, where until then none of them crossed.
@row_add n=109 row=decision/screenshot-in-band area=session-lifecycle rec=36
name: Screenshot pixels in band
rule: A screenshot's pixels cross in band, a PNG as padded base64 inside the answer's one JSON line `{"width":…,"height":…,"png":…}`, under the unchanged answer bound.
why: Ratified; the largest whole-screen answer on the stand uses 2.1 % of the bound.
@row_add n=110 row=contract/session-wire/graph-answer area=session-lifecycle rec=36
boundary: Session wire
item: graph answer
promise: Carries ids, owner ids, component names, registered state names, handler event names, the declared writes, the flag words `undeclared` · `unconfirmed` and booleans — never a value or typed text, and no `NodeId`.
@row_add n=111 row=contract/session-state/directory area=session-lifecycle rec=37
boundary: Session state directory
item: directory
promise: A directory the caller names, mode `0700`, holding `session.sock` alone while a session is up and absent after `stop` or the host's idle expiry.
@row_add n=112 row=contract/session-state/socket area=session-lifecycle rec=37
boundary: Session state directory
item: session.sock
promise: The session's socket, mode `0600`.
@row_add n=113 row=contract/session-state/path-bound area=session-lifecycle rec=37,1
boundary: Session state directory
item: path bound
promise: A socket address leaves about a hundred bytes for the whole path: 94 bytes on Linux, 90 elsewhere, a longer one refused with that bound stated.
