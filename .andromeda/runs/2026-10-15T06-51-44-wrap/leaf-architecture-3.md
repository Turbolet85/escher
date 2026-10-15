group: architecture-3
read: .claude/docs/stack.md · 59 · «4. Commit as `chore: bump {crate} to {version}`.»
read: .claude/docs/workflow.md · 47 · «A reference-based CLAUDE.md: a thin index, with depth in `.c»

- leaf: .claude/docs/stack.md:39   class: omission
  leaf-text: «`cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings`, run as legs of»
  source: .andromeda/architecture.md:105 «`cargo clippy --workspace --locked -- -D warnings` — then, only when that line is green and since 2026-10-14-quality-gates, upstream's own blitz-dom line `cargo clippy -p blitz-dom»
  replace-with: «`cargo fmt --all --check`, `cargo clippy --workspace --locked -- -D warnings` — then, only when that line is green and since 2026-10-14-quality-gates, upstream's own blitz-dom line `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings` — run as legs of»

- leaf: .claude/docs/stack.md:39   class: stale
  leaf-text: «a cargo-deny advisory audit (`deny.toml`) and a cargo-llvm-cov coverage report with no threshold run as the `audit` and `coverage` legs.»
  source: .andromeda/architecture.md:225 «a cargo-deny audit of the advisories and licences of the whole workspace and a cargo-llvm-cov coverage report gated by per-crate floors, with the `stand-repeat` leg's zero-failure budget beside them»
  replace-with: «a cargo-deny audit of the advisories and licences of the whole workspace's graph (`deny.toml`) and a cargo-llvm-cov coverage report gated since 2026-10-14-quality-gates by per-crate line floors through `.github/scripts/coverage_floor.py` (escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %) run as the `audit` and `coverage` legs, and the `stand-repeat` leg (not a fast leg) holds the stand checks to a zero-failure budget over three rounds.»

- leaf: .claude/docs/stack.md:41   class: omission
  leaf-text: «accessibility (a11y) tests, coverage report and the windows/macos/ios/android matrix»
  source: .andromeda/architecture.md:180 «coverage report, stand checks repeated (`stand-repeat`, since 2026-10-14-quality-gates: `ubuntu-latest`, needing the fast four»
  replace-with: «accessibility (a11y) tests, coverage report, stand checks repeated (`stand-repeat`, since 2026-10-14-quality-gates) and the windows/macos/ios/android matrix»

- leaf: .claude/docs/stack.md:41   class: stale
  leaf-text: «a11y restores the test job's `workspace-test` key and never saves»
  source: .andromeda/architecture.md:180 «except `a11y` and, since 2026-10-14-quality-gates, `stand-repeat`, which restore the test job's `workspace-test` key and never save»
  replace-with: «a11y and, since 2026-10-14-quality-gates, stand-repeat restore the test job's `workspace-test` key and never save»

- leaf: .claude/docs/stack.md:42   class: omission
  leaf-text: «(`wpt_diff_to_pr.py`, upstream's `wpt_area_changes.py`, unittest suites;»
  source: .andromeda/architecture.md:66 «since 2026-10-14-quality-gates escher's `coverage_floor.py`, the coverage leg's floor, and its tests `test_coverage_floor.py`, stdlib only, no package added»
  replace-with: «(`wpt_diff_to_pr.py`, upstream's `wpt_area_changes.py`, since 2026-10-14-quality-gates escher's `coverage_floor.py` — the coverage leg's floor — and its tests `test_coverage_floor.py`, stdlib only, unittest suites;»

- leaf: .claude/docs/stack.md:42   class: stale
  leaf-text: «using the runner image's `sudo`, GNU `timeout` and `apt-get`, no package or action added.»
  source: .andromeda/architecture.md:66 «uses the runner image's `sudo`, GNU `timeout`, `apt-get` and, since 2026-10-14-quality-gates, `dpkg` — `dpkg --configure -a`, its bounded repair before a retried attempt —»
  replace-with: «using the runner image's `sudo`, GNU `timeout`, `apt-get` and, since 2026-10-14-quality-gates, `dpkg` (`dpkg --configure -a`, its bounded repair before a retried attempt), no package or action added.»

- leaf: .claude/docs/workflow.md:15   class: omission
  leaf-text: «`a6c24d24`, 2026-10-13), the next sync's merge base.»
  source: .andromeda/architecture.md:205 «upstream `main` read at `e72650a6` on 2026-10-15, three commits past it (#1198, #1212, #1211) and not merged, none of them touching the range or keyboard merge surface»
  replace-with: «`a6c24d24`, 2026-10-13), the next sync's merge base; upstream `main` was read at `e72650a6` on 2026-10-15, three commits past it (#1198, #1212, #1211) and not merged, none of them touching the range or keyboard merge surface.»

- leaf: .claude/docs/workflow.md:39   class: omission
  leaf-text: «builds, docs, dependency audit, a11y, coverage, the matrix;»
  source: .andromeda/architecture.md:225 «a cargo-deny audit of the advisories and licences of the whole workspace and a cargo-llvm-cov coverage report gated by per-crate floors, with the `stand-repeat` leg's zero-failure budget beside them»
  replace-with: «builds, docs, dependency and licence audit, a11y, coverage with its per-crate floors, the stand checks repeated three rounds at a zero-failure budget (`stand-repeat`), the matrix;»
