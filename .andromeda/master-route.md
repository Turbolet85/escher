# Master Route — escher

<!--
Cross-version immutable index. APPEND-ONLY via promotion in /andromeda-phase — route never adds records.
One record per promoted chunk, grouped under its version:
  {marker} · {status: pending|gated|complete} · {super-laconic description} · → {link to chunk folder}
marker = {date}-{slug} (e.g. 2026-06-04-otlp-http-ingest), minted at promotion.
-->

## escher-0.1.0
2026-10-05-as-built-baseline · complete · workspace build + blitz-tests green on this host, wall-clock recorded · → escher-0.1.0/chunks/2026-10-05-as-built-baseline/
2026-10-05-fork-ci-reached · complete · Fork CI on the build branch — cached, fast/slow split, failure artifacts, signing jobs excluded · → escher-0.1.0/chunks/2026-10-05-fork-ci-reached/
2026-10-05-ci-gate-legs · complete · CI gate legs — dependency audit, SHA-pinned actions, least-privilege tokens, coverage report, a11y leg, real rustdoc gate · → escher-0.1.0/chunks/2026-10-05-ci-gate-legs/
2026-10-06-telemetry-bootstrap · complete · Telemetry bootstrap — escher-telemetry: stderr-only tracing subscriber, service identity, chaining panic logging, allowlist scrub, adopted by the stand; opt-in OTel export deferred (CARRY on Driver command spans) · → escher-0.1.0/chunks/2026-10-06-telemetry-bootstrap/
2026-10-06-headless-stand · complete · Headless stand — seven_guis lean four in TaskShell, no display, fixed viewport, bundled fonts, no network, fresh per check · → escher-0.1.0/chunks/2026-10-06-headless-stand/
2026-10-06-stand-test-contract · complete · Stand test contract — agent-invocable boot/run/status/cleanup + JSON-line logs for stand checks and blitz-tests · → escher-0.1.0/chunks/2026-10-06-stand-test-contract/
2026-10-06-cold-agent-run-pipe · complete · Cold-agent run pipe — fresh agent session with only a stub tool; transcript, wrong-call count and verdict recorded green · → escher-0.1.0/chunks/2026-10-06-cold-agent-run-pipe/
2026-10-06-upstream-sync-element-identity · complete · Upstream sync ahead of element identity — upstream/main 23354585 merged, our changes additive, our tests and CI green · → escher-0.1.0/chunks/2026-10-06-upstream-sync-element-identity/
2026-10-06-stable-element-ids · complete · Stable element ids — author key else component path, on every stand element (v010-01) · → escher-0.1.0/chunks/2026-10-06-stable-element-ids/
2026-10-06-id-persistence · complete · Id persistence — same id across re-render, remount and fresh process on the stand (v010-02) · → escher-0.1.0/chunks/2026-10-06-id-persistence/
2026-10-06-project-readme · complete · Project README — the repository front page describes escher, not Blitz · → escher-0.1.0/chunks/2026-10-06-project-readme/
2026-10-06-accessibility-tree-identity · complete · Accessibility-tree identity — stable id on every accessibility node, stand controls carrying role and name (v010-03) · → escher-0.1.0/chunks/2026-10-06-accessibility-tree-identity/
2026-10-06-upstream-sync-observation-model · complete · Upstream sync ahead of the observation model — upstream/main still 23354585, 0 ahead: measured no-op, no merge · → escher-0.1.0/chunks/2026-10-06-upstream-sync-observation-model/
2026-10-06-snapshot-model · complete · Snapshot model — screen as a tree of id, role, name, state, bounds (v010-04) · → escher-0.1.0/chunks/2026-10-06-snapshot-model/
2026-10-06-id-stability-across-code-edits · complete · ids hold across code edits — paths anchor at the nearest keyed ancestor; actionable elements must be keyed, with a check · → escher-0.1.0/chunks/2026-10-06-id-stability-across-code-edits/
2026-10-06-snapshot-state-fidelity · complete · Snapshot state fidelity — enabled, checked, value, focused read true per control; password values masked (v010-05) · → escher-0.1.0/chunks/2026-10-06-snapshot-state-fidelity/
2026-10-06-compact-snapshot-serialization · complete · Compact snapshot serialization — the snapshot as one text (a line per node), each lean stand screen 755 to 2034 bytes under a recorded 10,000-byte budget; a file input's value masked (v010-04 advanced, not claimed) · → escher-0.1.0/chunks/2026-10-06-compact-snapshot-serialization/
2026-10-07-change-tracking-and-diff · complete · change tracking and the post-action diff — changed-node set drained per step, truthful change flag, empty diff for a no-op · → escher-0.1.0/chunks/2026-10-07-change-tracking-and-diff/
2026-10-07-audit-corrections · complete · audit corrections — seven surviving dioxus-native-dom mutants killed by six unit tests in a new test file, id walk and three stand checks under the complexity ceiling, the stand's tables and helpers stated once in a shared module · → escher-0.1.0/chunks/2026-10-07-audit-corrections/
2026-10-07-upstream-sync-driver-core · complete · Upstream sync ahead of the driver core — upstream/main still 23354585, 0 ahead: measured no-op, no merge · → escher-0.1.0/chunks/2026-10-07-upstream-sync-driver-core/
2026-10-07-driver-session · complete · Driver session — one headless stand instance held across commands, own lifecycle (start, attach, stop), the one process both CLI and MCP drive · → escher-0.1.0/chunks/2026-10-07-driver-session/
2026-10-07-sink-target-allowlist · complete · Sink target allowlist — escher's log sink drops every record from a target outside its allowlist; session host and windowed stand print no id or name at trace · → escher-0.1.0/chunks/2026-10-07-sink-target-allowlist/
2026-10-07-settle-detection · complete · Settle detection — UI quiescence across render, layout, timers and pending loads; delayed stand update passes with no sleep · → escher-0.1.0/chunks/2026-10-07-settle-detection/
2026-10-07-command-and-refusal-schema · complete · Command and refusal schema — one verb set, argument and result shapes, malformed arguments refused, refusal causes each with a remedy · → escher-0.1.0/chunks/2026-10-07-command-and-refusal-schema/
2026-10-07-act-by-id · complete · Act by id — driver actions addressed by stable id, returning after settle with the diff · → escher-0.1.0/chunks/2026-10-07-act-by-id/
2026-10-07-refusal-detection · complete · Refusal detection — not found, stale, disabled, covered by another element, off-screen named per action · → escher-0.1.0/chunks/2026-10-07-refusal-detection/
2026-10-07-driver-command-spans · complete · Driver command spans — one span per driver command covering settle wait, diff size and refusal cause, through the scrub layer · → escher-0.1.0/chunks/2026-10-07-driver-command-spans/
