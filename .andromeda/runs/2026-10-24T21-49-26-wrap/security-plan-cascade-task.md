# Task — security-plan · 2026-10-24-a11y-re-read

## The session
The session widened the `a11y` CI leg (`.github/scripts/ci-leg.sh:39`, the `a11y)` arm, one line replaced) from six `--test` targets to fourteen. It is still one `cargo test --workspace --locked` line. The eight joining targets are `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select`, on the founder's word «Плюс проверки дерева (Рекомендую)» (2026-10-24).
The pin `LegScriptTest.test_gate_legs_dispatch_to_cargo_and_write_their_log` now names the fourteen in its a11y row, one `assertIn` per word, in no order. That row is at `.github/scripts/test_ci_workflows.py:298-302` (was `:298-299`), and every line of that file after 299 moved +3. `test_ci_workflows.py` still runs 33 tests and the `ci-scripts` leg still runs 100.
Measured: the a11y leg and the CI "Accessibility (a11y) tests" job each read 14 files, 84 passed, 0 failed, 0 ignored (was 6 · 43), on CI#40134371519 (sha `4cbf0a4c9`, green 17/17). Every other leg and CI job tally is unchanged, including workspace 206 · 1105 · 0 · 12 and stand-repeat 198 · 837 · 0 · 21.
Changed none of these: engine, stand, Rust source, `ci.yml`, manifest, `Cargo.lock`, `deny.toml`, `rust-toolchain.toml` or `packages/` (manifest probe 0). The session added no crate, dependency, env read, listener, carrier, verb, field, log site or span, and retired nothing. `--locked` stays on the a11y line.

## Reached files
- .github/scripts/ci-leg.sh
- .github/scripts/test_ci_workflows.py
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/scope.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/research.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/plan.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/inputs/andromeda-inputs.json
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/inputs/I1-a11y-reread-founder-2026-10-24.md.txt
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/counts.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/re-read.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/ci.md
- escher-0.2.0/chunks/2026-10-24-a11y-re-read/evidence/operator-pass.md

## Touches
- Line citations into `.github/scripts/test_ci_workflows.py` at any line after 299 are now off by +3 and need re-pointing. A citation at or before line 298, such as the SHA-pin assertion at `:186-192` in Dependency Security, did not move.
- Any citation of the a11y tuple in `.github/scripts/test_ci_workflows.py` at `:298-299` is now `:298-302`.
- The coverage-leg `LegScriptTest` cases cited in input-validation (the sweep's `leg-pin` row at `security-plan-detail/input-validation.md:31`): the claim is unchanged (no-change in the sweep). Any line number it gives into `test_ci_workflows.py` past 299 moved +3.
- Any statement that the a11y leg runs six files or 43 tests, or that its pin names six words in order, is now stale. It reads fourteen files, 84 tests, and a pin of fourteen words in no order.
- Lockfile verification (Supply chain integrity, and the `--locked` pinning bullet): the a11y command keeps `--locked`, and its range `.github/scripts/ci-leg.sh:20-65` still holds because `ci-leg.sh`'s line count did not change. Re-read it and confirm; nothing moved.
- Any count of the a11y leg's targets, or any list of which blitz-tests files the a11y leg runs, now reads fourteen.
- Test witnesses named in the markup-attribute rules (ARIA states, `aria-checked`, `inert`, the select's typeahead and choice): `accessibility_states`, `accessibility_label`, `accessibility_description`, `accessibility_merged_states`, `accessibility_active_descendant_unread`, `inert_subtree`, `select_interaction` and `stand_act_select` now also run in the `a11y` CI leg. Each keeps every leg it ran in before: workspace `test`, `stand-repeat`, and `run stand` for `stand_act_select`. A claim that one of them runs outside the a11y leg is stale.
- `focus_within`, `focus_redraw`, `hit_test_dropped_nodes` and `accessibility_names` stay outside the a11y leg. A claim that puts them outside it holds, but any "the leg's six" count beside it is stale.
- Per-job CI tallies, if any are stated: the a11y job now reads 14 · 84 · 0 · 0 on CI#40134371519. Every other job is unchanged from CI#40125511019.
- The CI pipeline's a11y job: its slow tier, `needs`, `workspace-test` restore with `save-if: false`, failure-only log upload and lack of a required status check are all unchanged, with `ci.yml` byte-identical. A rule that states them still holds.
- `test_ci_workflows.py`'s own test count: 33, unchanged. The `ci-scripts` leg: 100, unchanged.
- Two `a11y-tally` sweep rows in security-plan, `security-plan-detail/authentication-authorization.md:11` and `security-plan-detail/secret-management.md:25` ("the 17 listed s06 files"), are false matches. They are not about the a11y leg; no change.
- The `owner-generic` sweep row `security-plan-detail/input-validation.md:197` (the actionable-key check owned by another entry): no change.

## New facts
- (none)

## Untouched
- The driver's wire, socket, MCP server, recording, kept checks and replay: no Rust source, carrier, verb, argument kind, field, cause, session error or bound changed.
- The audit tool and `deny.toml` (the ignores by ID, the licence gate): the manifest probe is 0, and no advisory or ignore moved.
- Dependency pins, the dependency graph and `Cargo.lock`: no package entered or left. The stand graph still holds neither `accesskit_xplat` nor `accesskit_winit`.
- Secret Management's environment values and its secret storage: no env read was added and no secret was reached.
- Logging & Monitoring and escher's sink (the allowlist, scrub sets and span fields): no log site, field or span was added, and the silence census reads 0.
- Threat Model attack-surface vectors and trust boundaries: no new surface. The a11y leg change is CI selection only.
- The engine's markup-attribute reads (ARIA, `inert`, range, select, keyboard activation): no engine code changed. Only the CI leg that runs their checks changed.
- Error Handling's typed errors, panic paths and graceful degradation: no code changed.
- Data Protection: no medium, file mode or committed check changed.
- API Security (outbound requests, `LOAD_WAIT`, the local socket bounds): untouched.
- ci.yml's action pins and the audit job's tier and placement: `ci.yml` is byte-identical.
