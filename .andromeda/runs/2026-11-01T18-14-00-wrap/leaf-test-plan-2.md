group: test-plan-2
read: .claude/rules/testing.md · 62 · «- 2026-10-31: `git grep -l -F 'mod fixtures;'` over `tests/b»
read: .claude/rules/verification-harness.md · 42 · «- 2026-10-31: State what a CI job did from that job's own lo»
- leaf: .claude/rules/testing.md:44   class: stale
  leaf-text: «`bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → CI scripts); no local gate builds `blitz-paint/font-embolden`, so a paint change under that OS-gated feature has its local witness in `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`»
  source: .andromeda/test-plan.md:391 «`bash .github/scripts/ci-leg.sh fast` — fmt → clippy → test → ci-scripts → embolden — `embolden` the `blitz-paint/font-embolden` witness»
  replace-with: «`bash .github/scripts/ci-leg.sh fast` (fmt → clippy → test → CI scripts → embolden, stopping at the first red); a paint change under the OS-gated `blitz-paint/font-embolden` has its local witness in `embolden`, `cargo test -p blitz-tests --locked --no-fail-fast --features blitz-paint/font-embolden --test select_popup_painted`, a leg with no CI job of its own, macOS exercising the feature there»
- leaf: .claude/rules/testing.md:48   class: stale
  leaf-text: «(dioxus-native-dom 2791/3240 = 86.14 %, blitz-test-harness 968/1008 = 96.03 %)»
  source: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:17 «dioxus-native-dom 2791/3241 = 86.12 %, blitz-test-harness 999/1040 = 96.06 %, those two inferred from counts, not from the lcov»
  replace-with: «(dioxus-native-dom 2791/3241 = 86.12 %, blitz-test-harness 999/1040 = 96.06 %, those two inferred from counts, not from the lcov)»
- leaf: .claude/rules/testing.md:34   class: omission
  leaf-text: «test URLs use the `.test` TLD or `example.com`.»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:50 «each bind a `std::net::TcpListener` on `127.0.0.1:0`, test-only TCP on loopback, served by no binary and reached by no caller»
  replace-with: «test URLs use the `.test` TLD or `example.com`, while blitz-net's `delivers_network_and_bad_url_errors` and `http_error_body_is_not_delivered_as_css` each bind a `std::net::TcpListener` on `127.0.0.1:0`, test-only TCP on loopback, served by no binary and reached by no caller, beside the module's test-local `Handler`.»
- leaf: .claude/rules/verification-harness.md:32   class: omission
  leaf-text: «one still loading at the bound `NotSettled(Loads)`»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:5 «one still loading at the bound `NotSettled(Loads)`, a failed load ending the wait where it would otherwise have run to `LOAD_WAIT`, `settle_waiting` unedited»
  replace-with: «one still loading at the bound `NotSettled(Loads)`, a failed load ending the wait where it would otherwise have run to `LOAD_WAIT` (the crate-private load counter forwards `NetHandler::error`, so a failed stylesheet stops counting as pending and a harness document does not read `NotSettled { busy: Loads }` for it)»
- leaf: .claude/rules/verification-harness.md:37   class: omission
  leaf-text: «`None` with no text or where an element on the walk has `opacity` below 1, a filter or a background image»
  source: .andromeda/test-plan-detail/test-strategy.md:8 «`None` with no text or where an element on the walk has `opacity` below 1, a filter, a background image or a `mix-blend-mode` other than `normal`»
  replace-with: «`None` with no text or where an element on the walk has `opacity` below 1, a filter, a background image or a `mix-blend-mode` other than `normal`»
