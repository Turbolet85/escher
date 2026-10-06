---
paths:
  - "scripts/agent-run.*"
  - "packages/blitz-test-harness/**"
  - "tests/blitz-tests/tests/harness_*.rs"
---

# Verification Harness Rules

Path-scoped rules for the agent-driven verification harness. Source: `.andromeda/test-plan.md` §3.
Development Style is agent-driven — escher's own driver is meant to become the harness.

## The 5-command contract (measured — `scripts/agent-run.sh`)
- Run from the repository root: `bash scripts/agent-run.sh boot`, then `run stand` · `run all` · `run {blitz-tests file}`, `status`, `logs`, `cleanup`. `scripts/agent-run.ps1` only forwards to it (Windows; untested here — no `pwsh`). The scripts are project-authored: setup preserves them, never re-renders them.
- Exit grammar, every verb: `0` success · `1` the verb ran and failed — a build failure, a failing run, or an EMPTY run (cargo 0 with no test line parsed is never a pass) · `2` usage, checked before · `3` a precondition is unmet (not booted; `logs` with nothing to read).
- `{name}` must match `^[a-z0-9_]+$` and name an existing `tests/blitz-tests/tests/{name}.rs`; `run stand` with no `stand_*.rs` files is an empty run (exit 1, no cargo call); under `all`, `run.start.files` is `[]`.
- Stdout is JSON lines only — `boot`, `run.start`, one `test {file, test, outcome}` per libtest line, `run.end {passed, failed, ignored, cargo_exit, outcome}`, `status`, `cleanup` — encoded by python3's `json`, never by bash string building. No event carries captured test output or a panic message, and no field takes a scrub-set name (`url href src html text value attrs path request error`); raw cargo output stays in `target/agent-run/run.log`.
- State lives in `target/agent-run/{status.json, events.jsonl, run.log}` only — no daemon, PID file, socket, port or env var (the stand is an in-process boot; adding one is an arch §Occupied Resources registration). One driver at a time: a killed run leaves `running`, the next `run` overwrites it.
- libtest's `--format json` is nightly-only, so the parser reads the pretty lines (`Running …`, `Doc-tests …`, `test … ... ok|FAILED|ignored`); the contract tests are `.github/scripts/test_agent_run.py` (a `cargo` shim, run by the `ci-scripts` leg).
- NOT YET MEASURED: a test-data bootstrap mechanism.

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
