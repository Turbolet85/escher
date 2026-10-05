# Tests Summary — escher

_Distilled from `.andromeda/test-plan.md` (adopted reading). wrap-session's cascade re-derives it when the plan changes._

## Test tier

**Tier:** 0
**Justification:** NOT YET MEASURED — the reading recorded no tier justification or critical-path list.

## Harness contract (§3)

- **Test runner:** Rust built-in harness (`#[test]`, `#[tokio::test]`); `test-that` matchers in some tests; WPT runner for conformance; `unittest` for CI scripts.
- **In-process harness (measured):** `blitz-test-harness::Harness` — construct (`from_html`, `from_component`, `from_vdom`), `pump`/`tick`, input helpers that pump, inspection (`query`, `layout_rect`, `text_content`, `focused`, `hit`, `dom_string`). See `.claude/rules/verification-harness.md`.
- **5-command discipline (boot / run / status / cleanup / logs):** NOT YET MEASURED → owned by the working route's "Stand test contract" chunk; no `scripts/agent-run.*` exists yet.
- **Status endpoint · PID file · log format · tempdir:** NOT YET MEASURED (same owner; log format bound to obs-plan §3).

## Coverage as built
- `tests/blitz-tests` — integration tests across accessibility, layout, text, paint, hit testing, input, scrolling, invalidation, leaks, stylesheet loading, Dioxus integration.
- Differential oracle: incremental vs full layout compared node-by-node after every mutation step.
- Pixel tests on a CPU buffer (box-shadow expectations from Chromium, 1px AA tolerance).
- `blitz-vibey-script`: 26 DOM API tests + 2 headless Preact TodoMVC flows.
- WPT `css` + `svg` in upstream CI only (diffed against main, posted to PRs); on the fork WPT runs on the host.
- CI (§9): every linux leg runs through `.github/scripts/ci-leg.sh` — fast legs fmt · clippy · `test` (`cargo test --workspace --locked`) · CI scripts, then build/MSRV/counter/wasm/docs and the windows/macos/ios/android matrix; `test_ci_workflows.py` pins the workflow invariants (16 CI-script tests).
- Local baseline (§9, dev host, dev profile `debug = "line-tables-only"`): `cargo test -p blitz-tests` 255 passed · 0 failed · 3 ignored (`paint_tree_bench`), 45 s cold; `cargo test --workspace` 407 · 0 · 3, 52 s cold (was 1633 s under full debuginfo).
- Apps: only the browser crates test (about pages, history, favicon, suggestions, persistence).

## E2E coverage (§6)
- **Headless harness smoke** — HTML + Dioxus documents built, inspected and driven (`harness_smoke.rs`).
- **Preact TodoMVC** — add / toggle / filter / destroy / clear-completed through `ScriptDocument`.
- **Windowed E2E** — NOT YET MEASURED; the stand flows are route chunks ("Headless stand", "Stand requirement sweep").

## Quality gates (§10)
| Gate | Threshold | Tool |
|---|---|---|
| Coverage | NOT YET MEASURED (tooling absent) | owned by "CI gate legs" / "Quality gates" |
| Flakiness budget | NOT YET MEASURED | "Quality gates" |
| Performance budget | NOT YET MEASURED | — |
| Format / lint / docs | must pass — fmt and clippy green at baseline; workspace rustdoc red (3 crates, 9 errors), owned by "CI gate legs" | `ci-leg.sh fmt` · `ci-leg.sh clippy` (`cargo clippy --workspace --locked -- -D warnings`) · `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` |

## Universal anti-patterns
> NO RECORDED INTENT.

## Critical decisions
> NO RECORDED INTENT — conventions as built are in `.claude/rules/testing.md`.

---

**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules: `.claude/rules/testing.md`.
