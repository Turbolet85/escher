@task_prose part=session
The eleventh upstream take: upstream/main read at ea07ee716, three upstream commits cherry-picked as upstream wrote them, thirteen new ones left out with measured reasons (112 left out in all).
One pick makes the engine read an @font-face rule's font-stretch (font-width) descriptor into font matching, as a new public width field on blitz-dom's font-face overrides; it adds no log site, span, event, metric, env read or fetch (census 0).
Two picks move the WPT pinned commit and add four tests to the WPT runner's timeout quarantine, each with the known reason dynamic-load-events.
The taffy crate moves 0.15.0 to 0.15.1 from the crates.io registry by a lock-only update, carrying 13 of the 14 Taffy-main commits upstream took; no package enters or leaves and no git source enters. The git Taffy take was measured green and left out.
The founder ratified both behaviour changes for every document (the font-stretch read and taffy 0.15.1's layout fixes) and granted an exception for Taffy alone to the no-git-dependency rule; neither moves an obs surface.
Fork CI green 17/17 with no test count moved in any job; nothing retired.
@task_file n=1 path=packages/blitz-dom/src/net.rs mark=merge
what: blitz-dom's net provider and font-face fetching, where @font-face rules are read and their descriptor overrides built (the engine's resource loading)
@task_file n=2 path=packages/blitz-dom/src/document.rs mark=merge
what: blitz-dom's document, the engine component that holds the DOM and registers fetched fonts with their overrides
@task_file n=3 path=wpt/WPT_COMMIT mark=merge
what: the WPT pinned commit, the upstream web-platform-tests revision the WPT runner checks out
@task_file n=4 path=wpt/runner/timeout-quarantine.txt mark=merge
what: the WPT runner's timeout quarantine list, tests skipped by the runner with a known reason each
@task_file n=5 path=Cargo.lock
what: the workspace lockfile, the resolved dependency set with registry sources and checksums
@task_item n=1 part=touches
Engine tracing behind a cargo feature and engine tracing set by the build's invocation: blitz-dom's font-face fetching and document changed; the change adds no tracing call, so the rule holds as written.
@task_item n=2 part=touches
Engine targets print safe fields only, and the sink target allowlist: a new font-face override field (a numeric font width) exists in blitz-dom but is logged at no site; no target or field enters the sink.
@task_item n=3 part=touches
Fonts-unavailable refusal cause and anything that reads font matching: font matching now also takes the @font-face font-stretch descriptor, for every document, remote pages included; no refusal cause, name or count moved.
@task_item n=4 part=touches
No file: loads from non-file documents, and the failed-stylesheet pending-set rule: the engine's resource-loading component changed (font-face descriptor read only); no load, fetch or request was added, so both hold.
@task_item n=5 part=touches
WPT report message and subtest message contracts: the WPT pinned commit moved and four tests joined the timeout quarantine with reason dynamic-load-events; the report's message shapes did not change.
@task_item n=6 part=touches
WPT scores published from upstream only and WPT results published upstream only: the runner's pinned revision and quarantine changed what it runs; publishing is unchanged.
@task_item n=7 part=touches
Metric coverage overview naming blitz-dom's resolve phases and the WPT runner's figures: taffy 0.15.1's layout fixes and the WPT pin move reach those in-process figures; no metric is emitted and no count a row holds moved.
@task_item n=8 part=touches
The absences of a telemetry backend, metrics exporters and an error-reporting service: the dependency set moved (taffy 0.15.1) but no package entered or left, so each absence still holds.
@task_item n=9 part=touches
Log coverage: the take's silence line (census 0 added log sites, prints, env reads or requests) has no row; no row holds a per-take silence line, so nothing is owed here.
@task_item n=10 part=touches
Mutation-buffer and page-content rules (session logs carry no page content): a page's @font-face font-stretch value now reaches font matching; it stays inside the engine and reaches no log, answer or record.
@task_item n=1 part=untouched
escher's telemetry sink, its log line, span line, scrub set, resource keys and RUST_LOG configuration: no sink, format, field or startup event changed.
@task_item n=2 part=untouched
The escher-driver command and tool_call spans, refusal cause names other than through font matching, and the driver's answers (graph, actions, shot, linted): no driver code or span field changed.
@task_item n=3 part=untouched
escher-session contracts: its stdout, mcp and coverage roles, exit codes, error object, host wire, session lifecycle and state or recording directories: untouched.
@task_item n=4 part=untouched
The agent-run harness and the cold-agent pipe: their events, verdict fields, reasons, counted verbs, kept checks, transcripts and cleanup: untouched.
@task_item n=5 part=untouched
Script error surfaces: window error event, window.onerror, script error source labels and the embedder's script-diagnostics drain: no script engine code changed.
@task_item n=6 part=untouched
CI integration rows: fork CI artifacts, per-leg log upload on failure, coverage upload, the package-install diagnostic line, and local state never uploaded: no workflow or CI script changed.
@task_item n=7 part=untouched
PII rows over select lists, snapshots, page events, form entries, optgroup labels and the state registry: no such component changed.
@task_item n=8 part=untouched
The WASM console panic hook and escher-telemetry's panic hook: untouched.
