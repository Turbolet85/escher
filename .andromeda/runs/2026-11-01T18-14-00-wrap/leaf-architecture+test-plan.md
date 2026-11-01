group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»
- leaf: .claude/docs/commands.md:13   class: stale
  leaf-text: «fmt → clippy → test → ci-scripts, stops at the first red»
  source: .andromeda/architecture-detail/standard-contracts.md:343 «`fast` (fmt → clippy → test → ci-scripts → embolden, stopping at the first red: the local pre-push gate)»
  replace-with: «fmt → clippy → test → ci-scripts → embolden, stops at the first red»
- leaf: .claude/docs/commands.md:14   class: omission
  leaf-text: «`bash .github/scripts/ci-leg.sh {leg}` — one CI leg exactly as CI runs it:»
  source: .andromeda/architecture-detail/standard-contracts.md:340 «`.github/scripts/ci-leg.sh {leg}` — legs `embolden` (the last step of `fast`, below) · `fmt` · `clippy`»
  replace-with: «`bash .github/scripts/ci-leg.sh {leg}` — one leg: `embolden` (the `blitz-paint/font-embolden` witness, the last step of `fast`, with no CI job) or one CI leg exactly as CI runs it:»
- leaf: .claude/docs/commands.md:29   class: stale
  leaf-text: «the local witness for a paint change under `blitz-paint/font-embolden`, which no local gate builds and CI's macOS leg alone exercises, reading on linux»
  source: .andromeda/test-plan-detail/ci-integration.md:46 «a paint change under that feature has its local witness in the `embolden` step of `fast`, its last (§9 The `embolden` leg)»
  replace-with: «the local witness for a paint change under `blitz-paint/font-embolden`, the command of the `embolden` leg, the last step of `ci-leg.sh fast`, with no CI job, macOS exercising the feature in CI, reading on linux»
- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «47,949 bytes in 740 lines as read before `cells-grid` took its `id` and `aria_label`, no later reading claimed, and 5 parts»
  source: .andromeda/test-plan-detail/test-data-fixtures.md:27 «booted through `stand::boot_cells`: 47,089 bytes in 740 lines and 5 parts of `SNAPSHOT_TEXT_BUDGET`, read through the binary»
  replace-with: «47,089 bytes in 740 lines and 5 parts»
- leaf: .claude/docs/commands.md:62   class: omission
  leaf-text: «run by the workspace `test` leg, `run all`, by name and as named `--test`s of the `stand-repeat` leg, not by `run stand` or the `a11y` leg»
  source: .andromeda/test-plan-detail/test-scope-summary.md:283 «a named `--test` of the `stand-repeat` leg, and, with `--features blitz-paint/font-embolden`, as the `embolden` leg, the last step of `fast`, with no CI job»
  replace-with: «run by the workspace `test` leg, `run all`, by name and as named `--test`s of the `stand-repeat` leg, not by `run stand` or the `a11y` leg, `select_popup_painted` also, with `--features blitz-paint/font-embolden`, as the `embolden` leg, the last step of `fast`, with no CI job»
- leaf: .claude/docs/commands.md:63   class: omission
  leaf-text: «firing `input` and no `change`, 2 tests»
  source: .andromeda/test-plan-detail/ci-integration.md:28 «it holds no `stand_` prefix and is none of `stand-repeat`'s seventeen named `--test` words, so neither `stand-repeat` nor `run stand` runs it»
  replace-with: «firing `input` and no `change`, 2 tests; and `cargo test -p blitz-tests --locked --test key_press_after_container_lost_its_box` — a key press with focus on a control whose anonymous block was dropped, by `hidden`, `style="display:none"` or CSS alone, not panicking, a select's choice still moving past the dropped block and a control in no anonymous block reading the same, 3 tests, both layout modes (since 2026-11-01-upstream-sync-ui-graph; run by the workspace `test` leg and `run all`, not by `run stand`, the `stand-repeat` leg or the `a11y` leg)»
- leaf: .claude/docs/services/blitz-test-harness.md:15   class: stale
  leaf-text: «whose `filter` list is not empty or whose `background-image` holds a layer other than `none`;»
  source: .andromeda/architecture-detail/standard-contracts.md:144 «whose `filter` list is not empty, whose `background-image` holds a layer other than `none` or whose `mix-blend-mode` is other than `normal`»
  replace-with: «whose `filter` list is not empty, whose `background-image` holds a layer other than `none` or whose `mix-blend-mode` is other than `normal`;»
- leaf: .claude/docs/services/blitz-test-harness.md:16   class: omission
  leaf-text: «for at most `LOAD_WAIT` (5 s of wall time), moving no harness time.»
  source: .andromeda/architecture-detail/standard-contracts.md:241 «until the load is answered, a failed load answering it, or `LOAD_WAIT` has passed»
  replace-with: «for at most `LOAD_WAIT` (5 s of wall time), a failed load answering it and ending the wait, moving no harness time.»
- leaf: .claude/docs/services/blitz-test-harness.md:21   class: omission
  leaf-text: «a provider that fetches is wrapped in a crate-private counter (counts only);»
  source: .andromeda/architecture-detail/standard-contracts.md:148 «its handler `CountedHandler` forwards `NetHandler::error`, so a failed load reaches the document on a harness or driver session and ends the wait»
  replace-with: «a provider that fetches is wrapped in a crate-private counter (it stores counts only, and its handler `CountedHandler` forwards `NetHandler::error`, so a failed load reaches the document on a harness or driver session and ends the wait);»
- leaf: .claude/docs/services/blitz-test-harness.md:34   class: stale
  leaf-text: «the crate's 11 settle unit tests (the load counter, the loop over a scripted document, the messages)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:130 «the crate holds 24, the load counter's fifth `a_failed_stylesheet_reaches_the_document_through_the_counter_and_unblocks_it` in `settle.rs`»
  replace-with: «the crate's 12 settle unit tests (the load counter, its fifth a failed stylesheet reaching the document through the counter and unblocking it, the loop over a scripted document, the messages)»
- leaf: .claude/docs/services/blitz-test-harness.md:34   class: stale
  leaf-text: «(22 in the crate since 2026-10-13-stand-contrast-harness, 17 before it;»
  source: .andromeda/test-plan-detail/test-scope-summary.md:130 «the crate holds 24»
  replace-with: «(24 in the crate since 2026-11-01-upstream-sync-ui-graph, 22 since 2026-10-13-stand-contrast-harness, 17 before it;»
- leaf: .claude/docs/services/blitz-test-harness.md:34   class: stale
  leaf-text: «its 5 contrast unit tests (since 2026-10-13-stand-contrast-harness: the WCAG 2.x ratio table, the large-text threshold table, a button reading its own text and background, a translucent ancestor composited over the page, an element with no text reading `None`)»
  source: .andromeda/test-plan-detail/unit-test-strategy.md:26 «four fixture tests: a button reads its own text and background, a translucent ancestor is composited over the page, an element with no text reads `None`, and a blended ancestor reads `None`»
  replace-with: «its 6 contrast unit tests (since 2026-10-13-stand-contrast-harness: the WCAG 2.x ratio table, the large-text threshold table, a button reading its own text and background, a translucent ancestor composited over the page, an element with no text reading `None`; since 2026-11-01-upstream-sync-ui-graph a blended ancestor reading `None` and an unblended one its pair)»
