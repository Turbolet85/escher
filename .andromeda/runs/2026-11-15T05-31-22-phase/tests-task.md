@task_prose part=chunk
Takes upstream/main at ea07ee716 into the fork as its eleventh upstream sync: a cherry-pick of the 16 new upstream commits onto the registry dependency pins, each commit left out keeping its measured reason.
Measures the take of upstream's four Taffy-main commits whole (merge breaks, edits, dependency audit and MSRV legs with a git Taffy) and takes them only if a registry taffy release carries them; otherwise leaves them out and routes the no-git-dependency reversal to the founder as one question.
Re-reads each left-out commit's reason from the last take against the new pin, and each earlier merge surface row by row.
Re-runs every escher proof after the take: fast and doc legs, stand checks in both layout modes, the incremental oracle, the a11y leg, the dependency audit, the embolden witness and fork CI green with every job settled.
Re-reads the per-OS test tallies of the take's green CI run and re-checks the sink allowlist against the taken delta.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace root manifest, holding the coupled dependency pin set (Taffy, AnyRender, Dioxus) the take lands on and that stays on registry versions
@task_file n=2 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the check pinning the blitz-tests target list, one file and one target per behaviour, that upstream's new and edited test files run under
@task_file n=3 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg script whose fast, doc and dependency audit legs are re-run after the take
@task_file n=4 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the standing sink capture checks, held at 0 needles, re-read against the taken delta's new log call sites and targets
@task_file n=5 path=packages/blitz-dom/clippy.toml mark=re-reads
what: the blitz-dom clippy ban on building requests outside the one stamping constructor, read against the font-face loading commit
@task_file n=6 path=packages/blitz-dom/src/events/pointer.rs
what: the pointer click handling, its arms standing as private click functions, onto which an upstream edit of a click arm is re-applied by hand
@task_item n=1 part=needs
The upstream take itself: the 16 commits bring SVG presentation attributes, inline SVG overflow and transform mapped to CSS, svg_source namespace declarations, the hr element's color, noshade and size, the presentational-hint helpers refactor, font-face font-stretch passed to the font library, SVG image paint clipping and an isolated layer — engine behaviour changed for every document, remote pages included.
@task_item n=2 part=needs
The workspace takes no git dependency: the git Taffy and every AnyRender git pin bump stay out unless a registry release carries them; Dioxus stays on 0.7.
@task_item n=3 part=needs
Coupled dependency pins are taken as one set or none; a layout, text or paint number the take moves is re-measured and recorded, never loosened to pass.
@task_item n=4 part=needs
The minimum supported Rust version stays 1.91, verified by build, measured too with a git Taffy for the founder question.
@task_item n=5 part=needs
The dependency audit (advisories and licences) runs green after the take, every new or removed dependency reviewed by hand; also measured with a git Taffy.
@task_item n=6 part=needs
Upstream's new and edited blitz-tests test files: one file and one target per behaviour, the blitz-tests manifest's difference from upstream kept, the target list check passing.
@task_item n=7 part=needs
Upstream tests kept as written where they run; upstream's mounted-flush test stays left out on Dioxus 0.7, returning only with the Dioxus 0.8 move.
@task_item n=8 part=needs
Conflict resolutions keep both sides' intent: the one virtual clock, the select model and its list painted by the scene painter, the hit stops and scrolled-box bounds, stable element ids and the snapshot, range and keyboard models, inert, and the accessibility state reads; each resolution recorded.
@task_item n=9 part=needs
Stand checks run in both layout modes on the bundled font, with a zero-failure budget, after the take.
@task_item n=10 part=needs
The incremental/non-incremental layout differential oracle passes after the take.
@task_item n=11 part=needs
The a11y leg and its membership stay green after the take.
@task_item n=12 part=needs
The embolden local witness for font-embolden paint is re-run after the take.
@task_item n=13 part=needs
Fork CI green on the pushed commit with every job settled; the clippy legs over the workspace and upstream blitz-dom; cargo legs run locked; per-package green is not the workspace green.
@task_item n=14 part=needs
Per-OS tallies re-read from the take's green CI job logs by the recorded method (ANSI stripped), each moved count attributed by test name and explained.
@task_item n=15 part=needs
Sink allowlist re-checked: every new tracing or log call site, target, print and env read in the taken delta read against the telemetry target allowlist; the sink still drops third-party targets and capture stays counted by target at 0.
@task_item n=16 part=needs
blitz-dom builds requests through the one stamping constructor: an upstream sub-resource request added by the take, or an edited stylesheet handler or font-face fetch caller, takes the document URL.
@task_item n=17 part=needs
The cognitive-complexity ceiling read on the host, whose exception names the click arm functions: an upstream edit to a click arm is re-applied by hand to its arm function.
@task_item n=18 part=needs
If the take touches the hit-test path, the inline-root early return's border-box coordinate is fixed and a check of the returned coordinate is added as this chunk's work.
@task_item n=19 part=needs
Additivity: edits of ours to upstream-owned code are limited to what a conflict, a gate failure or a ruled fork requires, each named and recorded; no new crate, port, env var or listener of ours, so the CDP server listener commit stays out.
@task_item n=20 part=needs
The buffered render on the Dioxus 0.7 bridge with its ported upstream test kept as written stays unchanged by the take.
@task_item n=21 part=needs
WPT-only upstream commits taken keep the fork's WPT reading report-only and the WPT workflow upstream-only.
@task_standing n=1
convention: a regression test added for a bug names the bug and documents it
home: 2-test-strategy
@task_standing n=2
convention: a new test is controlled red before green
home: unit-test-strategy
@task_standing n=3
convention: a unit test sits in an inline test module of its source file
home: 2-test-strategy · test-strategy
@task_standing n=4
convention: a new test asserts no sleep or wall-clock read and drives time through the virtual clock
home: integration-test-strategy
@task_standing n=5
convention: a test failure prints no argument value
home: unit-test-strategy
@task_item n=1 part=not_needed
The session lifecycle area: the session host, wire, socket, state directory, recording and session errors; the sync changes no session protocol.
@task_item n=2 part=not_needed
The escher-session command line, MCP role, its exit codes, help, snapshot budget and tool results; the chunk adds no verb and changes no output of the binary.
@task_item n=3 part=not_needed
The agent-run and cold-agent harness contracts: their verbs, events, exit codes, verdict and isolation; the sync touches no harness script.
@task_item n=4 part=not_needed
Driver refusal causes, driver verbs, actions and graph answers, and handler coverage; the take changes no driver code.
@task_item n=5 part=not_needed
Harness contrast, lint defects and text contrast readings; no contrast or lint rule changes.
@task_item n=6 part=not_needed
Per-crate coverage floors and the mutation baseline; the sync adds no escher unit whose coverage or mutants are read.
@task_item n=7 part=not_needed
Committed recorded checks, their recording script and exclusions; no check is re-recorded by a sync.
@task_item n=8 part=not_needed
Mocking and stubbing discipline: hand-written stand-ins, network stand-in counting and the stand timer; no stand-in is added or changed.
@task_item n=9 part=not_needed
Windowed end-to-end tests and the real MCP client operator pass; neither lies in this chunk.
@task_item n=10 part=not_needed
CI action SHA pins, secrets, read-only permissions, cache save branches and trigger branches; the sync adds no CI job.
