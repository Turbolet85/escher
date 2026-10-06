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
