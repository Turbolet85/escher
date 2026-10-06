# security extract

## Relevance
partial. The chunk adds no surface, input boundary, secret or listener. Its security weight is supply chain: new taffy/parley git revs and a merged `Cargo.lock` enter the audited graph, and upstream edits land in files the plan lists as validation or no-panic boundaries.

## Constraints
- Git dependencies stay pinned by commit `rev`, and every ci.yml cargo leg keeps `--locked`. Upstream's taffy `4142c9d8` and parley `e41dfea5` revs must arrive as `rev =` pins, and the merged `Cargo.lock` must build under `--locked` without a manual unlock (per security-plan §Dependency Security, Pinning · Supply chain integrity / Lockfile verification).
- The audit leg (`ci-leg.sh audit`, cargo-deny 0.20.2, root `deny.toml`) must pass on the merged lockfile. If it fires a new advisory, the fix is a semver-compatible update or a per-ID ignore with a written reason. A blanket allow is not allowed, and neither is setting `unmaintained`/`unsound` to none (per security-plan §Dependency Security, Audit tool).
- The standing RUSTSEC-2026-0192 ignore assumes no fix is reachable without a lone bump of the coupled winit pin. After the merge, check that this still holds, and keep the ignore only while its written reason stays true. Whether upstream's set changes the winit → ttf-parser chain is research's question (per security-plan §Dependency Security, Audit tool).
- The audit's reach is limited to cargo-deny's resolved graph, and the paste/memmap2 advisories sit outside it. Any crate the merge newly adds to the lockfile (`git diff` of `Cargo.lock` base..merge) gets a hand review and does not rest on a green audit alone (per security-plan §Dependency Security, Audit tool).
- Upstream's edits to `stylo_taffy/src/convert.rs` (#1063 `normal` alignment) and `blitz-dom/src/layout/inline.rs` touch boundaries the plan records. Unknown alignment flags must map to none rather than panic. Inline-box heights must stay finite. The `unsafe` calc-pointer block's count and scope must stay the same, or a change must be named. Whether the merged code still does each of these is research's question (per security-plan §Error Handling, Graceful degradation · §Input Validation, Layout values · §Dependency Security, Unsafe code).
- Upstream's `blitz-paint/src/render.rs` edit must keep culling non-finite or beyond-f32::MAX geometry before paint. Upstream's `wpt/runner/.../attr_test.rs` edit must keep non-f32 checkLayout values as a subtest error rather than a panic (per security-plan §Input Validation, Paint values · WPT runner / checkLayout).
- Upstream-only workflows keep their `github.repository` guard, so secret-bearing jobs do not re-arm on the fork. The scope records no `.github/` change in the delta. Verify that the merge result leaves the guards byte-identical (per security-plan §Secret Management, Storage · §Authentication & Authorization, RBAC / permissions).

## Patterns to follow
- Handle coupled pins as a set, and record any advisory disposition as a per-ID `deny.toml` entry with a written reason. The RUSTSEC-2026-0192 entry is the precedent (per security-plan §Dependency Security, Audit tool).
- Measure lockfile and audit evidence on the dev host and record it in the chunk's evidence. The model is `escher-0.1.0/chunks/2026-10-05-ci-gate-legs/evidence/audit.md` (per security-plan §Dependency Security, Audit tool).
- New engine log call sites go behind `#[cfg(feature = "tracing")]`. Fields on engine targets are redacted by escher-telemetry's allowlist unless they are in the safe set (per security-plan §Logging & Monitoring, Log format and backends).

## Anti-patterns to avoid
- A one-sided lockfile fix: hand-bumping or downgrading one crate, or running `cargo update` beyond what the merge requires, to quiet the audit or a build (per security-plan §Dependency Security, Pinning · Audit tool).
- A blanket advisory allow, or an ignore with no written reason (per security-plan §Dependency Security, Audit tool).
- Admitting a new upstream log field that carries user content (url/href/text/value/html) into escher's sink allowlist (per security-plan §Logging & Monitoring, Log format and backends).

## Contract bindings
- The CI security gate binds to the tests domain's CI integration. The `audit` job in ci.yml runs on the pushed merge in the slow tier, and fork CI green includes it (per security-plan §Dependency Security, CI integration).
- Log scrubbing binds to obs. If upstream adds `tracing` fields under `blitz*` / `stylo_taffy` targets, the existing `telemetry_*` tests are the proof that escher's allowlist scrub still redacts them (per security-plan §Logging & Monitoring, Log format and backends).

## Acceptance criteria contributions
- `bash .github/scripts/ci-leg.sh audit` exits 0 on the merge commit. Any new `deny.toml` ignore is per-ID with a written reason, and fork CI's `audit` job is green (per security-plan §Dependency Security, Audit tool · CI integration).
- `Cargo.toml` pins taffy and parley by `rev` at upstream's values, and `cargo build --workspace --locked` succeeds with no lockfile rewrite. The added-crate list from `git diff` of `Cargo.lock` (base..merge) is recorded with a hand-review note (per security-plan §Dependency Security, Pinning · Supply chain integrity).
- `git diff <pre-merge>..<merge> -- .github/` is empty, so the `github.repository` guards on the publish, WPT and post-results jobs are unchanged (per security-plan §Secret Management, Storage).
- A grep for `unsafe` in `packages/stylo_taffy/src/convert.rs` and `packages/blitz-dom/src/layout/{mod,table}.rs` returns the same blocks as before the merge, or each new one is named in the report (per security-plan §Dependency Security, Unsafe code).
