# security · Dependency Security — category read

## Carried
- (.andromeda/security-plan-detail/dependency-security.md:23) The chunk bumps no dependency whichever way the founder's forks fall, so the coupled pins must stay at these values with no git dependency, and the audit leg must read both gates green at its close with no ignore added.
  > the pins stay — Stylo "0.22.0", Taffy the registry "0.15", Dioxus "0.7.3", winit `=0.31.0-beta.3`, no git dependency; the audit leg reads `advisories ok` and `licenses ok`, every standing ignore encountered and none added
- (.andromeda/security-plan-detail/dependency-security.md:8) The chunk's check of its own tree must be this audit invocation, over the whole workspace's graph, so an engine fix or a new check is gated on advisories and licences alike.
  > **Audit tool:** cargo-deny 0.20.2 — `cargo deny --locked --workspace --all-features check advisories licenses` against the RustSec advisory database and the licence table, over the whole workspace's graph since 2026-10-14-quality-gates
- (.andromeda/security-plan-detail/dependency-security.md:8) `deny.toml` must keep this shape and the count of eight ignores; an advisory arriving during the chunk may only be met per ID with a written reason, never a blanket allow.
  > `[advisories]` per-ID ignores only, each with a written reason, no blanket allow, `unmaintained` and `unsound` at their defaults (deny.toml:1-29; .github/scripts/ci-leg.sh:38). The file also holds a `[licenses]` table, which the audit leg reads since 2026-10-14-quality-gates (deny.toml:31-50; Supply chain integrity below). Eight ignores stand.
- (.andromeda/security-plan-detail/dependency-security.md:41) The served pages' contrast rows (debt 6) must read through this contrast reader as it stands: the rows add no dependency to blitz-tests, no `tracing` or `style` to blitz-test-harness, and no package to `Cargo.lock`.
  > Since 2026-10-13-stand-contrast-harness blitz-test-harness takes `color = { workspace = true }` (the workspace's "0.3", resolved 0.3.3, already in the graph through blitz-dom), the luminance source of its contrast reader (packages/blitz-test-harness/Cargo.toml:27) — reviewed at that chunk: no package enters `Cargo.lock` (its one changed line `"color",` inside blitz-test-harness's dependency list), the audit leg reads `advisories ok` with `deny.toml` unchanged, blitz-test-harness takes no `tracing` and no `style`, and blitz-tests gains no dependency
- (.andromeda/security-plan.md:264) Every check and fix the chunk adds must build and run under `--locked` in the ci.yml legs, so `Cargo.lock` must stay as it is for the legs to pass.
  > **Lockfile verification:** builds pass `--locked` (flake.nix:100; .github/workflows/publish-browser.yml:157), and so does every ci.yml cargo leg
- (.andromeda/security-plan.md:253) The chunk's CI verdict must include this job green, in its place in the slow tier.
  > **CI integration:** ci.yml's `audit` job ("Dependency audit") runs `bash .github/scripts/ci-leg.sh audit` on every pull request and push, in the slow tier behind the four fast jobs, uncached, its log uploaded on failure as `ci-log-audit` (.github/workflows/ci.yml:225-244).

## Bindings
- **Coupled pins ↔ the architecture plan's pins and Epoch 3's sync**: no fix here moves a pin, and a fix that would need upstream's `0980cce5f` or a dependency move goes to Epoch 3's sync rather than into this chunk; `(.andromeda/security-plan-detail/dependency-security.md:23)`.
- **Contrast reader ↔ a11y plan §5 (SC 1.4.3) and test plan**: the served pages' contrast rows are the a11y plan's open gap and the test plan's checks, and the reader they use stays within this category's dependency line; `(.andromeda/security-plan-detail/dependency-security.md:41)`.
- **Audit job ↔ test plan's CI integration**: the audit runs as one job of the one workflow, and the chunk's green verdict reads it there; `(.andromeda/security-plan.md:253)`.
- **Audit tool ↔ the body's Dependency Security section**: the body's broad line and the detail's invocation state one gate and must stay in step; `(.andromeda/security-plan.md:229)` and `(.andromeda/security-plan-detail/dependency-security.md:8)`.
