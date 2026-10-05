# Observability Summary — escher

_Distilled from `.andromeda/obs-plan.md` (adopted reading). wrap-session's cascade re-derives it when the plan changes._

## Obs tier

**Tier:** 0
**Justification:** NOT YET MEASURED — the reading recorded no telemetry-surface table or must-trace paths.

## Harness contract (§3)

- **Logger / library:** `tracing` 0.1 events behind per-crate `tracing` features (no-op when off); `tracing-subscriber` 0.3 `fmt::init()` in apps; `tracing_wasm` on wasm; `log` + `env_logger` in the WPT runner.
- **OTel SDK:** observed absent.
- **Log sink path · log format JSON schema · service identity · heartbeat · status endpoint:** NOT YET MEASURED.
- **Embedder drain:** `take_js_errors` / `take_messages` on `ScriptDocument` (≤256 errors retained between drains).

## Metrics and timing (as built)
| Operation | Output |
|---|---|
| `resolve` phase times (style … subdocs) | printed, `log-phase-times` |
| Frame times | `log-frame-times` per anyrender backend |
| `debug_timer` labelled instants | stdout when `enable` |
| Browser FPS overlay | in-app |
| WPT per-test / per-run stats | printed; scores to Pages |

No counters, histograms or exporters exist.

## SLO invariants (§10)
> NO RECORDED INTENT.

## PII
- Logged as-is: visited URLs, resource URLs, link hrefs, element attributes, outer HTML of failed SVGs, CSS values, text-node contents (dioxus debug). Scrubbing observed absent.

## Bootstrap phases (owners on the working route)
- `otel-sdk-install` + subscriber, service identity, panic logging, scrub layer, logs never on stdout → "Telemetry bootstrap".
- `pii-scrubbing-wire` → "Telemetry bootstrap" (scrub layer); driver spans through it → "Driver command spans".

## Universal anti-patterns
> NO RECORDED INTENT.

## Critical decisions
> NO RECORDED INTENT.

---

**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: `.claude/rules/observability.md`.
