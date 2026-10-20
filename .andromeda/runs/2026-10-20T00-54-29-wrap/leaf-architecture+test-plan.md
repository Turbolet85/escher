group: architecture+test-plan
read: .claude/docs/commands.md · 82 · «- Fonts missing in headless tests → `system-fonts` feature /»
read: .claude/docs/services/blitz-test-harness.md · 37 · «- `.claude/rules/verification-harness.md` · `.claude/docs/te»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «--test stand_replay` · `cargo test -p seven_guis --locked --lib stand::`»
  source: .andromeda/test-plan.md:27 «`stand_lint` 5, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on the in-process `Session::lint` (PROVISIONAL: in process only): a planted page (`lint_page`, tests/blitz-te»
  replace-with: «--test stand_replay --test stand_lint` · `cargo test -p seven_guis --locked --lib stand::`»

- leaf: .claude/docs/commands.md:61   class: omission
  leaf-text: «and a check on a session of another app running nothing)»
  source: .andromeda/test-plan.md:27 «`stand_lint` 5, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on the in-process `Session::lint` (PROVISIONAL: in process only): a planted page (`lint_page`, tests/blitz-te»
  replace-with: «and a check on a session of another app running nothing; since 2026-10-19-visual-lint `stand_lint`, 5 tests, on the in-process `Session::lint` (PROVISIONAL: in process only, no verb): a planted page reading exactly its seven findings in document order with their reasons and its negatives nothing, each lean task at boot 0 findings, the lint reading and changing nothing, the last CRUD row after twelve Creates `clipped` inside `crud-list` with its `click` refused `off-screen`, and an app with no font refused `fonts-unavailable`; picked up by `run stand` and the stand-repeat leg by its prefix)»

- leaf: .claude/docs/commands.md:63   class: omission
  leaf-text: «and a document built with no clock fading by the wall clock (not selected by `run stand`)»
  source: .andromeda/test-plan.md:12 «since 2026-10-19-visual-lint the read-only reader `BaseDocument::text_extent` (`TextExtent { content, text }`), by blitz-tests' `text_extent` 3, both layout modes, on HTML pages booted with the stand'»
  replace-with: «and a document built with no clock fading by the wall clock (not selected by `run stand`); and `cargo test -p blitz-tests --locked --test text_extent` — the read-only reader `BaseDocument::text_extent` (since 2026-10-19-visual-lint), 3 tests, both layout modes, on HTML pages booted with the stand's options: a line that cannot wrap reading wider than its box, a wrapped paragraph inside its box, and an element with no inline content or a stale id reading `None` (run by the workspace `test` leg, `run text_extent` and `run all`, not by `run stand`)»

- leaf: .claude/docs/services/blitz-test-harness.md:16   class: omission
  leaf-text: «then hands `ms` to the session's time step where one was handed (since 2026-10-18-one-virtual-clock, ratified by the founder, 2026-10-18).»
  source: .andromeda/architecture.md:198 «`contrast`, a node with direct non-whitespace text or a text control, read through `Harness::text_contrast` and reported below `required_ratio()`, exempt at `enabled == Some(false)`, `None` not measur»
  replace-with: «then hands `ms` to the session's time step where one was handed (since 2026-10-18-one-virtual-clock, ratified by the founder, 2026-10-18), and whose in-process `Session::lint` (since 2026-10-19-visual-lint, PROVISIONAL) reads its `contrast` defect through `Harness::text_contrast`, reported below `required_ratio()`, exempt at `enabled == Some(false)`, `None` not measured.»
