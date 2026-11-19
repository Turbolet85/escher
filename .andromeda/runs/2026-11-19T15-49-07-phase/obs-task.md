@task_prose part=chunk
Merges upstream/main (re-measured at 547d7514b, 212 commits ahead, merge base 23b9b26a9) into build/escher-0.4.0 as one real two-parent merge — never a rebase, squash or cherry-pick series — resolving every conflict so both sides' intent survives, and names each upstream change it undoes with its reason.
Re-reads the earlier left-out table (res-7qgqygtj) row by row against the new pin, and the merge-surface residuals the capability names, re-measuring any a taken move resolves.
Re-reads the merged lock for every new and moved package through the audit and the advisory ignores, and re-checks the sink allowlist against every new tracing/log call site, target, print and env read in the merged delta.
Collects every boundary widening the merge carries — the Chrome DevTools Protocol listener crate first — in one file for the founder, answering none, and states whether position: sticky (blitz#887) is in the take.
Re-runs every escher proof after the merge: the fast and doc CI legs, the stand checks, the incremental oracle, the a11y leg, the dependency audit, the screenshot and cross-OS readings, and fork CI green on the pushed commit.
@task_file n=1 path=Cargo.toml mark=merge
what: the workspace root manifest — workspace members (escher crates, seven_guis, upstream's new CDP server and code-editor crates) and the coupled dependency pin set (Stylo, Dioxus, parley, skrifa, winit, svgtypes, Taffy's git rev)
@task_file n=2 path=Cargo.lock mark=merge
what: the workspace lockfile — every resolved package, read after the merge for new and moved packages, their licences and advisories
@task_file n=3 path=deny.toml mark=re-reads
what: the dependency audit's licence and advisory check, with its standing advisory ignores and each ignore's reason
@task_file n=4 path=.github/scripts/ci-leg.sh mark=re-reads
what: the CI leg runner — the fast and doc legs every escher proof is re-run through
@task_file n=5 path=.github/scripts/test_blitz_tests_targets.py mark=re-reads
what: the blitz-tests rule check — one test file and one target per behaviour, applied to upstream's new and edited test files
@task_file n=6 path=.github/workflows/ci.yml mark=merge
what: fork CI's workflow — its jobs, SHA-pinned actions and read-only contents permission, kept under any CI workflow edit the merge carries
@task_file n=7 path=packages/stylo_taffy/src/convert.rs mark=re-reads
what: the Stylo-to-Taffy style conversion — the position mapping read for whether sticky positioning arrives in the take
@task_item n=1 part=needs
The merge's new tracing and log call sites, targets, prints and env reads (BLITZ_CDP_PORT among them) from upstream's delta, each judged against the sink target allowlist; the standing capture checks stay at 0 needles.
@task_item n=2 part=needs
Engine and upstream crate log sites the merge brings or edits must write fixed messages and print only safe fields under engine targets; escher targets keep redacting the content-named set.
@task_item n=3 part=needs
Engine tracing stays compiled in only behind the tracing cargo feature after the merge, while escher crates (escher-telemetry, escher-driver) keep tracing ungated; a merged manifest or feature table must not change which side is gated.
@task_item n=4 part=needs
escher's telemetry sink — installed first in each native main, its RUST_LOG configuration, output stream, log-line format, closed span lines and field scrub — keeps its behaviour through every conflict the merge resolves in the native binaries and their manifests.
@task_item n=5 part=needs
The driver's command and tool_call spans and their closed fields keep their shape: the merge's moves in dioxus-native-dom (ids, snapshot, graph, reached record, buffered render) and the engine surfaces escher changed must not alter what those spans carry.
@task_item n=6 part=needs
Upstream's Chrome DevTools Protocol server crate and its served 127.0.0.1 listener read from BLITZ_CDP_PORT: a new listener, port and env var arriving by the merge — a boundary widening collected for the founder, unanswered, and no new crate, port, env var or listener of escher's own.
@task_item n=7 part=needs
No telemetry backend, OTel export, metrics exporter, metric emission or error-reporting service may arrive with the merge's new and moved dependencies; any that does is a boundary widening named for the founder.
@task_item n=8 part=needs
The script diagnostics drain and the script error surface (window error event fields, onerror arguments, error source labels) as upstream's engine changes reach them: a resolution keeps escher's drain and error retention.
@task_item n=9 part=needs
The file: refusal from non-file documents and the failed stylesheet leaving the pending critical set, kept through conflicts in the stamped sub-resource request path, now beside upstream's refusal of @import rules whose URL does not parse.
@task_item n=10 part=needs
The select model and its open list, the snapshot and diff, and the state registry keep their no-logging rules (chosen set, list row labels, snapshot values never logged) through every resolution in those engine surfaces.
@task_item n=11 part=needs
Fork CI's workflows after the merge: actions pinned to 40-hex SHAs, the read-only contents permission, upstream's workflow files guarded by the repository check so WPT results stay published from upstream only, and the fork's two uploaded artifacts kept.
@task_item n=12 part=needs
The CI verdict of d9af170dc, unread and in progress at the take-up, re-read before this chunk's push and never folded as green; fork CI must settle green on the pushed merge commit, every job.
@task_item n=13 part=needs
Empty-stderr and capture checks re-run after the merge keep unsetting RUST_LOG and keep failing by kind and count, so upstream's new log sites do not leak into them.
@task_item n=14 part=needs
A moved screenshot, layout, text or paint reading from the coupled pin moves (Stylo, parley, skrifa, Dioxus) is re-measured and recorded, never loosened; the proof that reads it is the measure, not a widened tolerance.
@task_standing n=1
convention: every new or merged engine log line writes a fixed message and safe fields only, under a target the sink allowlist names or judges
home: pii-scrubbing-compliance · 8-pii-scrubbing-compliance
@task_standing n=2
convention: every new log target or call site is checked against the sink target allowlist and the capture's warn-and-above count
home: log-coverage · 6-log-coverage
@task_standing n=3
convention: every new test that asserts empty stderr unsets RUST_LOG and installs no subscriber of its own
home: 3-observability-harness-contract
@task_standing n=4
convention: every CI job added or edited uploads its per-leg log on failure and publishes nothing local
home: ci-integration · 9-ci-integration
@task_item n=1 part=not_needed
The cold-agent pipe's verdict fields, reasons, counted verbs and events: the chunk runs no cold-agent pass and changes nothing in that pipe.
@task_item n=2 part=not_needed
The agent-run harness events and its raw log rules: no agent run is part of this sync.
@task_item n=3 part=not_needed
The build-directory sweep's stdout line: the sweep belongs to the previous chunk and this merge does not touch it.
@task_item n=4 part=not_needed
Session recording files, recording bounds and the private session-state and recording directories: the session's recording surface is escher's own and no upstream change reaches it.
@task_item n=5 part=not_needed
escher-session's exit codes, coverage role, host wire version and error-object remedy: the escher-session binary's own surfaces are not edited by this sync.
