@task_prose part=chunk
Syncs upstream into the build branch as a real two-parent merge of upstream/main (re-measured at the take-up: 547d7514b, 212 commits ahead), never a rebase, squash or cherry-pick series; every resolution keeps both sides' intent.
Names each upstream commit left out, with its reason, as an undo in the merge's tree, and re-reads the earlier left-out table row by row against the new pin; a reason that no longer holds puts the commit back in.
Collects every boundary widening the merge carries, first the CDP listener and its crate, in one file for the founder, unanswered; states that position: sticky (blitz#887) is not in the take as a scroll-relative behaviour (sticky stays laid out as relative).
Re-reads the lock and the named merge-surface residuals, re-checks the sink allowlist over the merged delta, and re-measures every escher proof: the fast and doc CI legs, the stand checks in both layout modes, the incremental oracle, the a11y leg, the audit, the screenshot and cross-OS readings, and fork CI on the pushed commit.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace root manifest — the coupled dependency pins (Stylo, Dioxus, parley, skrifa, winit, svgtypes, Taffy's pinned rev, AnyRender) and the workspace members, escher crates and seven_guis among them
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lockfile — every new and moved package read for licence and advisories after the merge
@task_file n=3 path=packages/blitz-vibey-cdp-server mark=merge
what: upstream's Chrome DevTools Protocol server crate, a served listener on 127.0.0.1 via BLITZ_CDP_PORT — a boundary widening left to the founder
@task_file n=4 path=packages/parley_code_editor mark=merge
what: upstream's new code-editor crate added as a workspace member by the take
@task_file n=5 path=.github/workflows/ci.yml mark=merge
what: the fork's CI workflow — every action pinned to a 40-hex SHA, read-only contents permission, upstream's workflows guarded by the repository check
@task_file n=6 path=packages/stylo_taffy/src/convert.rs mark=re-reads
what: the style-to-layout position conversion, read to confirm Sticky still maps to taffy's Sticky and lays out as relative
@task_file n=7 path=packages/blitz-dom/src/layout/construct.rs mark=re-reads
what: layout tree construction, read to confirm a sticky box is read beside relative
@task_file n=8 path=packages/blitz-dom/src/node/node.rs mark=re-reads
what: the DOM node, read to confirm a sticky box is read beside relative
@task_file n=9 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests targets check — one test file and one target per behaviour
@task_file n=10 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg runner — the fast and doc legs re-run after the merge
@task_file n=11 path=deny.toml mark=re-reads
what: the dependency audit's licence and advisory policy — each standing ignore's reason re-read
@task_item n=1 part=needs
The merge is a real two-parent merge: its parents are the build branch's tip and the upstream commit taken; no rebase, squash or cherry-pick series, and no workaround bought only to keep upstream code untouched — where staying additive forces contorted logic, the upstream code is changed and the change says so.
@task_item n=2 part=needs
Each upstream commit left out is undone in the merge's tree and named with its reason, including any pin held back from upstream's coupled set; the earlier left-out table is re-read row by row against the new pin.
@task_item n=3 part=needs
Coupled dependency pins: the take carries upstream's pins as one coupled set or holds it; a moved layout, text or paint number is re-measured and recorded, never loosened; no dependency is bumped beyond upstream's own set; the workspace takes no git dependency save Taffy's pinned rev; MSRV stays 1.91 unless a taken pin requires more, then named.
@task_item n=4 part=needs
Boundary widenings carried by the merge — the CDP listener on 127.0.0.1 via BLITZ_CDP_PORT and its crate first — are collected in one evidence file for the founder, each checked against the escalate patterns, none answered here; no new crate, port, env var or listener of ours.
@task_item n=5 part=needs
The stand's stable element ids and the snapshot that carries them survive the merge unchanged in dioxus-native-dom (ids, snapshot, graph, reached record, buffered render).
@task_item n=6 part=needs
The scroll behaviours escher changed keep their behaviour through conflict resolution: the hit stops and scrolled-box bounds, the reversed scroll origins, the into-view scroll moving every holding scroller, and the visible region read through clipping boxes.
@task_item n=7 part=needs
The select model and its list keep their behaviour: the flight type select reads its two options at boot and open list rows read their own bounds.
@task_item n=8 part=needs
The accessibility state reads, the range and keyboard models (the CRUD list as one keyboard region among them) and inert keep their behaviour; stand names, roles and states come from attributes alone.
@task_item n=9 part=needs
The changed set's contract, the clock invariant, the stamped sub-resource request with the file: refusal (the load page alone having a network provider), and the buffered render keep escher's behaviour.
@task_item n=10 part=needs
The stand checks pass in both layout modes and the incremental/non-incremental oracle holds; the headless stand's pinned viewport, held instance reading as a fresh boot and lint-clean boot stay as they are.
@task_item n=11 part=needs
The escher-session driver's verbs, answers, refusals, exit codes and MCP server over local stdio keep their contracts through the merge; the served tasks still boot.
@task_item n=12 part=needs
The screenshot and cross-OS readings are re-measured; a moved reading is recorded, never loosened; the embolden witness still passes.
@task_item n=13 part=needs
ShellProvider window chrome controls (drag, maximise, minimise, close, decorations) keep their contract across the winit beta move the take carries.
@task_item n=14 part=needs
Position: sticky is in the take only as a paint-order change; the take brings no scroll-relative sticky offset, and the chunk builds no sticky behaviour of its own.
@task_item n=15 part=needs
The sink allowlist is re-checked over the merged delta for every new tracing or log call site, target, print and env read; the standing capture checks stay at 0 needles and no caller input is written back.
@task_item n=16 part=needs
Upstream's new and edited tests run under the fork's one-file-one-target-per-behaviour rule.
@task_item n=17 part=needs
CI workflow edits keep every action pinned to a 40-hex SHA and the read-only contents permission; upstream's workflow files stay guarded by the repository check; fork CI is green on the pushed commit with every job settled, the a11y leg and the dependency audit among them.
@task_item n=18 part=needs
The lock is read for every new and moved package (licences through the audit, advisories through the standing ignores, each reason re-read); the named merge-surface residuals are re-read, and one a taken move resolves (the Dioxus 0.8 move) is re-measured, never assumed.
@task_standing n=1
convention: every log line or env read the merge brings stays inside the sink allowlist and never writes caller input back
home: surface-cli
@task_item n=1 part=not_needed
The mobile-native surface: not measured, and no fact of this merge reaches mobile layout.
@task_item n=2 part=not_needed
The web-spa surface: not measured; the chunk ships no web navigation, header or footer.
@task_item n=3 part=not_needed
The decisions log: it holds no recorded intent.
@task_item n=4 part=not_needed
The cold-agent run pipe and the agent-run test contract (their usage, exits, record directories and Windows pass-throughs): the chunk runs no cold-agent or agent run and changes neither pipe.
@task_item n=5 part=not_needed
The stand's visual choices — accent colour, fixed document skeleton with main mount, order page controls built to cause, every stand control keyed — are escher's own stand app code the upstream take does not reach.
