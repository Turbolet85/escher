@task_prose part=chunk
Syncs upstream/main (re-measured at the take-up: 547d7514b, 212 commits ahead) into the build branch as one real two-parent merge, never a rebase, squash or cherry-pick series, each conflict resolved keeping both sides' intent.
Names every upstream commit left out, as an undo in the merge's tree with its reason, and re-reads the earlier left-out table row by row against the new upstream tip.
Collects every boundary widening the merge carries, the CDP listener crate first, in one file for the founder, answering none of them.
Re-reads the lock, the advisory ignores and the named merge-surface residuals, re-checks the log sink allowlist over the merged delta, and states whether position: sticky arrives by the take.
Re-runs every escher proof after the merge: the fast and doc legs, stand checks in both layout modes, the incremental oracle, the a11y leg, the dependency audit, the embolden witness, screenshot and cross-OS readings, and fork CI green.
@task_file n=1 path=Cargo.toml mark=merge
what: the root workspace manifest — the coupled dependency pins (Stylo, selectors, Dioxus, parley, skrifa, winit, svgtypes, Taffy's git rev), the workspace member list with the escher crates and seven_guis members, and the MSRV
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lockfile — every new and moved package the merge brings, read through the dependency audit's licence and advisory checks
@task_file n=3 path=.github/workflows/ci.yml mark=merge
what: the fork's CI workflow — the fast/slow legs, actions pinned to commit SHAs, the read-only contents permission, and the upstream-only publishing guards
@task_file n=4 path=deny.toml mark=re-reads
what: the dependency audit's advisory and licence policy — each standing advisory ignore and its reason re-read against the merged lock
@task_file n=5 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the check pinning one blitz-tests file and one test target per behaviour, re-run over upstream's new and edited test files
@task_file n=6 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg script — the fast and doc legs, the a11y leg, the dependency audit and the stand-repeat leg re-run after the merge
@task_file n=7 path=packages/stylo_taffy/src/convert.rs mark=re-reads
what: the Stylo-to-Taffy style conversion — its mapping of sticky positioning, read back on both sides to say whether the take brings sticky behaviour
@task_item n=1 part=needs
The upstream sync itself: a two-parent merge whose parents are the build branch tip and the upstream commit taken; a left-out commit stays an ancestor and its change is undone in the merge's tree, each undo named with its reason.
@task_item n=2 part=needs
Coupled dependency pins: the merge carries upstream's pin set (Stylo 0.24, Dioxus 0.8 alpha, parley 0.13, skrifa 0.49, winit beta.4, svgtypes) as one coupled set or holds it, holding being an undo named with its reason; a moved layout, text or paint number is re-measured and recorded, never loosened.
@task_item n=3 part=needs
The workspace takes no git dependency save Taffy's pinned rev; no bump beyond upstream's own set; new and removed dependencies reviewed by hand and through the dependency audit's licence and advisory checks.
@task_item n=4 part=needs
MSRV stays 1.91, verified by build, unless a taken pin requires more, then named.
@task_item n=5 part=needs
The CDP listener crate and its served loopback listener reached by the take: a boundary widening collected in the founder's fork file, needing a security-plan amendment before it lands; no new crate, port, env var or listener of ours.
@task_item n=6 part=needs
Sink allowlist and telemetry targets re-checked over the merged delta: every new tracing or log call site, target, print and env read (BLITZ_CDP_PORT among them); the standing sink capture checks stay at zero needles and third-party targets stay dropped.
@task_item n=7 part=needs
Conflict resolutions keep escher's behaviour on the engine surfaces escher changed: the changed set's contract, the one virtual clock invariant, the select model and its painted list, hit stops and scrolled-box bounds, reversed scroll origins and into-view readings, range and keyboard models, inert, and the accessibility state reads.
@task_item n=8 part=needs
dioxus-native-dom under the Dioxus move keeps its stable ids, snapshot, graph, reached record and buffered render; the buffered-render bridge decision and the kept and left-out upstream bridge tests are re-read against the Dioxus 0.8 move.
@task_item n=9 part=needs
blitz-dom sub-resource requests stay built through the one stamping constructor, and file: URLs stay loadable only from a file: document, across the merged network and loading code.
@task_item n=10 part=needs
Upstream's new and edited test files follow the fork's blitz-tests rule: one file and one test target per behaviour, the blitz-tests manifest's difference from upstream kept and pinned by its check.
@task_item n=11 part=needs
CI workflow edits arriving by the merge keep every action pinned to a 40-hex SHA, the read-only contents permission and no secrets; upstream's publishing and WPT workflows stay guarded to the upstream repository.
@task_item n=12 part=needs
Every escher proof re-run after the merge: the fast and doc legs, clippy over the workspace and upstream blitz-dom, the a11y leg, the dependency audit, the embolden local witness, and fork CI green on the pushed commit with every job settled.
@task_item n=13 part=needs
Stand checks run in both layout modes and repeated three rounds with a zero-failure budget, and the incremental/non-incremental layout differential oracle, both still green after the merge.
@task_item n=14 part=needs
Screenshot and cross-OS readings: a reading the merge moves (paint order, text shaping, font metrics) is re-measured and recorded, never loosened; pixel tests stay on the CPU buffer with the bundled font.
@task_item n=15 part=needs
position: sticky: the take brings no scroll-relative sticky offset (one paint-order commit for positioned descendants touches it), so a sticky box stays laid out as relative; the chunk states this and builds no sticky behaviour.
@task_item n=16 part=needs
Merge-surface residuals re-read against the merge, and a residual a taken move resolves (the Dioxus 0.8 move) re-measured, never assumed resolved.
@task_item n=17 part=needs
Conflicts measured on a fresh trial worktree under the build directory with rerere off; where staying additive would force contorted logic the upstream code is changed and the change says so.
@task_item n=18 part=needs
The last wrap's CI verdict, unread at take-up, is re-read before this chunk's push and never folded as green; per-package green is not read as the workspace green.
@task_standing n=1
convention: each upstream regression test taken documents and is named for its bug, sits in its own file and target, and asserts in both layout modes where it asserts layout
home: 2-test-strategy · test-strategy
@task_standing n=2
convention: each Rust test runs on the built-in harness, unit tests in inline cfg(test) modules of their source file, test data stated in code with nothing seeded
home: 3-test-harness-contract
@task_standing n=3
convention: each cargo leg runs locked through the leg script, its package installs bounded and retried, its logs uploaded on failure
home: 9-ci-integration · ci-integration
@task_item n=1 part=not_needed
The cold-agent live run, its verdict, prompt, isolation and wrong-call bar: fired once off CI, and the merge reaches none of it.
@task_item n=2 part=not_needed
Mutation testing, the mutation baseline and the cognitive-complexity ceiling: dev-host-only readings this chunk does not take.
@task_item n=3 part=not_needed
The agent-run five-verb contract, its selections, events and exit codes: the harness metadata script is not changed by the sync.
@task_item n=4 part=not_needed
The absence of windowed end-to-end tests: the merge adds no automated test that launches a window.
@task_item n=5 part=not_needed
The Epoch 1 evolve diagnosis and building sticky positioning ourselves: both left out of this chunk by the operator and the scope.
