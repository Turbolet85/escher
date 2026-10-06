
## 2026-10-05-ci-gate-legs — CI gate legs, SHA-pinned actions, a read-only token, a real rustdoc gate
**Section:** §Stack and Technologies (CI/CD · Code quality) · §Conventions (Formatting and lints) · §Standard Contracts (CI contracts) · §Infrastructure Patterns (Build system · CI/CD) · §Inherited Defaults (Code quality)
**Change:**
- was a docs job running bare `cargo doc --locked`, documenting only the lib-less root package — a nominal doc gate the workspace failed (3 crates, 9 errors); now the `doc` leg runs `cargo doc --workspace --no-deps --locked` under `-D warnings`, green over every workspace library crate, the `browser` bin `blitz` `doc = false` so the lib `blitz` alone writes `target/doc/blitz/`;
- was nine linux leg jobs; now twelve — `ci-leg.sh` adds `audit` (`cargo deny --locked check advisories`, root `deny.toml`), `a11y` (`accessibility_hidden`, `accessibility_roles`, `focusability_updates`) and `coverage` (`cargo llvm-cov` lcov to `target/coverage/lcov.info`, then the per-file report, no threshold); each new job in the slow tier with its failure-only `ci-log-{job id}`; the docs job installs `libfontconfig1-dev`;
- the `coverage` job also uploads `coverage-report` (`target/coverage/`, on success, 7 days) — the one fork-CI artifact outside `target/ci-logs/`;
- was tag-pinned actions; now every ci.yml `uses:` is pinned to a 40-hex commit SHA with its ref as a trailing comment, each `dtolnay/rust-toolchain` step naming its `toolchain`; CI installs cargo-deny 0.20.2 and cargo-llvm-cov 0.9.1 through `taiki-e/install-action`;
- ci.yml declares a workflow-level `permissions: contents: read`, no job-level grant;
- was rust-cache on every compiling job; now every compiling job but `coverage`, `a11y` restore-only on the test job's key.
**Why:** the CI gate legs chunk landed the audit, a11y and coverage legs, the pins and the read-only token, and discharged the as-built baseline's CARRY (a real rustdoc gate). The `coverage-report` upload is a boundary widening recorded PROVISIONAL — delegate overseer, 2026-10-05, under the founder's standing delegation of technical decisions (relayed verbatim by overseer); the founder's own later word supersedes it. A pinned `dtolnay/rust-toolchain` loses the toolchain its `@stable` branch name selected — every pinned step names one.
**Supersedes:** 2026-10-05-as-built-baseline — the rustdoc doc gate reaches no library crate
**Ref:** .andromeda/runs/2026-10-05T23-50-17-wrap/

