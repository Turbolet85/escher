group: architecture+security-plan+test-plan-1
read: .claude/docs/services/escher-driver.md · 63 · «- The command line, through the real `escher-session` binary»

- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «35 calls per mode, since 2026-10-20-large-screen-in-parts one `snapshot` with `part` 1 among them, its line `verb="snapshot"` alone»
  source: .andromeda/test-plan-detail/test-scope-summary.md:109 «36 calls per mode — one `snapshot` with `part` 1 and, on the shared `many_items` page held as `paged`, one with `part` 2 read as `Outcome::Part` with `parts` at least 2, among them, each line»
  replace-with: «36 calls per mode, since 2026-10-20-large-screen-in-parts one `snapshot` with `part` 1 and, on the shared `many_items` page, one with `part` 2 read as `Outcome::Part` with `parts` at least 2 among them, each line `verb="snapshot"` alone, the page's 400 `item-NNN` ids and its buttons' names joining the needles»

- leaf: .claude/docs/services/escher-driver.md:61   class: stale
  leaf-text: «`stand_requirement_sweep` (2 tests in both layout modes»
  source: .andromeda/test-plan-detail/test-scope-summary.md:118 «`stand_requirement_sweep` 3, a stand check `run stand` picks up by its prefix, declaring `mod common;` and `mod session_common;` — through `Session::run` alone, in both layout modes, on each lean task»
  replace-with: «`stand_requirement_sweep` (3 tests in both layout modes»

- leaf: .claude/docs/services/escher-driver.md:61   class: omission
  leaf-text: «and `WITNESSES`, 16 rows naming each requirement's witness tests)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:118 «— and `WITNESSES_020`, the 0.2.0 record: nine rows v020-01 to v020-09 in order, 46 witnesses, v020-01 and v020-02 holding `Unit` witnesses alone (six crate unit tests in `packages/blitz-dom`, `packages/escher-driver`»
  replace-with: «and `WITNESSES`, 16 rows naming each requirement's witness tests — and `WITNESSES_020`, the 0.2.0 record, nine rows and 46 witnesses, read by its third test, `every_0_2_0_requirement_names_its_witness_inside_stand_repeat`: a `Unit` witness naming a crate's unit test, run by the workspace `test` leg and outside `stand-repeat` by construction, every other witness found inside `stand-repeat` by the `stand_` prefix or by the ten named `--test`s of `ci-leg.sh`'s own text)»

- leaf: .claude/docs/services/escher-driver.md:62   class: stale
  leaf-text: «`stand_lint` (6, declaring `mod fixtures;` and `mod session_common;`, both layout modes, on `Session::lint` in process and, its sixth, on the `lint` verb»
  source: .andromeda/test-plan-detail/test-scope-summary.md:122 «`stand_lint` 7, declaring `mod fixtures;` and `mod session_common;`, in both layout modes, on `Session::lint` in process and, in one test, on the `lint` verb through `Session::run`»
  replace-with: «`stand_lint` (7, declaring `mod fixtures;` and `mod session_common;`, both layout modes, on `Session::lint` in process and, in one test, on the `lint` verb»

- leaf: .claude/docs/services/escher-driver.md:62   class: stale
  leaf-text: «a clipped or off-screen node holding text also reading `truncated` is asserted by no check)»
  source: .andromeda/test-plan-detail/test-scope-summary.md:124 «is asserted by `a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it`, in both layout modes, on its in-file page `reading_page` — `fx-cut-text`, a nowrap line past its 100 × 40 `overflow: hidden` box»
  replace-with: «a clipped or off-screen node holding text also reading `truncated` is asserted by `a_clipped_or_off_screen_node_holding_text_reads_truncated_beside_it` on its in-file page `reading_page` — `fx-cut-text` read clipped and truncated, `fx-padded-text` clipped alone, `fx-off-text` truncated with no visible part and off-screen — the `lint` verb answering the same)»

- leaf: .claude/docs/services/escher-driver.md:63   class: omission
  leaf-text: «since 2026-10-19-session-recording a second test, a recording host reading 0 of the recording needles — the kept check's names and its directory's name — on stderr)»
  source: .andromeda/registries/contracts/test-plan/session-lifecycle.md:11 «seven_guis `host_log` 3 (1 until 2026-10-19-session-recording added a recording host that logs nothing of its check; … unix only, a recording session's clients — `start crud --record`, a typed sentinel, `snapshot`, `snapshot --part 1`, `lint` and `stop`, each a run»
  replace-with: «since 2026-10-19-session-recording a second test, a recording host reading 0 of the recording needles — the kept check's names and its directory's name — on stderr; and a third, unix only, reading a recording session's clients — `start crud --record`, a typed sentinel, `snapshot`, `snapshot --part 1`, `lint` and `stop`, each a run of the binary at `RUST_LOG=trace` in `hl-client` and `hl-client-rec` — 0 id, name, sentinel, recording-path and recorded needles over their own stderr, under both builds; a paged `part` read through the host's own sink stays unmeasured, the host booting only the four lean tasks and none paging)»
