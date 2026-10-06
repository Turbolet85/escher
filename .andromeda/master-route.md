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
