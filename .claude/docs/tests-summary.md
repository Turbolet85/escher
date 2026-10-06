# Tests Summary — escher

_Distilled from `.andromeda/test-plan.md` (adopted reading). wrap-session's cascade re-derives it when the plan changes._

## Test tier

**Tier:** 0
**Justification:** NOT YET MEASURED — the reading recorded no tier justification or critical-path list.

## Harness contract (§3)

- **Test runner:** Rust built-in harness (`#[test]`, `#[tokio::test]`); `test-that` matchers in some tests; WPT runner for conformance; `unittest` for CI scripts.
- **In-process harness (measured):** `blitz-test-harness::Harness` — construct (`from_html`, `from_component`, `from_vdom`; `HarnessOptions` also takes `font_ctx` and `incremental`), `pump`/`tick`, input helpers that pump, inspection (`query`, `layout_rect`, `text_content`, `focused`, `hit`, `dom_string`). See `.claude/rules/verification-harness.md`.
- **Headless stand (measured):** `seven_guis::stand::boot(LeanTask, options)` / `boot_timer(options)` mount counter, flight booker, timer or CRUD in TaskShell — fresh `VirtualDom` per boot, pinned 800×600 · scale 1 · Light, bundled DejaVu Sans with system fonts off, offline (`net_provider: None`); the timer advances only through its `TimerTicks` handle (`deliver(n)` then `pump`).
- **5-command discipline (boot / run / status / cleanup / logs):** NOT YET MEASURED → owned by the working route's "Stand test contract" chunk; no `scripts/agent-run.*` exists yet.
- **Stand log format (measured):** `seven_guis_native` writes stderr only, one `{RFC 3339 UTC} {LEVEL} {target} service.name=… service.version=… {field}={value}…` line per event, scrubbed (escher-telemetry, bound to obs-plan §3); exercised by the four `telemetry_*` integration files and a 10 s boot smoke (exit 124 + `service.name=seven_guis`).
- **Status endpoint · PID file · JSON-line logs · tempdir:** NOT YET MEASURED (same owner).

## Coverage as built
- `tests/blitz-tests` — integration tests across accessibility, layout, text, paint, hit testing, input, scrolling, invalidation, leaks, stylesheet loading, Dioxus integration (incl. falsy-`disabled` clearing), the headless stand (`stand_*`: TaskShell mount, bundled-font text, two-boot and incremental `dom_string` identity, zero net requests, one driven interaction per lean task, timer ticks — asserted unconditionally, no font skip) and process telemetry (stdout silent, scrub, panic hook, idempotent init — one process per file).
- `escher-telemetry`: 5 inline unit tests (scrub decision branches, a bridged `log` record, the identity macro).
- Differential oracle: incremental vs full layout compared node-by-node after every mutation step.
- Pixel tests on a CPU buffer (box-shadow expectations from Chromium, 1px AA tolerance).
- `blitz-vibey-script`: 26 DOM API tests + 2 headless Preact TodoMVC flows.
- WPT `css` + `svg` in upstream CI only (diffed against main, posted to PRs); on the fork WPT runs on the host.
- CI (§9): every linux leg runs through `.github/scripts/ci-leg.sh` — fast legs fmt · clippy · `test` (`cargo test --workspace --locked`) · CI scripts, then build/MSRV/counter/wasm/docs and the windows/macos/ios/android matrix; `test_ci_workflows.py` pins the workflow invariants (16 CI-script tests).
- Local baseline (§9, dev host, dev profile `debug = "line-tables-only"`): `cargo test -p blitz-tests` 255 passed · 0 failed · 3 ignored (`paint_tree_bench`), 45 s cold (2026-10-05; +18 tests, +1 ignored since, not re-measured per crate); `cargo test --workspace` 430 · 0 · 4 at 2026-10-06-headless-stand (416 · 0 · 4 at telemetry-bootstrap; 407 · 0 · 3, 52 s cold at the 2026-10-05 baseline, was 1633 s under full debuginfo).
- Apps: only the browser crates test (about pages, history, favicon, suggestions, persistence).

## E2E coverage (§6)
- **Headless harness smoke** — HTML + Dioxus documents built, inspected and driven (`harness_smoke.rs`).
- **Preact TodoMVC** — add / toggle / filter / destroy / clear-completed through `ScriptDocument`.
- **Headless stand** — each lean task booted in TaskShell and driven through one interaction (`stand_counter` / `stand_flight_booker` / `stand_timer` / `stand_crud`).
- **Windowed E2E** — NOT YET MEASURED; the remaining stand flows are a route chunk ("Stand requirement sweep").

## Quality gates (§10)
| Gate | Threshold | Tool |
|---|---|---|
| Coverage | no threshold — first reading 53.23 % lines (dev host), 53.25 % (CI); a floor is "Quality gates" | `ci-leg.sh coverage` (cargo-llvm-cov; `coverage-report` artifact) |
| Flakiness budget | NOT YET MEASURED | "Quality gates" |
| Performance budget | NOT YET MEASURED | — |
| Format / lint / docs | must pass — fmt, clippy and the workspace rustdoc gate green | `ci-leg.sh fmt` · `ci-leg.sh clippy` (`cargo clippy --workspace --locked -- -D warnings`) · `ci-leg.sh doc` (`RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked`) |

## Universal anti-patterns
> NO RECORDED INTENT.

## Critical decisions
> NO RECORDED INTENT — conventions as built are in `.claude/rules/testing.md`.

---

**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: `.claude/rules/testing.md`.
