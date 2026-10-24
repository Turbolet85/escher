group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:14   class: omission
  leaf-text: «three rounds of `cargo test -p blitz-tests --locked` over every `tests/blitz-tests/tests/stand_*.rs`, then seven_guis'»
  source: .andromeda/test-plan-detail/ci-integration.md:9 «ten named targets as literal `--test` words, `inert_subtree`, `accessibility_states`, `scrolled_box_absolute_position`, `hit_clipped_at_scrolling_box`, `password_caret_click`,»
  replace-with: «three rounds of `cargo test -p blitz-tests --locked` over every `tests/blitz-tests/tests/stand_*.rs` and, on the same line, ten named targets outside that prefix (`inert_subtree` · `accessibility_states` · `scrolled_box_absolute_position` · `hit_clipped_at_scrolling_box` · `password_caret_click` · `transformed_client_rect` · `accessibility_active_descendant_unread` · `virtual_clock_engine` · `select_interaction` · `text_extent`), then seven_guis'»

- leaf: .claude/docs/commands.md:60   class: omission
  leaf-text: «the host's stderr at `RUST_LOG=trace` (no id, no name)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:15 «and the third reading the clients of a recording session — `start crud --record` through `stop`, a part read and a `lint` among them — 0 needles of each class over their own stderr at `trace`»
  replace-with: «the host's stderr at `RUST_LOG=trace` (no id, no name; 3 tests, unix only, the third reading a recording session's clients — `start crud --record` through `stop`, a part read and a `lint` among them — 0 needles over their own stderr at `trace`)»

- leaf: .claude/docs/commands.md:61   class: stale
  leaf-text: «`stand_lint`, 6 tests, on `Session::lint` in process and, its sixth, on the `lint` verb through `Session::run`»
  source: .andromeda/test-plan-detail/test-scope-summary.md:122 «`stand_lint` 7, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on `Session::lint` in process and, in one test, on the `lint` verb through `Session::run`»
  replace-with: «`stand_lint`, 7 tests, on `Session::lint` in process and, in one test, on the `lint` verb through `Session::run`»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «and an app with no font refused `fonts-unavailable`; since 2026-10-20-load-wait»
  source: .andromeda/test-plan-detail/test-scope-summary.md:124 «a clipped or off-screen node holding text also reading `truncated` is asserted by `a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it`, in both layout modes»
  replace-with: «and an app with no font refused `fonts-unavailable`, and (since 2026-10-24-stand-requirement-sweep) a clipped or off-screen node holding text reading `truncated` beside it on an in-file page, the `lint` verb answering the same; since 2026-10-20-load-wait»

- leaf: .claude/docs/commands.md:62   class: omission
  leaf-text: «run by the workspace `test` leg, by name and by `run all`, not by `run stand`)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:125 «(run by the workspace `test` leg, the `stand-repeat` leg — a named `--test` of its list —, by name and by `run all`, not by `run stand`)»
  replace-with: «run by the workspace `test` leg, the `stand-repeat` leg (a named `--test` of its list), by name and by `run all`, not by `run stand`)»

- leaf: .claude/docs/commands.md:63   class: stale
  leaf-text: «3 tests, both layout modes, on HTML pages booted with the stand's options»
  source: .andromeda/test-plan-detail/test-scope-summary.md:22 «by blitz-tests' `text_extent` 6, both layout modes, on HTML pages booted with the stand's options»
  replace-with: «6 tests, both layout modes, on HTML pages booted with the stand's options»

- leaf: .claude/docs/commands.md:63   class: omission
  leaf-text: «a wrapped paragraph inside its box, and an element with no inline content or a stale id reading `None`»
  source: .andromeda/test-plan-detail/test-scope-summary.md:23 «its stated limits are asserted by three of those tests under `-p blitz-tests` and `--workspace` — RTL text read as a left-to-right line from the content box»
  replace-with: «a wrapped paragraph inside its box, an element with no inline content or a stale id reading `None`, and its three stated limits — RTL text read as a left-to-right line, vertical text as a horizontal line, a transformed box's lines at its client rect»

- leaf: .claude/docs/commands.md:63   class: omission
  leaf-text: «(run by the workspace `test` leg, `run text_extent` and `run all`, not by `run stand`)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:22 «its own auto-discovered target, run by the workspace `test` leg, the `stand-repeat` leg (a named `--test` of its list), `run text_extent` and `run all`, not by `run stand`;»
  replace-with: «(run by the workspace `test` leg, the `stand-repeat` leg (a named `--test` of its list), `run text_extent` and `run all`, not by `run stand`)»
