@row_add n=1 row=decision/png-encodes-screenshots area=dependency-security rec=1
name: png encodes a screenshot
rule: The png crate is what encodes a screenshot in escher-driver, and blitz-tests takes it only as a dev-dependency.
@row_add n=2 row=decision/no-base64-crate area=dependency-security rec=1
name: No base64 crate
rule: A screenshot's base64 is written by hand; no base64 crate is taken as a dependency.
@row_add n=3 row=decision/color-is-contrast-luminance-source area=dependency-security rec=2
name: color is the contrast reader's luminance source
rule: The color crate, already in the graph through blitz-dom, is what supplies luminance to blitz-test-harness's contrast reader.
@row_add n=4 row=decision/licence-gate area=dependency-security rec=3
name: Licence gate on every push
rule: The fork gates on licences: cargo-deny's licenses check runs over the whole workspace graph on every push, against a licences table byte-identical to upstream's, while upstream's separate licenses job stays reverted.
why: An open question, decided and ratified as the fork's chosen option.
guard: the audit leg
@row_add n=5 row=decision/free-disk-only-on-stand-repeat area=dependency-security rec=4,5
name: Runner disk freed only for the repeated stand job
rule: Only the `Stand checks repeated` CI job frees runner disk, with the free-disk-space action pinned to a commit; the `matrix_test` job holds no such step.
guard: test_only_stand_repeat_frees_disk
@row_add n=6 row=decision/embolden-locked-fast-leg area=dependency-security rec=6
name: font-embolden tested by a locked fast leg
rule: The blitz-paint font-embolden feature is tested by a locked cargo leg among the fast legs, with no CI job of its own.
@row_add n=7 row=decision/closed-refusal-cause-set area=error-handling rec=7
name: Closed refusal cause set
rule: A driver command refusal is a typed value whose cause comes from a closed set of eight, with a fault only beside `malformed`; a new verb or argument shape is refused within the set, never by adding a cause.
why: The closed set of eight was ratified.
@row_add n=8 row=decision/refusal-holds-no-call-input area=error-handling rec=7
name: A refusal holds nothing a call supplied
rule: Every refusal's name, meaning and remedy is a fixed static text holding no path, and a fault carries at most the schema's own argument name, so a refusal cannot hold anything a call supplied.
@row_add n=9 row=contract/refusal-causes/unknown-verb area=error-handling rec=7
boundary: Driver refusal causes
item: unknown-verb
promise: One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
@row_add n=10 row=contract/refusal-causes/malformed area=error-handling rec=7
boundary: Driver refusal causes
item: malformed
promise: The one cause that carries a fault; a call breaking the schema, such as a `graph` or `actions` call with an argument, is refused with it and nothing runs.
@row_add n=11 row=contract/refusal-causes/not-found area=error-handling rec=7
boundary: Driver refusal causes
item: not-found
promise: One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
@row_add n=12 row=contract/refusal-causes/stale area=error-handling rec=7
boundary: Driver refusal causes
item: stale
promise: One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
@row_add n=13 row=contract/refusal-causes/disabled area=error-handling rec=7
boundary: Driver refusal causes
item: disabled
promise: One of the eight closed causes, with a fixed name, meaning and remedy holding no path.
@row_add n=14 row=contract/refusal-causes/covered area=error-handling rec=7
boundary: Driver refusal causes
item: covered
promise: The target is covered at its hit point; its fixed meaning states the hit's limits, among them that an option of a closed select reads covered, remedied by clicking the select first.
@row_add n=15 row=contract/refusal-causes/off-screen area=error-handling rec=7
boundary: Driver refusal causes
item: off-screen
promise: The element lies outside the visible part, including outside a scrolling box that holds it; its remedy names the `scroll` verb.
@row_add n=16 row=contract/refusal-causes/fonts-unavailable area=error-handling rec=7
boundary: Driver refusal causes
item: fonts-unavailable
promise: One of the eight closed causes, returned by the executor, with a fixed name, meaning and remedy holding no path.
@row_add n=17 row=contract/refusal-json/cause area=error-handling rec=7
boundary: Refusal JSON
item: cause
promise: The refusal's cause name, inside `{"refused":{...}}`, built from the cause's fixed strings.
@row_add n=18 row=contract/refusal-json/meaning area=error-handling rec=7
boundary: Refusal JSON
item: meaning
promise: The cause's fixed meaning text.
@row_add n=19 row=contract/refusal-json/remedy area=error-handling rec=7
boundary: Refusal JSON
item: remedy
promise: The cause's fixed remedy text.
@row_add n=20 row=contract/refusal-json/fault area=error-handling rec=7
boundary: Refusal JSON
item: fault
promise: Present only on a `malformed` refusal, naming the fault.
@row_add n=21 row=contract/session-error-json/kind area=error-handling rec=7
boundary: Session error JSON
item: kind
promise: One fixed kebab-case word per session error variant, inside `{"error":{...}}`.
@row_add n=22 row=contract/session-error-json/message area=error-handling rec=7
boundary: Session error JSON
item: message
promise: The error's fixed meaning, holding no path.
@row_add n=23 row=contract/session-error-json/remedy area=error-handling rec=7
boundary: Session error JSON
item: remedy
promise: The error's fixed remedy text.
@row_add n=24 row=decision/session-errors-fixed-table area=error-handling rec=7
name: Session errors as one fixed table
rule: Each session error's kind, meaning and remedy is one row of a fixed table served by `help`, every text a fixed string holding no path, carried on stdout and in MCP results as the program's own text.
why: Ratified as the program's own fixed text inside the ratified carriers.
@row_add n=25 row=contract/mcp-tool-result/is-error area=error-handling rec=7
boundary: MCP tool result
item: isError
promise: A refusal or session error is a tool result `{content:[{type:"text",text}], isError: true}`, its text the same JSON as on the command line, a refused `screenshot` included.
@row_add n=26 row=contract/mcp-protocol-errors/parse-error area=error-handling rec=7
boundary: MCP protocol errors
item: parse error
promise: A fixed code and fixed message with no `data`; only the admitted request id is written back.
@row_add n=27 row=contract/mcp-protocol-errors/invalid-request area=error-handling rec=7
boundary: MCP protocol errors
item: invalid request
promise: A fixed code and fixed message with no `data`; only the admitted request id is written back.
@row_add n=28 row=contract/mcp-protocol-errors/method-not-found area=error-handling rec=7
boundary: MCP protocol errors
item: method not found
promise: A fixed code and fixed message with no `data`; only the admitted request id is written back.
@row_add n=29 row=contract/mcp-protocol-errors/invalid-params area=error-handling rec=7
boundary: MCP protocol errors
item: invalid params
promise: A fixed code and fixed message with no `data`; only the admitted request id is written back.
@row_add n=30 row=contract/mcp-protocol-errors/unknown-tool area=error-handling rec=7
boundary: MCP protocol errors
item: unknown tool
promise: A fixed code and fixed message with no `data`; no reply holds the tool name.
@row_add n=31 row=decision/refusal-log-cause-name-only area=error-handling rec=7
name: Only a refusal's cause name is logged
rule: Exactly one thing of a refusal reaches a log: its cause's fixed name, as the `cause` field of the call's driver span; no fault, meaning, remedy or display text is logged.
@row_add n=32 row=contract/cli-exit-codes/1 area=error-handling rec=7,8
boundary: escher-session CLI exit codes
item: 1
promise: The call was refused; the refusal JSON is written to stdout.
@row_add n=33 row=contract/cli-exit-codes/2 area=error-handling rec=8
boundary: escher-session CLI exit codes
item: 2
promise: The argv shape is not one the command takes; the usage line is printed.
@row_add n=34 row=contract/cli-exit-codes/3 area=error-handling rec=8
boundary: escher-session CLI exit codes
item: 3
promise: The session failed, such as a boot that fails, ending with its session error.
@row_add n=35 row=decision/coverage-refuses-bad-checks-malformed area=error-handling rec=8,36
name: Coverage refuses an unusable check as malformed
rule: The `coverage` role refuses a check file it cannot read, one past the check bound, one that is no check or a check of another app as `malformed`, holding nothing of the path or the file, and adds no cause.
@row_none n=1 rec=9 type=5
@row_none n=2 rec=10 type=5
@row_add n=36 row=decision/parallel-style-traversal-serialised area=error-handling rec=11
name: Parallel style traversals serialised
rule: Every parallel style traversal takes one process-wide lock, its poison recovered, so two documents resolving parallel at once are serialised.
why: Two concurrent parallel traversals otherwise panic `already mutably borrowed`; the lock adds no panic path.
@row_none n=3 rec=12 type=4
relied: The select list's public readers and writer read a row with no styles as None and add no panic path.
@row_add n=37 row=decision/dioxus-render-into-buffer area=error-handling rec=13
name: Dioxus renders into a buffer before borrowing
rule: A Dioxus document renders mutations into a buffer with the document unborrowed, then borrows it once to replay, and flushes queued mounted events after the borrow is dropped.
why: So a task polled during render can reach the document through its mounted handle with no `RefCell already borrowed` panic.
guard: dioxus_runtime_probe mounted_data_in_a_task_a_key_handler_spawned_acts tests
@row_none n=4 rec=14 type=4
relied: The render-path recorder holds no unwrap, expect or panic and has no slicing panic path.
@row_none n=5 rec=15 type=9
@row_add n=38 row=decision/skip-mounted-event-of-removed-element area=error-handling rec=16
name: Mounted event of a removed element skipped
rule: Flushing queued mounted events skips an element removed since its event was queued, for every document.
why: Differs from upstream: upstream's test asserting no mounted event is left out, as it reads one on Dioxus 0.7.
@row_add n=39 row=decision/dropped-anonymous-block-clears-parent area=error-handling rec=17
name: A dropped anonymous block clears its children's parent
rule: Dropping an anonymous block clears the layout parent of every child still naming it, so a key press on a control whose block was dropped does not panic and still acts.
why: So the id does not go stale, curing every reader of a stale layout parent.
guard: key_press_after_container_lost_its_box
@row_none n=6 rec=18 type=4
relied: A select removed, disabled, hidden or made inert with its list open closes the list with no panic, and a dropped option is no row.
@row_none n=7 rec=19 type=4
relied: A failed, empty or non-CSS stylesheet does not block render, and a net handler wrapper that does not forward error drops the failure silently.
@row_none n=8 rec=20 type=4
@row_none n=9 rec=21 type=4
@row_none n=10 rec=22 type=4
@row_none n=11 rec=23 type=4
@row_add n=40 row=decision/agent-run-selection-allowlist area=input-validation rec=24
name: agent-run.sh selection reaches cargo only as a test stem
rule: agent-run.sh exits 2 with usage before any cargo call on an unknown verb, a wrong argument count or a selection other than `stand`, `all` or an existing test-file stem matching `^[a-z0-9_]+$`.
@row_add n=41 row=decision/cold-agent-call-validation area=input-validation rec=25
name: cold-agent.sh call and task allowlist
rule: cold-agent.sh exits 2 with usage on stderr and empty stdout before any precondition or `claude` call on an unknown verb, a wrong argument count or a task outside the allowlist `stand`.
@row_add n=42 row=decision/cold-agent-fixed-prompt area=input-validation rec=25
name: Cold agent task given only as a fixed prompt
rule: A cold-agent task reaches the agent session only as a fixed prompt naming no element id and no app slug.
guard: test_the_prompt_names_no_stand_id
@row_add n=43 row=decision/apt-install-argument-validation area=input-validation rec=26
name: apt-install.sh validates every argument first
rule: apt-install.sh accepts only package names of a fixed form and bounded numeric option values, else prints usage and exits 2 before sudo, timeout, apt-get or dpkg runs.
guard: InstallScriptTest.test_a_bad_call_exits_2_before_anything_runs
@row_add n=44 row=decision/apt-install-no-secrets area=input-validation rec=26
name: apt-install.sh touches no environment or secret
rule: apt-install.sh reads no environment variable of its own, writes no file and names no secret, token or GitHub context variable.
@row_add n=45 row=decision/apt-install-bounded-repair area=input-validation rec=26
name: Bounded dpkg repair before a retry
rule: Every apt install attempt after the first first runs a time-bounded `dpkg --configure -a` repair, a failed repair failing that attempt; the first attempt runs no dpkg.
@row_add n=46 row=decision/per-crate-coverage-floors area=input-validation rec=27
name: Per-crate line coverage floors
rule: The coverage leg holds three fixed line coverage floors: escher-driver 97, dioxus-native-dom 83, blitz-test-harness 90.
guard: coverage leg
@row_add n=47 row=decision/coverage-floor-input-validation area=input-validation rec=27
name: Coverage floor checker validates and prints no paths
rule: coverage_floor.py, stdlib only, refuses a malformed or repeated floor or a crate with no record with exit 2, and prints only crate directories and counts, never a source or lcov path.
guard: CoverageFloorTest
@row_add n=48 row=decision/cold-agent-check-validation area=input-validation rec=28
name: Agent-written check validated before replay
rule: Only the reply's last fenced json block is taken, at most 65,536 bytes and 400 steps, each step of closed keys and a known verb; a check outside these is never replayed, and the check is never printed.
why: A boundary widening to a new input class, ratified.
@row_add n=49 row=decision/cold-agent-replay-isolation area=input-validation rec=28
name: Agent-written check replayed on fresh servers
rule: A valid agent-written check is replayed on fresh MCP server children that validate every step again, each reply under a 30 s bound, each child killed and reaped, a failed child reading the check failed.
@row_none n=12 rec=29 type=4
@row_add n=50 row=decision/session-socket-closed-grammar area=input-validation rec=30
name: Session socket request is a closed grammar
rule: A session socket connection carries one request line and one reply line, parsed against a closed versioned grammar with strict percent-decoding; anything outside it is refused and runs nothing, and a refusal or broken connection changes nothing.
@row_add n=51 row=contract/session-socket-request/hello area=input-validation rec=30
boundary: Session socket request
item: hello
promise: `hello v3` asks for the session's lifecycle reply.
@row_add n=52 row=contract/session-socket-request/stop area=input-validation rec=30
boundary: Session socket request
item: stop
promise: `stop v3` stops the session once its reply is written.
@row_add n=53 row=contract/session-socket-request/call area=input-validation rec=30
boundary: Session socket request
item: call
promise: `call v3 <verb> [<name>=<kind>:<value>]…` runs a validated verb, kinds `t` text, `n` whole number, `f` flag, verb, names and text percent-escaped.
@row_add n=54 row=contract/session-socket-reply/refused-version area=input-validation rec=30
boundary: Session socket reply
item: refused version
promise: A known word with another wire version; nothing runs.
@row_add n=55 row=contract/session-socket-reply/refused-malformed area=input-validation rec=30
boundary: Session socket reply
item: refused malformed
promise: A request line the grammar does not read or over 16,384 bytes; nothing runs.
@row_add n=56 row=contract/session-socket-reply/ok-accepted area=input-validation rec=30
boundary: Session socket reply
item: ok v3 accepted
promise: The call ran and was accepted; its answer JSON follows on the line.
@row_add n=57 row=contract/session-socket-reply/ok-refused area=input-validation rec=30,7
boundary: Session socket reply
item: ok v3 refused
promise: The call was refused; the refusal JSON follows on the line.
@row_add n=58 row=contract/session-socket-reply/ok-oversize area=input-validation rec=30
boundary: Session socket reply
item: ok v3 oversize
promise: The call ran but its answer is over 1,048,576 bytes.
@row_add n=59 row=contract/session-socket-reply/ok-recording-failed area=input-validation rec=30
boundary: Session socket reply
item: ok v3 recording-failed
promise: The call ran and the host's recording ended; the client reads it as a recording-failed session error.
@row_add n=60 row=decision/session-socket-bounds area=input-validation rec=30
name: Session socket bounds
rule: One connection at a time; each read and write under 2 s, a client's read of an answer under 30 s, a request line at most 16,384 bytes, a lifecycle reply 128 bytes, an answer 1,048,576 bytes.
guard: an_answer_read_past_its_bound_is_a_timeout
@row_add n=61 row=decision/socket-carries-screen-reading area=input-validation rec=30
name: Session socket carries what the screen reads
rule: An answer carries what the screen reads, ids, accessible names, control values with password and file input masked, snapshot text, diff, lint findings, UI graph, never a NodeId or an attribute value as typed.
why: The crossing was ratified.
@row_add n=62 row=decision/screenshot-in-band area=input-validation rec=30
name: Screenshot carried in band
rule: A screenshot crosses inside its answer as a base64 PNG under the unchanged answer bound, a password input painted masked.
why: Ratified as a boundary widening.
@row_add n=63 row=decision/closed-value-field-no-widening area=input-validation rec=30
name: A closed-value field in a ratified carrier is no widening
rule: A new field of a closed value set inside an already ratified carrier is not a widening and needs no ratification; a field holding authored text, an open value set, a new carrier or a new exit still does.
why: Ratified as the standing rule.
@row_add n=64 row=contract/screenshot-answer/png area=input-validation rec=30
boundary: Screenshot answer
item: png
promise: `{"width":…,"height":…,"png":"<base64>"}`, a PNG of the whole screen or of one element's visible part.
@row_add n=65 row=contract/graph-answer/nodes-states area=input-validation rec=30
boundary: Graph answer
item: nodes and states
promise: `{"nodes":[…],"states":[…]}` holding ids, component names, state names, handler event names, the flag words `undeclared` and `unconfirmed` and booleans, never a control value, typed text or NodeId.
@row_add n=66 row=contract/actions-answer/nodes area=input-validation rec=30
boundary: Actions answer
item: nodes
promise: `{"nodes":[{"id","owner","keyed","verbs":[{"verb","cause"}]}]}` holding ids, owner ids, booleans, verb words and closed cause words, never a control value or NodeId; `keyed` is provisional.
@row_add n=67 row=contract/snapshot-text/selected area=input-validation rec=30
boundary: Snapshot text state words
item: selected
promise: A bare word written for an element that states it is selected.
@row_add n=68 row=contract/snapshot-text/unselected area=input-validation rec=30
boundary: Snapshot text state words
item: unselected
promise: A bare word written for an element that states it is not selected.
@row_add n=69 row=contract/snapshot-text/pressed area=input-validation rec=30
boundary: Snapshot text state words
item: pressed
promise: A bare word written for an element that states it is pressed.
@row_add n=70 row=contract/snapshot-text/unpressed area=input-validation rec=30
boundary: Snapshot text state words
item: unpressed
promise: A bare word written for an element that states it is not pressed.
@row_add n=71 row=contract/snapshot-text/invalid area=input-validation rec=30
boundary: Snapshot text state words
item: invalid
promise: A bare word written for an element that states it is invalid.
@row_add n=72 row=contract/diff-node/selected area=input-validation rec=30
boundary: Diff node keys
item: selected
promise: A boolean, written only where the element states it.
@row_add n=73 row=contract/diff-node/pressed area=input-validation rec=30
boundary: Diff node keys
item: pressed
promise: A boolean, written only where the element states it.
@row_add n=74 row=contract/diff-node/invalid area=input-validation rec=30
boundary: Diff node keys
item: invalid
promise: A boolean, always written.
@row_add n=75 row=contract/diff-node/differs area=input-validation rec=30
boundary: Diff node keys
item: differs
promise: On a changed node only, after `bounds`: a list of the node-key names that changed; an added node is written without it.
@row_add n=76 row=decision/session-socket-unix-only area=input-validation rec=30
name: Session socket is Unix only
rule: The driver session socket exists on Unix only; elsewhere a session is `Unsupported`.
@row_add n=77 row=contract/snapshot-text/clickable area=input-validation rec=31
boundary: Snapshot text state words
item: clickable
promise: Written after `focused` only when the element's current listeners hold `click` and it is no disabled form control.
@row_add n=78 row=contract/diff-node/clickable area=input-validation rec=31
boundary: Diff node keys
item: clickable
promise: A boolean always written beside `focused`, true where the element's listeners hold `click` and it is no disabled form control; a changed node's `differs` can name it.
@row_none n=13 rec=32 type=4
relied: An open select list's highlighted row is in no snapshot and no tree, and the lint does not read its contrast.
@row_add n=79 row=decision/state-dir-owner-only area=input-validation rec=33
name: Session state directory is owner-only
rule: The session state directory is created mode 0700 and a present one granting group or others any bit is refused; the socket file is set 0600.
@row_add n=80 row=decision/state-dir-deletes-only-own area=input-validation rec=33
name: Session cleanup deletes only its own files
rule: Stopping or idle expiry removes the socket file and then the directory, non-recursively, and nothing else is ever deleted.
@row_add n=81 row=decision/state-dir-path-is-parameter area=input-validation rec=33
name: Session state path is a parameter
rule: The session state directory path is a caller parameter, never read from the environment or defaulted relative to the working directory.
@row_add n=82 row=contract/session-errors/state-dir-too-long area=input-validation rec=33
boundary: Session errors
item: state-dir-too-long
promise: The state directory path is too long for a socket address; the message states the bound, 94 bytes on Linux and 90 elsewhere, holds no path, and the remedy names a shorter or relative path; status 3.
@row_add n=83 row=decision/recording-dir-owner-only area=input-validation rec=34
name: Recording directory is owner-only
rule: The recording directory is created mode 0700 and a present one open to group or others is refused `record-dir-not-private` with status 3, nothing written and nothing spawned.
why: Ratified.
@row_add n=84 row=decision/recording-never-replaces-files area=input-validation rec=34
name: Recording never replaces or removes foreign files
rule: A recorded check file is created 0600 with create-new under the smallest free number, rewritten whole through a temp file and rename; no file is replaced and a file the recorder did not create is never removed.
@row_add n=85 row=decision/recording-bounds-end-recording area=input-validation rec=34
name: A failed or overlong recording ends
rule: A failed write or a passed bound of 400 records or 1,048,576 bytes ends the recording, its check removed, the call run and not rolled back and answered `recording-failed`.
@row_add n=86 row=decision/recording-dir-never-named area=input-validation rec=34
name: Recording directory is never exposed
rule: No tool call, verb, argument or result field names the recording directory, and nothing of it is logged.
why: Ratified.
@row_add n=87 row=decision/recorded-check-strict-reader area=input-validation rec=35
name: Recorded check is read strictly as untrusted
rule: A kept check is read as untrusted bytes, at most 1,048,576 bytes and 400 records of closed keys; anything else is refused `malformed` with no fault, holding nothing of the file, with no panic path.
@row_add n=88 row=contract/recorded-check/start-record area=input-validation rec=35
boundary: Recorded check file
item: start record
promise: The array opens with `{"tool":"start","arguments":{"app":…}}`.
@row_add n=89 row=contract/recorded-check/tool area=input-validation rec=35
boundary: Recorded check file
item: tool
promise: One of the ten instance verbs.
@row_add n=90 row=contract/recorded-check/arguments area=input-validation rec=35
boundary: Recorded check file
item: arguments
promise: Argument names drawn from that verb's schema.
@row_add n=91 row=contract/recorded-check/masked area=input-validation rec=35
boundary: Recorded check file
item: masked
promise: True on a `type` record alone.
@row_add n=92 row=contract/recorded-check/accepted area=input-validation rec=35
boundary: Recorded check file
item: accepted
promise: One of the closed record keys, whether the call was accepted.
@row_add n=93 row=contract/recorded-check/digest area=input-validation rec=35
boundary: Recorded check file
item: digest
promise: 16 lowercase hex characters.
@row_add n=94 row=contract/recorded-check/answer area=input-validation rec=35
boundary: Recorded check file
item: answer
promise: Required and a JSON object; key order is not enforced.
@row_add n=95 row=decision/replay-through-session-run area=input-validation rec=35
name: Replay runs each step through the session
rule: A replayed check runs each kept step through the session's own validated run alone, runs nothing on a session of another app and stops at the first step whose acceptance or kept answer differs.
@row_add n=96 row=decision/cold-agent-check-reader-counts-only area=input-validation rec=35
name: Cold-agent recording reader keeps counts only
rule: The cold-agent pipe's reader of its live child's kept checks holds bools and counts only, reads an unreadable directory as no check kept, and never prints or copies a check's content.
@row_add n=97 row=decision/session-label-form area=input-validation rec=37
name: Session label form
rule: A session label is 1 to 32 bytes of a-z, 0-9 and -, checked before the caller's boot runs; it is never document text, and is the one caller string a reply carries.
