@task_prose part=chunk
Syncs upstream blitz into the build branch as one real two-parent merge of upstream/main (re-measured at 547d7514b, 212 commits ahead), never a rebase, squash or cherry-pick series.
Each commit left out is named with its reason as an undo in the merge's tree, and the earlier left-out table is re-read row by row; holding a pin upstream moved counts as such an undo.
Every conflict is resolved keeping both sides' intent: escher's engine changes (changed set, select model and list, hit stops, stable ids and snapshot, range and keyboard models, inert, accessibility state reads, buffered render) survive the take.
Re-runs every escher proof after the merge (fast and doc legs, stand checks, a11y leg, audit, screenshot readings), re-reads the lock and the named residuals, re-checks the log sink allowlist, and says whether position: sticky is in the take.
Collects every boundary widening the merge brings, the CDP listener first, in one file for the founder and answers none of them.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace manifest and its coupled dependency pins (Stylo, Dioxus, parley, skrifa, winit, AnyRender, Taffy rev) and its workspace members, escher crates and seven_guis among them
@task_file n=2 path=Cargo.lock mark=merge
what: the dependency lock, read for every new and moved package through the dependency audit's licence check and advisories
@task_file n=3 path=deny.toml mark=re-reads
what: the dependency audit's standing advisory ignores, each ignore's reason re-read
@task_file n=4 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI legs run after the merge, the fast leg and the doc leg
@task_file n=5 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests rule check, one test file and one target per behaviour
@task_file n=6 path=packages/stylo_taffy/src/convert.rs mark=re-reads
what: the style-to-layout conversion of the position value, read to state that a sticky box stays laid out as relative
@task_item n=1 part=needs
The engine's changed set keeps what it marks and the harness settle that leaves it, across every conflict the merge resolves in the engine.
@task_item n=2 part=needs
The single-choice and multiple select models keep their engine overlay list, keys, events, rows, colours and expanded state through the merge.
@task_item n=3 part=needs
The keyboard model survives: Tab order and focus events, Enter and Space activation, range input keys and pointer, focusability recomputed, focus fixup and removed-focus steps.
@task_item n=4 part=needs
Inert subtrees stay excluded from focus, hit-testing and the accessibility tree, and hidden content stays out of the tree, whatever upstream changes in the paint, hit and tree code.
@task_item n=5 part=needs
The accessibility tree built through the document's own method keeps its roles, accessible names, ARIA state and property reads and the native checked and select readings, and refreshes on change.
@task_item n=6 part=needs
The snapshot, its stable node ids, the UI graph mirror, the actions answer and the driver refusals keep reading the accessibility tree as they do now.
@task_item n=7 part=needs
The Dioxus move to 0.8 alpha reaches the buffered Dioxus render, the reached-handler record, the listeners read from the vdom walk and the accessibility feature taken from Dioxus only by name; each must keep escher's behaviour, and the resolution is recorded.
@task_item n=8 part=needs
The accessibility feature stays default-on in the engine and required by the session library, with the AccessKit tree and accesskit_xplat adapter, after the moved pins; any change in the AccessKit set the merge brings is a moved package the lock read covers.
@task_item n=9 part=needs
The parley, skrifa and Stylo moves can reshape text and computed colours: the stand contrast readings and screenshot readings are re-measured and recorded, never loosened to pass.
@task_item n=10 part=needs
The a11y CI leg, the stand checks in both layout modes repeated three rounds and the accessibility checks that gate CI all pass on the merged commit, with fork CI green and every job settled.
@task_item n=11 part=needs
The upstream paint-order change for positioned descendants of static boxes and the pseudo-element document-order change are takes that reach hit stops, scrolled-box bounds and tree order; each is read against escher's hit-testing and tree readings.
@task_item n=12 part=needs
position: sticky is reported as not in the take: a sticky box stays laid out as relative, and no sticky behaviour is built here.
@task_item n=13 part=needs
The log sink's target allowlist is re-checked against every new tracing or log call site, target, print and env read the merge brings, BLITZ_CDP_PORT among them; blitz and accesskit_xplat targets still print only their admitted fields and the capture checks stay at 0 needles.
@task_item n=14 part=needs
The CDP listener and its crate are a boundary widening: collected for the founder with every other widening in one evidence file, checked against the escalate patterns, never answered; the chunk adds no crate, port, env var or listener of its own.
@task_item n=15 part=needs
No workaround is bought only to keep upstream code untouched: where staying additive would force contorted logic, upstream code is changed and the change says so.
@task_item n=16 part=needs
CI workflow files taken keep every action pinned to a 40-hex SHA and the read-only contents permission; upstream's workflow files stay guarded by the repository check.
@task_standing n=1
convention: accessibility assertions are Rust tests over the AccessKit tree, the expected roles and names stated once in shared tables
home: 3-a11y-assertion-harness-contract · screen-reader-support
@task_standing n=2
convention: stand checks run in the a11y test leg of the slow tier and are repeated three rounds
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
The sections with no recorded intent (SLO invariants and budgets, anti-patterns, decisions log) hold no rows this sync can bind.
@task_item n=2 part=not_needed
The accessibility reach statement edited by hand and the tier stated for it: the chunk states no new reach and edits no capability statement.
@task_item n=3 part=not_needed
Building position: sticky behaviour ourselves: out of scope, heavy modern HTML belongs to 0.5.0.
@task_item n=4 part=not_needed
Sending escher's changes upstream and upstream's WPT conformance numbers: neither is part of this chunk.
@task_item n=5 part=not_needed
The Epoch 1 evolve diagnosis and the README and records entry: not run in this chunk.
