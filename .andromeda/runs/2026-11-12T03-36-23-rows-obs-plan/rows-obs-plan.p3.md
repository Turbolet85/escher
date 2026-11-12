@row_none n=1 rec=1 type=4
@row_none n=2 rec=2 type=4
relied: Resource load errors in blitz-dom end in a warn event with an `error` field and are not propagated to callers.
@row_none n=3 rec=3 type=4
@row_none n=4 rec=4 type=4
@row_none n=5 rec=5 type=4
@row_none n=6 rec=6 type=4
relied: A failed stylesheet fetch drops its request from the pending critical resources, so loading does not wait on it.
@row_none n=7 rec=7 type=4
@row_none n=8 rec=8 type=4
@row_add n=1 row=contract/wpt-report/subtest-message area=7-error-capture-reporting rec=9
boundary: The WPT report
item: subtest message
promise: A subtest's error strings are joined with newlines into the report's subtest message.
@row_none n=9 rec=10 type=S
@row_add n=2 row=contract/window-error-event/message area=7-error-capture-reporting rec=11
boundary: The window error event
item: message
promise: The window error event carries the error's message.
@row_add n=3 row=contract/window-error-event/filename area=7-error-capture-reporting rec=11
boundary: The window error event
item: filename
promise: The window error event carries filename as the empty string.
@row_add n=4 row=contract/window-error-event/lineno area=7-error-capture-reporting rec=11
boundary: The window error event
item: lineno
promise: The window error event carries lineno 0.
@row_add n=5 row=contract/window-error-event/colno area=7-error-capture-reporting rec=11
boundary: The window error event
item: colno
promise: The window error event carries colno 0.
@row_add n=6 row=contract/window-error-event/error area=7-error-capture-reporting rec=11
boundary: The window error event
item: error
promise: The window error event carries the thrown error.
@row_add n=7 row=contract/window-onerror/arguments area=7-error-capture-reporting rec=12
boundary: window.onerror
item: arguments
promise: window.onerror is called with message, source, lineno, colno and error, in that order.
@row_add n=8 row=contract/script-error-surface/error-handler-exception area=7-error-capture-reporting rec=13
boundary: The script error surface
item: exception thrown by an error handler
promise: An exception thrown by an error handler is recorded but fires no further error event.
@row_add n=9 row=contract/script-error-source-labels/timer-callback area=7-error-capture-reporting rec=14
boundary: Script error source labels in reports
item: timer callback
promise: An error raised in a timer callback is labelled "timer callback" in reports.
@row_add n=10 row=contract/script-error-source-labels/event-listener area=7-error-capture-reporting rec=14
boundary: Script error source labels in reports
item: event listener
promise: An error raised in an event listener is labelled "event listener" in reports.
@row_add n=11 row=contract/script-error-source-labels/error-event-listener area=7-error-capture-reporting rec=14
boundary: Script error source labels in reports
item: error event listener
promise: An error raised in an error event listener is labelled "error event listener" in reports.
@row_add n=12 row=contract/script-error-source-labels/timer-microtasks area=7-error-capture-reporting rec=14
boundary: Script error source labels in reports
item: timer microtasks
promise: An error raised in timer microtasks is labelled "timer microtasks" in reports.
@row_add n=13 row=contract/script-error-source-labels/event-microtasks area=7-error-capture-reporting rec=14
boundary: Script error source labels in reports
item: event microtasks
promise: An error raised in event microtasks is labelled "event microtasks" in reports.
@row_add n=14 row=decision/no-file-loads-from-non-file-documents area=7-error-capture-reporting rec=15
name: No file: loads from non-file: documents
rule: A file: load asked by a non-file: document is refused as a typed forbidden fetch error: a script fetch rejects with a TypeError, a module import fails, and a script element's source is skipped and recorded as a script error.
@row_none n=10 rec=16 type=S
@row_add n=15 row=absence/error-reporting-service area=7-error-capture-reporting rec=17
capability: An error-reporting client or service
@row_none n=11 rec=18 type=S
@row_none n=12 rec=19 type=S
@row_none n=13 rec=20 type=4
@row_none n=14 rec=21 type=4
@row_none n=15 rec=22 type=4
@row_none n=16 rec=23 type=4
@row_none n=17 rec=24 type=4
@row_none n=18 rec=25 type=4
@row_none n=19 rec=26 type=4
@row_none n=20 rec=27 type=4
@row_none n=21 rec=28 type=4
@row_none n=22 rec=29 type=4
relied: A log record from a target outside the sink's allowlist never reaches stderr.
@row_none n=23 rec=30 type=S
@row_add n=16 row=decision/sink-target-allowlist area=8-pii-scrubbing-compliance rec=31,33
name: Log sink target allowlist
rule: escher's log sink formatter allows targets by prefix: an engine target prints only safe fields, the rest [redacted]; an escher_ target prints every field but the content-named set; any other target is dropped whole at every level, whatever RUST_LOG names.
why: The accepted cost is that a third-party WARN or ERROR never reaches stderr, at any setting.
@row_add n=17 row=decision/scrub-at-sink-not-call-sites area=8-pii-scrubbing-compliance rec=31
name: Scrubbing at the sink, not at call sites
rule: Scrubbing is applied in escher's own sink, a closed span's line judged pair by pair by the same rule, and no engine call site is edited.
@row_add n=18 row=decision/content-span-fields-never-stored area=8-pii-scrubbing-compliance rec=32
name: Content-named span fields never stored
rule: The value of a content-named span field is never stored; its [redacted] marker is written at print time, and every other span field's value is held only in process memory until the span closes.
@row_add n=19 row=decision/scrub-sink-reach area=8-pii-scrubbing-compliance rec=33
name: Reach of the scrubbing sink
rule: The scrubbing sink is installed by the two binaries of the seven_guis package; engine events reach it only when an engine tracing feature is on.
except: The upstream apps' stdout subscribers and the WPT runner's env_logger stay unscrubbed.
@row_add n=20 row=decision/agent-run-log-no-user-content area=8-pii-scrubbing-compliance rec=34
name: Agent-run harness log carries no user content
rule: The agent-run harness log sits outside the sink and carries no user content: none of its event keys is in the content-named set and no captured test output enters it.
@row_add n=21 row=decision/agent-run-raw-log-unscrubbed area=8-pii-scrubbing-compliance rec=34
name: Agent-run raw log left unscrubbed
rule: The agent-run raw cargo/libtest output log stays unscrubbed, is never printed by logs, and is gitignored under target/.
why: Unscrubbed by design, like the CI logs.
@row_add n=22 row=decision/cold-agent-log-no-user-content area=8-pii-scrubbing-compliance rec=35
name: Cold-agent pipe log carries no user content
rule: The cold-agent pipe log sits outside the sink and carries no user content: no event or verdict key is content-named and no transcript text, tool argument, answer, reply or check enters an event or the verdict.
@row_add n=23 row=decision/cold-agent-transcript-raw area=8-pii-scrubbing-compliance rec=35
name: Cold-agent transcript kept raw
rule: The cold-agent transcript, raw model output and tool I/O, carries the stand's screens to the model provider and holds host paths, unscrubbed, never printed, and gitignored under target/.
why: Carried to the model provider by design.
@row_add n=24 row=decision/cold-agent-kept-checks-at-rest area=8-pii-scrubbing-compliance rec=35
name: Cold-agent kept checks held at rest only
rule: The live cold-agent session's kept checks live in an owner-only record directory, holding calls and answers at rest, never printed, copied or committed.
@row_none n=24 rec=36 type=8
@row_none n=25 rec=37 type=7
@row_none n=26 rec=38 type=S
@row_add n=25 row=decision/ci-platform-github-workflows area=9-ci-integration rec=39
name: CI on GitHub workflows
rule: CI runs as GitHub workflows under .github/workflows/.
@row_none n=27 rec=40 type=S
@row_add n=26 row=decision/wpt-publishing-upstream-only area=9-ci-integration rec=41,48,49
name: WPT results published upstream only
rule: The WPT report and wptscores.json are published to GitHub Pages and the WPT results repository only on upstream DioxusLabs/blitz; the publishing jobs are repository-guarded and never run on the fork.
@row_add n=27 row=decision/fork-ci-two-artifacts area=9-ci-integration rec=41
name: Fork CI uploads two artifacts
rule: The fork's CI uploads exactly two artifacts, a leg's merged log on failure and the coverage report on success, each kept 7 days, neither holding user data.
@row_add n=28 row=decision/local-state-never-uploaded area=9-ci-integration rec=42,52,53,55
name: Local state stays local
rule: The agent-run and cold-agent state, a session's state directory and a recording directory stay local and are uploaded by no CI leg.
@row_add n=29 row=decision/committed-kept-checks-clean area=9-ci-integration rec=42
name: Committed kept checks hold no secrets
rule: The kept checks committed in git hold no typed text, host path or secret.
@row_none n=28 rec=43 type=6
@row_add n=30 row=contract/apt-install-diagnostic/attempt area=9-ci-integration rec=44
boundary: The package-install diagnostic line
item: attempt
promise: One stderr line per failed attempt names the attempt number, and nothing of a user's.
@row_add n=31 row=contract/apt-install-diagnostic/phase area=9-ci-integration rec=44
boundary: The package-install diagnostic line
item: phase
promise: The line names the phase as one of update, install or repair.
@row_add n=32 row=contract/apt-install-diagnostic/exit-status area=9-ci-integration rec=44
boundary: The package-install diagnostic line
item: exit status
promise: The line names the failed attempt's exit status.
@row_none n=29 rec=45 type=S
@row_none n=30 rec=46 type=S
@row_none n=31 rec=47 type=S
@row_add n=33 row=decision/ci-log-artifact-on-failure area=ci-integration rec=50
name: Per-leg CI log uploaded on failure
rule: Each CI leg's merged stdout and stderr is written to its own log, truncated at the leg's start, and uploaded only on failure as artifact ci-log-{job id}, kept 7 days; it is unscrubbed build output holding no user data.
@row_add n=34 row=decision/coverage-artifact-on-success area=ci-integration rec=51
name: Coverage report uploaded on success
rule: The coverage leg's lcov file, line counts of the workspace's public source with no user data, is uploaded only on success as artifact coverage-report, kept 7 days.
@row_add n=35 row=decision/agent-run-cleanup-scoped area=ci-integration rec=52
name: Agent-run cleanup touches only its own state
rule: The agent-run cleanup removes the harness's status, events and raw run log and touches nothing else under target/.
@row_add n=36 row=decision/session-state-dir-private area=ci-integration rec=53
name: Session state directory private and empty of content
rule: A session's state directory is created 0700 by the host and holds the socket file alone while a session is up and nothing after stop or idle expiry; no log, event or screen content is written into it.
@row_add n=37 row=contract/session-lifecycle/idle-expiry area=ci-integration rec=53
boundary: The session lifecycle
item: idle expiry
promise: The host's idle expiry ends a session exactly as stop does.
@row_add n=38 row=decision/recording-dir-private area=ci-integration rec=54
name: Recording directory private
rule: A recording directory is created 0700 with its parent existing and is refused when open to its group or others; it holds call content at rest and never a log or event line; recording is unix only.
@row_add n=39 row=contract/record-refusals/record-dir-not-private area=ci-integration rec=54
boundary: Session recording refusals
item: record-dir-not-private
promise: A recording directory open to its group or others is refused with record-dir-not-private.
@row_add n=40 row=contract/recording-file/check-file area=ci-integration rec=54
boundary: The recording file
item: check-{n}.json
promise: Each recorded session gets one check-{n}.json, mode 0600, created new and never replaced, rewritten whole after every call through a temporary file and a rename.
@row_add n=41 row=decision/recording-bounds area=ci-integration rec=54
name: Recording bounds
rule: A recording ends on a failed write or a passed bound of 400 records or 1,048,576 bytes, and only then does the recorder remove it; escher otherwise never removes a recording.
@row_add n=42 row=contract/recording-file/typed-text area=ci-integration rec=54
boundary: The recording file
item: typed text
promise: Typed text is held unmasked, except a type to a masked or unresolved control, held as masked.
@row_add n=43 row=contract/recording-file/answer area=ci-integration rec=54
boundary: The recording file
item: answer
promise: Each call's answer is kept whole as the line the driver wrote, masked as on every carrier, with its accepted flag and digest; a shot's answer is held as width and height without png.
@row_add n=44 row=decision/cold-agent-cleanup-scoped area=ci-integration rec=55
name: Cold-agent cleanup touches only its own state
rule: The cold-agent cleanup removes the pipe's state and touches nothing else under target/; the per-run temporary session directory is removed when the run exits.
