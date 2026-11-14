@task_prose part=chunk
Bundled fonts: a run's text is shaped and measured from one bundled font set, the stand's DejaVu Sans, with the host's system fonts off, on every path a run boots through — the stand's boot, the driver session's served tasks, the shared fixture pages and a harness-built document that names no font context of its own.
The generic families a run's styles name (sans-serif, serif, monospace, an unknown family) reach the bundled set, not the host.
The bundled set is held where a later canvas painter can reach it (the document's font context), and the acceptance names canvas text as a future reader of it; the canvas renderer itself is not built.
fonts-unavailable is kept and re-asserted: an app whose font context holds no paintable family is still refused before anything is painted or read, its text unchanged.
The proof: every CI OS of the test leg (Linux, macOS, Windows) reads one state of a run the same way, compared by its readings — snapshot text, bounds, text extents, lint findings — never by a shot's bytes.
@task_file n=1 path=examples/seven_guis/src/stand.rs mark=edits
what: the seven_guis headless stand, its boot and the font context its checks run under (the bundled font) — edits: the stand's font context, built from the bundled DejaVu Sans
@task_file n=2 path=packages/blitz-test-harness/src/harness.rs mark=edits
what: the harness construction and its options, the font context a harness-built document receives — edits: HarnessOptions::font_ctx default
@task_file n=3 path=packages/blitz-dom/src/document.rs
what: the engine document's construction, which turns an absent font context into the host's system collection — BaseDocument::new
@task_file n=4 path=examples/seven_guis/assets/DejaVuSans.woff2
what: the stand's bundled font asset, DejaVu Sans, already in the tree
@task_file n=5 path=packages/escher-driver/src/refusal.rs mark=re-reads
what: the driver refusal causes, the fonts-unavailable refusal re-asserted as kept
@task_file n=6 path=packages/blitz-test-harness/src/screenshot.rs mark=re-reads
what: the harness screenshot's fonts-unavailable refusal (ScreenshotError::FontsUnavailable), re-asserted as kept
@task_item n=1 part=needs
The font context of every run path — stand boot, driver session served tasks, shared fixture pages, and a harness-built document naming no font context — holds the bundled DejaVu Sans with system fonts off.
@task_item n=2 part=needs
Harness construction: a document built through the harness with no font context of its own reaches the bundled set, not the host's system collection.
@task_item n=3 part=needs
Generic family resolution: sans-serif, serif, monospace and an unknown family each resolve to the bundled font, never a host font.
@task_item n=4 part=needs
fonts-unavailable kept: the driver refusal cause and the harness screenshot refusal still refuse an app whose font context holds no paintable family, before anything is painted or read, with its text unchanged.
@task_item n=5 part=needs
Cross-OS reading equality: one state of a run read on Linux, macOS and Windows gives the same readings — snapshot text, bounds, text extents, lint findings.
@task_item n=6 part=needs
No shot compared by bytes across OSes: the cross-OS proof compares readings only and holds no screenshot step.
@task_item n=7 part=needs
Stand checks run under the bundled pinned real font and never skip for a missing font; font-metric tests never skip.
@task_item n=8 part=needs
Font-dependent assertions no longer read the host's system fonts on a run path; any existing rule that such assertions rely on system fonts is reached by this change.
@task_item n=9 part=needs
The native platform CI matrix: the test leg's Linux, macOS and Windows runners are where the cross-OS reading is held.
@task_item n=10 part=needs
Stand checks are run in both layout modes and repeated in the stand-repeat rounds under a zero-failure budget; the bundled-font change must stay inside both.
@task_item n=11 part=needs
Shared fixture pages, stated once, boot under the bundled font context; the stand boots offline from an asset already in the tree.
@task_item n=12 part=needs
Recorded checks replayed as tests and kept byte-equal to the binary's recording: a reading the bundled font changes (text extents, bounds) means the committed recordings are re-recorded and stay fresh.
@task_item n=13 part=needs
Boundary on the surface: no new carrier, verb, argument, field, refusal cause, log field or span; the refusal cause set and its printed form stay as they are.
@task_item n=14 part=needs
Boundary on the engine: no change to which font a remote page or the browser app reads, unless brought to the founder as an engine-change fork; a fork goes to the run's founder-forks file and a deferral to the 0.4.0 deferred list.
@task_item n=15 part=needs
No new dependency crate; the bundled font is a file already in the tree.
@task_item n=16 part=needs
Canvas text left open: the bundled set is reachable through the document's font context for a later GlyphRun painter, and the acceptance names canvas text as a future reader; handler coverage's canvas limits are untouched.
@task_standing n=1
convention: each new test is an integration test target of its own per behaviour, one target per file, with shared helpers in a shared module that is no target
home: 1-test-scope-summary · test-strategy
@task_standing n=2
convention: each new test data and fixture is stated in code, nothing seeded, with shared fixture pages stated once
home: 7-test-data-fixtures · test-data-fixtures
@task_standing n=3
convention: each new check is hand-written with real components and no mocking library; time is driven by the virtual clock, never sleep or wall clock
home: 8-mocking-stubbing-discipline · mocking-stubbing-discipline
@task_standing n=4
convention: each new check is controlled red-before-green, its failure prints no argument value, and a table-driven test asserts its row count
home: 4-unit-test-strategy · unit-test-strategy
@task_standing n=5
convention: each new CI step runs through the leg script, with SHA-pinned actions, read-only permissions, no secrets and bounded retried package installs
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
WPT conformance runner, its CLI, outputs, status set, pinned commit and Thai-font reftests: the chunk does not touch WPT runs.
@task_item n=2 part=not_needed
The agent-run and cold-agent pipes, their verbs, events, exit codes, verdicts and isolation: no agent-run or cold-agent behaviour changes.
@task_item n=3 part=not_needed
The session wire, session host lifecycle, state directory, socket, idle expiry and session error set: the chunk adds nothing to them.
@task_item n=4 part=not_needed
The escher-session command line and MCP role surfaces — commands, exit codes, help, tools/list, protocol revision, stdout and stderr lines: no verb, argument or line is added or changed.
@task_item n=5 part=not_needed
Handler coverage's bubbling, unreached-screen and replay rules, and the coverage line: the chunk changes no listener counting.
@task_item n=6 part=not_needed
Virtual clock, advance, settle, timer ticks and load waits: fonts do not change engine time or settling.
@task_item n=7 part=not_needed
Password masking, typed-password privacy and the session's owner-only files: the chunk carries no typed text.
@task_item n=8 part=not_needed
Contrast reading and its formula, WCAG ratio and required ratio: colours and contrast are not changed.
@task_item n=9 part=not_needed
Select list colours and painting, scrollbar fade and thumbs, box-shadow expectations and the buffered render bridge: no paint path of these is reached.
