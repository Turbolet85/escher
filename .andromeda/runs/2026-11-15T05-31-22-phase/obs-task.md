@task_prose part=chunk
The fork's eleventh upstream take: the 16 upstream commits since the last pin, up to upstream/main at ea07ee716, cherry-picked onto the registry pins, each commit left out keeping its measured reason, and the left-out delta of earlier takes re-read against the new pin.
The four Taffy-main commits are measured whole (merge breaks, edits, dependency audit and MSRV legs with a git Taffy, and whether a registry taffy release carries them); they are taken only through a registry release, else left out and the git-dependency reversal goes to the founder as one question.
The take brings SVG presentation attributes, inline SVG overflow and transform, the hr element's color, noshade and size, @font-face font-stretch, SVG image paint clipping and WPT-only changes; every conflict resolution keeps escher's intent and is recorded.
It re-runs every escher proof (fast, doc, a11y and audit legs, both layout modes, the incremental oracle, the embolden witness, fork CI green), re-reads the per-OS test tallies of the take's own CI run, and re-checks the sink allowlist against every new tracing/log call site, target, print and env read in the taken delta.
@task_file n=1 path=Cargo.toml
what: the root workspace manifest — the registry dependency pins (taffy 0.15) the take keeps, moved only to a registry release, never to a git dependency
@task_file n=2 path=packages/blitz-dom/src/events/pointer.rs mark=conflict
what: the pointer event handling where handle_click's arms stand as private click_* functions; an upstream edit to a handle_click arm is re-applied by hand to its arm function
@task_file n=3 path=apps/browser/src/tab.rs mark=edits
what: the browser app's tab web view, holding an expect_used allow with no justification beside it, written only if this take touches the browser app — edits: TabWebView's expect_used allow
@task_file n=4 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the standing capture checks of escher's telemetry sink, which stay at 0 needles after the take
@task_file n=5 path=packages/blitz-dom/clippy.toml mark=re-reads
what: the clippy ban on Request::get in blitz-dom outside stamped_request, read against the take's @font-face loading change
@task_file n=6 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the check pinning the blitz-tests target list, one file and one target per behaviour, that upstream's new and edited test files run under
@task_file n=7 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg runner whose fast, doc and dependency audit legs are re-run after the take
@task_item n=1 part=needs
The sink target allowlist re-checked: every new tracing or log call site, target, print and env read the taken delta brings is read, and escher's sink capture checks stay at 0 needles.
@task_item n=2 part=needs
Engine tracing set by the build's invocation: any tracing event a taken upstream commit adds to an engine crate stays behind the tracing cargo feature, so the escher binaries gain no engine line without it.
@task_item n=3 part=needs
Engine targets print safe fields only: the new attribute reads the take brings on every document (SVG presentation attributes, hr color, noshade and size, inline SVG transform) put no attribute value or page content into any log field.
@task_item n=4 part=needs
Scrubbing stays at the sink, not at call sites: an upstream log call site taken in is judged by the sink's scrub set and allowlist, never rewritten at the call site to pass.
@task_item n=5 part=needs
No new env var or listener of ours: the CDP server's served listener (127.0.0.1 via BLITZ_CDP_PORT) stays out with its commit, so no new env read reaches the sink configuration or a native main.
@task_item n=6 part=needs
The telemetry sink keeps its configuration: RUST_LOG remains the only env the sink reads, it is installed first in each native main, and its output stream is unchanged by the take.
@task_item n=7 part=needs
New and removed dependencies are reviewed by hand and run through the audit's licence check; none may bring a telemetry backend, OTel SDK, metrics exporter or error-reporting client into the workspace.
@task_item n=8 part=needs
No file: loads from non-file: documents: the @font-face font-stretch change routes any sub-resource request through stamped_request with its document's URL, and an edited StylesheetHandler construction or fetch_font_face caller takes the new document_url.
@task_item n=9 part=needs
A failed stylesheet still leaves the pending critical set wherever the take edits stylesheet or font-face loading.
@task_item n=10 part=needs
Conflict resolutions keep both sides' intent: the changed set's contract, the clock invariant, the select model and its list, the hit stops and scrolled-box bounds, the stable ids and the snapshot, the range and keyboard models, inert and the accessibility state reads.
@task_item n=11 part=needs
The inline-root hit coordinate: if the take touches hit_inner, the clips_to_padding_box early return is fixed to return content-box coordinates for an inline root, with a check that reads the returned coordinate.
@task_item n=12 part=needs
dioxus-native-dom keeps escher's behaviour: ids, snapshot, graph, reached record and the buffered render, the mutation buffer's contents staying in memory; the Dioxus 0.8 move and its mounted-event test stay out.
@task_item n=13 part=needs
The workspace takes no git dependency: the git Taffy and every AnyRender git-pin bump stay out unless a registry release carries them; coupled pins move as one set or none, and a moved layout, text or paint number is re-measured, never loosened.
@task_item n=14 part=needs
The WPT-only upstream commits (the skia CPU raster renderer option among them) may change the WPT runner; WPT scores and results stay published from upstream only.
@task_item n=15 part=needs
Per-OS tallies re-read from the test-bearing job logs of the take's own green fork CI run by the test plan's method, each moved count explained; the earlier route-only commit's unread verdict is re-read before the push.
@task_item n=16 part=needs
Every boundary widening the take carries (the new attribute reads on every document, remote pages included) is listed for the operator at P4, each marked PROVISIONAL as an engine change for every document.
@task_standing n=1
convention: every new check or test is silent on success and a failing log check names a kind and a count, never content
home: 3-observability-harness-contract
@task_standing n=2
convention: every new check that asserts an empty stderr unsets RUST_LOG for the process it runs
home: 3-observability-harness-contract
@task_standing n=3
convention: every new engine log line is compiled only with the tracing cargo feature, and escher's own crates take tracing ungated
home: 2-telemetry-strategy · telemetry-strategy
@task_standing n=4
convention: every new log line on an engine target prints safe fields only and every escher target redacts the content-named set
home: 8-pii-scrubbing-compliance · pii-scrubbing-compliance
@task_item n=1 part=not_needed
The escher-driver command and tool_call span contracts and their field lists: the take brings no driver code and adds no span.
@task_item n=2 part=not_needed
The cold-agent pipe and agent-run harness rows (events, verdict fields, counted verbs, kept checks, transcripts): the take touches neither harness.
@task_item n=3 part=not_needed
escher-session's commands, exit codes, mcp and coverage roles, error object and host wire: the take changes no session binary surface.
@task_item n=4 part=not_needed
The script error surface: the window error event, window.onerror, script error source labels and the embedder's drain of script diagnostics belong to later Errors and explanation entries; no taken commit edits scripting.
@task_item n=5 part=not_needed
The session recording rows (recording file, recording bounds, recording and state directories, session lifecycle idle expiry): nothing in the take records sessions.
@task_item n=6 part=not_needed
The panic hooks (escher-telemetry's previous-hook chaining, the WASM console panic hook) and auto-exploration's panic catching: the take changes no panic path.
@task_item n=7 part=not_needed
The package-install diagnostic line and the fork CI artifact uploads: the chunk adds no CI job and changes no upload.
