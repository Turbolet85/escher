@task_prose part=chunk
Takes upstream blitz at the pinned sha 563d3db73 into this fork, in the shape the founder chooses at P4 (cherry-pick on the registry pins, that plus upstream's registry moves, a whole merge, or nothing), resolving each conflict so both sides' intent stands.
Folds the two external CI reds: the clippy needless_bool lint under the new stable toolchain, and the dependency-audit advisory on the SQLite crate under the browser's persistence.
Adds two headless frame tests, each in both layout modes: blend mode and isolation paint, and an inline box's skipped own decorations; pins every workflow action to a commit SHA and widens the pinning check to every workflow.
Re-reads and re-asserts every escher proof after the take: the fast, doc, a11y and audit legs, the stand checks in both layout modes, the incremental oracle, the embolden witness, fork CI green; re-reads the per-OS test tallies and re-checks the telemetry sink allowlist against the taken delta.
Re-reads the carried left-out commits and merge surfaces against the new pin, and lists every boundary widening the take brings for the operator at P4.
@task_file n=1 path=packages/blitz-dom/src/document.rs mark=edits
what: the engine's base document, holding the anonymous block box deallocation fix that is a merge surface — edits: a needless_bool site flagged by clippy
@task_file n=2 path=packages/blitz-dom/src/layout/damage.rs mark=edits
what: the layout damage and changed-set write sites of the engine — edits: two needless_bool sites flagged by clippy
@task_file n=3 path=packages/dioxus-native-dom/src/dioxus_document.rs
what: the Dioxus native document holding the buffered render: its two render sites, the buffered mutations field and the ported render-time task test — a merge surface re-read at the take
@task_file n=4 path=packages/dioxus-native-dom/src/mutation_buffer.rs
what: escher's own mutation buffer for the buffered render, which upstream's Dioxus 0.8 version replaces whole if that move is taken
@task_file n=5 path=packages/dioxus-native-dom/src/lib.rs
what: the dioxus-native-dom crate root, its one module line for the mutation buffer
@task_file n=6 path=packages/blitz-paint/src/layers.rs
what: the paint layers' blend mode and isolation conversion, whose unit tests are its only cover today
@task_file n=7 path=packages/blitz-paint/src/render.rs
what: the element render path, whose skip of an inline box's own decorations reads on no frame today
@task_file n=8 path=.github/workflows/wpt.yml mark=edits
what: the upstream-only WPT workflow — edits: every action reference pinned to a 40-hex commit SHA with its ref as a trailing comment
@task_file n=9 path=.github/workflows/publish-browser.yml mark=edits
what: the browser publishing workflow — edits: every action reference pinned to a 40-hex commit SHA with its ref as a trailing comment
@task_file n=10 path=.github/workflows/wpt-post-results.yml mark=edits
what: the WPT results posting workflow — edits: every action reference pinned to a 40-hex commit SHA with its ref as a trailing comment
@task_file n=11 path=.github/scripts/test_ci_workflows.py mark=edits
what: the CI workflow guard checks — edits: the action pinning check, widened from the main CI workflow to every workflow
@task_file n=12 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests target list check, one file and one target per behaviour, pinned list
@task_file n=13 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI legs re-run after the take: fast, doc, a11y and dependency audit
@task_file n=14 path=packages/escher-telemetry/src/format.rs mark=re-reads
what: the standing telemetry sink capture checks, held at zero needles
@task_item n=1 part=needs
The upstream sync itself: the coupled dependency pins (Taffy, Stylo, Parley, AnyRender/vello, winit, Dioxus) carried as one set at the chosen sha or not at all, and the registry-only, no-git-dependency rule.
@task_item n=2 part=needs
Moved layout, style, text or paint numbers after a pin move: a stand check's exact bounds, a snapshot's byte size, a lint finding or a screenshot's pixels re-measured and recorded, never loosened.
@task_item n=3 part=needs
The changed set and its write sites: what marks an element changed and the never-list (creation, hover, active, scroll, resize, load, animation, layout), with the qualifier that a scroll closing an open select list marks its select.
@task_item n=4 part=needs
The virtual clock invariant: animation, settle, advance and load wait all driven by the one virtual clock, kept through any conflict.
@task_item n=5 part=needs
The select model and its engine-overlay list: opening gestures, list events, list placement, hit-first, painting after the root, colours, and the snapshot and accessibility reads of a select.
@task_item n=6 part=needs
Hit-testing stops at a paint clip and the scrolled-box bounds, including stacking-context grouping of paint and hit, which a conflict in hit-testing must keep.
@task_item n=7 part=needs
Stable element ids, the snapshot, its text, parts and diff, the UI graph line and the reached record in dioxus-native-dom, decided by the id, snapshot, graph and handler-coverage stand checks.
@task_item n=8 part=needs
The buffered render replayed under one borrow, and MountedData calls during a render (document state, set_focus_to), as a merge surface of the take and the Dioxus 0.8 move.
@task_item n=9 part=needs
The range and keyboard models, keyboard activation, inert, and the accessibility state reads, re-proven by the a11y leg against the WCAG 2.1.1, 1.4.3 and 2.4.3 target.
@task_item n=10 part=needs
Paint features the take may bring (text-shadow, CSS masks, AccessKit refresh, default-font fallback, empty alt as decorative) and how they meet escher's engine-colour and font decisions.
@task_item n=11 part=needs
Font resolution: the default generic family laid out as sans-serif, tests and test engine built with system fonts, and the screenshot never painting with text skipped, as a default-font fallback arrives.
@task_item n=12 part=needs
Blend mode, isolation and inline-box decoration paint, covered by the two new headless frame tests in both layout modes; the screenshot verb as the frame read.
@task_item n=13 part=needs
Boundary widenings the take can bring — a DevTools/CDP socket or port, lossy page decoding, new attribute reads, new env reads, any new script API — each listed for the operator at P4, never settled silently.
@task_item n=14 part=needs
The dependency audit: the SQLite advisory on the browser persistence crate, settled either by a semver-compatible update or an ignore by ID with a written reason, every new crate reviewed and licence-checked, MSRV held at 1.91.
@task_item n=15 part=needs
Every GitHub Actions reference pinned to a commit SHA across all workflows, and the guard that checks it reading every workflow.
@task_item n=16 part=needs
The telemetry sink allowlist: every new tracing or log call site, target, print or env read in the taken delta, with the standing capture checks at zero needles.
@task_item n=17 part=needs
Additivity: any edit of ours to upstream-owned code is named, minimal and recorded as non-additive; no new crate, port, env var or listener of ours.
@task_item n=18 part=needs
Upstream's new and edited test files run under the blitz-tests rule: one file and one target per behaviour, the pinned target list.
@task_item n=19 part=needs
The escher-session driver verbs and their outputs (snapshot, screenshot, actions, coverage, graph lines, exit statuses) re-proven unchanged by the stand checks after the take.
@task_standing n=1
convention: each new headless frame test runs in both layout modes and reads its frame through the screenshot verb without skipping text
home: surface
@task_standing n=2
convention: each new test is built with system fonts, not bundled fonts
home: typography
@task_standing n=3
convention: each new test that drives time moves only the one virtual clock and never waits on animation
home: motion · motion-calibrated-to-expression-level-unrecorded
@task_item n=1 part=not_needed
Brand identity and border radius: the take adds no brand marks and no radius scale.
@task_item n=2 part=not_needed
The WPT runner's own output colours and test-name hyperlink: upstream's WPT conformance numbers and runner output are out of scope.
@task_item n=3 part=not_needed
The shell provider's window controls (maximize, minimize, decorations toggle, window drag) and the window compositing alpha mode: no desktop shell change is made.
@task_item n=4 part=not_needed
Cursor icons, cursor resolution order and the favicon read: nothing in the take or its folded work reaches them.
@task_item n=5 part=not_needed
The escher-session task fixtures (counter, timer, crud, cells, flight-booker, load): no task is added or changed.
@task_item n=6 part=not_needed
The JS window object's size and pixel-ratio values and the logical viewport from scale: the chunk adopts no new upstream API in escher's own code.
