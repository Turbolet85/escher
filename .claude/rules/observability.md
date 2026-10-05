---
paths:
  - "packages/debug_timer/**"
  - "**/telemetry/**"
  - "**/tracing/**"
  - "**/logging/**"
  - "**/*telemetry*.rs"
  - "**/*tracing*.rs"
  - "**/*logging*.rs"
  - "**/panic*.rs"
---

# Observability Rules

Path-scoped rules for telemetry, logging and timing code. Source: `.andromeda/obs-plan.md` (obs tier 0).

## Logging (as built)
- **Library:** `tracing` 0.1 events, compiled only with each crate's `tracing` cargo feature; every call site is `#[cfg(feature = "tracing")]` with a `#[cfg(not(feature = "tracing"))] let _ = …;` fallback.
- The `tracing` feature forwards down the crate chain (dioxus-native → dioxus-native-dom, blitz-shell, blitz-dom, blitz-html, blitz-net); `blitz` turns it on by default, `blitz-vibey-script` does not.
- Subscribers: `tracing_subscriber::fmt::init()` in apps under `tracing`; `tracing_wasm` on wasm; the WPT runner uses the `log` facade with `env_logger`.
- JS console output goes to the `log` crate at debug level, target `js_console`, keeping stdout/stderr clean; embedders drain JS errors with `take_js_errors` (≤256 retained between drains).
- No spans, `#[instrument]`, metrics or OTel exist yet (observed absent).

## Timing
- Phase timing is opt-in: `log-phase-times` (→ `debug_timer/enable`) and `log-frame-times`; `debug_timer` swaps in a zero-cost dummy when `enable` is off and prints to stdout when on.

## PII
- URLs, attribute values, text-node contents and outer HTML are logged as-is and no scrub layer exists (obs-plan §8). New log fields carry no user content until the scrub layer lands.

## Not yet measured — owned by the working route
- Subscriber, service identity, opt-in OTel export, panic logging, scrub layer, logs off stdout → "Telemetry bootstrap" chunk.
- One span per driver command (settle wait, diff size, refusal cause) → "Driver command spans" chunk.

## Session Additions
_This section is owned by `/andromeda-wrap-session`. setup-project preserves content added here on re-run._
