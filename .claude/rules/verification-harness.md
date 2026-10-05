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
- No product boot / run / status / cleanup / logs command, status shape, PID file or log format exists yet, so `scripts/agent-run.{sh,ps1}` is not rendered. It is owned by the working route's "Stand test contract" chunk (agent-invocable boot, run, status, cleanup and JSON-line logs for stand checks and blitz-tests); that chunk amends test-plan §3 and then the scripts are generated.

## In-process `Harness` (measured — `packages/blitz-test-harness`)
- Constructors `from_html`, `from_html_with(html, HarnessOptions)`, `from_component`, `from_vdom` pump once; `wrap` does not.
- `pump` polls with no waker and resolves at harness time; `dispatch` / `dispatch_recorded` do NOT pump. Input helpers (`click`, `type_text`, `press`, `tap`, `wheel_at`, `drag`, `ime`, …) pump after dispatch.
- `dispatch_recorded` drives the underlying `BaseDocument` and bypasses Dioxus VirtualDom forwarding — use input helpers for Dioxus behaviour.
- Synthesized pointer events set page, screen and client coordinates equal; `key_event` uses `Code::Unidentified` and fills text only for pressed character keys.
- `HarnessOptions` defaults: 800×600, scale 1, light scheme; harness documents always use `HtmlProvider`.
- `dom_string()` is a stable one-node-per-line serialization with geometry (`<div #box .a .b> @ (20,10) 100x50`) — the snapshot-style assertion surface.

## Session Additions
_This section is owned by `/andromeda-wrap-session`. setup-project preserves content added here on re-run._
