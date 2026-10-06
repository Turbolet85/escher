---
paths:
  - "scripts/agent-run.*"
  - "packages/blitz-test-harness/**"
  - "tests/blitz-tests/tests/harness_*.rs"
---

# Verification Harness Rules

Path-scoped rules for the agent-driven verification harness. Source: `.andromeda/test-plan.md` §3.
Development Style is agent-driven — escher's own driver is meant to become the harness.

## The 5-command contract — NOT YET MEASURED
- No product boot / run / status / cleanup / logs command, status shape, PID file or JSON-line log format exists yet (the stand's stderr text line is measured — test-plan §3, escher-telemetry), so `scripts/agent-run.{sh,ps1}` is not rendered. It is owned by the working route's "Stand test contract" chunk (agent-invocable boot, run, status, cleanup and JSON-line logs for stand checks and blitz-tests); that chunk amends test-plan §3 and then the scripts are generated.

## In-process `Harness` (measured — `packages/blitz-test-harness`)
- Constructors `from_html`, `from_html_with(html, HarnessOptions)`, `from_component`, `from_vdom` pump once; `wrap` does not.
- `pump` polls with no waker and resolves at harness time; `dispatch` / `dispatch_recorded` do NOT pump. Input helpers (`click`, `type_text`, `press`, `tap`, `wheel_at`, `drag`, `ime`, …) pump after dispatch.
- `dispatch_recorded` drives the underlying `BaseDocument` and bypasses Dioxus VirtualDom forwarding — use input helpers for Dioxus behaviour.
- Synthesized pointer events set page, screen and client coordinates equal; `key_event` uses `Code::Unidentified` and fills text only for pressed character keys.
- `HarnessOptions` defaults: 800×600, scale 1, light scheme, no `font_ctx` (system fonts) and no `incremental` override; harness documents always use `HtmlProvider`.
- The headless stand boots through `seven_guis::stand::{boot, boot_timer}` over `from_vdom` with `stand::options(incremental)` (bundled DejaVu Sans, system fonts off, offline); the timer advances only via its `TimerTicks` handle — `deliver(n)`, then `pump`.
- `dom_string()` is a stable one-node-per-line serialization with geometry (`<div #box .a .b> @ (20,10) 100x50`) — the snapshot-style assertion surface.

## Session Additions
_This section is owned by `/andromeda-wrap-session`. setup-project preserves content added here on re-run._
- 2026-10-06: A boot smoke bounded by `timeout N` exits 124 when the stand stays up, but the Andromeda gate tool reads any entry exit 124 or 137 as its own bound firing (`timeout`), so `expect = ['exit 124']` never reads green — author the entry so its own exit is 0 on the stay-up case, or drive it by hand and record its exit and atoms under the chunk's `evidence/`.
