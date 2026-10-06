
## 2026-10-05-ci-gate-legs — shared test cache, the coverage path, the cache budget re-measured over
**Section:** §Occupied Resources (Filesystem · CI infrastructure) · every section citing `ci.yml`, `ci-leg.sh` or `test_ci_workflows.py` lines
**Change:**
- Filesystem registers the coverage leg's `target/coverage/lcov.info`, uploaded on success as `coverage-report`; `target/ci-logs/` stays the failure artifacts' only path;
- CI infrastructure was "12 entries, ≈ 9.73 GB of the 10 GB budget, one rust cache per compiling ci.yml job (11)", per "2026-10-05-fork-ci-reached — upstream-only signing, WPT and dispatch; the fork's cache budget"; now the test job saves under `shared-key: workspace-test`, which `a11y` restores with `save-if: false`, and `audit` and `coverage` carry no cache; after the first run on a re-keyed lockfile the cache held 12 entries, 10 723 071 252 B — over the budget — the `workspace-test` and clippy entries already evicted while two pre-re-key matrix entries (≈ 2.16 GB) stood;
- 70 citations on 28 lines re-pointed by a measured line map (ci.yml +4 … +91, ci-leg.sh +7 after its doc arm); no claim text changed by the re-point.
**Why:** a `Cargo.lock` change re-mints every rust-cache key and evicts until the old keys age out — over budget, a new cache entry or a lockfile/toolchain change starves the legs that restore last.
**Kept:** `deny.toml` is not registered under Filesystem — a repository config file is below the registry's grain; Build system names it.
**Ref:** .andromeda/runs/2026-10-05T23-50-17-wrap/
