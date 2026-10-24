# security · Dependency Security — category read

## Carried
- (.andromeda/security-plan.md:264) The widened `a11y` leg (`.github/scripts/ci-leg.sh:39`) must keep passing `--locked` on every cargo command it runs, the new targets included, since the lockfile verification names the a11y leg among the locked legs.
  > and so does every ci.yml cargo leg — the leg script's build, test, clippy (both lines), doc, msrv, counter, wasm, audit, a11y, coverage and, since 2026-10-14-quality-gates, stand-repeat commands and the matrix command
- (.andromeda/security-plan.md:236) The edited leg script stays inside the range the pinning record cites for `--locked`, so no leg line the chunk adds or rewrites may drop it.
  > Builds pass `--locked` in the flake, in `dx bundle` and in every ci.yml cargo leg but `examples/wasm_hello`, which has no `Cargo.lock` (flake.nix:100; .github/workflows/publish-browser.yml:153; .github/scripts/ci-leg.sh:20-65; .github/workflows/ci.yml:417)
- (.andromeda/security-plan.md:237) Any check the chunk adds or re-asserts in blitz-tests must take no new dev-dependency; if one were ever needed, it must be inherited with `workspace = true` like every other blitz-tests dev-dependency (the scope adds no crate).
  > every dev-dependency of blitz-tests (tests/blitz-tests/Cargo.toml:15-42)
- (.andromeda/security-plan.md:229) The audit runs on every push over the whole workspace graph with each ignore read again, so the chunk must leave `Cargo.lock` and `deny.toml` alone (no ignore added, none dropped) and the audit must stay green on its push.
  > `cargo deny --locked --workspace --all-features check advisories licenses` over the whole workspace's graph, configured by the root `deny.toml` (the six platforms ci.yml builds, all features; ignores per ID only, each with a written reason, no blanket allow; `unsound` at its default, which keeps memmap2's unsound advisory out of the gate); each ignore re-read on every push
- (.andromeda/security-plan-detail/dependency-security.md:11) This is the audit baseline the chunk's push must keep reading: tokio past RUSTSEC-2026-0386 with no ignore added, every standing ignore still encountered, and the leg's two known warnings and nothing beyond them.
  > every standing ignore is still encountered, and the leg prints two warnings, `yanked` clipboard-win 5.4.1 and `license-not-encountered` NCSA (deny.toml:46)
- (.andromeda/security-plan.md:245) Re-asserting contrast goes through `Harness::text_contrast`, whose luminance source is the workspace `color`, so blitz-test-harness must take no new dependency while doing it and must keep taking no `tracing` and no `style`.
  > blitz-test-harness takes `color = { workspace = true }`, already in the graph through blitz-dom, as the luminance source of its contrast reader; no package enters `Cargo.lock`, and blitz-test-harness takes no `tracing` and no `style`.
- (.andromeda/security-plan-detail/dependency-security.md:40) The contrast reader's dependency record pins the harness and blitz-tests sides: blitz-tests takes no dependency for the contrast rows.
  > blitz-test-harness takes no `tracing` and no `style`, and blitz-tests gains no dependency

## Bindings
- **The a11y leg's `--locked` ↔ test-plan CI integration / the leg pin**: the scope widens `ci-leg.sh:39` and moves `test_ci_workflows.py`'s `LegScriptTest` a11y tuple with it. The new leg line must keep `--locked`, which is the security side's lockfile verification (.andromeda/security-plan.md:264) (.andromeda/security-plan.md:236).
- **No new dependency ↔ arch §Occupied Resources and the scope's "no crate added" boundary**: blitz-tests dev-dependencies stay workspace-inherited, and `Cargo.lock` and `deny.toml` stay untouched, so the audit job keeps its standing ignores and its two-warning baseline (.andromeda/security-plan.md:237) (.andromeda/security-plan.md:229) (.andromeda/security-plan-detail/dependency-security.md:11).
- **Contrast reader's `color` dependency ↔ a11y-plan contrast harness (`stand_contrast`, SC 1.4.3)**: the contrast rows are re-asserted through `Harness::text_contrast`, and its dependency footprint must not grow (.andromeda/security-plan.md:245) (.andromeda/security-plan-detail/dependency-security.md:40).
