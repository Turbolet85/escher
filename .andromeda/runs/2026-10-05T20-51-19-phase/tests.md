# tests extract

## Relevance
relevant: the chunk reshapes the CI legs that run the workspace's tests (test-plan §9 CI Integration, §1 Surfaces under test) and is measured against the local baseline that test-plan §9 says is the reference for the fork's CI.

## Constraints
- test-plan §9 CI Integration states that tests run on PRs and on pushes to main/v0.* only. The build-branch trigger gap in scope is that platform fact, so after the change test-plan §9's platform line must be amended to name the build-branch pattern.
- test-plan §9 (Local baseline) is "the reference the fork's CI is compared against". It fixes the conditions: dev profile, workspace default features, every cargo command `--locked`, cold = first run after `cargo clean`, warm = an immediate re-run. Any CI timing or local recipe this chunk compares must use those conditions, or it must re-measure the baseline under the new ones. A reduced-debuginfo `[profile.dev]` change alters the "dev profile" condition itself.
- test-plan §1 Surfaces under test: the CI matrix runs `test --all --tests` on windows, macos and linux and `build --all` on ios and android. The fast/slow split may move these legs to slow but must not drop a platform. Only the linux legs count as host-reproducible.
- test-plan §1 Coverage scope: the CI test leg is `cargo test --workspace` with default features, and CI Python scripts are a separate leg (`python3 -m unittest discover -s .github/scripts`). Both are host-reproducible legs that need a local invocation identical to the workflow's.
- test-plan §9 (Benchmarks): `paint_tree_bench` tests are `#[ignore]` and run only with `--release … -- --ignored`. They are the 3 ignored in the baseline counts and belong outside any fast leg.
- test-plan §9 (Fonts) and §2 (Font-dependent tests): font-dependent assertions rely on `system-fonts`, which is on by default when the whole workspace is tested, and they skip at runtime with `eprintln!` when no usable font exists. Research must answer whether a narrower fast-leg invocation (per package, not `--workspace`) still enables `system-fonts` through feature unification.
- test-plan §3 Runners and invocation and §6 (WPT): WPT is `cargo build -rp wpt` then `cargo run -rp wpt css svg` with `WPT_DIR`, against a WPT checkout pinned by `./wpt/WPT_COMMIT` (§7), and reftests need a Thai font installed. These are inputs for classifying `wpt.yml` as fast, slow or excluded and for whether it can be reproduced on the host.

## Patterns to follow
- The cold/warm measurement method in test-plan §9 (Local baseline): `cargo clean`, entries run in sequence in one `target/`, then an immediate re-run. Use it to prove "cached builds", meaning a warm run does not recompile the dependency graph.
- Existing artifact upload: the WPT workflow uploads its diff artifact (test-plan §9 WPT workflow), and the runner writes `wpt_expectations.txt` and `wptreport.json` to its output directory (test-plan §3 WPT outputs). Model "failure artifacts uploaded" on this.
- Running secret-backed scripts locally: `wpt_diff_to_pr.py --dry-run` prints instead of calling the GitHub API (test-plan §8 GitHub API row). This lets a `WPT_GITHUB_TOKEN`-backed leg be reproduced on the host without the secret.
- Hang and timeout guards for loaded CI machines: the worker timeout guard and the WPT harness timeout multiplier (test-plan §9 Hang guard / WPT timeouts). Keep them when legs move to different runners or groups.

## Anti-patterns to avoid
- test-plan §11 Test Anti-Patterns records no intent, so no plan ban applies. The following cautions come from measured facts:
- Do not let a fast leg pass while font-dependent assertions silently skip (test-plan §2 Font-dependent tests, §9 Fonts). A font-skip line in the output is a regression signal, not a pass.
- Do not compare a CI warm run against the test-plan §9 baseline after a profile change without re-measuring the baseline under the changed profile.

## Contract bindings
- tests ↔ security: excluding signing-secret jobs, and how `WPT_GITHUB_TOKEN` is used in `wpt.yml`/`wpt-post-results.yml`, bind to the security plan's secrets rule. The local reproduction of the WPT post step uses the `--dry-run` path (test-plan §8).
- tests ↔ arch: a `[profile.dev]` change is workspace-wide and shifts the test-plan §9 baseline conditions. Any new env var for a local recipe is an arch §Occupied Resources registration (`WPT_DIR` already exists, per test-plan §3).
- tests ↔ obs: the test-plan §3 5-command contract (boot / run / status / cleanup / logs) is not yet measured, so this chunk has no harness ↔ obs binding. `scripts/agent-run.*` belongs to "Stand test contract".
- The keyed contract `coverage-tooling-install` (test-plan §3 Bootstrap phases) points to coverage being absent (test-plan §9 Observed absent). The coverage report belongs to "CI gate legs", not this chunk.

## Acceptance criteria contributions
- On the pushed `build/escher-0.1.0` sha, the linux test leg (`cargo test --workspace`, default features) concludes green with 0 failed. Its ignored set is only the `paint_tree_bench` tests (per test-plan §1 Coverage scope, §9 Benchmarks).
- A second CI run on a cache-warm sha shows no dependency-graph recompile in the test and build legs. Wall-clock is read and compared against test-plan §9's cold/warm split (test leg 1632.88 s / 13.08 s, dominated by the cold test-profile compile) (per test-plan §9 Local baseline).
- Every host-reproducible leg, at least the workspace test leg, fmt, clippy and `python3 -m unittest discover -s .github/scripts`, has a local invocation that runs the workflow's exact command with `--locked`, and that invocation exits 0 on the dev host (per test-plan §1 Coverage scope, §9 Local baseline).
- If the dev/test profile changes, the test-plan §9 baseline is re-measured cold and warm for its three entries and amended. `cargo test -p blitz-tests` still reads 255 passed · 0 failed · 3 ignored, with the `text_selection_anonymous_block` font-skip line absent (per test-plan §9 Local baseline).
