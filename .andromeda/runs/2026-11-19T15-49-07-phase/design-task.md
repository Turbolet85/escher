@task_prose part=chunk
Syncs upstream/main (re-measured at the take-up to 547d7514b, 212 commits ahead) into the build branch as one real two-parent merge, never a rebase, squash or cherry-pick series.
Resolves every conflict keeping both sides' intent, and names each upstream commit whose change the merge's tree undoes with its reason, re-reading the earlier left-out table row by row.
Carries upstream's coupled dependency pins (Stylo 0.24, Dioxus 0.8 alpha, parley 0.13, skrifa 0.49, winit beta.4) as one set or holds it as a named undo, and collects every boundary widening the merge brings, the CDP listener first, in one file for the founder.
Re-reads the lock, the advisories and licences, the merge-surface residuals and the sink allowlist; says whether position: sticky (blitz#887) is in the take.
Re-runs every escher proof after the merge: the fast and doc legs, stand checks in both layout modes, the incremental oracle, the a11y leg, the audit, the screenshot and cross-OS readings re-measured never loosened, and fork CI green.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest — its member list (the escher crates and seven_guis members beside upstream's new crates) and the coupled dependency pins of Stylo, Dioxus, parley, skrifa, winit, svgtypes and Taffy's git rev
@task_file n=2 path=Cargo.lock mark=merge
what: the dependency lock — every new and moved package read for licence and advisory after the merge
@task_file n=3 path=packages/blitz-vibey-cdp-server mark=merge
what: upstream's Chrome DevTools Protocol server crate — a served listener on 127.0.0.1 through an env-read port, a boundary widening for the founder
@task_file n=4 path=packages/parley_code_editor mark=merge
what: upstream's new code-editor text crate added as a workspace member
@task_file n=5 path=packages/dioxus-native-dom mark=conflict
what: the dioxus-native DOM integration — stable ids, the snapshot, the UI graph, the reached record and the buffered render replayed under one borrow
@task_file n=6 path=packages/blitz-dom mark=conflict
what: the engine document — the changed set's contract, the select model and its list overlay, hit stops and scrolled-box bounds, reversed scroll origins and into-view readings, range and keyboard models, inert, the accessibility state reads
@task_file n=7 path=packages/blitz-net mark=conflict
what: the network provider — the stamped sub-resource request and the file: refusal
@task_file n=8 path=packages/blitz-paint mark=conflict
what: the painter — paint order of positioned boxes, the select list painted after the root, hit-testing stopping at a paint clip
@task_file n=9 path=packages/blitz-shell mark=conflict
what: the desktop shell — the virtual clock driving animation and the shell provider
@task_file n=10 path=packages/dioxus-native mark=conflict
what: the dioxus-native configuration and launch surface moved by the Dioxus 0.8 alpha pin
@task_file n=11 path=packages/stylo_taffy/src/convert.rs mark=re-reads
what: the Stylo-to-Taffy style conversion — the reading that Sticky maps to taffy Sticky and is laid out as relative
@task_file n=12 path=packages/blitz-dom/src/layout/construct.rs mark=re-reads
what: the layout tree construction — the reading of sticky beside relative positioning
@task_file n=13 path=.github/workflows/ci.yml mark=conflict
what: the fork CI workflow — every action pinned to a 40-hex SHA, read-only contents permission, upstream workflows guarded by the repository check
@task_file n=14 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests rule check — one file and one target per behaviour
@task_file n=15 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg runner — the fast and doc legs
@task_file n=16 path=deny.toml mark=re-reads
what: the dependency audit configuration — licence allowances and the standing advisory ignores, each reason re-read
@task_item n=1 part=needs
The merge itself: a two-parent merge commit whose parents are the build branch tip and the upstream commit taken; each upstream change the merged tree undoes is named with its reason, and the earlier left-out table is re-read so a reason no longer holding puts the commit back in.
@task_item n=2 part=needs
What the take brings: upstream's Stylo 0.22 to 0.24 (selectors 0.43) move changes cascade, selector matching and style resolution behaviour in the engine; any decision about recascade, presentational hints, root-relative units or font-size resolution is re-proved after the move.
@task_item n=3 part=needs
What the take brings: Dioxus 0.7.3 to 0.8.0-alpha.3 changes the dioxus-native configuration, the mounted-data calls during a render and the virtual-DOM integration that ids, snapshot, graph and buffered render rest on; re-measured against the residuals it may resolve.
@task_item n=4 part=needs
What the take brings: parley 0.13 and skrifa 0.49 move text shaping and font loading — font fallback, the default font, the bundled WASM font context and system-font test builds — so moved text numbers in layout and screenshots are re-measured and recorded, never loosened.
@task_item n=5 part=needs
What the take brings: winit beta.4 moves the desktop shell's window, cursor, decorations, maximize, minimize and window-drag surfaces and the logical viewport scale; the shell provider's behaviour is re-read.
@task_item n=6 part=needs
Coupled dependency pins: the take carries upstream's pins as one coupled set or holds the set; holding a pin upstream moved is itself an undo named with its reason; no dependency is bumped beyond upstream's own set; the workspace takes no git dependency save Taffy's pinned rev; MSRV stays 1.91 unless a taken pin requires more, then named.
@task_item n=7 part=needs
Conflict resolution keeps escher's behaviour on dioxus-native-dom: stable ids, the snapshot, the UI graph, the reached record and the render buffered then replayed under one borrow.
@task_item n=8 part=needs
Conflict resolution keeps the engine's select model: the select list as an engine overlay painted after the root and hit first, its colours, its placement inside the viewport, its open gestures, list events, wheel and key scroll, and its snapshot and accessibility readings.
@task_item n=9 part=needs
Conflict resolution keeps hit-testing and scroll: hit stops at a paint clip, stacking-context grouping of paint and hit, scrolled-box bounds, reversed scroll origins, instant into-view and driver scroll in nested boxes, overlay scrollbar timing and smooth-scroll wheel cancel.
@task_item n=10 part=needs
Conflict resolution keeps the virtual clock invariant: animation driven by the one virtual clock, load wait moving no clock, settle not waiting on animation, and the driver verbs advance, click, press and type.
@task_item n=11 part=needs
Conflict resolution keeps the changed set's contract, the range and keyboard models, inert, the accessibility state reads, the stamped sub-resource request and the file: refusal, and the escher crates and seven_guis workspace members.
@task_item n=12 part=needs
Upstream's paint-order change for positioned descendants of static boxes (#1445) reaches the auto z-index paint level and stacking-context paint and hit grouping; the resolution keeps both intents and the screenshot readings are re-measured.
@task_item n=13 part=needs
position: sticky (blitz#887): the take brings no scroll-relative sticky offset; a sticky box stays laid out as relative; the chunk states this and builds no sticky behaviour of its own.
@task_item n=14 part=needs
No workaround bought only to keep upstream code untouched: where staying additive would force contorted logic, upstream code is changed and the change says so.
@task_item n=15 part=needs
Boundary widenings: every widening the merge carries, the CDP listener on a localhost port read from an env var first, is collected in one evidence file for the founder, checked against the escalate patterns, and answered by no one in this chunk; the chunk adds no crate, port, env var or listener of its own.
@task_item n=16 part=needs
Sink allowlist: every new tracing or log call site, target, print and env read in the merged delta is read, and the standing capture checks stay at zero needles.
@task_item n=17 part=needs
Lock, audit and residuals: every new and moved package passes the licence check and the advisory ignores, each ignore's reason re-read; the merge-surface residuals are each re-read and a residual the take resolves is re-measured, never assumed.
@task_item n=18 part=needs
CI workflow edits arriving by the merge keep every action pinned to a 40-hex SHA, ci.yml's read-only contents permission, and upstream's workflows guarded by the repository check.
@task_item n=19 part=needs
Upstream's new and edited test files run under the blitz-tests rule: one file and one target per behaviour, pinned by the targets check.
@task_item n=20 part=needs
Every escher proof re-passes after the merge: fast and doc legs, stand checks in both layout modes, the incremental and non-incremental oracle, the a11y leg, the dependency audit, the embolden witness, the screenshot and cross-OS readings re-measured and recorded, and fork CI green with every job settled, the prior chunk commit's verdict re-read.
@task_item n=21 part=needs
The escher-session surface and the driver's screenshot, snapshot, actions, coverage and graph answers, exit statuses and refusals keep their contracts through the merge, held by the existing checks.
@task_standing n=1
convention: tests and the test engine are built with system fonts, and a screenshot never paints with text skipped
home: typography
@task_standing n=2
convention: driver output stays agent-read, with no colour and no terminal test, exit statuses read from the schema's table
home: surface
@task_item n=1 part=not_needed
The brand identity area: no recorded intent, and a dependency sync reaches no brand.
@task_item n=2 part=not_needed
The border radius area: no radius scale exists and the merge sets none.
@task_item n=3 part=not_needed
The anti-patterns, self-validation protocol and design decisions log areas: none carries recorded intent for a sync to act on.
