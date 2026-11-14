@task_prose part=session
The session added a seeded source of randomness to the Dioxus native document: a run's randomness comes from one seed, and two runs of one seed read the same state at every step.
The source is a splitmix64 generator whose clones share one stream, a hook that hands it to components, and a default seed of 0; outside any run (the windowed shell) it seeds from std's per-process host random keys, reading no env var.
The test harness's Dioxus construction now provides the default run seed as a root context wherever the builder gave none, as it supplies the one virtual clock; so every driver session boot (command line, host, mcp and coverage roles, replay) reads the default seed.
No seed crosses any driver carrier: no verb, argument, argument kind, CLI flag, MCP tool field, kept-check key, span, log site, env read or exit was added; the operator answered the fork so (no seed on the carriers), PROVISIONAL until the founder confirms or replaces it.
A new stand check proves it in both layout modes, in a second process (the test binary re-executes itself with a fixed argv, no env change) and through the replay of a kept check; no dependency was added and the lockfile did not move; page-script Math.random stays unseeded, owned as a residual.
@task_file n=1 path=packages/dioxus-native-dom/src/seed.rs
what: the seeded source of randomness in the Dioxus native bridge — the shared-stream generator, its hook and the default run seed, ungated by the accessibility feature, logging nothing
@task_file n=2 path=packages/dioxus-native-dom/src/lib.rs
what: the Dioxus native bridge crate root — its module list and public exports, where the ungated modules (reached record, signal registry, now the seed) stand outside the accessibility feature
@task_file n=3 path=packages/blitz-test-harness/src/harness.rs
what: the test harness — the Dioxus-backed harness construction every driver session boots through, which supplies the virtual clock and now the default run seed; it takes no tracing and no style
@task_file n=4 path=tests/blitz-tests/tests/stand_seeded_randomness.rs
what: a stand check of seeded runs — one seed two runs alike, two seeds differ, the default seed, a second process by re-executing its own test binary, and a kept check's replay
@task_item n=1 part=touches
Boundary widening rule: the fork on whether a run seed crosses the driver's carriers was answered no seed crosses (PROVISIONAL, the founder's to confirm or replace); the absence belongs beside the rule and the closed driver verb schema.
@task_item n=2 part=touches
Closed driver verb schema, driver argument kinds and escher-session flags: no seed verb, argument, argument kind or flag was added; a row beside them must say a run seed is no carrier input.
@task_item n=3 part=touches
Recorded check file and kept check contents (start record, arguments): no seed key was added; a kept check replays its seeded run by the default seed alone — beside those contracts.
@task_item n=4 part=touches
Replay runs each step through the session: replay boots through the harness construction that now provides the default seed, so a replayed run reads the same randomness as the recorded one.
@task_item n=5 part=touches
Test harness takes no tracing and no style: the harness construction changed (it now provides the run seed); it still takes no tracing — true as written, over a component the session changed.
@task_item n=6 part=touches
Time moves only on advance and advance moves the virtual clock exactly: the harness now supplies a run seed beside the one virtual clock, the same determinism of a run; the seed belongs beside those rows.
@task_item n=7 part=touches
Driver reads only RUST_LOG and the RUST_LOG environment-variable contract, driver installs no subscriber and reads no env: the seed module and the harness's provide read no env var and add no log site — the host fallback seeds from std's per-process random keys, no env read; true as written.
@task_item n=8 part=touches
No process-local handle crosses with an id, signal registry holds no state value: the seeded source's Debug writes its type name alone, never its state, and nothing of it crosses a carrier.
@task_item n=9 part=touches
Session state directory rows (owner-only, path is a parameter, cleanup deletes only its own files): the new check holds a session whose state directory sits under the build's temporary directory through the shared session test module — one more reader, covered as written.
@task_item n=10 part=touches
Runner disk freed only for the repeated stand job: the new stand check joins the stand-repeat leg by its glob; no CI step, leg or job changed.
@task_item n=11 part=touches
Locked builds and workspace-inherited dependency versions: no dependency added or bumped, the lockfile unmoved — the generator is written by hand and the host seed is std's; true as written.
@task_fact n=1 slug=input-validation
fact: No run seed crosses any driver carrier — no verb, argument, CLI flag, MCP tool field or kept-check key takes or carries one; every driver session's run reads the default seed. PROVISIONAL: the operator's answer, the founder's to confirm or replace.
@task_fact n=2 slug=input-validation
fact: The seeded source takes its seed only from in-process code (a root context the builder of the virtual DOM gives, else the default seed 0); outside any run it seeds from std's per-process host random keys and reads no env var, file or carrier.
@task_fact n=3 slug=data-protection
fact: The seeded source holds a number and nothing else: its Debug writes the type name alone, never its state, and it logs nothing.
@task_item n=1 part=untouched
Authentication and authorization, CI tokens, publish and signing secrets: no auth, secret, token or CI workflow changed.
@task_item n=2 part=untouched
API security's outbound requests and navigation (blitz-net, fetch methods, per-host limits, CORS and same-origin absences, idle expiry, load waits): no network path, request or listener changed; the session socket and MCP server are unchanged.
@task_item n=3 part=untouched
Masking, select list, range input, password and accessibility-adapter rows: no control, value mask, paint or accessibility crossing changed.
@task_item n=4 part=untouched
Refusal causes, session errors, MCP protocol errors and exit codes: no cause, error, exit or reply shape was added or moved.
@task_item n=5 part=untouched
Logging and telemetry sink rows (allowlist, scrub, span lines, stderr sink): no span, field, log site or print was added; the census reads none.
@task_item n=6 part=untouched
Stored data (history database, cookies, HTTP cache, clipboard, file dialog, TLS): nothing stored or encrypted changed.
@task_item n=7 part=untouched
Dependency audit, licence gate, advisory ignores and pinned CI actions: no crate, advisory or action moved.
@task_item n=8 part=untouched
Stable element id contracts and the actions, graph and coverage answers: no id derivation, answer field or carrier changed.
@task_item n=9 part=untouched
WPT runner, rdme and cold-agent scripts: none reached.
@task_item n=10 part=untouched
Page-script JavaScript (Math.random, script fetchers, JS opt-in): page-script randomness stays unseeded and no script path changed.
