@task_prose part=chunk
The fork's eleventh upstream take: cherry-picks onto the registry pins the 16 upstream commits since the last pin, up to upstream/main at ea07ee716, taking what builds without git Taffy, git AnyRender or Dioxus 0.8, each commit left out keeping its measured reason.
What the take brings: SVG presentation attributes and inline SVG overflow and transform mapped to CSS, svg_source() namespace declarations, the hr element's color, noshade and size hints, a presentational-hint helpers refactor, @font-face font-stretch passed to the font system, SVG image paint clipping and an isolated layer, and WPT-only changes.
It measures the four Taffy-main commits whole (breaks, edits, audit and MSRV legs with a git Taffy) and whether a registry taffy carries them; without one they stay out and the reversal of the no-git-dependency rule goes to the founder as one question.
It re-reads each earlier left-out commit's reason against the new pin, re-reads the merge surfaces row by row, re-reads the per-OS test tallies of its own green CI run and re-checks the log sink allowlist on the taken delta.
It re-asserts every escher proof after the take: the fast and doc legs, the stand checks in both layout modes, the incremental/non-incremental oracle, the a11y leg, the dependency audit and the embolden witness.
@task_file n=1 path=Cargo.toml mark=conflict
what: the root workspace manifest holding the coupled dependency pins (the registry taffy 0.15 pin against upstream's git Taffy and git AnyRender pins)
@task_file n=2 path=.github/scripts/test_blitz_tests_targets.py mark=edits
what: the check pinning the blitz-tests target list, one file and one target per behaviour — edits: the target list, for upstream's new and edited test files
@task_file n=3 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI legs re-run after the take — the fast leg, the doc leg and the dependency audit leg
@task_file n=4 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the standing capture checks of the log sink allowlist, held at 0 needles
@task_file n=5 path=packages/blitz-dom/clippy.toml mark=re-reads
what: the blitz-dom ban on Request::get outside stamped_request, read against the @font-face loading commit
@task_file n=6 path=.andromeda/runs/2026-11-15T05-31-22-phase/founder-forks.md mark=edits
what: the run's founder question list — edits: the one question on reversing the no-git-dependencies rule, with its numbers
@task_item n=1 part=needs
The new SVG presentation attributes and the inline SVG overflow and transform mapped to CSS: an engine change read on every document, remote pages included, carried as a provisional engine change.
@task_item n=2 part=needs
The hr element's color, noshade and size presentational hints and the refactored presentational-hint helpers, which the font element's face and size hints and the table cellspacing, cellpadding and valign hints also run through.
@task_item n=3 part=needs
The @font-face font-stretch value now passed to the font system: font matching, the default-font fallback for unavailable families and the system-font test engine must still hold.
@task_item n=4 part=needs
SVG image paint clipping and the isolated layer it paints in: paint clips and stacking-context grouping change, and hit-testing must still stop where paint clips.
@task_item n=5 part=needs
Hit-test resolution keeps both sides' intent: the hit stops, the scrolled-box bounds and the open select list answering hits first; if the take touches the node hit function, the inline-root hit coordinate fix and its check are this chunk's.
@task_item n=6 part=needs
Every conflict resolution keeps escher's select model and its list: the list as an engine overlay painted after the root, its events, its accessibility and snapshot reads, its colours and placement in the viewport.
@task_item n=7 part=needs
The clock invariant survives the take: animation and scroll stay driven by the one virtual clock, the load wait moves no clock and settle does not wait on animation.
@task_item n=8 part=needs
The dioxus-native integration keeps escher's behaviour: stable ids, the snapshot, the UI graph, the reached record and the render buffered then replayed under one borrow, with MountedData calls during a render answering as before.
@task_item n=9 part=needs
Range and keyboard models, inert and the accessibility state reads keep escher's behaviour through every resolution.
@task_item n=10 part=needs
An upstream edit to a click arm does not pick cleanly and is re-applied by hand to its private arm function in the pointer event handling.
@task_item n=11 part=needs
Coupled dependency pins move as one set or none; a moved layout, text or paint number is re-measured and recorded, never loosened to pass.
@task_item n=12 part=needs
The workspace takes no git dependency: the git Taffy and every git AnyRender bump stay out unless a registry release carries them; MSRV stays 1.91.
@task_item n=13 part=needs
Screenshots and the stand checks re-run in both layout modes must still never paint with text skipped, and the embolden witness still passes.
@task_item n=14 part=needs
Edits of ours to upstream-owned code are limited to what a conflict, a gate failure or a ruled fork requires, each named and recorded; each new attribute read the take carries is listed for the operator.
@task_item n=15 part=needs
No new crate, port, env var or listener of ours: the CDP served listener commit stays out unless the take cannot stand without it.
@task_standing n=1
convention: every test the take brings or adds runs on an engine built with system fonts
home: typography
@task_item n=1 part=not_needed
The escher-session command, answer, exit status, stdout, stderr, MCP role and task rows: the take adds no command, answer or status of the session binary, and its smoke is a later entry.
@task_item n=2 part=not_needed
The shell provider's window decorations, maximize, minimize and drag, and the cursor icon crate and cursor resolution: no upstream commit of this take touches the window shell or cursors.
@task_item n=3 part=not_needed
The JS window object's size and pixel-ratio values and the script runtime's document fonts: the take changes no scripted surface.
@task_item n=4 part=not_needed
The web-spa surface and the WASM bundled font context: no WASM page or canvas host changes in this take.
@task_item n=5 part=not_needed
Brand identity and border radius: no recorded rows the take could touch.
@task_item n=6 part=not_needed
The actions line, coverage line, coverage check files and the actions decided by the verbs' own checks: the take adds nothing to exploration or coverage output.
