# Codebase Research — 2026-10-05-ci-gate-legs

## Scope
- **Depth:** moderate · **Reads:** 14 · **Globs/Greps:** 11
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — 0 additions applied (its Session Additions section is empty; its rules cover the in-process `Harness` and the not-yet-rendered `agent-run.*`, neither of which this chunk drives). The only live leg is the operator CI push, whose read form is `ci.py conclusion` (the 2026-10-05-fork-ci-reached precedent).
- **Platform issues consulted:** none. No runner-only bullet was folded (Setup 5a read green), and the CI-reading entry is the operator leg.
- **External inputs:** none. Every fact this chunk turns on lives in this repository, in a CI run read by id, or in a public registry read at research time (the advisory database, the action tags' commit SHAs). Those readings are recorded below with their date.

## Files inspected
- `.github/workflows/ci.yml` (full) — 11 jobs; no `permissions:` block anywhere (`grep -n permissions .github/workflows/ci.yml` → 0 hits); 6 distinct third-party action refs, all tag- or branch-pinned (list below); the `doc` job (190-207) runs `bash .github/scripts/ci-leg.sh doc`; workflow env `RUSTDOCFLAGS: "-D warnings"` (15-16).
- `.github/scripts/ci-leg.sh` (full) — `LEGS=(fmt clippy test ci-scripts build msrv counter wasm doc fast)` (:8); `doc) cargo doc --locked` (:32) — no `--workspace`, no `--no-deps`; `RUSTDOCFLAGS="-D warnings"` exported (:54); log tee'd to `target/ci-logs/$leg.log` (:56).
- `.github/scripts/test_ci_workflows.py` (full) — 12 tests; `test_b` matches the cache step by EXACT string `s.get("uses") == "Swatinem/rust-cache@v2"` (:76), so SHA-pinning that action breaks the test as written; `test_c` pins the fast set; `test_d` asserts the linux job set equals `LINUX_JOB_LEGS` (:96), so every new linux job must be added there; `test_e` matches upload by `startswith("actions/upload-artifact@")` (:110), which survives a SHA pin.
- `.github/workflows/{publish-browser,wpt,wpt-post-results}.yml` (`uses:` and `permissions` lines) — upstream-only (repository-guarded). They hold `awalsh128/cache-apt-pkgs-action@latest` (wpt.yml:48, publish-browser.yml:148), `actions/checkout@v5`, `peter-evans/repository-dispatch@v4` and others. Their permissions are already explicit: wpt-post-results.yml:8-11, wpt.yml:18-21, publish-browser.yml:38-39 (job-level `contents: write`).
- `Cargo.toml` (1-80, 190-275) — root `[package] blitz-examples` has no `[lib]` (the "virtual package", :239-250); `[profile.dev] debug = "line-tables-only"` (:195-196).
- `apps/browser/Cargo.toml` (1-12) — `[[bin]] name = "blitz"` (:8-10), the colliding name against `packages/blitz` (lib `blitz`).
- `packages/blitz-vibey-script/src/document.rs` (111-126, 256-259) — the 3 blitz-vibey-script rustdoc errors.
- `packages/blitz-vibey-script/Cargo.toml` (15-46) — `blitz-dom` is a normal dependency (:19); `tracing` is optional (:40, feature :15).
- `packages/blitz-dom/src/node/node.rs` (129, 1596), `layout/replaced.rs` (20), `traversal.rs` (47, 78) — the 5 blitz-dom rustdoc errors.
- `examples/transparent/src/main.rs` (9-12) — the 1 transparent rustdoc error (`[`Config`]` in a `//!` block).
- `tests/blitz-tests/Cargo.toml` (16) — `blitz-dom` dev-dep with `features = ["accessibility", "floats", "system-fonts"]`.
- `tests/blitz-tests/tests/{accessibility_hidden,accessibility_roles,focusability_updates}.rs` — 6 + 6 + 3 `#[test]` (`grep -c '#\[test\]'`); 0 font/eprintln hits (`grep -ciE 'eprintln|font'` → 0 each).
- `Cargo.lock` (reverse-dependency walk by script) — the advisory chains below.

## Graph impact
- **offset_parent** (`packages/blitz-dom/src/node/node.rs:1598`) and **is_offset_parent** (`node.rs:1574`): both `fn` on `impl Node`, so the doc link from a public item to a private one is a doc-text change only.
- **Document::poll** is defined in **blitz-dom** (`packages/blitz-dom/src/document.rs:142`), not in blitz-traits. So `blitz_traits::Document::poll` cannot resolve. The fix re-points the link to `blitz_dom::Document::poll`, which works because blitz-vibey-script depends on blitz-dom.
- No signature changes. Every rustdoc fix is in doc comments, plus one `doc = false` manifest key, so no caller set is involved. Trace: `.andromeda/runs/2026-10-05T22-31-57-phase/tree-query-2026-10-05-ci-gate-legs.json` (rows 3).

## Patterns detected
- **One leg script for CI and host** (`.github/scripts/ci-leg.sh:17-34`): every linux job is `bash .github/scripts/ci-leg.sh {leg}`, and a new leg is one `case` arm plus a `LEGS` entry.
- **Workflow invariants pinned by unittest** (`.github/scripts/test_ci_workflows.py:61-131`): `CiWorkflowTest` reads ci.yml with PyYAML, `UpstreamGuardTest` reads the guarded workflows, and `LegScriptTest` runs the script with a `cargo` shim on PATH.
- **Per-job failure log** (`.github/workflows/ci.yml:44-50` and its siblings): `actions/upload-artifact` with `if: failure()`, `name: ci-log-{job id}`, `path: target/ci-logs/{leg}.log`, `retention-days: 7`, `if-no-files-found: ignore`.
- **Pinned CI tool installs**: `cargo install cross --git … --rev 426e811` (`ci.yml:269`), and `taiki-e/install-action@v2` in `publish-browser.yml:124` (the install action already used in this repo).

## Conventions to follow
- **Every cargo leg `--locked`** (`ci-leg.sh:20-32`), except `examples/wasm_hello`.
- **Fast/slow split** (`ci.yml:33` and the other `needs` lines; `test_ci_workflows.py:82-91`): a new job `needs: [fmt, clippy, test-features-default, ci-scripts]` unless it is deliberately added to `FAST_JOBS`.
- **Cache step on every compiling job** (`test_ci_workflows.py:71-80`): `save-if` carries `refs/heads/main` and `refs/heads/build/`.

## Measurements at HEAD (`15e36b8a`, dev host, 2026-10-05)
- **Workspace rustdoc**: `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked --keep-going` → exit 101. 3 crates fail: blitz-vibey-script (3 errors: `document.rs:113:18`, `:125:18` unresolved `blitz_traits::Document::poll`; `:258:11` unresolved `tracing::error`), blitz-dom (5: `node/node.rs:129:29` unresolved `Node::style`; `:1596:64` public→private `Self::is_offset_parent`; `layout/replaced.rs:20:6` bare URL; `traversal.rs:47:54` and `:78:48` redundant explicit link target), transparent (1: `main.rs:11:12` unresolved `Config`). That is 9 errors, plus `warning: output filename collision at target/doc/blitz/index.html`. Without `--keep-going` the run stops at the first failing crate and lists only blitz-dom's 5. The CARRY's figures hold exactly under the new profile.
- **Advisories** (`cargo audit --json`, cargo-audit 0.22.2, 990 lockfile deps, DB fetched 2026-10-05): exit 1. 1 vulnerability: **RUSTSEC-2026-0285** `rustls 0.23.43`, patched `>=0.23.45`. Lockfile chain: rustls ← hyper-rustls 0.27.9 ← reqwest 0.13.4 ← blitz-net, plus ← tokio-rustls 0.26.5. 3 warnings, which cargo-audit does not fail on by default: **RUSTSEC-2024-0436** `paste 1.0.15` unmaintained (← rav1e 0.8.1); **RUSTSEC-2026-0192** `ttf-parser 0.25.1` unmaintained (← owned_ttf_parser ← ab_glyph ← sctk-adwaita); **RUSTSEC-2026-0186** `memmap2 0.5.10` unsound (← cacache 13.1.0 ← http-cache ← blitz-net's optional `cache` feature). None of the four sits on a CLAUDE.md coupled pin (html5ever↔web_atoms, skrifa↔parley/vello, svgtypes↔usvg, taffy/parley revs, winit beta). rustls 0.23.43 → 0.23.45 is a semver-patch `cargo update -p rustls --precise 0.23.45`.
- **The same advisories under cargo-deny** (P4 mechanism check, cargo-deny 0.20.2, no `deny.toml`): `cargo deny --locked check advisories` → exit 1, `advisories FAILED`, ONE finding, `error[unmaintained]` **RUSTSEC-2026-0192** `ttf-parser`. The same result with `--all-features`. Its `-L debug` log prints `filtered rustls 0.23.43`, `filtered memmap2 0.5.10`, `filtered paste 1.0.15` (and hyper-rustls, tokio-rustls, rustls-webpki, pastey), so cargo-deny's default graph drops them, while cargo-audit reads the lockfile whole. `cargo tree --workspace --all-features --target all -e all -i rustls` → `nothing to print`: **no build graph of this workspace reaches rustls**. It sits in Cargo.lock only, through reqwest's never-enabled rustls feature. paste (← rav1e ← ravif ← image ← anyrender_svg) and memmap2 0.5.10 (← cacache ← http-cache) ARE in the `--all-features --target all` tree, so why deny filters them is not settled here. So the set of advisories the gate fires on depends on the graph `deny.toml` declares (features, targets), and /implement measures it with the committed config.
- **a11y leg selection** (P4 mechanism check): `cargo test --workspace --locked --test accessibility_hidden --test accessibility_roles --test focusability_updates` → exit 0. It prints `running 6 tests` / `running 6 tests` / `running 3 tests` and three `test result: ok.` lines (6 + 6 + 3 passed), so `--workspace` with `--test` selects across the workspace without erroring on packages that lack those targets. The workspace selection keeps the `test` leg's feature resolution.
- **Host tools** (`command -v`, `--version`): cargo-deny 0.20.2, cargo-audit 0.22.2, cargo-llvm-cov 0.9.1 present; cargo-tarpaulin absent; rustup component `llvm-tools` NOT installed on `stable` (cargo-llvm-cov needs it: `rustup component add llvm-tools-preview`). No `deny.toml` or `.cargo/audit.toml` exists.
- **Action refs → commit SHA** (`gh api repos/{owner}/{repo}/commits/{ref} --jq .sha`, 2026-10-05). These are the ci.yml refs to pin; implement re-resolves at its own time:
  - `actions/checkout@v4` → `11d5960a326750d5838078e36cf38b85af677262`
  - `actions/upload-artifact@v7` → `043fb46d1a93c77aae656e7c1c64a875d1fc6a0a`
  - `Swatinem/rust-cache@v2` → `6323deb102c322ba6fcbdcafc7e3dddab59af2b6`
  - `dtolnay/rust-toolchain@master` → `7e38f4b43b4db5c8dd498af069a4f6196df1d067`. ci.yml also uses `@stable` (:58, :77, :97, :196), a BRANCH that selects the toolchain by its name. A SHA-pinned rust-toolchain needs an explicit `with: toolchain: stable` input, or the toolchain choice is lost.
  - `jlumbroso/free-disk-space@v1.3.1` → `54081f138730dfa15788a46383842cd2f914a1be`
  - `awalsh128/cache-apt-pkgs-action@latest` → `553a35bb8ebd9fcabcb1c9451aa4c98e1b4ca8a9`
  - `taiki-e/install-action@v2` → `183e4297cca2404691e9380e1307288dced5c82a` (candidate for installing cargo-deny / cargo-llvm-cov at a pinned tool version)
- **CI-scripts count**: 16 tests at HEAD (`grep -c "def test_"`: test_ci_workflows.py 12 + test_wpt_diff_to_pr.py 4).
- **a11y test set**: `accessibility_hidden` 6 + `accessibility_roles` 6 + `focusability_updates` 3 = 15 tests, none font-dependent. `-p blitz-tests` keeps `accessibility` on through blitz-tests' own dev-dep features (`tests/blitz-tests/Cargo.toml:16`), so a `--test`-selected leg cannot drop the feature.

## New files to create
- `deny.toml` — the cargo-deny configuration with the advisory per-ID ignores and their reasons (only if P4 picks cargo-deny)

## Files to modify
- `.github/workflows/ci.yml` — workflow `permissions:`; SHA pins; new jobs (audit, coverage, a11y); doc job unchanged in shape
- `.github/scripts/ci-leg.sh` — new legs (audit, coverage, a11y); `doc` leg becomes workspace-wide `--no-deps`
- `.github/scripts/test_ci_workflows.py` — invariants for pins, permissions, the new legs, the doc-leg form; `test_b` matcher made pin-tolerant
- `Cargo.lock` — rustls 0.23.43 → 0.23.45
- `apps/browser/Cargo.toml` — `doc = false` on the `blitz` bin (the filename collision)
- `packages/blitz-vibey-script/src/document.rs` — 3 doc-link fixes
- `packages/blitz-dom/src/node/node.rs` — 2 doc-link fixes
- `packages/blitz-dom/src/layout/replaced.rs` — bare URL to an autolink
- `packages/blitz-dom/src/traversal.rs` — 2 redundant link targets removed
- `examples/transparent/src/main.rs` — 1 doc-link fix

## Open questions
- Resolved at P4 (overseer, on the founder's standing delegation of technical forks, 2026-10-05): audit tool **cargo-deny**; coverage leg **uncached** (cold each run). Pinning ci.yml only is a decisive lean (plan.md §Provenance).
- Which advisories the committed `deny.toml` graph fires on (1 at default graph; up to 4 lockfile-wide) → blocks: implementation-scope (the ignore list is measured at /implement, each entry an upgrade or a reasoned per-ID ignore)
