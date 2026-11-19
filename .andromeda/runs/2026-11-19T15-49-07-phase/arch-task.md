@task_prose part=chunk
Syncs upstream Blitz into the build branch as one real two-parent merge of upstream/main (re-measured at 547d7514b, 212 commits, merge base 23b9b26a9) — never a rebase, squash or cherry-pick series — resolving every conflict so both sides' intent survives.
Names each upstream commit left out (its change undone in the merged tree) with its reason, re-reading the earlier left-out table row by row; a held dependency pin counts as such an undo.
Collects every boundary widening the merge carries — the Chrome DevTools Protocol listener crate first — in one file for the founder, unanswered; states whether position: sticky arrives (it does not: sticky stays laid out as relative).
Re-reads the lock for new and moved packages, the merge-surface residuals and the log sink allowlist; re-runs every escher proof (CI legs, stand checks in both layout modes, oracle, a11y leg, audit, embolden witness, screenshot and cross-OS readings) and fork CI on the pushed commit.
@task_file n=1 path=Cargo.toml mark=merge
what: the root virtual manifest — the workspace members, the shared workspace dependency table and its coupled pins (Stylo, Dioxus, parley, skrifa, winit, svgtypes, Taffy's git rev, AnyRender), rust-version
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lockfile, run locked by the cargo legs and read by the dependency and licence audit
@task_file n=3 path=packages/blitz-vibey-cdp-server mark=merge
what: upstream's new Chrome DevTools Protocol server crate, a workspace member serving a 127.0.0.1 listener on a port read from BLITZ_CDP_PORT
@task_file n=4 path=packages/parley_code_editor mark=merge
what: upstream's new code-editor crate over Parley, a new workspace member
@task_file n=5 path=.github/workflows/ci.yml mark=merge
what: the fork's CI workflow — SHA-pinned actions, read-only contents permission, upstream-only jobs guarded by github.repository
@task_file n=6 path=packages/stylo_taffy/src/convert.rs mark=re-reads
what: the Stylo-to-Taffy style conversion, read for how Stylo's Sticky position maps to Taffy's position
@task_file n=7 path=packages/blitz-dom/src/layout/construct.rs mark=re-reads
what: blitz-dom layout box construction, read for sticky handled beside relative position
@task_file n=8 path=packages/blitz-dom/src/node/node.rs mark=re-reads
what: the blitz-dom Node, read for sticky handled beside relative position
@task_file n=9 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the check pinning blitz-tests to one file and one test target per behaviour
@task_file n=10 path=.github/scripts/ci-leg.sh mark=re-reads
what: the one CI leg runner, its fast and doc legs re-run after the merge
@task_file n=11 path=deny.toml mark=re-reads
what: the cargo-deny dependency and licence audit, its standing advisory ignores and each ignore's reason re-read
@task_file n=12 path=.github/scripts/test_ci_workflows.py mark=re-reads
what: the CI workflow contract tests, among them the check that every action is pinned to a 40-hex commit SHA
@task_item n=1 part=needs
The upstream sync itself: a real two-parent merge whose parents are the build branch's tip and the upstream commit taken; the upstream sync merge-base rule and any rule on how upstream changes are taken (merge versus cherry-pick) bind its form.
@task_item n=2 part=needs
Each upstream commit left out is named with its reason: in a two-parent merge a left-out commit stays an ancestor, so leaving it out means undoing its change in the merged tree; the earlier left-out table (res-7qgqygtj) is re-read row by row, and a reason that no longer holds puts the commit back in.
@task_item n=3 part=needs
Coupled dependency pins: the merge brings Stylo 0.24 (selectors 0.43), Dioxus 0.8.0-alpha.3, parley 0.13, skrifa 0.49, winit exact beta.4 and svgtypes 0.16.2, adds anyrender_tiny_skia and drops arboard from the workspace table; the set is carried whole or held, holding a moved pin is an undo named with its reason, and every moved layout, text or paint number is re-measured and recorded, never loosened to pass. Rules tying one crate's version to another's (skrifa to parley, svgtypes to usvg, the HTML parser to Stylo's atoms, Stylo prefs by version names, winit's exact beta pin, the deferred Stylo and Dioxus bumps) are re-read against it.
@task_item n=4 part=needs
The Chrome DevTools Protocol listener the merge carries (a served 127.0.0.1 listener on a port named by an environment variable) widens the occupied-resources boundary and the security plan's surfaces, against the absence of any served network port or listener; whether the merge keeps the crate and the listener is the founder's fork, not answered by this chunk.
@task_item n=5 part=needs
Every boundary widening the merge carries is collected in one file in the chunk's evidence for the founder, each checked against the playbook's escalate patterns, none answered.
@task_item n=6 part=needs
No git dependency enters the workspace save Taffy's pinned rev; the chunk adds no crate, port, environment variable or listener of escher's own.
@task_item n=7 part=needs
Conflict resolutions keep both sides' intent: the changed set's contract, the document clock invariant, the select model and its open list, the hit stops and scrolled-box bounds, the reversed scroll origins and the scroll-into-view readings, the stable element ids and the snapshot, the range and keyboard models, inert, the accessibility state reads, the stamped sub-resource request and the file: refusal, the bridge's buffered render, and the escher crates and seven_guis members of the workspace.
@task_item n=8 part=needs
dioxus-native-dom (stable ids, snapshot, UI graph, reached record, buffered render) and every engine surface escher changed keep escher's behaviour through the merge, each resolution recorded.
@task_item n=9 part=needs
No workaround is bought only to keep upstream code untouched: where staying additive would force contorted logic, the upstream code is changed and the change says so, as a direct engine change flagged as such.
@task_item n=10 part=needs
Upstream behaviour the take brings is an engine behaviour change ratified per take: among it pseudo-elements ordered by box position in document-order comparison, @import rules with unparsable URLs refused, positioned descendants of static boxes painted with their positioned ancestor, and the rest of the 212 commits.
@task_item n=11 part=needs
position: sticky (blitz#887): the take brings no scroll-relative sticky offset — Stylo's Sticky still maps to Taffy's Sticky and is read beside relative, so a sticky box stays laid out as relative; P3 confirms against the commits touching scroll and paint.
@task_item n=12 part=needs
Upstream's new and edited test files run under the fork's blitz-tests rule: integration tests in blitz-tests, one file and one test target per behaviour, the manifest's one test table.
@task_item n=13 part=needs
Every escher proof passes after the merge: the fast and doc CI legs, the stand checks in both layout modes, the incremental and non-incremental layout oracle, the a11y leg, the dependency audit, the font-embolden witness, the screenshot and cross-OS stand readings (a moved reading re-measured and recorded, never loosened), and fork CI green on the pushed commit with every job settled.
@task_item n=14 part=needs
The lockfile after the merge is read for every new and moved package: licences through the audit, advisories through the standing ignores, each ignore's reason re-read; cargo legs stay locked.
@task_item n=15 part=needs
The merge-surface residuals res-7qgqygtj, res-xpkmxnvl, res-xchmbfjr, res-vuvpa25q, res-q4s3p5ga, res-kk4tzmi5, res-derughab and res-ipcah32k are each re-read against the merge; one a taken move resolves (the Dioxus 0.8 move for res-xpkmxnvl and res-xchmbfjr) is re-measured, never assumed.
@task_item n=16 part=needs
The telemetry sink allowlist is re-checked over the merged delta: every new tracing or log call site, target, print and environment read (BLITZ_CDP_PORT among them) is read against the telemetry scrub and target filter, and the standing capture checks stay at zero needles.
@task_item n=17 part=needs
CI workflow edits the merge brings keep every action pinned to a 40-hex commit SHA and the read-only contents permission; upstream's own workflows and CI resources stay guarded to the upstream repository.
@task_item n=18 part=needs
MSRV stays 1.91 unless a taken pin requires more, which is then named; new and removed dependencies are reviewed by hand and through the audit's licence check.
@task_item n=19 part=needs
Conflicts are measured on a fresh trial worktree under the build directory's tmp area with rerere disabled, so no recorded resolution is replayed.
@task_item n=20 part=needs
The last chunk's CI verdict (d9af170dc, in progress) is never folded as green: it is re-read at P3 and before this chunk's push.
@task_item n=21 part=needs
The Dioxus bridge's contracts upstream's Dioxus move reaches — element ids, reached record, registry dispatch, use_graph_state, buffered render — hold under the moved Dioxus version.
@task_standing n=1
convention: integration tests live in blitz-tests, one behaviour per file and one test target per file
home: conventions · design-philosophy
@task_standing n=2
convention: scenarios and stand checks run in both layout modes, which give identical layouts
home: conventions · design-philosophy
@task_standing n=3
convention: dependencies are declared once in the workspace; crates inherit package metadata and take the workspace licence; in-repo crates are depended on without default features
home: conventions · infrastructure-patterns
@task_standing n=4
convention: GitHub Actions are pinned to commit SHAs and CI runs with read-only permissions
home: stack-and-technologies · infrastructure-patterns
@task_standing n=5
convention: engine tracing is gated per call site and every escher binary prints only through the escher-telemetry sink
home: conventions · cross-cutting-patterns
@task_standing n=6
convention: code a resolution writes keeps the code conventions — justified expect and unwrap, annotated unsafe, spec URLs cited, known gaps marked TODO or FIXME, module and public item docs
home: conventions
@task_item n=1 part=not_needed
The driver's carriers — driver verbs, argument kinds, answer JSON, refusal causes and faults, the session socket's wire, the MCP server and mcp role, command-line exit codes: the merge adds no verb, carrier, answer field or refusal.
@task_item n=2 part=not_needed
Cold-agent and agent-run tooling — their events, exit codes, state directory, verdict fields and wrong-call bar: no cold-agent run is made by this chunk.
@task_item n=3 part=not_needed
Kept-check files, the recording directory and check replay: no check is recorded or its file shape changed.
@task_item n=4 part=not_needed
The lint verb, lint reason keys and layout-reason contracts: escher's lint and layout explanations gain no defect or cause here.
@task_item n=5 part=not_needed
WPT runner result kinds, statuses, diff JSON and diff-to-PR tooling: upstream's WPT conformance numbers are out of scope.
@task_item n=6 part=not_needed
Absences of modern CSS (anchor positioning, container queries, subgrid, masonry, 3D transforms) and building sticky positioning: heavy modern HTML is a later version's, and the chunk builds no layout feature.
@task_item n=7 part=not_needed
The capability statement's sections and the root README contracts: «README and records» is a later entry.
