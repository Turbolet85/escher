group: test-plan
read: .claude/docs/tests-summary.md · 55 · «**Full plan:** `.andromeda/test-plan.md`. Path-scoped rules:»
read: .claude/rules/testing.md · 57 · «- 2026-10-10: Before calling a standing check unaffected by »
read: .claude/rules/verification-harness.md · 42 · «- 2026-10-06: A boot smoke bounded by `timeout N` exits 124»

- leaf: .claude/docs/tests-summary.md:29   class: stale
  leaf-text: «(79 CI-script tests in six files — 78 until 2026-10-13-upstream-sync-polish-and-ship, whose upstream merge added one to test_wpt_area_changes.py)»
  source: .andromeda/test-plan.md:8 «94 tests in seven test files (§4; 79 in six until 2026-10-14-quality-gates, which added test_coverage_floor.py's 7, 5 leg cases and 3 install cases;»
  replace-with: «(94 CI-script tests in seven files — 79 in six until 2026-10-14-quality-gates, which added test_coverage_floor.py's 7, 5 leg cases and 3 install cases; 78 until 2026-10-13-upstream-sync-polish-and-ship, whose upstream merge added one to test_wpt_area_changes.py)»

- leaf: .claude/docs/tests-summary.md:29   class: omission
  leaf-text: «coverage and the windows/macos/ios/android matrix»
  source: .andromeda/test-plan.md:315 «stand-repeat (since 2026-10-14-quality-gates, its job `Stand checks repeated`, not a fast leg: three rounds, each printing `stand-repeat: round R of 3`»
  replace-with: «coverage (its per-crate floor last, the leg exiting with the floor's status), stand-repeat (since 2026-10-14-quality-gates, its job `Stand checks repeated`: three rounds of the 40 `stand_*.rs` files and seven_guis' seven process checks, the first red ending the leg — no retry) and the windows/macos/ios/android matrix»

- leaf: .claude/docs/tests-summary.md:29   class: omission
  leaf-text: «the CI install script's contract against a stand-in `sudo` and `apt-get` (`InstallScriptTest`)»
  source: .andromeda/test-plan.md:145 «against a stand-in `sudo`, `apt-get` and, since 2026-10-14-quality-gates, `dpkg`»
  replace-with: «the CI install script's contract against a stand-in `sudo`, `apt-get` and, since 2026-10-14-quality-gates, `dpkg` (`InstallScriptTest`, eight tests, five until then)»

- leaf: .claude/docs/tests-summary.md:29   class: omission
  leaf-text: «up to 3 attempts, each `apt-get` call under a 120 s bound, exits 0 · 1 · 2»
  source: .andromeda/test-plan.md:319 «pausing 10 s before the next attempt, which since 2026-10-14-quality-gates first runs a bounded repair, `sudo timeout --kill-after=10 120 dpkg --configure -a` in phase `repair`»
  replace-with: «up to 3 attempts, each `apt-get` call under a 120 s bound, every attempt after the first opening since 2026-10-14-quality-gates with a bounded repair, `dpkg --configure -a` in phase `repair` (a failed repair failing that attempt), exits 0 · 1 · 2»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`escher-driver`: 72 inline unit tests in thirteen files (66 in twelve until 2026-10-12-headless-screenshot»
  source: .andromeda/test-plan.md:21 «**escher-driver:** 73 inline unit tests in thirteen files (72 until 2026-10-14-quality-gates, 66 in twelve until 2026-10-12-headless-screenshot»
  replace-with: «`escher-driver`: 73 inline unit tests in thirteen files (72 until 2026-10-14-quality-gates, 66 in twelve until 2026-10-12-headless-screenshot»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «`client.rs` 2 (a call's reply as its answer or a session error; a schema refusal answered without a session)»
  source: .andromeda/test-plan.md:21 «client.rs 3 (a call's reply reads as its answer or as a session error; a call the schema refuses is answered without a session; since 2026-10-14-quality-gates»
  replace-with: «`client.rs` 3 (a call's reply as its answer or a session error; a schema refusal answered without a session; since 2026-10-14-quality-gates, in a unix-only `client::imp::tests`, the answer's per-read bound pinned equal to `wire::ANSWER_BOUND`, 30 s, and a read past a shortened bound on a listener that never answers ending as `SessionError::Timeout`)»

- leaf: .claude/docs/tests-summary.md:24   class: stale
  leaf-text: «seven_guis' `cli_commands` (11, 10 until 2026-10-12-headless-screenshot»
  source: .andromeda/test-plan.md:200 «`cli_commands` (12, 11 until 2026-10-14-quality-gates,»
  replace-with: «seven_guis' `cli_commands` (12, 11 until 2026-10-14-quality-gates, 10 until 2026-10-12-headless-screenshot»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «witnessed on the fork's macOS CI leg)»
  source: .andromeda/test-plan.md:200 «a run with its stdout closed before it starts keeps its ending's status — `help` 0, `snapshot` with no session 3 with exactly `NoSession`'s message — printing no panic and writing nothing typed back»
  replace-with: «witnessed on the fork's macOS CI leg; since 2026-10-14-quality-gates a run with its stdout closed before it starts keeping its ending's status — `help` 0, `snapshot` with no session 3 with exactly `NoSession`'s message — printing no panic and writing nothing typed back)»

- leaf: .claude/docs/tests-summary.md:24   class: omission
  leaf-text: «(`stand_act_spans`: fixed words and counts only, no id, name or typed text, and no driver line at the default level)»
  source: .andromeda/test-plan.md:26 «since 2026-10-14-quality-gates it also reads a not-settled call: an in-file `restless_fixture` mounts a freshly keyed element on every render, and `press tab` on it reads not settled»
  replace-with: «(`stand_act_spans`: fixed words and counts only, no id, name or typed text, and no driver line at the default level; since 2026-10-14-quality-gates a not-settled call too — `press tab` on an in-file `restless_fixture` reading not settled with `Busy::Render`, its `command` line carrying `settled=false` and `busy="render"` and no `passes`)»

- leaf: .claude/docs/tests-summary.md:42   class: stale
  leaf-text: «| Coverage | no threshold — first reading 53.23 % lines (dev host), 53.25 % (CI); a floor is "Quality gates" | `ci-leg.sh coverage` (cargo-llvm-cov; `coverage-report` artifact) |»
  source: .andromeda/test-plan.md:343 «since 2026-10-14-quality-gates the `coverage` leg ends with `.github/scripts/coverage_floor.py` over three per-crate line floors — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %»
  replace-with: «| Coverage | per-crate line floors since 2026-10-14-quality-gates — `packages/escher-driver` 97 %, `packages/dioxus-native-dom` 83 %, `packages/blitz-test-harness` 90 % (read 98.34 %, 84.25 %, 91.69 % there); the workspace 67.06 % of lines at that chunk, first reading 53.23 % (dev host), 53.25 % (CI) | `ci-leg.sh coverage` (cargo-llvm-cov, then `.github/scripts/coverage_floor.py`, the leg exiting with its status; `coverage-report` artifact) |»

- leaf: .claude/docs/tests-summary.md:43   class: stale
  leaf-text: «| Flakiness budget | NOT YET MEASURED | "Quality gates" |»
  source: .andromeda/test-plan.md:344 «the `stand-repeat` leg and its job `Stand checks repeated` run the 40 `stand_*.rs` files and seven_guis' seven process checks three times with no retry»
  replace-with: «| Flakiness budget | zero failures since 2026-10-14-quality-gates — the 40 `stand_*.rs` files and seven_guis' seven process checks run three times with no retry (20 local rounds 0 failures; CI#38692629940 green 17/17) | `ci-leg.sh stand-repeat` (job `Stand checks repeated`) |»

- leaf: .claude/docs/tests-summary.md:45   class: omission
  leaf-text: «`ci-leg.sh clippy` (`cargo clippy --workspace --locked -- -D warnings`)»
  source: .andromeda/test-plan.md:315 «clippy (the workspace line, then, only when green and since 2026-10-14-quality-gates, upstream's blitz-dom line `cargo clippy -p blitz-dom --no-default-features»
  replace-with: «`ci-leg.sh clippy` (`cargo clippy --workspace --locked -- -D warnings`, then, only when green and since 2026-10-14-quality-gates, `cargo clippy -p blitz-dom --no-default-features --features svg,woff,accessibility,system-fonts,file-input,custom-widget --all-targets --locked -- -D warnings`)»

- leaf: .claude/rules/testing.md:47   class: retired
  leaf-text: «Coverage is measured with no threshold (`bash .github/scripts/ci-leg.sh coverage`, cargo-llvm-cov); a coverage floor, quality gates and the flakiness budget (§10) are owned by the working route's "Quality gates" chunk.»
  source: .andromeda/test-plan.md:346 «> NOT YET MEASURED — a performance budget: none is set»
  replace-with: «A performance budget (§10) is not set. Since 2026-10-14-quality-gates the `coverage` leg (`bash .github/scripts/ci-leg.sh coverage`, cargo-llvm-cov) ends with per-crate line floors — escher-driver 97 %, dioxus-native-dom 83 %, blitz-test-harness 90 %, by `.github/scripts/coverage_floor.py` — and the stand checks hold a zero-failure flakiness budget (`ci-leg.sh stand-repeat`, three rounds, no retry).»
